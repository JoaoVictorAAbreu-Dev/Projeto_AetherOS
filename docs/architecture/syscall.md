# Syscall Architecture

## Direction

Syscalls should be introduced only after the memory and task model are stable enough to support a meaningful kernel/user boundary.

## Principles

- Keep ABI explicit and versioned
- Centralize syscall dispatch
- Avoid exposing internal kernel structures
