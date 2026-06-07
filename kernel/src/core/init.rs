use aether_bootinfo::BootInfo;

pub fn initialize(boot_info: &BootInfo) {
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
}
