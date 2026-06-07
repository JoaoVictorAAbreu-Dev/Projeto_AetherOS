# Testing

## Layers

- Formatting and linting
- Host-side crate tests
- QEMU integration tests
- Regression tests

## Current Status

The repository now includes real kernel bring-up stages plus a minimal shell and in-memory VFS. In this environment, however, Rust and QEMU are still unavailable, so runtime validation remains pending here.

## Current Manual Validation Targets

- boot reaches `_start`
- serial logging works
- framebuffer visual renders when available
- timer interrupts generate ticks
- keyboard input reaches the shell
- shell commands `help`, `ls`, `cat`, `mem`, and `tasks` work

## Current Automated Smoke Tests

- boot-info structure test
- interrupt index offset test
- memory-region and higher-half mapping test
