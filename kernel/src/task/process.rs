use crate::task::context::TaskContext;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Process {
    pub id: usize,
    pub name: &'static str,
    pub context: TaskContext,
    pub state: ProcessState,
}

impl Process {
    pub const fn new(id: usize, name: &'static str) -> Self {
        Self {
            id,
            name,
            context: TaskContext::new(id, 0),
            state: ProcessState::Ready,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Idle,
}
