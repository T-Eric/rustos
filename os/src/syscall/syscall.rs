// 管理应用运行的syscall

use crate::batch::run_next_app;
use crate::Log;

pub fn sys_exit(exit_code: i32) -> ! {
    println_info!(Log::Info, "[kernel] App exit with code {}", exit_code);
    run_next_app();
}
