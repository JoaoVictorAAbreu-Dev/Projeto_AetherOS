# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-06-08

### Added

- Initial repository structure
- Workspace scaffolding
- Architecture documentation
- ADR for boot protocol selection
- Limine boot request layout
- Early serial logging path
- BootInfo handoff for the kernel bring-up stage
- Initial framebuffer-backed visual boot stage
- Professional README focused on education and discoverability
- Contributor onboarding tutorial
- GitHub issue templates and stronger PR template
- Security and support policies
- Wiki, demo, and release planning documentation
- Headless QEMU boot validation through `xtask boot-check`
- CI jobs for host validation, staged UEFI tree generation, and headless boot verification
- Early boot-stage markers and panic-stage reporting for bring-up diagnostics
- Stabilized keyboard input path with Shift-aware shell input
- Cleaner VFS-backed shell file reads and clearer scheduler observability

### Validation

- `cargo +nightly-x86_64-pc-windows-gnu fmt --all`
- `cargo +nightly-x86_64-pc-windows-gnu check --workspace`
- `cargo run -p xtask -- test`
- `cargo run -p xtask -- stage`
- `cargo run -p xtask -- boot-check`

### Scope Notes

- v1 focuses on stable bring-up, observability, and educational subsystem boundaries
- user-mode process execution remains out of scope for this release
- persistent storage and advanced driver coverage remain out of scope for this release
