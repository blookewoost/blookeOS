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
use bootloader::{BootInfo, entry_point};

use blooke_os::println;

// Use the provided macro to identify the OS entry point for the bootloader.
entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {

    use x86_64::{VirtAddr, structures::paging::Translate};

    blooke_os::println!("Welcome to BlookeOS!");
    blooke_os::init();

    let mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe {
        blooke_os::memory::init(mem_offset)
    };

    let addresses = [
        0xb8000, // VGA buffer page
        0x201008, // code page
        0x0100_0020_1a10, // stack page
        boot_info.physical_memory_offset // virtual address mapped to physical address 0
    ];

    for &addr in &addresses {
        let virt = VirtAddr::new(addr);
        let phys = mapper.translate_addr(virt);
        println!("{:?} => {:?}", virt, phys);
    }

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



