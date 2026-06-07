# Architecture Overview

## Goals

- Keep boot concerns isolated from kernel services.
- Separate `x86_64` details from generic kernel logic.
- Keep reusable interfaces in independent crates when that improves testability and growth.

## Layers

1. Boot layer
2. Architecture layer
3. Kernel core
4. Kernel services
5. Userland boundary

## Initial Module Map

- `kernel/src/arch/x86_64`: architecture-specific integration
- `kernel/src/core`: kernel bootstrap and cross-cutting runtime concerns
- `kernel/src/memory`: memory management subsystem
- `kernel/src/task`: task and process model
- `kernel/src/syscall`: kernel/user boundary
- `kernel/src/drivers`: device-facing boundaries
- `kernel/src/fs`: filesystem abstractions
- `crates/*`: reusable shared contracts and utilities

## Initial Principles

- Prefer simple subsystem contracts.
- Avoid global coupling across modules.
- Make architecture decisions explicit in ADRs.
- Keep observability available from the earliest milestones.
