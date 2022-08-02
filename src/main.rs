// cargo build --target thumbv7em-none-eabihf -> cross compile
// target triple has no underlying operating system

#![no_std]  // std depends on the os, so we have to disable it
#![no_main]  // "main" doesn't make sense without an underlying runtime that calls it

use core::panic::PanicInfo;

#[no_mangle]  // ensure that the Rust compiler outputs a function with the name _start ::
              // otherwise -> _7Nwan_os4_start7kdgkusdg f. ex.
pub extern "C" fn _start() -> ! {  //  overwrites the os entry point 
    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {   // ! :: returns never
    loop {}
}
