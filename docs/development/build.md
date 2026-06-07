# Build

The repository uses a Cargo workspace layout.

## Current Build Reality

The kernel has real bring-up code, interrupt foundations, memory foundations, and a minimal in-kernel shell. What is still missing from a fully reproducible local run is the final artifact assembly path that packages the kernel with Limine into a bootable image.

## Current Validation Commands

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

## Build Boundary

Implemented:

- workspace compilation path
- architecture-specific kernel layout
- initramfs/VFS/shell integration in source

Still pending:

- bootable image assembly flow
- scripted Limine image creation
- end-to-end QEMU run automation
