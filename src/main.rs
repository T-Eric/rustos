#![no_std]
#![no_main]
#[macro_use]
mod console;

mod lang_items;
mod sbi;
use console::Log;
use core::arch::global_asm;

// fn main() {}
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    println!("Original Hello world!");
    println_info!(Log::Error, "Error Hello world!");
    println_info!(Log::Warning, "Warning Hello world!");
    println_info!(Log::Info, "Info Hello world!");
    println_info!(Log::Debug, "Debug Hello world!");
    println_info!(Log::Trace, "Trace Hello world!");

    panic!("\x1b[31mShutdown! Bye bye dumbass!\x1b[0m");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
