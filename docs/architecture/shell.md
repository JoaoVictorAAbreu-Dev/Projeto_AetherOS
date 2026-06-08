# Shell Architecture

## Goal

The first AetherOS shell is intentionally simple. It is a kernel-resident interactive command loop designed to validate:

- keyboard input flow
- line buffering
- command dispatch
- VFS reads
- runtime introspection

## Non-Goals

- user-mode shell
- job control
- piping
- process spawning
- complex parsing

## Initial Commands

- `help`
- `info`
- `ticks`
- `mem`
- `tasks`
- `ls`
- `cat <FILE>`
- `write <FILE> <TEXT>`
- `rm <FILE>`
- `storage`
- `clear`

## Input Model

The shell currently accepts:

- printable keyboard characters
- Enter
- Backspace
- basic Shift-modified input for uppercase letters and common punctuation

Path lookup for `cat <FILE>` is resolved by the VFS layer so command parsing does not hardcode individual filename aliases.

The shell can now write and remove files through a writable in-memory VFS overlay. This remains non-persistent by design and exists to validate the filesystem boundary before disk-backed persistence is introduced.

## Why This Design

This shell is not meant to be final. It is meant to prove that the kernel can accept input, preserve state, read from a filesystem abstraction, and expose meaningful runtime information through a stable interactive path.
