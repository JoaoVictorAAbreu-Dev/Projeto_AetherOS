use core::str;

use spin::{Mutex, Once};

use crate::fs::initramfs::InitramfsFile;

static INITIALIZED: Once<()> = Once::new();
static WRITABLE_FILES: Mutex<[OverlayFile; MAX_OVERLAY_FILES]> =
    Mutex::new([OverlayFile::EMPTY; MAX_OVERLAY_FILES]);

const MAX_OVERLAY_FILES: usize = 8;
const MAX_PATH_LEN: usize = 32;
const MAX_CONTENT_LEN: usize = 256;

pub fn initialize() {
    INITIALIZED.call_once(|| {
        *WRITABLE_FILES.lock() = [OverlayFile::EMPTY; MAX_OVERLAY_FILES];
    });
}

pub fn read(path: &str) -> Option<&'static str> {
    let mut normalized = [0u8; MAX_PATH_LEN];
    let normalized = normalize_path(path, &mut normalized).ok()?;

    if let Some(contents) = read_overlay(normalized) {
        return Some(contents);
    }

    find(normalized).map(|file| file.contents)
}

pub fn exists(path: &str) -> bool {
    let mut normalized = [0u8; MAX_PATH_LEN];
    let Ok(normalized) = normalize_path(path, &mut normalized) else {
        return false;
    };

    read_overlay(normalized).is_some() || find(normalized).is_some()
}

pub fn write(path: &str, contents: &str) -> Result<(), VfsError> {
    let mut normalized = [0u8; MAX_PATH_LEN];
    let normalized = normalize_path(path, &mut normalized)?;
    let content_bytes = contents.as_bytes();
    if content_bytes.len() > MAX_CONTENT_LEN {
        return Err(VfsError::ContentTooLong);
    }

    let mut files = WRITABLE_FILES.lock();

    if let Some(entry) = files
        .iter_mut()
        .find(|entry| entry.matches(normalized.as_bytes()))
    {
        entry.write(normalized, content_bytes);
        return Ok(());
    }

    if let Some(entry) = files.iter_mut().find(|entry| !entry.used) {
        entry.write(normalized, content_bytes);
        return Ok(());
    }

    Err(VfsError::NoSpace)
}

pub fn remove(path: &str) -> Result<(), VfsError> {
    let mut normalized = [0u8; MAX_PATH_LEN];
    let normalized = normalize_path(path, &mut normalized)?;
    let mut files = WRITABLE_FILES.lock();

    if let Some(entry) = files
        .iter_mut()
        .find(|entry| entry.matches(normalized.as_bytes()))
    {
        *entry = OverlayFile::EMPTY;
        return Ok(());
    }

    if find(normalized).is_some() {
        return Err(VfsError::ReadOnly);
    }

    Err(VfsError::NotFound)
}

pub fn for_each_entry(mut visitor: impl FnMut(&str)) {
    let files = WRITABLE_FILES.lock();

    for entry in files.iter().filter(|entry| entry.used) {
        if let Some(path) = entry.path() {
            visitor(path);
        }
    }

    for file in crate::fs::initramfs::files() {
        if files
            .iter()
            .any(|entry| entry.matches(file.path.as_bytes()))
        {
            continue;
        }
        visitor(file.path);
    }
}

fn find(path: &str) -> Option<&'static InitramfsFile> {
    crate::fs::initramfs::files()
        .iter()
        .find(|file| file.path == path)
}

fn read_overlay(path: &str) -> Option<&'static str> {
    let files = WRITABLE_FILES.lock();
    let entry = files.iter().find(|entry| entry.matches(path.as_bytes()))?;
    let bytes = entry.contents_bytes();
    let text = str::from_utf8(bytes).ok()?;

    Some(leak_str(text))
}

fn normalize_path<'a>(
    path: &'a str,
    buffer: &'a mut [u8; MAX_PATH_LEN],
) -> Result<&'a str, VfsError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(VfsError::InvalidPath);
    }

    let normalized = if trimmed.starts_with('/') {
        trimmed
    } else {
        copy_with_slash_prefix(trimmed, buffer)?
    };

    if normalized.len() > MAX_PATH_LEN {
        return Err(VfsError::PathTooLong);
    }

    Ok(normalized)
}

fn copy_with_slash_prefix<'a>(
    path: &str,
    buffer: &'a mut [u8; MAX_PATH_LEN],
) -> Result<&'a str, VfsError> {
    if path.len() + 1 > MAX_PATH_LEN {
        return Err(VfsError::PathTooLong);
    }

    buffer[0] = b'/';
    buffer[1..path.len() + 1].copy_from_slice(path.as_bytes());
    str::from_utf8(&buffer[..path.len() + 1]).map_err(|_| VfsError::InvalidPath)
}

fn leak_str(value: &str) -> &'static str {
    // Overlay contents stay valid for the kernel lifetime once written.
    unsafe { &*(value as *const str) }
}

#[derive(Clone, Copy)]
struct OverlayFile {
    used: bool,
    path_len: usize,
    contents_len: usize,
    path: [u8; MAX_PATH_LEN],
    contents: [u8; MAX_CONTENT_LEN],
}

impl OverlayFile {
    const EMPTY: Self = Self {
        used: false,
        path_len: 0,
        contents_len: 0,
        path: [0; MAX_PATH_LEN],
        contents: [0; MAX_CONTENT_LEN],
    };

    fn write(&mut self, path: &str, contents: &[u8]) {
        self.used = true;
        self.path_len = path.len();
        self.contents_len = contents.len();
        self.path[..path.len()].copy_from_slice(path.as_bytes());
        self.contents[..contents.len()].copy_from_slice(contents);
    }

    fn matches(&self, path: &[u8]) -> bool {
        self.used && self.path_len == path.len() && self.path[..self.path_len] == *path
    }

    fn path(&self) -> Option<&str> {
        str::from_utf8(&self.path[..self.path_len]).ok()
    }

    fn contents_bytes(&self) -> &[u8] {
        &self.contents[..self.contents_len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VfsError {
    InvalidPath,
    PathTooLong,
    ContentTooLong,
    NoSpace,
    ReadOnly,
    NotFound,
}

impl VfsError {
    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidPath => "invalid path",
            Self::PathTooLong => "path too long",
            Self::ContentTooLong => "content too long",
            Self::NoSpace => "filesystem overlay full",
            Self::ReadOnly => "cannot remove read-only initramfs file",
            Self::NotFound => "file not found",
        }
    }
}
