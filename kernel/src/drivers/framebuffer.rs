use aether_bootinfo::FramebufferInfo;

pub fn initialize(framebuffer: Option<FramebufferInfo>) {
    if let Some(framebuffer) = framebuffer {
        crate::arch::x86_64::vga::render_boot_visual(framebuffer);
    } else {
        crate::println!("AetherOS: no framebuffer available, serial output only");
    }
}
