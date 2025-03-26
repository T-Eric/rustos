pub mod context;

use core::arch::{asm, global_asm};
use riscv::interrupt::supervisor::Exception;
use riscv::interrupt::Interrupt;
use riscv::register::{
    scause::{self, Trap},
    stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(stvec::Stvec::from_bits(__alltraps as usize));
        // stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(tc: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    let cause: Trap<Interrupt, Exception> = scause.cause().try_into().unwrap();
    match cause {
        Trap::Exception(Exception::UserEnvCall) => {
            tc.sepc += 4; // sepc是user ecall位置，此增加可返回原位置继续执行
            tc.x[10] = syscall(tc.x[17], [tc.x[10], tc.x[11], tc.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error_info!("[kernel] PageFault in app, kernel had to kill it.");
            exit_cur_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error_info!(
                "[kernel] Illegal instruction appeared in app, kernel had no idea but to kill it."
            );
            exit_cur_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_cur_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorSoft) => {
            // 又要我自己实现！？
            let sip = riscv::register::sip::read().bits();
            unsafe {
                asm!("csrw sip, {sip}", sip = in(reg) sip ^ 2);
            }
            set_next_trigger();
            // suspend_cur_and_run_next();
            // todo 这个东西还不知道怎么实现，recore上的看不懂
        }
        _ => {
            panic!(
                "Unsupported trap {:?} whose stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    tc
}

use crate::console::Log;
use crate::syscall::syscall;
use crate::task::{exit_cur_and_run_next, suspend_cur_and_run_next};
use crate::tesbi::timer::set_next_trigger;
pub use context::TrapContext;
