#![no_std]

//! Bootloader-provided information contracts live here.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootInfo {
    pub hhdm_offset: u64,
    pub memory_map_entries: usize,
}

impl BootInfo {
    pub const fn new(hhdm_offset: u64, memory_map_entries: usize) -> Self {
        Self {
            hhdm_offset,
            memory_map_entries,
        }
    }
}
