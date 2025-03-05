use super::clock_interrupt::{clock_interrupt_handler, INTERVAL};
use super::{MTIMECMP_BASE, MTIME_BASE};
use crate::tesbi::uart::UART;
use riscv::register::mstatus::MPP;
use riscv::register::*;

pub fn mode_init() {
    unsafe {
        mstatus::set_mpp(MPP::Supervisor);
        mepc::write(0x80000014); // 这一步在entry.asm中做
                                 // 结果没能直接跳到0x80200000，而是先跳到mret的下一步，再call
                                 // 否则要自己配置sp和ra，可能出现异常

        // 暂时禁用页表，如果需要使用虚拟地址则后面手动重启
        satp::write(satp::Satp::from_bits(0x0));

        // 启用中断
        medeleg::write(medeleg::Medeleg::from_bits(0xffff));
        mideleg::write(mideleg::Mideleg::from_bits(0xffff));
        sie::set_sext(); //SEIE
        sie::set_ssoft(); //SSIE
        sie::set_stimer(); //STIE

        // 配置physical mem protection
        pmpaddr0::write(0x3fffffffffffff);
        pmpcfg0::write(0xf);

        // 启用时钟中断
        let mtimecmp_addr = MTIMECMP_BASE + 8 * mhartid::read();
        let mtime = (MTIME_BASE as *mut u64).read_volatile();
        (mtimecmp_addr as *mut u64).write_volatile(mtime.wrapping_add(INTERVAL as u64));
        mscratch::write(mtimecmp_addr); // 仅传递计算好的地址，INTERVAL是约定好的
                                        // 虽然但是mscratch真不知道该怎么用，虽然以后可能也不会用就是了

        mtvec::write(mtvec::Mtvec::from_bits(clock_interrupt_handler as usize)); // 这样写需要cih的地址四字节对齐

        mstatus::set_mie();
        mie::set_mtimer();

        // 调用mret指令
        // asm!("mret"); // 也在asm中直接写
    }
}

pub fn uart_init() {
    lazy_static::initialize(&UART);
}
