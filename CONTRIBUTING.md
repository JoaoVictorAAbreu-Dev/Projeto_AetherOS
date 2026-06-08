# Contributing to AetherOS

## Project Mindset

AetherOS is both a kernel project and an educational artifact. Contributions should improve at least one of these dimensions:

- technical correctness
- architectural clarity
- documentation quality
- contributor experience
- educational value

## Before You Start

Read these files first:

1. [README.md](README.md)
2. [ROADMAP.md](ROADMAP.md)
3. [docs/architecture/overview.md](docs/architecture/overview.md)
4. [docs/tutorials/first-contribution.md](docs/tutorials/first-contribution.md)

## Mandatory Rules

- Keep changes small and reviewable.
- Respect the implementation order defined by the project.
- Do not add dependencies without a clear architectural reason.
- Do not mix architecture-specific code with generic kernel code.
- Update documentation whenever structure or behavior changes.
- Do not claim tests passed unless they were actually run.
- Prefer explicit technical reasoning over hidden assumptions.

## Implementation Order

All contributions should respect this project order:

1. Boot
2. VGA
3. Keyboard
4. Interrupts
5. Memory Management
6. Scheduler
7. Virtual File System
8. Shell
9. Process Management
10. Drivers

If you want to work outside this order, explain why the exception is necessary.

## Branch Naming

Use one focused topic per branch:

- `feature/boot-limine`
- `feature/vga-text-buffer`
- `feature/interrupt-descriptor-table`
- `feature/frame-allocator`
- `docs/readme-overhaul`
- `docs/wiki-outline`
- `test/qemu-boot-flow`

## What a Good Pull Request Looks Like

A good PR for AetherOS is:

- narrowly scoped
- technically justified
- documented
- easy to review

Every PR should state:

- what changed
- why it changed
- architectural impact
- how it was validated
- known risks or limitations

## Validation Policy

When possible, use:

```bash
cargo run -p xtask -- test
cargo run -p xtask -- stage
```

If runtime validation exists for your change, document the exact command used. Preferred command:

```bash
cargo run -p xtask -- boot-check
```

If you could not run validation, write:

`Not tested`

## Documentation Expectations

Update documentation when you change:

- repository structure
- boot flow
- subsystem boundaries
- setup steps
- contributor workflow
- roadmap status

If the contribution introduces an architectural decision, add or update an ADR.

## Code Style

- Rust is the primary language.
- Assembly is allowed only when strictly required.
- Prefer explicit names over clever abstractions.
- Refactor duplicated logic instead of copying patterns forward.
- Add comments only when they clarify a non-obvious decision.

## Good First Contributions

Good first contributions include:

- improving setup documentation
- clarifying architecture docs
- adding Mermaid diagrams
- tightening naming consistency
- improving issue templates
- improving tutorial quality

## Communication

When opening an issue or PR:

- be specific
- reference exact files
- describe observed behavior and expected behavior
- include logs or screenshots when relevant

## Review Standard

The project values:

- correctness first
- clarity second
- convenience third

Educational value matters, but it should not come at the cost of weak technical decisions.
