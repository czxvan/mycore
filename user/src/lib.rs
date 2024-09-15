#![no_std]
// pub mod console;
mod lang_items;

pub mod syscall;
use syscall::*;
pub fn exit(code: isize) -> ! {
    sys_exit(code);
    panic!("unreachable after sys_exit")
}