# ADR-0001: Select Limine as the Initial Boot Protocol

## Status

Accepted

## Context

The project needs a practical boot path for a Rust-first `x86_64` kernel running in QEMU. The main objective of the early milestones is kernel architecture, not firmware engineering depth.

## Decision

Adopt Limine as the initial boot protocol.

## Consequences

### Positive

- Lower bring-up complexity
- Faster path to a debuggable Rust kernel
- Cleaner focus on memory, interrupts, and scheduling

### Negative

- Boot protocol becomes an external dependency in the early milestones
- A future migration to another protocol would require an adapter review

## Follow-up

- Keep boot protocol concerns isolated under `boot/` and architecture entry points
- Document boot assumptions early
