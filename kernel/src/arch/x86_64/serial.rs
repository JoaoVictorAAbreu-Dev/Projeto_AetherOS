use core::fmt;
use core::fmt::Write;

use spin::{Mutex, Once};
use x86_64::instructions::port::Port;

const COM1_BASE: u16 = 0x3F8;
const LINE_STATUS_DATA_READY: u8 = 1 << 0;
const LINE_STATUS_TRANSMITTER_EMPTY: u8 = 1 << 5;

static SERIAL1: Once<Mutex<SerialPort>> = Once::new();

pub fn init() {
    let mut serial = serial_port().lock();
    serial.init();
}

pub fn write_line(message: &str) {
    let _ = write_fmt(format_args!("{message}\n"));
}

pub fn write_fmt(args: fmt::Arguments<'_>) -> fmt::Result {
    let mut serial = serial_port().lock();
    serial.write_fmt(args)
}

fn serial_port() -> &'static Mutex<SerialPort> {
    SERIAL1.call_once(|| Mutex::new(unsafe { SerialPort::new(COM1_BASE) }))
}

struct SerialPort {
    data: Port<u8>,
    interrupt_enable: Port<u8>,
    fifo_control: Port<u8>,
    line_control: Port<u8>,
    modem_control: Port<u8>,
    line_status: Port<u8>,
}

impl SerialPort {
    const unsafe fn new(base: u16) -> Self {
        Self {
            data: Port::new(base),
            interrupt_enable: Port::new(base + 1),
            fifo_control: Port::new(base + 2),
            line_control: Port::new(base + 3),
            modem_control: Port::new(base + 4),
            line_status: Port::new(base + 5),
        }
    }

    fn init(&mut self) {
        unsafe {
            self.interrupt_enable.write(0x00);
            self.line_control.write(0x80);
            self.data.write(0x03);
            self.interrupt_enable.write(0x00);
            self.line_control.write(0x03);
            self.fifo_control.write(0xC7);
            self.modem_control.write(0x0B);
        }
    }

    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.write_raw_byte(b'\r');
        }

        self.write_raw_byte(byte);
    }

    fn write_raw_byte(&mut self, byte: u8) {
        while !self.can_transmit() {}

        unsafe {
            self.data.write(byte);
        }
    }

    fn can_transmit(&mut self) -> bool {
        unsafe { self.line_status.read() & LINE_STATUS_TRANSMITTER_EMPTY != 0 }
    }

    #[allow(dead_code)]
    fn has_data(&mut self) -> bool {
        unsafe { self.line_status.read() & LINE_STATUS_DATA_READY != 0 }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }

        Ok(())
    }
}
