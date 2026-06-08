# Scheduler Architecture

## Initial Direction

The first scheduler should be intentionally simple: a small round-robin kernel scheduler for threads under controlled conditions.

## Goals

- Simple context model
- Clear state transitions
- Minimal hidden coupling with architecture code
- Honest observability for what the scheduler really does in v1

## Non-Goals for Early Phases

- SMP
- Complex priorities
- Real-time guarantees
- Preemptive multitasking
- Dynamic process creation
- User-mode scheduling

## Current v1 Scope

The current scheduler is a cooperative round-robin demo over a fixed in-kernel task set.

It does:

- maintain a small static task table
- rotate the active task on timer ticks
- expose the current task name and state for shell/debug commands

It does not do:

- real context switching between independently executing threads
- blocking and wake-up queues
- process spawning
- address-space isolation
