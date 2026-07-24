# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` preserves Java-oracle floors 6–9 with exact room bounds;
  floor 6 matches through pre-items.
- Floor 7 matches both pre-mobs and pre-items RNG boundaries, including the
  Wandmaker hook and ambient `createMobs` draw shape.
- `StatueRoom` exports and reserves its pinned Statue cell (`2138`).
- Remaining floor-7 mob mismatch: Guard is at Rust cell `232` versus oracle
  cell `278`; cell `232` is incorrectly passable PillarsRoom terrain.
- Overall status remains `partial`.

## Next phase

Continue floor 7 from the exact pre-items boundary:

1. Compare pinned `PillarsRoom` terrain/flags around cells `232` and `278`.
2. Port the narrow geometry correction without regressing floor 6 fixtures.
3. Require exact floor-7 mobs, then continue to the earliest item divergence.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
