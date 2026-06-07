use aether_bootinfo::{BootInfo, FramebufferInfo, MemoryRegion, MemoryRegionKind, MAX_MEMORY_REGIONS};

#[test]
fn boot_info_stores_framebuffer_and_memory_regions() {
    let mut regions = [MemoryRegion::EMPTY; MAX_MEMORY_REGIONS];
    regions[0] = MemoryRegion::new(0x1000, 0x4000, MemoryRegionKind::Usable);

    let boot_info = BootInfo::new(
        0xffff_8000_0000_0000,
        1,
        Some(FramebufferInfo::new(0xB8000, 800, 600, 3200, 32)),
        regions,
    );

    assert_eq!(boot_info.hhdm_offset, 0xffff_8000_0000_0000);
    assert_eq!(boot_info.memory_map_entries, 1);
    assert!(boot_info.framebuffer.is_some());
    assert_eq!(boot_info.memory_regions[0].kind, MemoryRegionKind::Usable);
}
