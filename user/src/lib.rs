#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
mod syscall;
mod lang_items;

use syscall::*;

// 用户库的真实入口，
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

// 与os一样的初始化bss，但是用start_bss代替sbss
fn clear_bss(){
    unsafe extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}