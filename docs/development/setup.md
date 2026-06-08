# Development Setup

## Tooling

- Rust nightly
- On Windows, prefer the `nightly-x86_64-pc-windows-gnu` toolchain
- QEMU with UEFI firmware support
- LLVM tools
- network access for the first Limine bundle download

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
3. Install QEMU with `edk2-x86_64` firmware support.
4. Confirm the workspace is readable by Cargo.
5. Run the baseline validation commands below.

## Local Validation

```bash
cargo run -p xtask -- test
```

## Notes

- `cargo run -p xtask -- run` downloads the official Limine bundle into `dist/limine/` on first use.
- On Windows with the GNU toolchain, prefer cloning into an ASCII-only path without spaces if the linker reports object-file lookup failures.
- If QEMU is not installed, build validation may still work but runtime milestones cannot be demonstrated.
- When the project begins shipping demo assets, this setup guide should also document how to capture serial logs and screenshots.
