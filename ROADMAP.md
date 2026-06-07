# Roadmap

## v0.1.0 - Bring-up

- Cargo workspace and target configuration
- Limine-based boot flow definition
- QEMU execution scaffolding
- Minimal kernel entry layout
- Serial-first observability strategy
- Boot metadata handoff contract
- Contributor and documentation foundation for public growth
- Initial framebuffer-backed boot visual

## v0.2.0 - CPU and Exceptions

- GDT and IDT layout
- Exception stubs and handlers
- Early timer integration
- Safe halt path

## v0.3.0 - Memory

- Boot memory map parsing
- Physical frame allocator
- Virtual memory abstraction
- Kernel heap initialization

## v0.4.0 - Tasks

- Kernel thread model
- Cooperative scheduling baseline
- Context representation
- Basic synchronization primitives

## v0.5.0 - Syscalls

- System call ABI definition
- Dispatch table
- First kernel/user boundary contracts

## v0.6.0 - Drivers

- Serial
- Framebuffer
- Keyboard
- Timer

## v0.7.0 - Filesystem

- Initramfs support
- Minimal VFS abstraction
- Read-only boot-time file access

## v0.8.0 - Processes

- Process abstraction
- Address-space separation
- User-mode execution evolution

## v0.9.0 - Stability

- Regression tests in QEMU
- Better diagnostics
- Debugging workflow hardening

## v1.0.0 - Academic Stable Base

- Stable boot flow
- Core memory subsystem
- Interrupt and scheduler baseline
- Minimal syscall surface
- Documented architecture and contributor workflow
