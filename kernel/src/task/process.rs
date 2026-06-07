use crate::task::context::TaskContext;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Process {
    pub id: usize,
    pub context: TaskContext,
    pub state: ProcessState,
}

impl Process {
    pub const fn new(id: usize) -> Self {
        Self {
            id,
            context: TaskContext::new(id),
            state: ProcessState::Ready,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
}
