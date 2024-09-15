// main.rs
#![no_std]
#![no_main]

use core::arch::asm;

/// 使用系统调用进行进程退出。否则会导致coredump
pub fn exit(code: isize){
    unsafe {
        asm!(
        "syscall",
        in("rax") 60, // exit
        in("rdi") code,
        options(noreturn)
        );
    }
}

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main () {
    // loop {}
    exit(0)
}

#[no_mangle]
fn __libc_start_main() {
    main();
}