default:
    @just --list

fmt:
    cargo fmt --all

check:
    cargo check --workspace

test:
    cargo test --workspace
