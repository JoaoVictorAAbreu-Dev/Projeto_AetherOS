# Interrupt Architecture

## Scope

- Exceptions
- Hardware interrupts
- Timer infrastructure
- Interrupt controller abstraction

## Order of Introduction

1. GDT
2. IDT
3. Exception handlers
4. Timer interrupt
5. Broader IRQ integration

## Current Sprint 1 Scope

The current implementation target is intentionally minimal:

- GDT with TSS support for a safe double-fault stack
- IDT with core exception handlers
- PIC remapping
- PIT timer interrupt
- keyboard IRQ logging path

This is the minimum interrupt foundation required before advancing deeper into memory management or scheduling.
