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
