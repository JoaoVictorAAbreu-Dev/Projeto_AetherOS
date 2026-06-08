# Boot Flow

## Selected Direction

AetherOS adopts Limine as the initial boot protocol.

## Boot Sequence

1. Firmware transfers control to the bootloader.
2. Limine loads the kernel image and prepares boot information.
3. The kernel `_start` entrypoint validates the negotiated Limine base revision.
4. Early serial output is initialized before wider subsystem setup.
5. The boot layer emits explicit serial-stage markers for revision validation and boot-info collection.
6. The boot layer collects the HHDM offset and memory-map metadata into `BootInfo`.
7. `BootInfo` is handed off to generic kernel initialization.
8. Memory management and later subsystem bring-up can build on that stable contract.

## Current Implementation Boundary

Implemented in this milestone:

- Limine request markers and request section layout
- Kernel `_start` entrypoint
- Base revision validation
- Serial-first boot diagnostics
- Explicit boot-stage markers for entry, Limine negotiation, HHDM, memmap, framebuffer, and kernel handoff
- `BootInfo` contract with HHDM offset, memory-map entry count, and optional framebuffer metadata
- Initial framebuffer-backed visual boot rendering

Intentionally deferred:

- Keyboard
- Interrupt handling
- Memory allocator setup

## Why Limine

- Reduces early bootstrap complexity
- Keeps focus on kernel design instead of firmware plumbing
- Supports later growth without forcing a rewrite of the kernel layout
