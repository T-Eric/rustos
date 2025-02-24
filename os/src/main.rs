#![no_std]
#![no_main]
#[macro_use]
mod console;

mod lang_items;
mod sbi;
mod batch;
mod sync;
mod trap;
mod uart;

use console::Log;
use core::arch::global_asm;

// fn main() {}
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));// build.rs生成

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
    // (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
    unsafe{core::slice::from_raw_parts_mut(sbss as usize as *mut u8,ebss as usize-sbss as usize).fill(0)};
    // 起始位置不能直接改*mut u8，因为函数返回值不能直接用
}
