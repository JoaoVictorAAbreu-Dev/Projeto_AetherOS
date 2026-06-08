#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserProgramImage {
    pub name: &'static str,
    pub entry_point: u64,
    pub stack_top: u64,
    pub image_size: usize,
}

impl UserProgramImage {
    pub const fn new(
        name: &'static str,
        entry_point: u64,
        stack_top: u64,
        image_size: usize,
    ) -> Self {
        Self {
            name,
            entry_point,
            stack_top,
            image_size,
        }
    }
}
