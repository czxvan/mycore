#![no_main]
#![no_std]
use core::arch::global_asm;

mod lang_items;
mod sbi;
#[macro_use]
mod console;
mod logging;

use sbi::shutdown;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() {
    clear_bss();
    
    print_memory_layout();
    // panic!("czxcx");
    shutdown(false);
}

fn print_memory_layout() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack_lower_bound();
        fn boot_stack_top();
    }
    logging::init();
    println!("[kernel] Hello, world!");
    log::trace!(
        "[kernel] .text  \t[{:#x}, {:#x})",
        stext as usize, etext as usize
    );
    log::debug!(
        "[kernel] .rodata\t[{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    log::info!(
        "[kernel] .data  \t[{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    log::warn!(
        "[kernel] .stack \t[{:#x}, {:#x})",
        boot_stack_lower_bound as usize, boot_stack_top as usize
    );
    log::error!("[kernel] .bss   \t[{:#x}, {:#x})",
        sbss as usize, ebss as usize
    );
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0)}
    })
}
