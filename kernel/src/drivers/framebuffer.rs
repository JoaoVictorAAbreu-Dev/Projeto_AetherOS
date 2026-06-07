use aether_bootinfo::FramebufferInfo;
use spin::Mutex;

static FRAMEBUFFER: Mutex<Option<FramebufferInfo>> = Mutex::new(None);

pub fn initialize(framebuffer: Option<FramebufferInfo>) {
    if let Some(framebuffer) = framebuffer {
        *FRAMEBUFFER.lock() = Some(framebuffer);
        crate::arch::x86_64::vga::render_boot_visual(framebuffer);
    } else {
        crate::println!("AetherOS: no framebuffer available, serial output only");
    }
}

pub fn redraw_boot_shell_surface() {
    if let Some(framebuffer) = *FRAMEBUFFER.lock() {
        crate::arch::x86_64::vga::render_boot_visual(framebuffer);
    }
}
