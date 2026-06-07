# Scheduler Architecture

## Initial Direction

The first scheduler should be intentionally simple: a small round-robin kernel scheduler for threads under controlled conditions.

## Goals

- Simple context model
- Clear state transitions
- Minimal hidden coupling with architecture code

## Non-Goals for Early Phases

- SMP
- Complex priorities
- Real-time guarantees
