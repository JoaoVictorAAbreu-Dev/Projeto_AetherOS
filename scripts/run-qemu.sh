#!/usr/bin/env sh

if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
  echo "qemu-system-x86_64 not found in PATH." >&2
  exit 1
fi

echo "AetherOS QEMU launcher"
echo "This project still needs a bootable disk or ISO artifact wired into the script."
echo "Suggested next integration:"
echo "- build kernel artifact"
echo "- assemble Limine image"
echo "- launch QEMU with serial stdio"
