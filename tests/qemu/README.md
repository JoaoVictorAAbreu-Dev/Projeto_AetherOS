# QEMU Tests

This directory documents the runtime verification contract for AetherOS on QEMU.

## Current Runtime Entry Point

Use:

```bash
cargo run -p xtask -- boot-check
cargo run -p xtask -- shell-check
```

This command:

- stages the UEFI boot tree
- launches QEMU in headless mode
- writes serial output to `dist/serial.log`
- waits for a success marker from the kernel
- fails on timeout or early QEMU exit

The shell interaction check additionally:

- opens a QEMU monitor socket
- injects `help`, `info`, and `ls`
- verifies expected serial output from the shell
- captures `dist/framebuffer-shell.ppm`

## Expected Success Marker

The current boot success marker is:

```text
AetherOS: kernel initialized
```

The expected early-boot stage sequence is captured in [`../fixtures/qemu-boot-markers.txt`](../fixtures/qemu-boot-markers.txt).

## Failure Triage

When `boot-check` fails, inspect:

1. the serial log tail shown by `xtask`
2. `dist/serial.log`
3. the current boot-stage marker emitted by the kernel
4. QEMU stderr if the process exited early

## Scope

This is currently a boot-regression gate, not a full interaction test.

Covered:

- firmware to Limine handoff
- kernel entry
- boot-info collection
- generic kernel initialization
- scripted shell interaction through keyboard injection
- framebuffer dump generation through QEMU screendump

Not covered by the current boot gate:

- broad keyboard layout coverage
- semantic framebuffer content assertions beyond image existence/format
