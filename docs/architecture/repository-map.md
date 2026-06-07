# Repository Map

## Purpose

This page explains why the repository is organized the way it is, so contributors can navigate it without guessing.

## Directory Roles

- `boot/`: boot protocol notes and bootloader-facing assets
- `config/`: target, linker, boot, and QEMU configuration
- `crates/`: reusable low-level support crates shared across the workspace
- `docs/`: architecture, onboarding, tutorials, and project strategy
- `kernel/`: kernel implementation
- `scripts/`: local workflow helpers
- `tests/`: integration and future QEMU-driven validation
- `tools/`: project-side tooling and automation helpers
- `user/`: reserved for future userland-facing artifacts

## Design Intent

The repository is structured to keep these concerns separate:

- boot boundary
- architecture-specific code
- generic kernel logic
- reusable contracts
- contributor-facing knowledge
