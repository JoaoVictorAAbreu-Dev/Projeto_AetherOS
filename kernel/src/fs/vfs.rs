use spin::Once;

use crate::fs::initramfs::InitramfsFile;

static INITIALIZED: Once<()> = Once::new();

pub fn initialize() {
    INITIALIZED.call_once(|| {});
}

pub fn list_root_entries() -> [&'static str; 3] {
    let files = crate::fs::initramfs::files();
    [files[0].path, files[1].path, files[2].path]
}

pub fn read(path: &str) -> Option<&'static str> {
    find(path).map(|file| file.contents)
}

fn find(path: &str) -> Option<&'static InitramfsFile> {
    crate::fs::initramfs::files()
        .iter()
        .find(|file| file.path == path)
}
