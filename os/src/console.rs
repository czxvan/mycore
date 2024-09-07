use crate::sbi::console_putchar;
use core::fmt::{self, Write};

// ANSI escape codes for colors
pub const RESET: &str = "\x1B[0m";
pub const INFO_COLOR: &str = "\x1B[32m"; // Green
pub const DEBUG_COLOR: &str = "\x1B[34m"; // Blue
pub const ERROR_COLOR: &str = "\x1B[31m"; // Red

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            console_putchar(b as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::println!("{}{}", $crate::console::INFO_COLOR, format_args!($fmt $(, $($arg)+)?));
        $crate::print!("{}", $crate::console::RESET);
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::println!("{}{}", $crate::console::DEBUG_COLOR, format_args!($fmt $(, $($arg)+)?));
        $crate::print!("{}", $crate::console::RESET);
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::println!("{}{}", $crate::console::ERROR_COLOR, format_args!($fmt $(, $($arg)+)?));
        $crate::print!("{}", $crate::console::RESET);
    }
}
