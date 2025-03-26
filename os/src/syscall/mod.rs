// 实现call类型指令

use crate::syscall::fs::sys_write;
use crate::syscall::syscall::sys_exit;

mod fs;
pub mod syscall;

const _SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

// 总接口
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => syscall::sys_yield(),
        SYSCALL_GET_TIME => syscall::sys_get_time(),
        _ => panic!("Unsupported syscall id: {}", syscall_id),
    }
}
