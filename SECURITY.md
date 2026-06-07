# Security Policy

## Scope

AetherOS is an academic operating system project and should not be treated as production-secure software.

Even so, security-relevant flaws are still valuable because they teach low-level systems design tradeoffs and safe engineering practices.

## How to Report

If you find a security issue:

1. Do not open a public exploit issue with sensitive reproduction steps first.
2. Open a private report if possible through GitHub Security Advisories.
3. If private reporting is unavailable, open a minimal issue and avoid publishing exploit details immediately.

## What Counts as a Relevant Security Issue

- memory safety boundary violations
- incorrect privilege assumptions
- unsafe boot trust assumptions
- unchecked pointer or mapping behavior
- malformed input paths that could break kernel invariants

## Disclosure Expectations

This project prefers responsible disclosure and educational writeups after triage and mitigation planning.
