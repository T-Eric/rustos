// 时钟中断处理函数，也是启用时钟中断后的初始化占位符
use crate::tesbi::MTIME_BASE;
use riscv::register::*;

pub const INTERVAL: usize = 1000000;

/// # 时钟中断处理函数
/// 读取mscratch获取mtimecmp；
/// 设置下一个中断时间为当前时间+interval，写入mtimecmp.
#[no_mangle]
pub extern "C" fn clock_interrupt_handler() {
    unsafe {
        let mtimecmp_addr = mscratch::read();
        let mtime = (MTIME_BASE as *mut u64).read_volatile();
        let next_time = mtime.wrapping_add(INTERVAL as u64);
        (mtimecmp_addr as *mut u64).write_volatile(next_time);
    }
}
