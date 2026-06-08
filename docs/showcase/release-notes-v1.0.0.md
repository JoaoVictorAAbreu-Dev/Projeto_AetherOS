# AetherOS v1.0.0

## Milestone

Academic stable base for a Rust-first `x86_64` kernel running on QEMU through Limine UEFI boot.

## What Was Implemented

- Limine-based UEFI boot flow for `x86_64`
- early serial diagnostics and boot-stage observability
- framebuffer-backed visual boot surface
- interrupt, timer, and keyboard bring-up
- boot memory map snapshot and kernel heap foundation
- cooperative scheduler observability
- minimal syscall inspection hooks
- in-kernel shell with initramfs-backed VFS
- `xtask` workflow for build, stage, run, test, and headless boot verification
- CI validation for host checks, staged boot assets, and headless boot checks

## Architecture Decisions

- boot complexity is intentionally delegated to Limine so kernel work stays focused on architecture and subsystem design
- architecture-specific code remains isolated under `kernel/src/arch/x86_64`
- bootloader data is normalized into `BootInfo` before generic kernel initialization
- runtime validation is split between host-safe checks and QEMU boot verification because the kernel uses a custom `no_std` target

## Validation Used

```bash
cargo run -p xtask -- test
cargo run -p xtask -- stage
cargo run -p xtask -- boot-check
```

Boot success marker:

```text
AetherOS: kernel initialized
```

## Scope of v1

Included:

- stable kernel bring-up path
- basic memory inspection
- keyboard-to-shell interactive path
- minimal scheduler state reporting
- reproducible tester workflow

Out of scope:

- user-mode processes
- persistent filesystem support
- advanced driver set
- automated framebuffer assertions

## Suggested Release Assets

- `dist/serial.log` from a successful `xtask boot-check`
- one framebuffer screenshot
- one short boot or shell GIF

## Next Contributor Opportunities

- automated shell interaction tests in QEMU
- richer VFS path handling
- user-mode execution foundation
- improved debugging and symbol tooling
