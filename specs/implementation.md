# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- Preserved `AAA-AAA-AAA` replay is exact through floor 9 at the available
  pre-paint, pre-mobs, and pre-items RNG boundaries.
- Floors 8–9 also match all pinned mobs and normalized structured heaps; no
  shop occurs on either floor.
- No generation change was needed for floors 8–9.

## Next phase

Extend the preserved `AAA-AAA-AAA` Java oracle beyond floor 9. Preserve any
depth-10 lifecycle state needed to capture floor 11, compare RNG boundaries,
mobs, and structured heaps, then port only the earliest proven divergence.

## Known limits

- Fixture-specific room painters and Generator histories may still diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are out of scope.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
