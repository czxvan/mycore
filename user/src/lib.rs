#![no_std]
#![feature(linkage)]
// pub mod console;
mod lang_items;
mod syscall;
pub mod console;

use syscall::*;

#[no_mangle]
#[linkage = "weak"]
fn main() -> isize {
    panic!("Cannot find main!")
}

#[no_mangle]
fn _start() -> ! {
    exit(main());
    panic!("unreachable after sys_exit")
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf.as_ptr(), buf.len())
}

pub fn exit(code: isize) -> isize {
    sys_exit(code)
}
