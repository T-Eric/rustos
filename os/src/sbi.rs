
/*
不对！需要自己实现sbi，不能用sbi_rt
实现一系列sbi_call,往uart中读写mmio地址
*/

const UART_MMIO_REG_BASE:usize=0x10000000;


// ----------------------------------------------------------------

pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
