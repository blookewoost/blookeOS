#![no_std] // Disable automatic linking of the standard library
#![no_main] // Don't use the C runtime entry point.

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
