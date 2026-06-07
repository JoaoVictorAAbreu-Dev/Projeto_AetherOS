use aether_bootinfo::{
    BootInfo,
    FramebufferInfo,
    MemoryRegion,
    MemoryRegionKind,
    MAX_MEMORY_REGIONS,
};
use limine::request::{
    FramebufferRequest,
    HhdmRequest,
    MemoryMapRequest,
    RequestsEndMarker,
    RequestsStartMarker,
    StackSizeRequest,
};
use limine::BaseRevision;
use spin::Once;

use crate::arch::x86_64::serial;

#[used]
#[unsafe(link_section = ".limine_requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(1024 * 1024);

#[used]
#[unsafe(link_section = ".limine_requests_start")]
static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".limine_requests_end")]
static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();

static BOOT_INFO: Once<BootInfo> = Once::new();

pub fn enter() -> ! {
    serial::init();
    serial::write_line("AetherOS: boot entry reached");

    if !BASE_REVISION.is_supported() {
        serial::write_line("AetherOS: unsupported Limine base revision");
        crate::core::panic::halt_forever();
    }

    let boot_info = BOOT_INFO.call_once(collect_boot_info);

    serial::write_line("AetherOS: boot context collected");
    crate::run(boot_info)
}

fn collect_boot_info() -> BootInfo {
    let hhdm_response = HHDM_REQUEST
        .get_response()
        .expect("Limine did not provide an HHDM response");
    let memory_map_response = MEMORY_MAP_REQUEST
        .get_response()
        .expect("Limine did not provide a memory map response");
    let mut regions = [MemoryRegion::EMPTY; MAX_MEMORY_REGIONS];

    for (index, entry) in memory_map_response.entries().iter().take(MAX_MEMORY_REGIONS).enumerate() {
        regions[index] = MemoryRegion::new(
            entry.base(),
            entry.length(),
            classify_memory_region(entry.entry_type() as u64),
        );
    }

    let framebuffer = FRAMEBUFFER_REQUEST
        .get_response()
        .and_then(|response| response.framebuffers().next())
        .map(|framebuffer| {
            FramebufferInfo::new(
                framebuffer.addr() as u64,
                framebuffer.width() as u32,
                framebuffer.height() as u32,
                framebuffer.pitch() as u32,
                framebuffer.bpp(),
            )
        });

    BootInfo::new(
        hhdm_response.offset() as u64,
        memory_map_response.entries().len(),
        framebuffer,
        regions,
    )
}

fn classify_memory_region(entry_type: u64) -> MemoryRegionKind {
    match entry_type {
        0 => MemoryRegionKind::Usable,
        5 => MemoryRegionKind::Reclaimable,
        6 => MemoryRegionKind::Kernel,
        7 => MemoryRegionKind::Framebuffer,
        1 | 2 | 3 | 4 => MemoryRegionKind::Reserved,
        _ => MemoryRegionKind::Unknown,
    }
}
