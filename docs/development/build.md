# Build

The repository uses a Cargo workspace layout.

## Current Build Reality

The kernel has real bring-up code, interrupt foundations, memory foundations, and a minimal in-kernel shell. The repository now stages a reproducible UEFI boot tree through `xtask` and launches it through Limine on QEMU.

## Current Validation Commands

```bash
cargo run -p xtask -- test
cargo run -p xtask -- build
cargo run -p xtask -- stage
cargo run -p xtask -- run
```

## Build Boundary

Implemented:

- workspace compilation path
- architecture-specific kernel layout
- initramfs/VFS/shell integration in source
- automatic Limine bundle download
- staged FAT-backed ESP directory for QEMU UEFI boot
- reusable `dist/edk2-x86_64-vars.fd` copy for writable firmware variables

Still pending:

- ISO release packaging
- automated screenshot or serial-log artifact capture during QEMU runs
