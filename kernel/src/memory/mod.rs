pub mod address_space;
pub mod frame_allocator;
pub mod heap;
pub mod mapper;

use aether_bootinfo::BootInfo;

pub fn initialize(boot_info: &BootInfo) {
    frame_allocator::initialize(boot_info);
    heap::initialize();
}
