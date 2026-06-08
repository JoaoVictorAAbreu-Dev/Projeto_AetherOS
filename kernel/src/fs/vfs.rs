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
    resolve(path).and_then(find).map(|file| file.contents)
}

pub fn exists(path: &str) -> bool {
    resolve(path).and_then(find).is_some()
}

fn find(path: &str) -> Option<&'static InitramfsFile> {
    crate::fs::initramfs::files()
        .iter()
        .find(|file| file.path == path)
}

fn resolve(path: &str) -> Option<&str> {
    let normalized = if path.starts_with('/') {
        path
    } else {
        match path {
            "README.TXT" | "readme.txt" => "/README.TXT",
            "STATUS.TXT" | "status.txt" => "/STATUS.TXT",
            "ROADMAP.TXT" | "roadmap.txt" => "/ROADMAP.TXT",
            "COMMANDS.TXT" | "commands.txt" => "/COMMANDS.TXT",
            _ => return None,
        }
    };

    Some(normalized)
}
