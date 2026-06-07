#![no_std]

//! Bootloader-provided information contracts live here.

pub const MAX_MEMORY_REGIONS: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootInfo {
    pub hhdm_offset: u64,
    pub memory_map_entries: usize,
    pub framebuffer: Option<FramebufferInfo>,
    pub memory_regions: [MemoryRegion; MAX_MEMORY_REGIONS],
}

impl BootInfo {
    pub const fn new(
        hhdm_offset: u64,
        memory_map_entries: usize,
        framebuffer: Option<FramebufferInfo>,
        memory_regions: [MemoryRegion; MAX_MEMORY_REGIONS],
    ) -> Self {
        Self {
            hhdm_offset,
            memory_map_entries,
            framebuffer,
            memory_regions,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryRegion {
    pub base: u64,
    pub length: u64,
    pub kind: MemoryRegionKind,
}

impl MemoryRegion {
    pub const EMPTY: Self = Self {
        base: 0,
        length: 0,
        kind: MemoryRegionKind::Reserved,
    };

    pub const fn new(base: u64, length: u64, kind: MemoryRegionKind) -> Self {
        Self { base, length, kind }
    }

    pub const fn end(self) -> u64 {
        self.base.saturating_add(self.length)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryRegionKind {
    Usable,
    Reserved,
    Reclaimable,
    Kernel,
    Framebuffer,
    Unknown,
}
