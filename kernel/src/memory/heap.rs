use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 128 * 1024;

#[global_allocator]
static GLOBAL_ALLOCATOR: BumpAllocator = BumpAllocator::new();

static HEAP_SPACE: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub fn initialize() {
    GLOBAL_ALLOCATOR.initialize(HEAP_SPACE.as_ptr() as usize, HEAP_SIZE);
}

pub fn heap_size() -> usize {
    HEAP_SIZE
}

pub fn heap_used() -> usize {
    let start = GLOBAL_ALLOCATOR.heap_start.load(Ordering::Acquire);
    let next = GLOBAL_ALLOCATOR.next.load(Ordering::Acquire);

    if start == 0 || next < start {
        0
    } else {
        next - start
    }
}

struct BumpAllocator {
    heap_start: AtomicUsize,
    heap_end: AtomicUsize,
    next: AtomicUsize,
}

impl BumpAllocator {
    const fn new() -> Self {
        Self {
            heap_start: AtomicUsize::new(0),
            heap_end: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
        }
    }

    fn initialize(&self, start: usize, size: usize) {
        self.heap_start.store(start, Ordering::Release);
        self.heap_end.store(start + size, Ordering::Release);
        self.next.store(start, Ordering::Release);
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align_mask = layout.align().saturating_sub(1);

        loop {
            let current = self.next.load(Ordering::Acquire);
            let heap_end = self.heap_end.load(Ordering::Acquire);

            if current == 0 {
                return null_mut();
            }

            let aligned = (current + align_mask) & !align_mask;
            let next = match aligned.checked_add(layout.size()) {
                Some(next) => next,
                None => return null_mut(),
            };

            if next > heap_end {
                return null_mut();
            }

            if self
                .next
                .compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return aligned as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
