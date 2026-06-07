# Syscall Architecture

## Direction

Syscalls should be introduced only after the memory and task model are stable enough to support a meaningful kernel/user boundary.

## Principles

- Keep ABI explicit and versioned
- Centralize syscall dispatch
- Avoid exposing internal kernel structures

## Current Sprint 3 Position

At this stage, syscalls are still internal kernel-facing hooks used for inspection and staged evolution. The shell uses direct kernel modules for now, while the syscall surface remains a transition point toward a cleaner future boundary.
