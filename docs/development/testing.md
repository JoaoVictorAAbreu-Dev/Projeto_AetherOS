# Testing

## Layers

- Formatting and linting
- Host-side crate tests
- QEMU integration tests
- Regression tests

## Current Status

The repository now includes real kernel bring-up stages plus a minimal shell and in-memory VFS. Host validation and runtime validation are intentionally split because the kernel crate is `no_std` and targets a custom `x86_64-aetheros` JSON target instead of the host ABI.

## Current Automated Validation

```bash
cargo run -p xtask -- test
```

This command currently performs:

- `cargo fmt --all -- --check`
- `cargo +nightly check --workspace`
- `cargo +nightly test -p aether-bootinfo -p xtask`

For QEMU runtime capture:

- `AETHER_QEMU_DISPLAY=none`
- `AETHER_QEMU_SERIAL=file:dist/serial.log`

## Current Manual Validation Targets

- boot reaches `_start`
- serial logging works
- framebuffer visual renders when available
- timer interrupts generate ticks
- keyboard input reaches the shell
- shell commands `help`, `ls`, `cat`, `mem`, and `tasks` work

## Current Automated Smoke Tests

- boot-info structure test
- workspace tooling smoke test
