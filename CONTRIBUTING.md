# Contributing

## Principles

- Keep changes small and reviewable.
- Preserve clean module boundaries.
- Prefer explicit architecture decisions over ad hoc growth.
- Update documentation when structure or behavior changes.

## Branch Naming

- `feature/boot-limine`
- `feature/memory-paging`
- `feature/scheduler-round-robin`
- `docs/architecture-overview`
- `test/qemu-boot-flow`

## Pull Requests

- Keep PRs focused on one topic.
- Include what changed, how it was validated, and known limitations.
- Do not claim tests passed unless they were actually run.

## Coding Rules

- Rust is the primary language.
- Assembly is allowed only when strictly required by the platform.
- Add comments only when they clarify a non-obvious decision.
