use spin::Once;

use crate::fs::initramfs::InitramfsFile;

static INITIALIZED: Once<()> = Once::new();

pub fn initialize() {
    INITIALIZED.call_once(|| {});
}

pub fn list_root_entries() -> &'static [&'static str] {
    static ROOT_ENTRIES: [&str; 4] = [
        "/README.TXT",
        "/STATUS.TXT",
        "/ROADMAP.TXT",
        "/COMMANDS.TXT",
    ];
    &ROOT_ENTRIES
}

pub fn read(path: &str) -> Option<&'static str> {
    find(path).map(|file| file.contents)
}

pub fn exists(path: &str) -> bool {
    find(path).is_some()
}

fn find(path: &str) -> Option<&'static InitramfsFile> {
    crate::fs::initramfs::files()
        .iter()
        .find(|file| file.path == path)
}
