use core::sync::atomic::{AtomicUsize, Ordering};

use spin::Mutex;

use crate::task::process::{Process, ProcessState};

const MAX_TASKS: usize = 16;

static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
static TASK_COUNT: AtomicUsize = AtomicUsize::new(0);
static CURRENT_TASK_ID: AtomicUsize = AtomicUsize::new(usize::MAX);

pub fn initialize() {
    let mut scheduler = SCHEDULER.lock();
    scheduler.add(Process::new(0, "idle"));
    scheduler.add(Process::new(1, "init"));
    scheduler.run_next();
    TASK_COUNT.store(scheduler.len(), Ordering::Release);
    CURRENT_TASK_ID.store(scheduler.current_task_id(), Ordering::Release);
}

pub fn task_count() -> usize {
    TASK_COUNT.load(Ordering::Acquire)
}

pub fn current_task_id() -> Option<usize> {
    let id = CURRENT_TASK_ID.load(Ordering::Acquire);
    if id == usize::MAX {
        None
    } else {
        Some(id)
    }
}

pub fn current_task_name() -> Option<&'static str> {
    SCHEDULER.lock().current_task().map(|task| task.name)
}

pub fn current_task_state() -> Option<ProcessState> {
    SCHEDULER.lock().current_task().map(|task| task.state)
}

pub fn scheduling_model() -> &'static str {
    "cooperative round-robin demo"
}

pub fn tick() {
    let mut scheduler = SCHEDULER.lock();
    scheduler.run_next();
    CURRENT_TASK_ID.store(scheduler.current_task_id(), Ordering::Release);
}

struct Scheduler {
    tasks: [Option<Process>; MAX_TASKS],
    len: usize,
    current: Option<usize>,
}

impl Scheduler {
    const fn new() -> Self {
        Self {
            tasks: [None; MAX_TASKS],
            len: 0,
            current: None,
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

        if let Some(current) = self.current {
            if let Some(mut process) = self.tasks[current] {
                process.state = if process.id == 0 {
                    ProcessState::Idle
                } else {
                    ProcessState::Ready
                };
                self.tasks[current] = Some(process);
            }
        }

        let next = match self.current {
            Some(current) => (current + 1) % self.len,
            None => 0,
        };

        self.current = Some(next);

        if let Some(mut process) = self.tasks[next] {
            process.state = ProcessState::Running;
            self.tasks[next] = Some(process);
        }
    }

    fn current_task(&self) -> Option<Process> {
        self.current.and_then(|index| self.tasks[index])
    }

    fn current_task_id(&self) -> usize {
        self.current_task()
            .map(|task| task.id)
            .unwrap_or(usize::MAX)
    }
}
