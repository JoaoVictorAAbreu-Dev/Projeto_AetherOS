use core::sync::atomic::{AtomicBool, Ordering};

use spin::Mutex;
use x86_64::instructions::interrupts as cpu_interrupts;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{
    InterruptStackFrame,
    PageFaultErrorCode,
};

use crate::arch::x86_64::pit;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn as_usize(self) -> usize {
        self.as_u8() as usize
    }
}

pub fn init() {
    {
        let mut pics = PICS.lock();
        unsafe {
            pics.initialize();
        }
    }

    pit::init(100);
    cpu_interrupts::enable();
    INITIALIZED.store(true, Ordering::Release);
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::println!("AetherOS: breakpoint exception");
    crate::println!("{stack_frame:#?}");
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    crate::println!("AetherOS: double fault, error_code={}", error_code);
    crate::println!("{stack_frame:#?}");
    crate::core::panic::halt_forever()
}

pub extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    crate::println!("AetherOS: general protection fault, error_code={}", error_code);
    crate::println!("{stack_frame:#?}");
    crate::core::panic::halt_forever()
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    crate::println!("AetherOS: page fault accessing {:?}", Cr2::read());
    crate::println!("AetherOS: page fault error_code={:?}", error_code);
    crate::println!("{stack_frame:#?}");
    crate::core::panic::halt_forever()
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    crate::drivers::timer::on_tick();
    notify_end_of_interrupt(InterruptIndex::Timer);
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    crate::drivers::keyboard::on_interrupt();
    notify_end_of_interrupt(InterruptIndex::Keyboard);
}

pub fn notify_end_of_interrupt(index: InterruptIndex) {
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe {
            PICS.lock().notify_end_of_interrupt(index.as_u8());
        }
    }
}

struct ChainedPics {
    offset1: u8,
    offset2: u8,
}

impl ChainedPics {
    const fn new(offset1: u8, offset2: u8) -> Self {
        Self { offset1, offset2 }
    }

    unsafe fn initialize(&mut self) {
        use x86_64::instructions::port::Port;

        let mut cmd1 = Port::<u8>::new(0x20);
        let mut data1 = Port::<u8>::new(0x21);
        let mut cmd2 = Port::<u8>::new(0xA0);
        let mut data2 = Port::<u8>::new(0xA1);

        let saved_mask1 = data1.read();
        let saved_mask2 = data2.read();

        cmd1.write(0x11);
        data1.write(self.offset1);
        data1.write(4);
        data1.write(0x01);
        data1.write(saved_mask1 & !0b11);

        cmd2.write(0x11);
        data2.write(self.offset2);
        data2.write(2);
        data2.write(0x01);
        data2.write(saved_mask2 & !0b11);
    }

    unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        use x86_64::instructions::port::Port;

        if interrupt_id >= self.offset2 {
            let mut cmd2 = Port::<u8>::new(0xA0);
            cmd2.write(0x20);
        }

        let mut cmd1 = Port::<u8>::new(0x20);
        cmd1.write(0x20);
    }
}
