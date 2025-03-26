use crate::config::{CLOCK_FREQ, TICKS_PER_SEC};
// 时钟中断处理函数，也是启用时钟中断后的初始化占位符
use crate::tesbi::{HART_ID, MTIMECMP_BASE, MTIME_BASE};
use riscv::register::*;
// # 时钟中断处理函数
// 读取mscratch获取mtimecmp；
// 设置下一个中断时间为当前时间+interval，写入mtimecmp.
// 要以汇编函数进入！不能直接用这个函数的地址。
// 所以直接借鉴recore的写法，直接在asm中写入

// #[no_mangle]
// pub extern "C" fn clock_interrupt_handler() {
//     // set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
//     unsafe {
//         HART_ID = mhartid::read(); // 不知道这一读有没有更新的作用
//         let mtimecmp_addr = MTIMECMP_BASE + 8 * HART_ID;
//         let mtime = (MTIME_BASE as *mut u64).read_volatile() as usize;
//         (mtimecmp_addr as *mut u64).write_volatile((mtime + CLOCK_FREQ / TICKS_PER_SEC) as u64);
//     }
// }

// todo 这个time::read是否与mtime是同一个东西？
// pub fn get_time() -> usize {
//     time::read()
// }
pub fn get_time() -> usize {
    unsafe { (MTIME_BASE as *const usize).read_volatile() }
}

pub fn get_time_cmp_addr() -> usize {
    unsafe { MTIMECMP_BASE + 8 * HART_ID }
}

// 设置下一个时钟中断，即mtimecmp
pub fn set_timer(time: usize) {
    unsafe {
        (get_time_cmp_addr() as *mut usize).write_volatile(time);
    }
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / 1000)
}
