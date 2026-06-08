default:
    @just --list

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
