use aether_bootinfo::MemoryRegion;
use aether_kernel::memory::mapper::to_higher_half;

#[test]
fn memory_region_end_is_computed_correctly() {
    let region = MemoryRegion::new(0x1000, 0x5000, aether_bootinfo::MemoryRegionKind::Usable);
    assert_eq!(region.end(), 0x6000);
}

#[test]
fn higher_half_mapping_adds_hhdm_offset() {
    let virt = to_higher_half(0x2000, 0xffff_8000_0000_0000);
    assert_eq!(virt.as_u64(), 0xffff_8000_0000_2000);
}
