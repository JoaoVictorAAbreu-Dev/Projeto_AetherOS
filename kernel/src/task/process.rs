use x86_64::VirtAddr;

use crate::memory::address_space::AddressSpace;
use crate::task::context::TaskContext;
use crate::task::image::UserProgramImage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Process {
    pub id: usize,
    pub name: &'static str,
    pub context: TaskContext,
    pub state: ProcessState,
    pub execution_mode: ExecutionMode,
    pub address_space: AddressSpace,
    pub image: Option<UserProgramImage>,
}

impl Process {
    pub const fn new(id: usize, name: &'static str) -> Self {
        Self {
            id,
            name,
            context: TaskContext::new(id, 0),
            state: ProcessState::Ready,
            execution_mode: ExecutionMode::Kernel,
            address_space: AddressSpace::new(VirtAddr::new(0)),
            image: None,
        }
    }

    pub const fn new_user_stub(
        id: usize,
        name: &'static str,
        image: UserProgramImage,
        hhdm_offset: u64,
        user_range_start: u64,
        user_range_end: u64,
    ) -> Self {
        Self {
            id,
            name,
            context: TaskContext::new(id, 0),
            state: ProcessState::Ready,
            execution_mode: ExecutionMode::User,
            address_space: AddressSpace::with_user_range(
                VirtAddr::new(hhdm_offset),
                VirtAddr::new(user_range_start),
                VirtAddr::new(user_range_end),
            ),
            image: Some(image),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Idle,
}

impl ProcessState {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Running => "running",
            Self::Idle => "idle",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionMode {
    Kernel,
    User,
}

impl ExecutionMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Kernel => "kernel",
            Self::User => "user",
        }
    }
}
