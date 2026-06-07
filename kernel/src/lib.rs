#![no_std]

use aether_bootinfo::BootInfo;
use x86_64::instructions::hlt;

pub mod arch;
pub mod core;
pub mod drivers;
pub mod fs;
pub mod memory;
pub mod sync;
pub mod syscall;
pub mod task;
pub mod utils;

pub fn run(boot_info: &'static BootInfo) -> ! {
    core::init::initialize(boot_info);

    loop {
        hlt();
    }
}
