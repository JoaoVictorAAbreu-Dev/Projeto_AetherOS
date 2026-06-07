#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskContext {
    pub id: usize,
}

impl TaskContext {
    pub const fn new(id: usize) -> Self {
        Self { id }
    }
}
