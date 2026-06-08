use spin::Once;

pub const INITRAMFS_FILE_COUNT: usize = 4;

pub struct InitramfsFile {
    pub path: &'static str,
    pub contents: &'static str,
}

static FILES: Once<[InitramfsFile; INITRAMFS_FILE_COUNT]> = Once::new();

pub fn initialize() {
    FILES.call_once(build_initramfs);
}

pub fn files() -> &'static [InitramfsFile; INITRAMFS_FILE_COUNT] {
    FILES.call_once(build_initramfs)
}

fn build_initramfs() -> [InitramfsFile; INITRAMFS_FILE_COUNT] {
    [
        InitramfsFile {
            path: "/README.TXT",
            contents: "AetherOS initramfs\nStatic in-memory files for shell validation and boot-time inspection.\n",
        },
        InitramfsFile {
            path: "/STATUS.TXT",
            contents: "Boot, interrupts, memory, scheduler, shell, and QEMU validation are active in this academic v1 baseline.\n",
        },
        InitramfsFile {
            path: "/ROADMAP.TXT",
            contents: "AetherOS v1 baseline is focused on stable bring-up, observability, and educational subsystem boundaries.\n",
        },
        InitramfsFile {
            path: "/COMMANDS.TXT",
            contents: "help\ninfo\nticks\nmem\ntasks\nls\ncat <FILE>\nclear\n",
        },
    ]
}
