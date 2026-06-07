use aether_kernel::arch::x86_64::interrupts::InterruptIndex;

#[test]
fn interrupt_indexes_match_expected_pic_offsets() {
    assert_eq!(InterruptIndex::Timer.as_u8(), 32);
    assert_eq!(InterruptIndex::Keyboard.as_u8(), 33);
}
