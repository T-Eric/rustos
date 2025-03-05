use crate::sync::UpSafeCell;
use crate::trap::TrapContext;
use crate::Log;
use core::arch::asm;
use lazy_static::lazy_static;

const MAX_APP_NUM: usize = 16; // 不支持多道。。。
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const USER_STACK_SIZE: usize = 4096 * 2;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_MAX_SIZE: usize = 0x20000;

// 保存应用数量和各自的位置信息，以及当前执行到第几个应用了。
// 根据应用程序位置信息，初始化好应用所需内存空间，并加载应用执行。
struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        println_info!(Log::Info, "[kernel] num of apps: {}", self.num_app);
        for i in 0..self.num_app {
            println_info!(
                Log::Info,
                "[kernel] app_{} in address: [{:#x},{:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    fn get_cur_app(&self) -> usize {
        self.current_app
    }

    fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    fn load_app(&self, id: usize) {
        if id >= self.num_app {
            panic!("All applications completed")
        }
        println_info!(Log::Info, "[kernel] Loading app_{}", id);

        unsafe {
            core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_MAX_SIZE).fill(0);

            // from app's home to 0x80400000 dest
            let app_src = core::slice::from_raw_parts(
                self.app_start[id] as *const u8,
                self.app_start[id + 1] - self.app_start[id],
            );
            let app_dst =
                core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
            app_dst.copy_from_slice(app_src);
            // memory fence to sync i-cache and mem
            asm!("fence.i");
        }
    }
}

// AppManager的全局可借用实例，使用lazy_static先读取app信息再建立
lazy_static! {
    static ref APP_MANAGER: UpSafeCell<AppManager> = unsafe { UpSafeCell::new({
        extern "C" { fn _num_app(); }
        let num_app_ptr = _num_app as usize as *const usize;// _num_app是link_app.S
        let num_app = num_app_ptr.read_volatile();
        let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
        let app_start_raw: &[usize] =  core::slice::from_raw_parts(
            num_app_ptr.add(1), num_app + 1
        );
        app_start[..=num_app].copy_from_slice(app_start_raw);
        AppManager {
            num_app,
            current_app: 0,
            app_start,
        }
    })};
}

// AppManager初始化
pub fn app_manager_init() {
    // lazy_static::initialize(&APP_MANAGER);
    APP_MANAGER.exclusive_access().print_app_info();
}

// run next app
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let cur_app = app_manager.get_cur_app();
    app_manager.load_app(cur_app);
    app_manager.move_to_next_app();
    drop(app_manager); //
    unsafe extern "C" {
        fn __restore(tc_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Reached unreachable code in batch::run_current_app!")
}

#[repr(align(4096))] // 保证align而不会中间断开出现“利用最大化”
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    // push trap context into kernel stack
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext; // ?
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
