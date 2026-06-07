#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    aether_kernel::arch::x86_64::boot::enter()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aether_kernel::core::panic::handle(info)
}
