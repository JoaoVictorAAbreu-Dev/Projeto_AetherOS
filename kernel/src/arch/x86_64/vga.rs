use aether_bootinfo::FramebufferInfo;

use crate::println;

const BACKGROUND: u32 = 0x0B1020;
const PANEL: u32 = 0x121A31;
const PANEL_BORDER: u32 = 0x2F4F8F;
const ACCENT: u32 = 0x62D0FF;
const ACCENT_MUTED: u32 = 0x24415F;
const SUCCESS: u32 = 0x6EE7B7;

pub fn render_boot_visual(framebuffer: FramebufferInfo) {
    if framebuffer.bpp != 32 {
        println!(
            "AetherOS: framebuffer detected, but unsupported bpp for visual stage: {}",
            framebuffer.bpp
        );
        return;
    }

    let mut display = FramebufferDisplay::new(framebuffer);
    display.clear(BACKGROUND);
    display.fill_rect(0, 0, framebuffer.width, 28, ACCENT_MUTED);
    display.fill_rect(0, framebuffer.height.saturating_sub(18), framebuffer.width, 18, PANEL);

    let panel_width = framebuffer.width.saturating_sub(120);
    let panel_height = framebuffer.height.min(220);
    let panel_x = 60;
    let panel_y = framebuffer.height.saturating_sub(panel_height) / 2;

    display.fill_rect(panel_x, panel_y, panel_width, panel_height, PANEL);
    display.stroke_rect(panel_x, panel_y, panel_width, panel_height, 4, PANEL_BORDER);

    display.fill_rect(panel_x + 28, panel_y + 28, panel_width.saturating_sub(56), 10, ACCENT);
    display.fill_rect(
        panel_x + 28,
        panel_y + 58,
        panel_width.saturating_sub(140),
        18,
        SUCCESS,
    );
    display.fill_rect(
        panel_x + 28,
        panel_y + 92,
        panel_width.saturating_sub(72),
        10,
        ACCENT_MUTED,
    );
    display.fill_rect(
        panel_x + 28,
        panel_y + 116,
        panel_width.saturating_sub(180),
        10,
        ACCENT_MUTED,
    );
    display.fill_rect(
        panel_x + 28,
        panel_y + 140,
        panel_width.saturating_sub(240),
        10,
        ACCENT_MUTED,
    );

    display.draw_mark(panel_x + panel_width.saturating_sub(84), panel_y + 38, 36, ACCENT);
    display.draw_progress(panel_x + 28, panel_y + panel_height.saturating_sub(42), panel_width.saturating_sub(56), 12);
    display.fill_rect(64, framebuffer.height.saturating_sub(84), framebuffer.width.saturating_sub(128), 48, 0x0A0F1D);
}

struct FramebufferDisplay {
    info: FramebufferInfo,
}

impl FramebufferDisplay {
    fn new(info: FramebufferInfo) -> Self {
        Self { info }
    }

    fn clear(&mut self, color: u32) {
        self.fill_rect(0, 0, self.info.width, self.info.height, color);
    }

    fn stroke_rect(&mut self, x: u32, y: u32, width: u32, height: u32, thickness: u32, color: u32) {
        self.fill_rect(x, y, width, thickness, color);
        self.fill_rect(x, y + height.saturating_sub(thickness), width, thickness, color);
        self.fill_rect(x, y, thickness, height, color);
        self.fill_rect(x + width.saturating_sub(thickness), y, thickness, height, color);
    }

    fn draw_mark(&mut self, x: u32, y: u32, size: u32, color: u32) {
        let third = size / 3;
        self.fill_rect(x, y + third, third, size.saturating_sub(third), color);
        self.fill_rect(x + third, y, third, size, color);
        self.fill_rect(x + (third * 2), y + third, third, size.saturating_sub(third), color);
    }

    fn draw_progress(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.fill_rect(x, y, width, height, ACCENT_MUTED);

        let filled = width.saturating_mul(2) / 5;
        self.fill_rect(x, y, filled, height, ACCENT);

        let marker_width = 14;
        let marker_x = x + filled.saturating_sub(marker_width / 2);
        self.fill_rect(marker_x, y.saturating_sub(4), marker_width, height + 8, SUCCESS);
    }

    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        if width == 0 || height == 0 {
            return;
        }

        let max_x = (x + width).min(self.info.width);
        let max_y = (y + height).min(self.info.height);
        let bytes_per_pixel = (self.info.bpp / 8) as usize;
        let base = self.info.address as *mut u8;

        for py in y..max_y {
            let row_offset = py as usize * self.info.pitch as usize;

            for px in x..max_x {
                let offset = row_offset + px as usize * bytes_per_pixel;

                unsafe {
                    let pixel = base.add(offset) as *mut u32;
                    pixel.write_volatile(color);
                }
            }
        }
    }
}
