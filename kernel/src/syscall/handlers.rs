use crate::syscall::table::Syscall;

pub fn dispatch(syscall: Syscall, arg0: u64) -> u64 {
    match syscall {
        Syscall::WriteDebug => {
            crate::println!("AetherOS syscall debug: {}", arg0);
            0
        }
        Syscall::QueryTicks => crate::drivers::timer::ticks(),
    }
}
