use super::uart::UART;
use crate::tesbi::PASS;

pub fn console_putchar(c: u8) {
    UART.putchar(c);
}

pub fn shutdown() -> ! {
    unsafe {
        (super::VIRT_TEST_ADDRESS as *mut u32).write_volatile(PASS as u32);
        unreachable!()
    }
}
