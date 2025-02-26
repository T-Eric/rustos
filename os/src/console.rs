use crate::tesbi::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as u8);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

// print macros

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}

// log color macros

pub enum Log {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

#[macro_export]
macro_rules! print_info {
    ($color: expr, $text:literal $(,$($arg: tt)+)?) => {
        match $color {
            Log::Error => $crate::console::print(format_args!(concat!("\x1b[31m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Warning => $crate::console::print(format_args!(concat!("\x1b[93m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Info => $crate::console::print(format_args!(concat!("\x1b[96m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Debug => $crate::console::print(format_args!(concat!("\x1b[32m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Trace => $crate::console::print(format_args!(concat!("\x1b[90m", $text, "\x1b[0m") $(, $($arg)+)?)),
        }
    };
}

macro_rules! println_info {
    ($color: expr, $text:literal $(,$($arg: tt)+)?) => {
        match $color {
            Log::Error => $crate::console::print(format_args!(concat!("\x1b[31m[ERROR] ", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Warning => $crate::console::print(format_args!(concat!("\x1b[93m[WARNING] ", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Info => $crate::console::print(format_args!(concat!("\x1b[96m[INFO] ", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Debug => $crate::console::print(format_args!(concat!("\x1b[32m", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Trace => $crate::console::print(format_args!(concat!("\x1b[90m", $text, "\n\x1b[0m") $(, $($arg)+)?)),
        }
    };
}
