// #![feature(alloc_error_handler)]
#![no_std]
#![no_main]
// extern crate alloc;
#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod sync;
mod syscall;
mod task;
mod tesbi;
mod trap;

use crate::tesbi::{init, timer};
use console::Log;
use core::arch::global_asm;

// fn main() {}
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S")); // build.rs生成

#[no_mangle]
pub fn boot_init() {
    // 在M mode执行的初始化操作
    init::uart_init();
    info_info!("Inited uart: Now you can IO!");
    clear_bss();
    print_logo();
    init::mode_init();
    info_info!("Inited mode: Now you can switch to S mode!");
}

#[no_mangle]
#[link_section = ".text.rust_main"]
pub fn rust_main() -> ! {
    debug_info!("---Inside rust main---");
    trap::init();
    loader::load_apps();
    timer::set_next_trigger(); // set the first 10ms
    task::run_first_task();
    panic!("unreachable in rust_main!")
    // panic!("\x1b[31mShutdown! Bye bye dumbass!\x1b[0m");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0)
    };
    // 起始位置不能直接改*mut u8，因为函数返回值不能直接用
}

fn print_logo() {
    println!("\x1b[91m        ,----,\x1b[0m");
    println!("\x1b[91m      ,/   .`|\x1b[0m");
    println!("\x1b[91m    ,`   .'  :\x1b[0m \x1b[93m    ,---,.\x1b[0m \x1b[92m  .--.--.   \x1b[0m \x1b[96m    ,---,.\x1b[0m \x1b[95m    ,---,\x1b[0m");
    println!("\x1b[91m  ;    ;     /\x1b[0m \x1b[93m  ,'  .' |\x1b[0m \x1b[92m /  /    '. \x1b[0m \x1b[96m  ,'  .'  \\\x1b[0m \x1b[95m,`--.' |\x1b[0m");
    println!("\x1b[91m.'___,/    ,' \x1b[0m \x1b[93m,---.'   |\x1b[0m \x1b[92m|  :  /`. / \x1b[0m \x1b[96m,---.' .' |\x1b[0m \x1b[95m|   :  :\x1b[0m");
    println!("\x1b[91m|    :     |  \x1b[0m \x1b[93m|   |   .'\x1b[0m \x1b[92m;  |  |--`  \x1b[0m \x1b[96m|   |  |: |\x1b[0m \x1b[95m:   |  '\x1b[0m");
    println!("\x1b[91m;    |.';  ;  \x1b[0m \x1b[93m:   :  |-,\x1b[0m \x1b[92m|  :  ;_    \x1b[0m \x1b[96m:   :  :  /\x1b[0m \x1b[95m|   :  |\x1b[0m");
    println!("\x1b[91m`----'  |  |  \x1b[0m \x1b[93m:   |  ;/|\x1b[0m \x1b[92m \\  \\    `. \x1b[0m \x1b[96m:   |    ;\x1b[0m \x1b[95m '   '  ;\x1b[0m");
    println!("\x1b[91m    '   :  ;  \x1b[0m \x1b[93m|   :   .'\x1b[0m \x1b[92m  `----.   \\\x1b[0m \x1b[96m|   :     \\\x1b[0m \x1b[95m|   |  |\x1b[0m");
    println!("\x1b[91m    |   |  '  \x1b[0m \x1b[93m|   |  |-,\x1b[0m \x1b[92m  __ \\  \\  |\x1b[0m \x1b[96m|   |   . |\x1b[0m \x1b[95m'   :  ;\x1b[0m");
    println!("\x1b[91m    '   :  |  \x1b[0m \x1b[93m'   :  ;/|\x1b[0m \x1b[92m /  /`--'  /\x1b[0m \x1b[96m'   :  '; |\x1b[0m \x1b[95m|   |  '\x1b[0m");
    println!("\x1b[91m    ;   |.'   \x1b[0m \x1b[93m|   |    \\\x1b[0m \x1b[92m'--'.     / \x1b[0m \x1b[96m|   |  | ;\x1b[0m  \x1b[95m'   :  |\x1b[0m");
    println!("\x1b[91m    '---'     \x1b[0m \x1b[93m|   :   .'\x1b[0m \x1b[92m  `--'---'  \x1b[0m \x1b[96m|   :   /\x1b[0m   \x1b[95m;   |.'\x1b[0m");
    println!("\x1b[91m              \x1b[0m \x1b[93m|   | ,'  \x1b[0m \x1b[92m            \x1b[0m \x1b[96m|   | ,'\x1b[0m    \x1b[95m'---'\x1b[0m");
    println!("\x1b[91m              \x1b[0m \x1b[93m`----'    \x1b[0m \x1b[92m            \x1b[0m \x1b[96m`----'\x1b[0m");
}

// show whether a function can reach anywhere
#[no_mangle]
pub fn marker() {
    debug_info!("mark!");
}
