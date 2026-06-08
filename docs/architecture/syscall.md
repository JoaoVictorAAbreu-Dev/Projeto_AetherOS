# Syscall Architecture

## Direction

Syscalls should be introduced only after the memory and task model are stable enough to support a meaningful kernel/user boundary.

## Principles

- Keep ABI explicit and versioned
- Centralize syscall dispatch
- Avoid exposing internal kernel structures

## Current v1 Position

In v1, syscalls are still internal kernel-facing hooks used for inspection and staged evolution. The shell uses direct kernel modules for now, while the syscall surface stays intentionally narrow until user-mode execution becomes a real milestone.

## Post-v1 Foundation

The current post-v1 groundwork now reserves syscall inspection for:

- execution mode queries
- address-space kind queries
- later transition to user-mode program loading

This still does not represent a stable user ABI, but it narrows the path toward one.
