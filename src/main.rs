//disable the standard library which is OS-dependent, since this is a binary file and we are trying to build an OS that operates over bare metal
#![no_std]
//disable main function as entry point, since _start is the default entry point name for most systems
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
