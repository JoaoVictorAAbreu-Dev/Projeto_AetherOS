# Debug

The intended debug workflow combines:

- serial output for early boot visibility
- framebuffer output for visual state
- optional GDB attachment later
- shell commands for runtime inspection

## Current Debug Surfaces

- serial boot logs
- serial boot stage markers
- exception diagnostics
- timer tick logging
- shell commands:
  - `info`
  - `ticks`
  - `mem`
  - `tasks`
  - `ls`
  - `cat <FILE>`

## Current Runner Entry Points

- `cargo run -p xtask -- build`
- `cargo run -p xtask -- stage`
- `cargo run -p xtask -- run`
- `cargo run -p xtask -- boot-check`
- `cargo run -p xtask -- test`

## Boot Failure Triage

If the kernel halts during early bring-up, the serial output should now reveal the last successful boot stage:

- `entry`
- `revision-accepted`
- `collecting-boot-info`
- `hhdm-ready`
- `memory-map-ready`
- `regions-mapped`
- `framebuffer-ready`
- `boot-info-ready`
- `kernel-init`

The panic path also reports the current boot stage before printing panic details.
