use spin::Once;

pub struct InitramfsFile {
    pub path: &'static str,
    pub contents: &'static str,
}

static FILES: Once<[InitramfsFile; 3]> = Once::new();

pub fn initialize() {
    FILES.call_once(|| {
        [
            InitramfsFile {
                path: "/README.TXT",
                contents: "AetherOS initramfs\nThis is the first in-memory filesystem.\n",
            },
            InitramfsFile {
                path: "/STATUS.TXT",
                contents: "Boot, interrupts, memory, scheduler, and shell foundations are active.\n",
            },
            InitramfsFile {
                path: "/ROADMAP.TXT",
                contents: "Next milestones: refine VFS, improve shell, add QEMU validation.\n",
            },
        ]
    });
}

pub fn files() -> &'static [InitramfsFile; 3] {
    FILES.call_once(|| {
        [
            InitramfsFile {
                path: "/README.TXT",
                contents: "AetherOS initramfs\nThis is the first in-memory filesystem.\n",
            },
            InitramfsFile {
                path: "/STATUS.TXT",
                contents: "Boot, interrupts, memory, scheduler, and shell foundations are active.\n",
            },
            InitramfsFile {
                path: "/ROADMAP.TXT",
                contents: "Next milestones: refine VFS, improve shell, add QEMU validation.\n",
            },
        ]
    })
}
