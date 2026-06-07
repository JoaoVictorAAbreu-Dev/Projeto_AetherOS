use aether_bootinfo::BootInfo;

pub fn initialize(boot_info: &BootInfo) {
    crate::arch::x86_64::gdt::init();
    crate::arch::x86_64::idt::init();
    crate::arch::x86_64::interrupts::init();
    crate::drivers::timer::initialize();
    crate::drivers::keyboard::initialize();
    crate::drivers::framebuffer::initialize(boot_info.framebuffer);
    crate::println!("AetherOS: kernel initialized");
    crate::println!("AetherOS: HHDM offset = {:#x}", boot_info.hhdm_offset);
    crate::println!(
        "AetherOS: memory map entries = {}",
        boot_info.memory_map_entries
    );
    crate::println!(
        "AetherOS: framebuffer = {}",
        if boot_info.framebuffer.is_some() {
            "available"
        } else {
            "unavailable"
        }
    );
    crate::println!("AetherOS: interrupts enabled");
}
