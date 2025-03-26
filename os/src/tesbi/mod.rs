pub(crate) mod init;
pub mod sbi;
pub mod timer;
pub mod uart;

pub const VIRT_UART0_ADDRESS: usize = 0x10000000;
pub const VIRT_TEST_ADDRESS: usize = 0x100000; // 关机地址
const MTIME_BASE: usize = 0x0200bff8;
const MTIMECMP_BASE: usize = 0x02004000;

#[link_section = ".bss.stack"]
#[no_mangle]
static mut HART_ID: usize = 0;

// 关机码用法：
//     #[inline]
//     pub fn fail(&self, code: u16) -> ! {
//         self.write(FAIL as u32 | (code as u32) << 16)
//     }
// #[inline]
// pub fn pass(&self) -> ! {
//   self.write(PASS(or RESET) as _)
// }

pub const _FAIL: u16 = 0x3333;
pub const PASS: u16 = 0x5555;
pub const _RESET: u16 = 0x7777;
