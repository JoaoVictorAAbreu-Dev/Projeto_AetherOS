#!/usr/bin/env sh

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found in PATH." >&2
  exit 1
fi

cargo run -p xtask -- run "$@"
