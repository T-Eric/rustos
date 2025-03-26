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

#[allow(dead_code)]
pub enum Log {
    Panic,
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
            Log::Panic => $crate::console::print(format_args!(concat!("\x1b[95m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Error => $crate::console::print(format_args!(concat!("\x1b[31m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Warning => $crate::console::print(format_args!(concat!("\x1b[93m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Info => $crate::console::print(format_args!(concat!("\x1b[96m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Debug => $crate::console::print(format_args!(concat!("\x1b[32m", $text, "\x1b[0m") $(, $($arg)+)?)),
            Log::Trace => $crate::console::print(format_args!(concat!("\x1b[90m", $text, "\x1b[0m") $(, $($arg)+)?)),
        }
    };
}

#[macro_export]
macro_rules! println_info {
    ($color: expr, $text:literal $(,$($arg: tt)+)?) => {
        match $color {
            Log::Panic => $crate::console::print(format_args!(concat!("\x1b[95m[Panic] ", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Error => $crate::console::print(format_args!(concat!("\x1b[31m[Error] ", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Warning => $crate::console::print(format_args!(concat!("\x1b[93m[Warning] ", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Info => $crate::console::print(format_args!(concat!("\x1b[96m[Info] ", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Debug => $crate::console::print(format_args!(concat!("\x1b[32m", $text, "\n\x1b[0m") $(, $($arg)+)?)),
            Log::Trace => $crate::console::print(format_args!(concat!("\x1b[90m", $text, "\n\x1b[0m") $(, $($arg)+)?)),
        }
    };
}

#[macro_export]
macro_rules! error_info {
    ($text:literal $(,$($arg: tt)+)?) => {
        println_info!(Log::Error,$text $(,$($arg)+)?)
    };
}

#[macro_export]
macro_rules! debug_info {
    ($text:literal $(,$($arg: tt)+)?) => {
        println_info!(Log::Debug,$text $(,$($arg)+)?)
    };
}

#[macro_export]
macro_rules! info_info {
    ($text:literal $(,$($arg: tt)+)?) => {
        println_info!(Log::Info,$text $(,$($arg)+)?)
    };
}

#[macro_export]
macro_rules! warn_info {
    ($text:literal $(,$($arg: tt)+)?) => {
        println_info!(Log::Warning,$text $(,$($arg)+)?)
    };
}

#[macro_export]
macro_rules! trace_info {
    ($text:literal $(,$($arg: tt)+)?) => {
        println_info!(Log::Trace,$text $(,$($arg)+)?)
    };
}
