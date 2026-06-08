use core::panic::PanicInfo;

use x86_64::instructions::hlt;

pub fn handle(info: &PanicInfo<'_>) -> ! {
    crate::println!("AetherOS panic: {}", info);
    halt_forever()
}

pub fn halt_forever() -> ! {
    loop {
        hlt();
    }
}
