#![no_main]
#![no_std]
use core::arch::global_asm;
mod lang_items;
mod sbi;
mod console;

use sbi::shutdown;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() {
    // println!("Hello, world!");
    
    clear_bss();
    
    println!("hello, world!");
    println!("你好，世界！");
    
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
    info!(".text\t[{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata\t[{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data\t[{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(".bss\t[{:#x}, {:#x})", sbss as usize, ebss as usize);
    debug!(".stack\t[{:#x}, {:#x})", boot_stack_lower_bound as usize, boot_stack_top as usize);
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
