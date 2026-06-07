use x86_64::VirtAddr;

pub fn to_higher_half(physical_address: u64, hhdm_offset: u64) -> VirtAddr {
    VirtAddr::new(physical_address.saturating_add(hhdm_offset))
}
