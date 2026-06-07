# Development Setup

## Tooling

- Rust nightly
- QEMU
- LLVM tools
- Limine integration tooling

## Recommended Reading Before Running Anything

1. [README.md](../../README.md)
2. [docs/architecture/overview.md](../architecture/overview.md)
3. [CONTRIBUTING.md](../../CONTRIBUTING.md)

## Environment Goals

The local environment should support:

- workspace formatting
- workspace type checking
- QEMU execution
- future serial-log capture
- future debugger attachment

## Typical Setup Checklist

1. Install Rust nightly and the required components.
2. Install QEMU.
3. Install or vendor Limine tooling.
4. Confirm the workspace is readable by Cargo.
5. Run the baseline validation commands below.

## Local Validation

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

## Notes

- If QEMU is not installed, build validation may still work but runtime milestones cannot be demonstrated.
- When the project begins shipping demo assets, this setup guide should also document how to capture serial logs and screenshots.
