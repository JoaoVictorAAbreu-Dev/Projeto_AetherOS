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
cargo run -p xtask -- boot-check
```

This command currently performs:

- `cargo fmt --all -- --check`
- `cargo +nightly check --workspace`
- `cargo +nightly test -p aether-bootinfo -p xtask`

The boot verification command currently performs:

- `cargo run -p xtask -- stage`
- headless QEMU launch with `-display none`
- serial log capture to `dist/serial.log`
- success detection using the `AetherOS: kernel initialized` marker
- timeout-based failure with serial log tail included in the error message

For QEMU runtime capture:

- `AETHER_QEMU_DISPLAY=none`
- `AETHER_QEMU_SERIAL=file:dist/serial.log`
- `AETHER_BOOT_TIMEOUT_SECS=30`
- `AETHER_BOOT_SUCCESS_MARKER="AetherOS: kernel initialized"`

## Current Manual Validation Targets

- boot reaches `_start`
- serial logging works
- framebuffer visual renders when available
- timer interrupts generate ticks
- keyboard input reaches the shell
- shell commands `help`, `ls`, `cat`, `mem`, and `tasks` work
- Shift-modified input and backspace behave consistently during shell entry
- scheduler output clearly reports its cooperative demo model and current task state

## Current Automated Smoke Tests

- boot-info structure test
- workspace tooling smoke test
- headless QEMU boot marker verification
