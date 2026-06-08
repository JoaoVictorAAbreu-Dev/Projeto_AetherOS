help:
	@echo "Available targets:"
	@echo "  fmt     - Run cargo fmt"
	@echo "  check   - Run cargo check"
	@echo "  test    - Run cargo test"
	@echo "  build   - Build the kernel target"
	@echo "  stage   - Stage boot assets"
	@echo "  run     - Attempt to launch QEMU"
	@echo "  boot-check - Verify headless QEMU boot"

fmt:
	cargo fmt --all

check:
	cargo run -p xtask -- test

test:
	cargo run -p xtask -- test

build:
	cargo run -p xtask -- build

stage:
	cargo run -p xtask -- stage

run:
	cargo run -p xtask -- run

boot-check:
	cargo run -p xtask -- boot-check
