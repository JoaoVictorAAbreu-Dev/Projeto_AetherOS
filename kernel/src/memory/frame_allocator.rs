use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use aether_bootinfo::{BootInfo, MemoryRegionKind};
use spin::Once;

const FRAME_SIZE: u64 = 4096;

static ALLOCATOR: Once<FrameAllocator> = Once::new();
static USABLE_FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn initialize(boot_info: &BootInfo) {
    let mut start = 0;
    let mut end = 0;
    let mut total_frames = 0usize;

    for region in boot_info
        .memory_regions
        .iter()
        .take(boot_info.memory_map_entries.min(boot_info.memory_regions.len()))
    {
        if region.kind != MemoryRegionKind::Usable || region.length < FRAME_SIZE {
            continue;
        }

        let aligned_start = align_up(region.base, FRAME_SIZE);
        let aligned_end = align_down(region.end(), FRAME_SIZE);

        if aligned_end <= aligned_start {
            continue;
        }

        if start == 0 {
            start = aligned_start;
            end = aligned_end;
        }

        total_frames += ((aligned_end - aligned_start) / FRAME_SIZE) as usize;
    }

    USABLE_FRAME_COUNT.store(total_frames, Ordering::Release);
    ALLOCATOR.call_once(|| FrameAllocator::new(start, end));
}

pub fn allocate_frame() -> Option<u64> {
    ALLOCATOR.get().and_then(FrameAllocator::allocate_frame)
}

pub fn usable_frame_count() -> usize {
    USABLE_FRAME_COUNT.load(Ordering::Acquire)
}

fn align_up(value: u64, align: u64) -> u64 {
    if value % align == 0 {
        value
    } else {
        value + (align - (value % align))
    }
}

fn align_down(value: u64, align: u64) -> u64 {
    value - (value % align)
}

pub struct FrameAllocator {
    current: AtomicU64,
    end: u64,
}

impl FrameAllocator {
    fn new(start: u64, end: u64) -> Self {
        Self {
            current: AtomicU64::new(start),
            end,
        }
    }

    fn allocate_frame(&self) -> Option<u64> {
        loop {
            let current = self.current.load(Ordering::Acquire);

            if current == 0 || current.saturating_add(FRAME_SIZE) > self.end {
                return None;
            }

            let next = current + FRAME_SIZE;
            if self
                .current
                .compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return Some(current);
            }
        }
    }
}
