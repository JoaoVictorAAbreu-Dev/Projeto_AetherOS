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
- `tests/`: host-safe integration notes, fixtures, and QEMU validation references
- `tools/`: project-side tooling and automation helpers
- `user/`: userland-facing space kept available for later milestones without affecting the v1 kernel layout

Within `kernel/`, the VFS now separates:

- read-only boot assets from `initramfs`
- writable overlay files managed in memory
- storage-facing expansion points under `drivers/storage`

The task and memory layers also now carry explicit post-v1 user-mode intent through:

- process execution mode metadata
- address-space kind metadata
- user program image descriptors

## Design Intent

The repository is structured to keep these concerns separate:

- boot boundary
- architecture-specific code
- generic kernel logic
- reusable contracts
- contributor-facing knowledge
