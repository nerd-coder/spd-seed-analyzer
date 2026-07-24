# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Generate and commit the pinned AAA-AAA-AAA floor-13 oracle fixture. Compare room
classes and bounds plus the pre-paint RNG probe. Fix only the first semantic
difference, then stop; defer later floor-13 boundaries to following phases.

## Current checkpoint

- Floor 12 matches Java's room classes, normalized bounds, pre-paint, pre-mobs,
  and pre-items RNG boundaries plus all 16 normalized final heaps.
- Floor 11 remains exact at its documented deterministic boundaries.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
