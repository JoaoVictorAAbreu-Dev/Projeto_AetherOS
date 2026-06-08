# v1 Validation

This page records the intended validation contract for the AetherOS v1 baseline.

## Required Commands

```bash
cargo run -p xtask -- test
cargo run -p xtask -- stage
cargo run -p xtask -- boot-check
```

## Success Criteria

- host-safe workspace validation completes without errors
- the UEFI boot tree is staged successfully
- QEMU headless boot reaches the serial marker:

```text
AetherOS: kernel initialized
```

## Evidence to Capture

- the exact commands used
- `dist/serial.log` or the boot-check output
- screenshots or GIFs if framebuffer output is being demonstrated publicly

## Scope of v1

Validated:

- Limine-based UEFI boot
- early serial diagnostics
- boot-info handoff
- interrupt/timer/keyboard bring-up
- memory inspection
- minimal scheduler observability
- in-kernel shell and initramfs-backed VFS

Out of scope for v1:

- user-mode process execution
- advanced driver coverage
- full filesystem persistence
- automated framebuffer assertions

## Post-v1 Runtime Extension

The post-v1 runtime workflow can also use:

```bash
cargo run -p xtask -- shell-check
```

This extends validation by:

- injecting shell commands through the QEMU monitor
- checking serial output for interactive shell responses
- capturing a framebuffer dump for visual regression groundwork
