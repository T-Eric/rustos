pub mod sbi;
pub mod uart;

pub const VIRT_UART0_ADDRESS: usize = 0x10000000;
pub const VIRT_TEST_ADDRESS: usize = 0x100000; // 关机地址

// 关机码，用法：
//     #[inline]
//     pub fn fail(&self, code: u16) -> ! {
//         self.write(FAIL as u32 | (code as u32) << 16)
//     }
// #[inline]
// pub fn pass(&self) -> ! {
//   self.write(PASS(or RESET) as _)
// }

pub const FAIL: u16 = 0x3333;
pub const PASS: u16 = 0x5555;
pub const RESET: u16 = 0x7777;
