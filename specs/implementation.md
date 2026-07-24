# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Capture and compare AAA-AAA-AAA floor 13 final mob facts. Fix only the first
semantic difference, then stop; defer final heap facts.

## Current checkpoint

- Floor 13 matches Java's room classes, normalized bounds, pre-paint, pre-mobs,
  and pre-items RNG boundaries. Final mob and heap facts are not yet pinned.
- Floor 12 remains exact through its documented final-heap boundary.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
