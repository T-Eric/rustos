pub mod context;

use core::arch::global_asm;
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
        stvec::write(stvec::Stvec::from_bits(__alltraps as usize | 0b00));
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
            println_info!(
                Log::Error,
                "[kernel] PageFault in app, kernel had to kill it."
            );
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println_info!(
                Log::Error,
                "[kernel] Illegal instruction appeared in app, kernel had no idea but to kill it."
            );
            run_next_app();
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

use crate::batch::run_next_app;
use crate::console::Log;
use crate::syscall::syscall;
pub use context::TrapContext;
