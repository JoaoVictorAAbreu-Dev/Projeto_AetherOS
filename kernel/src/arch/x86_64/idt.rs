use spin::Once;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::arch::x86_64::{gdt, interrupts};

static IDT: Once<InterruptDescriptorTable> = Once::new();

pub fn init() {
    IDT.call_once(build_idt).load();
}

fn build_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint
        .set_handler_fn(interrupts::breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(interrupts::double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt.general_protection_fault
        .set_handler_fn(interrupts::general_protection_fault_handler);
    idt.page_fault
        .set_handler_fn(interrupts::page_fault_handler);
    idt[interrupts::InterruptIndex::Timer.as_u8()]
        .set_handler_fn(interrupts::timer_interrupt_handler);
    idt[interrupts::InterruptIndex::Keyboard.as_u8()]
        .set_handler_fn(interrupts::keyboard_interrupt_handler);
    idt
}
