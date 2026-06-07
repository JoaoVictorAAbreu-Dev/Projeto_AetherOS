#!/usr/bin/env sh

echo "AetherOS environment check"
echo "--------------------------"
command -v cargo >/dev/null 2>&1 && echo "cargo: $(command -v cargo)" || echo "cargo: not found"
command -v rustc >/dev/null 2>&1 && echo "rustc: $(command -v rustc)" || echo "rustc: not found"
command -v qemu-system-x86_64 >/dev/null 2>&1 && echo "qemu-system-x86_64: $(command -v qemu-system-x86_64)" || echo "qemu-system-x86_64: not found"
echo
echo "Expected components:"
echo "- Rust nightly"
echo "- rust-src"
echo "- llvm-tools-preview"
echo "- QEMU"
echo "- Limine tooling"
