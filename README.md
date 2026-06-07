# AetherOS

AetherOS is an academic open source operating system project for the `x86_64` architecture, written primarily in Rust and designed to run on QEMU.

## Goals

- Modular kernel architecture
- Rust-first implementation
- Assembly only when strictly necessary
- Reproducible local build and emulation workflow
- Documentation-first engineering
- Structure prepared for long-term growth

## Current Scope

This repository currently contains the initial project structure, build scaffolding, architecture documents, and planning artifacts.

No production kernel features are implemented yet.

## Architecture Decisions

- Boot protocol: Limine
- Primary architecture: `x86_64`
- Emulator: QEMU
- Workspace model: Cargo workspace with separated kernel, shared crates, and tooling

## Repository Layout

```text
AetherOS/
├─ boot/
├─ config/
├─ crates/
├─ docs/
├─ kernel/
├─ scripts/
├─ tests/
└─ tools/
```

## Getting Started

### Prerequisites

- Rust toolchain
- `rust-src`
- `llvm-tools-preview`
- QEMU
- Limine tooling

### Initial Commands

```bash
cargo fmt --all
cargo check --workspace
```

Project-specific run and test scripts will evolve together with the kernel bring-up milestones.

## Documentation

- Architecture overview: [docs/architecture/overview.md](docs/architecture/overview.md)
- Boot flow: [docs/architecture/boot-flow.md](docs/architecture/boot-flow.md)
- Roadmap: [ROADMAP.md](ROADMAP.md)
- ADRs: [docs/decisions/](docs/decisions/)

## Roadmap

The short-term target is bootstrapping a minimal Rust kernel in QEMU with serial logging, controlled panic handling, and an architecture layout that supports memory, interrupts, scheduling, and syscalls as independent subsystems.
