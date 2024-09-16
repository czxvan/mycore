
use crate::println;
#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    // loop {}
    let msg = panic_info.message();
    if let Some(s) = msg.as_str() {
        println!("Panic: {}", s);
    } else {
        println!("Panic: unknown");
    }
    loop {}
}