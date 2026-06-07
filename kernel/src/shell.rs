use spin::Mutex;

use crate::drivers::keyboard;

const MAX_LINE_LEN: usize = 128;

static SHELL: Mutex<Shell> = Mutex::new(Shell::new());

pub fn initialize() {
    let mut shell = SHELL.lock();
    shell.initialize();
}

pub fn on_key(character: char) {
    let mut shell = SHELL.lock();
    shell.on_key(character);
}

struct Shell {
    initialized: bool,
    line_buffer: [u8; MAX_LINE_LEN],
    len: usize,
}

impl Shell {
    const fn new() -> Self {
        Self {
            initialized: false,
            line_buffer: [0; MAX_LINE_LEN],
            len: 0,
        }
    }

    fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        self.initialized = true;
        crate::println!("AetherOS shell ready");
        crate::println!("Type 'help' to list commands.");
        self.prompt();
    }

    fn on_key(&mut self, character: char) {
        if !self.initialized {
            return;
        }

        match character {
            '\n' => {
                crate::print!("\n");
                self.execute_current_line();
                self.prompt();
            }
            '\u{8}' => {
                if self.len > 0 {
                    self.len -= 1;
                    crate::print!("\u{8} \u{8}");
                }
            }
            _ => {
                if self.len < MAX_LINE_LEN {
                    self.line_buffer[self.len] = character as u8;
                    self.len += 1;
                    crate::print!("{}", character);
                }
            }
        }
    }

    fn execute_current_line(&mut self) {
        let line = match core::str::from_utf8(&self.line_buffer[..self.len]) {
            Ok(line) => line.trim(),
            Err(_) => {
                crate::println!("invalid input");
                self.len = 0;
                return;
            }
        };

        if line.is_empty() {
            self.len = 0;
            return;
        }

        execute_command(line);
        self.len = 0;
    }

    fn prompt(&self) {
        crate::print!("aether> ");
    }
}

fn execute_command(command: &str) {
    let mut parts = command.split_whitespace();
    let head = match parts.next() {
        Some(head) => head,
        None => return,
    };

    match head {
        "help" => {
            crate::println!("commands: help, info, ticks, mem, tasks, ls, cat <FILE>, clear");
        }
        "info" => {
            crate::println!("AetherOS academic kernel");
            crate::println!("keyboard initialized = {}", keyboard::is_initialized());
            crate::println!("task count = {}", crate::task::scheduler::task_count());
            crate::println!("current task = {:?}", crate::task::scheduler::current_task_name());
        }
        "ticks" => {
            crate::println!("ticks = {}", crate::drivers::timer::ticks());
        }
        "mem" => {
            crate::println!(
                "usable ranges = {}",
                crate::memory::frame_allocator::usable_range_count()
            );
            crate::println!(
                "usable frames = {}",
                crate::memory::frame_allocator::usable_frame_count()
            );
            crate::println!("heap size = {}", crate::memory::heap::heap_size());
            crate::println!("heap used = {}", crate::memory::heap::heap_used());
        }
        "tasks" => {
            crate::println!("task count = {}", crate::task::scheduler::task_count());
            crate::println!("current task = {:?}", crate::task::scheduler::current_task_name());
        }
        "ls" => {
            for name in crate::fs::vfs::list_root_entries() {
                crate::println!("{}", name);
            }
        }
        "cat" => {
            let Some(path) = parts.next() else {
                crate::println!("usage: cat <FILE>");
                return;
            };

            let normalized = normalize_path(path);

            match crate::fs::vfs::read(normalized) {
                Some(contents) => crate::println!("{}", contents),
                None => crate::println!("file not found"),
            }
        }
        "clear" => {
            crate::drivers::framebuffer::redraw_boot_shell_surface();
            crate::println!("screen redrawn");
        }
        _ => crate::println!("unknown command: {}", command),
    }
}

fn normalize_path(path: &str) -> &str {
    if path.starts_with('/') {
        path
    } else {
        match path {
            "README.TXT" => "/README.TXT",
            "STATUS.TXT" => "/STATUS.TXT",
            "ROADMAP.TXT" => "/ROADMAP.TXT",
            "COMMANDS.TXT" => "/COMMANDS.TXT",
            _ => path,
        }
    }
}
