/*

Compiling for a baremetal target. Disable the following:
    - The Rust standard library
    - The typical C runtime entrypoint (There is no underlying OS or runtime, therefore no default entry point)

Enable the custom_test_frameworks feature for our custom integration tests implementation, not relying on the standard library.
Re-define the test harness entry point as our test_runner function (src/lib.rs)

*/

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blooke_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use blooke_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    blooke_os::println!("Welcome to BlookeOS!");
    blooke_os::init();

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    #[allow(unconditional_recursion)] // Tests for the kernel involve intentional stack overflow. Silence the recursion warning.
    test_main();

    blooke_os::println!("All seems well!");
    blooke_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blooke_os::println!("Something went wrong, and I am too fragile to properly handle panic events!");
    blooke_os::println!("{}", info);
    blooke_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blooke_os::test_panic_handler(info);
}



