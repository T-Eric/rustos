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
    // TODO 目前是在 M mode 执行的关机，后面还要涉及到一个中断转换机制
    // TODO https://blog.kuangjux.top/2021/04/14/shutdown-without-SBI-in-OS/
}
