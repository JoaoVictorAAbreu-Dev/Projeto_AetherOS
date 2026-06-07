use crate::syscall::table::Syscall;

pub fn dispatch(syscall: Syscall, arg0: u64) -> u64 {
    match syscall {
        Syscall::WriteDebug => {
            crate::println!("AetherOS syscall debug: {}", arg0);
            0
        }
        Syscall::QueryTicks => crate::drivers::timer::ticks(),
        Syscall::QueryTaskCount => crate::task::scheduler::task_count() as u64,
        Syscall::QueryHeapUsage => crate::memory::heap::heap_used() as u64,
        Syscall::QueryUsableFrames => crate::memory::frame_allocator::usable_frame_count() as u64,
    }
}
