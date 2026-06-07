#![no_std]

//! Bootloader-provided information contracts live here.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootInfo {
    pub hhdm_offset: u64,
    pub memory_map_entries: usize,
    pub framebuffer: Option<FramebufferInfo>,
}

impl BootInfo {
    pub const fn new(
        hhdm_offset: u64,
        memory_map_entries: usize,
        framebuffer: Option<FramebufferInfo>,
    ) -> Self {
        Self {
            hhdm_offset,
            memory_map_entries,
            framebuffer,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FramebufferInfo {
    pub address: u64,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u16,
}

impl FramebufferInfo {
    pub const fn new(address: u64, width: u32, height: u32, pitch: u32, bpp: u16) -> Self {
        Self {
            address,
            width,
            height,
            pitch,
            bpp,
        }
    }
}
