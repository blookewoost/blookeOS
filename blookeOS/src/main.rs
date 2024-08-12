#![no_std] // Disable automatic linking of the standard library
#![no_main] // Don't use the C runtime entry point.

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to BlookeOS!");
    println!("This is the worst operating system of all time!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Shid pant!");
    println!("{}", info);
    loop {}
}
