# Dependency Strategy

## Initial Dependencies

- `limine`
- `x86_64`
- `spin`
- `bitflags`
- `log`

## Current Justification

- `limine`: selected boot protocol interface for the Boot milestone, keeping raw protocol handling out of the generic kernel path
- `x86_64`: architecture instructions and low-level CPU/port abstractions
- `spin`: `no_std` synchronization primitives for early kernel state

## Selection Rules

- Prefer mature low-level crates
- Avoid speculative dependencies
- Introduce new crates only with a clear subsystem need
