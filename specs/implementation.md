# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Record a pinned AAA-AAA-AAA floor-18 final-heaps fixture, replay preserved state
through depth 18, and compare room classes, normalized bounds, and the pre-paint
RNG boundary. Fix only the first semantic difference, then stop.

## Current checkpoint

- Floor 17 matches Java through pre-paint, pre-mobs, pre-items, final mobs, and
  normalized heaps.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
