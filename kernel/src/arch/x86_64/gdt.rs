use spin::Once;
use x86_64::instructions::segmentation::{Segment, CS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static GDT_STATE: Once<GdtState> = Once::new();

pub fn init() {
    let state = GDT_STATE.call_once(GdtState::new);

    state.gdt.load();

    unsafe {
        CS::set_reg(state.code_selector);
        load_tss(state.tss_selector);
    }
}

struct GdtState {
    gdt: GlobalDescriptorTable,
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

impl GdtState {
    fn new() -> Self {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(tss()));

        Self {
            gdt,
            code_selector,
            tss_selector,
        }
    }
}

fn tss() -> &'static TaskStateSegment {
    static TSS: Once<TaskStateSegment> = Once::new();

    TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&STACK);
            stack_start + STACK_SIZE as u64
        };
        tss
    })
}
