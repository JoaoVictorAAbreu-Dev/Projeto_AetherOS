#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u64)]
pub enum Syscall {
    WriteDebug = 0,
    QueryTicks = 1,
    QueryTaskCount = 2,
    QueryHeapUsage = 3,
    QueryUsableFrames = 4,
}
