# User-mode Foundation

## Goal

Prepare AetherOS for its first real kernel/user execution boundary without claiming that user-mode execution already exists.

## Current Foundation

The current codebase now defines:

- execution mode at the process level
- address-space intent for kernel-only and kernel-and-user layouts
- a compact `UserProgramImage` contract
- syscall inspection hooks for execution mode and address-space kind

## What This Means

The repository can now describe a future user task in a structured way.

It still does not:

- load ELF binaries
- switch privilege levels
- enter ring 3
- switch page tables for isolated user processes

## Why This Stage Matters

Without these explicit contracts, later user-mode work tends to leak assumptions into unrelated modules. This foundation keeps the next execution milestone disciplined and reviewable.
