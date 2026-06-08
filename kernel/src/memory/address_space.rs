use x86_64::VirtAddr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddressSpace {
    pub hhdm_offset: VirtAddr,
    pub kind: AddressSpaceKind,
    pub user_range_start: VirtAddr,
    pub user_range_end: VirtAddr,
}

impl AddressSpace {
    pub const fn new(hhdm_offset: VirtAddr) -> Self {
        Self {
            hhdm_offset,
            kind: AddressSpaceKind::KernelOnly,
            user_range_start: VirtAddr::new(0),
            user_range_end: VirtAddr::new(0),
        }
    }

    pub const fn with_user_range(
        hhdm_offset: VirtAddr,
        user_range_start: VirtAddr,
        user_range_end: VirtAddr,
    ) -> Self {
        Self {
            hhdm_offset,
            kind: AddressSpaceKind::KernelAndUser,
            user_range_start,
            user_range_end,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressSpaceKind {
    KernelOnly,
    KernelAndUser,
}

impl AddressSpaceKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::KernelOnly => "kernel-only",
            Self::KernelAndUser => "kernel-and-user",
        }
    }
}
