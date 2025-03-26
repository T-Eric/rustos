use super::{HART_ID, MTIMECMP_BASE, MTIME_BASE};
use crate::config::{CLOCK_FREQ, TICKS_PER_SEC};
use crate::tesbi::uart::UART;
use core::arch::global_asm;
use riscv::register::mstatus::MPP;
use riscv::register::*;

global_asm!(include_str!("timer.S"));

#[link_section = ".bss.stack"]
#[no_mangle]
pub static mut TIMER_SCRATCH: [[usize; 5]; 4] = [[0; 5]; 4];

pub fn mode_init() {
    extern "C" {
        fn _clock_interrupt_handler();
    }
    unsafe {
        mstatus::set_mpp(MPP::Supervisor);
        mepc::write(0x80000014); // 这一步在entry.asm中做
                                 // 结果没能直接跳到0x80200000，而是先跳到mret的下一步，再call
                                 // 否则要自己配置sp和ra，可能出现异常

        // 暂时禁用页表，如果需要使用虚拟地址则后面手动重启
        satp::write(satp::Satp::from_bits(0x0));

        // 启用中断
        // SSI是timer通知S mode的形式，应当处理
        medeleg::write(medeleg::Medeleg::from_bits(0xffff));
        mideleg::write(mideleg::Mideleg::from_bits(0xffff));
        sie::set_sext(); //SEIE
        sie::set_ssoft(); //SSIE
        sie::set_stimer(); //STIE

        // 配置physical mem protection
        pmpaddr0::write(0x3fffffffffffffusize);
        pmpcfg0::write(0xf);

        // 启用时钟中断
        HART_ID = mhartid::read();
        let mtimecmp_addr = MTIMECMP_BASE + HART_ID;
        let mtime = (MTIME_BASE as *mut u64).read_volatile();
        (mtimecmp_addr as *mut u64)
            .write_volatile(mtime.wrapping_add((CLOCK_FREQ / TICKS_PER_SEC) as u64));

        let scratch = &mut TIMER_SCRATCH[HART_ID];
        scratch[3] = mtimecmp_addr;
        scratch[4] = CLOCK_FREQ / TICKS_PER_SEC;
        // 参考recore的实现，0~2是用来存储t0~t2的
        mscratch::write(scratch as *mut usize as usize); // 最天才的设计：直接把scratch的地址写入mscratch，想存多少就存多少

        mtvec::write(mtvec::Mtvec::from_bits(_clock_interrupt_handler as usize)); // 这样写需要cih的地址四字节对齐

        mstatus::set_mie();
        mie::set_mtimer(); // todo mtimer

        // 调用mret指令
        // asm!("mret"); // 也在asm中直接写
    }
}

pub fn uart_init() {
    lazy_static::initialize(&UART);
}
