use core::sync::atomic::{AtomicUsize, Ordering};

use spin::Mutex;

use crate::task::process::{Process, ProcessState};

const MAX_TASKS: usize = 16;

static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
static TASK_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn initialize() {
    let mut scheduler = SCHEDULER.lock();
    scheduler.add(Process::new(0));
    scheduler.run_next();
    TASK_COUNT.store(scheduler.len(), Ordering::Release);
}

pub fn task_count() -> usize {
    TASK_COUNT.load(Ordering::Acquire)
}

struct Scheduler {
    tasks: [Option<Process>; MAX_TASKS],
    len: usize,
    current: usize,
}

impl Scheduler {
    const fn new() -> Self {
        Self {
            tasks: [None; MAX_TASKS],
            len: 0,
            current: 0,
        }
    }

    fn add(&mut self, process: Process) {
        if self.len < MAX_TASKS {
            self.tasks[self.len] = Some(process);
            self.len += 1;
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn run_next(&mut self) {
        if self.len == 0 {
            return;
        }

        self.current = (self.current + 1) % self.len;

        if let Some(mut process) = self.tasks[self.current] {
            process.state = ProcessState::Running;
            self.tasks[self.current] = Some(process);
        }
    }
}
