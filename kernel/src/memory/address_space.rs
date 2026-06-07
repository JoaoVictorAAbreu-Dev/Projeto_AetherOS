use x86_64::VirtAddr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddressSpace {
    pub hhdm_offset: VirtAddr,
}

impl AddressSpace {
    pub const fn new(hhdm_offset: VirtAddr) -> Self {
        Self { hhdm_offset }
    }
}
