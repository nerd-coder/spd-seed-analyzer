# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Continue AAA-AAA-AAA floor-17 parity at the pre-mobs RNG boundary, then compare
the pre-items boundary, mobs, and normalized heaps. Fix only the first semantic
difference, then stop.

## Current checkpoint

- Floor 17 matches Java room classes, normalized bounds, and pre-paint RNG.
- The next known difference is missing Rust pre-mobs probe capture at floor 17.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
