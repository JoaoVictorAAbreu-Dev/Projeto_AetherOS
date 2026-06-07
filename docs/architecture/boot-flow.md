# Boot Flow

## Selected Direction

AetherOS adopts Limine as the initial boot protocol.

## Boot Sequence

1. Firmware transfers control to the bootloader.
2. Limine loads the kernel image and prepares boot information.
3. The kernel `_start` entrypoint validates the negotiated Limine base revision.
4. Early serial output is initialized before wider subsystem setup.
5. The boot layer collects the HHDM offset and memory-map metadata into `BootInfo`.
6. `BootInfo` is handed off to generic kernel initialization.
7. Memory management and later subsystem bring-up can build on that stable contract.

## Current Implementation Boundary

Implemented in this milestone:

- Limine request markers and request section layout
- Kernel `_start` entrypoint
- Base revision validation
- Serial-first boot diagnostics
- `BootInfo` contract with HHDM offset and memory-map entry count

Intentionally deferred:

- VGA
- Keyboard
- Interrupt handling
- Memory allocator setup

## Why Limine

- Reduces early bootstrap complexity
- Keeps focus on kernel design instead of firmware plumbing
- Supports later growth without forcing a rewrite of the kernel layout
