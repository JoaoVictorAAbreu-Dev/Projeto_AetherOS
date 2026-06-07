default:
    @just --list

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
