use core::sync::atomic::{AtomicBool, Ordering};

use spin::Mutex;
use x86_64::instructions::port::Port;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static SHIFT_PRESSED: Mutex<bool> = Mutex::new(false);

pub enum KeyEvent {
    Char(char),
    Enter,
    Backspace,
}

pub fn initialize() {
    INITIALIZED.store(true, Ordering::Release);
    crate::println!("AetherOS: keyboard driver initialized");
}

pub fn is_initialized() -> bool {
    INITIALIZED.load(Ordering::Acquire)
}

pub fn on_interrupt() {
    if !INITIALIZED.load(Ordering::Acquire) {
        return;
    }

    let scancode = unsafe {
        let mut port = Port::<u8>::new(0x60);
        port.read()
    };

    if let Some(event) = decode_scancode(scancode) {
        crate::shell::on_key(event);
    }
}

fn decode_scancode(scancode: u8) -> Option<KeyEvent> {
    match scancode {
        0x2A | 0x36 => {
            *SHIFT_PRESSED.lock() = true;
            None
        }
        0xAA | 0xB6 => {
            *SHIFT_PRESSED.lock() = false;
            None
        }
        release if release & 0x80 != 0 => None,
        0x1C => Some(KeyEvent::Enter),
        0x0E => Some(KeyEvent::Backspace),
        make_code => decode_printable(make_code).map(KeyEvent::Char),
    }
}

fn decode_printable(scancode: u8) -> Option<char> {
    let shift = *SHIFT_PRESSED.lock();
    let character = match scancode {
        0x02 => {
            if shift {
                '!'
            } else {
                '1'
            }
        }
        0x03 => {
            if shift {
                '@'
            } else {
                '2'
            }
        }
        0x04 => {
            if shift {
                '#'
            } else {
                '3'
            }
        }
        0x05 => {
            if shift {
                '$'
            } else {
                '4'
            }
        }
        0x06 => {
            if shift {
                '%'
            } else {
                '5'
            }
        }
        0x07 => {
            if shift {
                '^'
            } else {
                '6'
            }
        }
        0x08 => {
            if shift {
                '&'
            } else {
                '7'
            }
        }
        0x09 => {
            if shift {
                '*'
            } else {
                '8'
            }
        }
        0x0A => {
            if shift {
                '('
            } else {
                '9'
            }
        }
        0x0B => {
            if shift {
                ')'
            } else {
                '0'
            }
        }
        0x0C => {
            if shift {
                '_'
            } else {
                '-'
            }
        }
        0x0D => {
            if shift {
                '+'
            } else {
                '='
            }
        }
        0x10 => alpha('q', shift),
        0x11 => alpha('w', shift),
        0x12 => alpha('e', shift),
        0x13 => alpha('r', shift),
        0x14 => alpha('t', shift),
        0x15 => alpha('y', shift),
        0x16 => alpha('u', shift),
        0x17 => alpha('i', shift),
        0x18 => alpha('o', shift),
        0x19 => alpha('p', shift),
        0x1A => {
            if shift {
                '{'
            } else {
                '['
            }
        }
        0x1B => {
            if shift {
                '}'
            } else {
                ']'
            }
        }
        0x1E => alpha('a', shift),
        0x1F => alpha('s', shift),
        0x20 => alpha('d', shift),
        0x21 => alpha('f', shift),
        0x22 => alpha('g', shift),
        0x23 => alpha('h', shift),
        0x24 => alpha('j', shift),
        0x25 => alpha('k', shift),
        0x26 => alpha('l', shift),
        0x27 => {
            if shift {
                ':'
            } else {
                ';'
            }
        }
        0x28 => {
            if shift {
                '"'
            } else {
                '\''
            }
        }
        0x29 => {
            if shift {
                '~'
            } else {
                '`'
            }
        }
        0x2B => {
            if shift {
                '|'
            } else {
                '\\'
            }
        }
        0x2C => alpha('z', shift),
        0x2D => alpha('x', shift),
        0x2E => alpha('c', shift),
        0x2F => alpha('v', shift),
        0x30 => alpha('b', shift),
        0x31 => alpha('n', shift),
        0x32 => alpha('m', shift),
        0x33 => {
            if shift {
                '<'
            } else {
                ','
            }
        }
        0x34 => {
            if shift {
                '>'
            } else {
                '.'
            }
        }
        0x35 => {
            if shift {
                '?'
            } else {
                '/'
            }
        }
        0x39 => ' ',
        _ => return None,
    };

    Some(character)
}

const fn alpha(lower: char, shift: bool) -> char {
    if shift {
        lower.to_ascii_uppercase()
    } else {
        lower
    }
}
