pub mod initramfs;
pub mod vfs;

pub fn initialize() {
    initramfs::initialize();
    vfs::initialize();
}
