use crate::config::*;
use crate::trap::TrapContext;
use core::arch::asm;

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_MAX_SIZE
}

pub fn get_num_app() -> usize {
    unsafe extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

// 与load_app不同，一次性全部加载
pub fn load_apps() {
    unsafe extern "C" {
        fn _num_app();
    }
    let num_app = get_num_app();
    let num_app_ptr = _num_app as usize as *const usize;
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    unsafe {
        for i in 0..num_app {
            let base_i = get_base_i(i); //目标加载地址
            core::slice::from_raw_parts_mut(base_i as *mut u8, APP_MAX_SIZE).fill(0);
            let app_src = core::slice::from_raw_parts(
                app_start[i] as *const u8,
                app_start[i + 1] - app_start[i],
            );
            let app_dst = core::slice::from_raw_parts_mut(base_i as *mut u8, app_src.len());
            app_dst.copy_from_slice(app_src);
        }
        asm!("fence.i");
    }
}

pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
        get_base_i(app_id),
        USER_STACK[app_id].get_sp(),
    ))
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, trap_cx: TrapContext) -> usize {
        let trap_cx_ptr = (self.get_sp() - size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *trap_cx_ptr = trap_cx;
        }
        trap_cx_ptr as usize
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
