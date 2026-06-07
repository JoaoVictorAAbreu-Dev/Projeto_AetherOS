# Boot Flow

## Selected Direction

AetherOS adopts Limine as the initial boot protocol.

## Boot Sequence

1. Firmware transfers control to the bootloader.
2. Limine loads the kernel image and prepares boot information.
3. Kernel entry receives the boot context.
4. Early initialization configures logging and panic behavior.
5. Memory map parsing and paging setup begin.
6. CPU tables and interrupt infrastructure are initialized.
7. Kernel services start in a controlled order.

## Why Limine

- Reduces early bootstrap complexity
- Keeps focus on kernel design instead of firmware plumbing
- Supports later growth without forcing a rewrite of the kernel layout
