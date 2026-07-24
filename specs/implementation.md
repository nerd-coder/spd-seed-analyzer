# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Compare AAA-AAA-AAA floor 13 at the pre-items RNG boundary. Fix only the first
semantic difference, then stop; defer final heap and mob facts.

## Current checkpoint

- Floor 13 matches Java's room classes, normalized bounds, pre-paint, and
  pre-mobs RNG boundaries. Its pinned oracle fixture covers later checkpoints.
- Floor 12 remains exact through its documented final-heap boundary.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
