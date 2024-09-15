#![no_std]
#![no_main]

extern crate user_lib;
use user_lib::exit;

#[no_mangle]
fn main () -> isize {

    0
}

#[no_mangle]
fn _start() -> ! {
    exit(main())
}

