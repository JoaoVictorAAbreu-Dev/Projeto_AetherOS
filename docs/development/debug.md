# Debug

The intended debug workflow combines:

- serial output for early boot visibility
- framebuffer output for visual state
- optional GDB attachment later
- shell commands for runtime inspection

## Current Debug Surfaces

- serial boot logs
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
- `cargo run -p xtask -- test`
