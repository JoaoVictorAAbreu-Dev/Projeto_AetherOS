use spin::Mutex;

use crate::drivers::keyboard::{self, KeyEvent};

const MAX_LINE_LEN: usize = 128;

static SHELL: Mutex<Shell> = Mutex::new(Shell::new());

pub fn initialize() {
    let mut shell = SHELL.lock();
    shell.initialize();
}

pub fn on_key(event: KeyEvent) {
    let mut shell = SHELL.lock();
    shell.on_key(event);
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

    fn on_key(&mut self, event: KeyEvent) {
        if !self.initialized {
            return;
        }

        match event {
            KeyEvent::Enter => {
                crate::print!("\n");
                self.execute_current_line();
                self.prompt();
            }
            KeyEvent::Backspace => {
                if self.len > 0 {
                    self.len -= 1;
                    crate::print!("\u{8} \u{8}");
                }
            }
            KeyEvent::Char(character) => {
                if self.len < MAX_LINE_LEN {
                    self.line_buffer[self.len] = character as u8;
                    self.len += 1;
                    crate::print!("{}", character);
                } else {
                    crate::print!("\n");
                    crate::println!("input too long");
                    self.len = 0;
                    self.prompt();
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
            crate::println!("commands:");
            crate::println!("  help        Show this command list");
            crate::println!("  info        Show kernel and keyboard status");
            crate::println!("  ticks       Show PIT tick counter");
            crate::println!("  mem         Show frame allocator and heap status");
            crate::println!("  tasks       Show scheduler task summary");
            crate::println!("  ls          List VFS root entries");
            crate::println!("  cat <FILE>  Read a VFS file");
            crate::println!("  write <FILE> <TEXT>  Create or update a writable overlay file");
            crate::println!("  rm <FILE>   Remove a writable overlay file");
            crate::println!("  storage     Show storage boundary status");
            crate::println!("  clear       Redraw the framebuffer shell surface");
        }
        "info" => {
            crate::println!("AetherOS academic kernel");
            crate::println!("keyboard initialized = {}", keyboard::is_initialized());
            crate::println!(
                "scheduler model = {}",
                crate::task::scheduler::scheduling_model()
            );
            crate::println!("task count = {}", crate::task::scheduler::task_count());
            crate::println!(
                "current task = {:?}",
                crate::task::scheduler::current_task_name()
            );
            crate::println!(
                "current task state = {}",
                crate::task::scheduler::current_task_state()
                    .map(|state| state.label())
                    .unwrap_or("unavailable")
            );
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
            crate::println!(
                "scheduler model = {}",
                crate::task::scheduler::scheduling_model()
            );
            crate::println!("task count = {}", crate::task::scheduler::task_count());
            crate::println!(
                "current task = {:?}",
                crate::task::scheduler::current_task_name()
            );
            crate::println!(
                "current task state = {}",
                crate::task::scheduler::current_task_state()
                    .map(|state| state.label())
                    .unwrap_or("unavailable")
            );
        }
        "ls" => {
            crate::fs::vfs::for_each_entry(|name| {
                crate::println!("{}", name);
            });
        }
        "cat" => {
            let Some(path) = parts.next() else {
                crate::println!("usage: cat <FILE>");
                return;
            };

            match crate::fs::vfs::read(path) {
                Some(contents) => crate::println!("{}", contents),
                None => crate::println!("file not found: {}", path),
            }
        }
        "write" => {
            let Some(args) = command.strip_prefix("write ") else {
                crate::println!("usage: write <FILE> <TEXT>");
                return;
            };
            let Some((path, contents)) = args.split_once(' ') else {
                crate::println!("usage: write <FILE> <TEXT>");
                return;
            };

            match crate::fs::vfs::write(path, contents) {
                Ok(()) => crate::println!("written: {}", path),
                Err(error) => crate::println!("write failed: {}", error.message()),
            }
        }
        "rm" => {
            let Some(path) = parts.next() else {
                crate::println!("usage: rm <FILE>");
                return;
            };

            match crate::fs::vfs::remove(path) {
                Ok(()) => crate::println!("removed: {}", path),
                Err(error) => crate::println!("remove failed: {}", error.message()),
            }
        }
        "storage" => {
            let device = crate::drivers::storage::primary_device();
            crate::println!("storage device = {}", device.name);
            crate::println!("writable = {}", device.writable);
            crate::println!("persistent = {}", device.persistent);
            crate::println!("block size = {}", device.block_size);
        }
        "clear" => {
            crate::drivers::framebuffer::redraw_boot_shell_surface();
            crate::println!("screen redrawn");
        }
        _ => crate::println!("unknown command: {}", command),
    }
}
