//disable the standard library which is OS-dependent, since this is a binary file and we are trying to build an OS that operates over bare metal
#![no_std]
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //C naming convention
    // this function is the entry point, since the linker looks for a function named `_start` by default
    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
