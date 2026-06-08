# Memory Architecture

## Planned Stages

1. Consume bootloader memory map
2. Build physical frame allocator
3. Introduce virtual memory mapper
4. Initialize kernel heap
5. Evolve toward address-space isolation

## Design Rules

- Keep physical and virtual memory concerns separate.
- Avoid embedding allocation policy inside unrelated modules.
- Prefer small explicit types over ad hoc pointer math spread across the codebase.

## Current v1 Scope

The current memory stage targets the minimum viable operational kernel core:

- copy a stable snapshot of the bootloader memory map into `BootInfo`
- discover usable physical frames
- expose a simple frame allocator
- initialize a small kernel heap
- expose inspectable memory state for later shell/debug use

This is intentionally conservative. The goal is to establish reliable internal state before introducing more advanced paging or address-space isolation.
