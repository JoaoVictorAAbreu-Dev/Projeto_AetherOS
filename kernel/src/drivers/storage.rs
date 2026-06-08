#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageDeviceInfo {
    pub name: &'static str,
    pub writable: bool,
    pub persistent: bool,
    pub block_size: usize,
}

pub fn primary_device() -> StorageDeviceInfo {
    StorageDeviceInfo {
        name: "memory-overlay",
        writable: true,
        persistent: false,
        block_size: 1,
    }
}
