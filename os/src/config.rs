pub const MAX_APP_NUM: usize = 16; // 不支持多道。。。
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_MAX_SIZE: usize = 0x20000;

pub const KERNEL_HEAP_SIZE: usize = 0x100000;
pub const KERNEL_HEAP_MIN: usize = 0x1000; // 4KiB
