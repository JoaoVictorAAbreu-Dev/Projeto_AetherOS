use core::panic::PanicInfo;

use x86_64::instructions::hlt;

pub fn handle(info: &PanicInfo<'_>) -> ! {
    crate::println!(
        "AetherOS panic stage: {}",
        crate::arch::x86_64::boot::current_boot_stage_label()
    );
    crate::println!("AetherOS panic: {}", info);
    halt_forever()
}

pub fn halt_forever() -> ! {
    loop {
        hlt();
    }
}
