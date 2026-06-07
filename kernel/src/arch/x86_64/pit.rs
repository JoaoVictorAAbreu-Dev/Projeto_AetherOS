use x86_64::instructions::port::Port;

const PIT_COMMAND: u16 = 0x43;
const PIT_CHANNEL0: u16 = 0x40;
const PIT_BASE_FREQUENCY: u32 = 1_193_182;

pub fn init(frequency_hz: u32) {
    let divisor = (PIT_BASE_FREQUENCY / frequency_hz.max(19)).clamp(1, u16::MAX as u32) as u16;

    unsafe {
        let mut command = Port::<u8>::new(PIT_COMMAND);
        let mut channel0 = Port::<u8>::new(PIT_CHANNEL0);

        command.write(0x36);
        channel0.write((divisor & 0xFF) as u8);
        channel0.write((divisor >> 8) as u8);
    }
}
