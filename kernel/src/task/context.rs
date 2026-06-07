#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskContext {
    pub id: usize,
    pub timeslice_ticks: u64,
}

impl TaskContext {
    pub const fn new(id: usize, timeslice_ticks: u64) -> Self {
        Self {
            id,
            timeslice_ticks,
        }
    }
}
