# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Extend the pinned Java final-heaps oracle through floor 17. Compare
AAA-AAA-AAA room classes and normalized bounds first, then RNG boundaries,
mobs, and heaps. Fix only the first semantic difference, then stop.

## Current checkpoint

- AAA-AAA-AAA floor 16 matches Java through normalized final heaps, with the
  floor-15 DM-300 lifecycle preserved.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
