help:
	@echo "Available targets:"
	@echo "  fmt     - Run cargo fmt"
	@echo "  check   - Run cargo check"
	@echo "  test    - Run cargo test"
	@echo "  build   - Build the kernel target"
	@echo "  stage   - Stage boot assets"
	@echo "  run     - Attempt to launch QEMU"

fmt:
	cargo fmt --all

check:
	cargo check --workspace

test:
	cargo test --workspace

build:
	cargo run -p xtask -- build

stage:
	cargo run -p xtask -- stage

run:
	cargo run -p xtask -- run
