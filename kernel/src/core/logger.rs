use core::fmt;

pub fn print(args: fmt::Arguments<'_>) {
    let _ = crate::arch::x86_64::serial::write_fmt(args);
}
