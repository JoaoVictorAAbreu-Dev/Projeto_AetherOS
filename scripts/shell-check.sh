#!/usr/bin/env sh

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found in PATH." >&2
  exit 1
fi

export AETHER_BOOT_TIMEOUT_SECS="${AETHER_BOOT_TIMEOUT_SECS:-30}"
export AETHER_QEMU_DISPLAY=none

cargo run -p xtask -- shell-check
