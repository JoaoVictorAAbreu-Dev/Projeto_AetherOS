# Post-v1 Sprints

## Sprint 7 - Runtime Interaction Automation

### Part 1

- add automated shell interaction on QEMU
- inject keyboard input through the QEMU monitor
- verify shell-visible command output through serial logs

### Part 2

- capture framebuffer output through QEMU screendump
- add a minimal framebuffer assertion workflow
- document interactive runtime verification beyond boot-only checks

## Sprint 8 - Persistent Filesystem Foundation

### Part 1

- define a persistent storage abstraction above the current initramfs
- introduce a minimal writable in-memory filesystem boundary compatible with later persistence
- keep shell commands aligned with the expanded VFS model

### Part 2

- add a block-storage oriented interface under `drivers/storage`
- document on-disk format direction and safety constraints
- prepare migration from static initramfs-only reads to writable filesystem operations

## Sprint 9 - User-mode Execution Foundation

### Part 1

- define a user-program image contract
- extend process and address-space abstractions for user-mode intent
- narrow the syscall surface around a future kernel/user boundary

### Part 2

- add first user-mode bring-up plan and staged loader boundary
- document execution lifecycle, risks, and validation approach
- prepare the repository for the first post-v1 execution milestone
