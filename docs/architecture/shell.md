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
- `cat README.TXT`
- `clear`

## Why This Design

This shell is not meant to be final. It is meant to prove that the kernel can accept input, preserve state, read from a filesystem abstraction, and expose meaningful runtime information through a stable interactive path.
