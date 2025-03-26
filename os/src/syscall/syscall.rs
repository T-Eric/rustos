// 管理应用运行的syscall

// // used batch.rs
// pub fn sys_exit(exit_code: i32) -> ! {
//   println_info!(Log::Info, "[kernel] App exit with code {}", exit_code);
//   run_next_app();
// }

use crate::console::Log;
use crate::task::{exit_cur_and_run_next, suspend_cur_and_run_next};
use crate::tesbi::timer::get_time_ms;

// use task
pub fn sys_yield() -> isize {
    suspend_cur_and_run_next();
    0
}

pub fn sys_exit(exit_code: i32) -> ! {
    println_info!(Log::Info, "[kernel] App exit with code {}", exit_code);
    exit_cur_and_run_next();
    panic!("unreachable in sys_exit!");
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
