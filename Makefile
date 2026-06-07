help:
	@echo "Available targets:"
	@echo "  fmt     - Run cargo fmt"
	@echo "  check   - Run cargo check"
	@echo "  test    - Run cargo test"

fmt:
	cargo fmt --all

check:
	cargo check --workspace

test:
	cargo test --workspace
