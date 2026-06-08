# Integration Tests

Cross-subsystem host tests live here when they do not require the custom kernel target or QEMU.

Current strategy:

- host-safe unit and integration tests stay in Rust crates that can run on the host ABI
- runtime boot verification lives under [`../qemu`](../qemu)

Planned use for this directory:

- VFS path normalization tests
- shell command parsing tests
- boot-info contract tests that do not require hardware emulation
