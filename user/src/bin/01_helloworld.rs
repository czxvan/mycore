#![no_std]
#![no_main]

// extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    
    0
}


#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}