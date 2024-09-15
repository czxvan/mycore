#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn exit(code: isize) -> ! {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("x10") code,
            in("x11") 0,
            in("x12") 0,
            in("x17") 93
        );
    }
    panic!("unreachable after exit");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    exit(main());
}

#[no_mangle]
fn main() -> isize {

    0
}