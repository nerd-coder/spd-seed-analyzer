# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Compare AAA-AAA-AAA floor 13 final heap facts. Fix only the first semantic
difference, then stop.

## Current checkpoint

- Floor 13 matches Java's room classes, normalized bounds, RNG boundaries, and
  all eight final mobs. Final heaps are not yet verified.
- Floor 12 remains exact through its documented final-heap boundary.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
