# First Contribution Guide

## Goal

This guide helps a new contributor make a useful AetherOS contribution without needing full kernel expertise on day one.

## Best First Tasks

Start with one of these:

- improve a setup or onboarding document
- clarify an architecture explanation
- add a Mermaid diagram
- improve naming consistency in docs
- improve issue templates
- improve a roadmap or ADR explanation

## Avoid for Your First PR

Avoid starting with:

- scheduler logic
- paging internals
- interrupt descriptor table logic
- unsafe pointer-heavy code

These are important, but they require more project context.

## Recommended Workflow

1. Read [README.md](../../README.md)
2. Read [CONTRIBUTING.md](../../CONTRIBUTING.md)
3. Read the architecture page related to your target area
4. Open or choose one small issue
5. Keep the PR focused on one concern

## How to Think About Contributions

Before editing anything, ask:

- Does this fit the current roadmap stage?
- Does this improve clarity or correctness?
- Does this create hidden coupling?
- Does documentation need to change too?

## Good Example PRs

- README clarification
- setup guide hardening
- boot-flow documentation update
- new issue template for documentation bugs

## Minimum PR Quality Bar

- small scope
- clear reason
- explicit validation status
- updated docs when needed
