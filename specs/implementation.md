# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

With floor-12 FigureEightBuilder classes and bounds aligned, compare the
AAA-AAA-AAA pre-paint RNG probe with the pinned oracle. Fix only the first
painter or room-construction semantic difference, then stop and record the next
boundary. Do not claim floor-12 lifecycle parity until pre-mobs and pre-items
probes also match.

## Current checkpoint

- Floor-12 FigureEightBuilder now matches Java's 19 room classes and normalized
  bounds. CaveExitRoom uses the pinned `{2, 1, 0}` size-category probabilities;
  this preserves Java's failed loop attempts and successful fourth layout.
- Painter, pre-mobs, and pre-items parity are not claimed for floor 12.
- Floor 11 remains exact at its documented deterministic boundaries.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
