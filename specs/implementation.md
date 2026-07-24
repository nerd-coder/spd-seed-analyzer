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
  cell `278`. Rust cell `232` is `EMPTY_DECO` while the oracle has `WALL` in
  the `PillarsRoom` bounded by `(34,2)–(42,10)`.
- The Rust `PillarsRoom` formulas match pinned source, including Room's
  inclusive width/height override. Replacing them with watabou Rect spans is
  incorrect: it regresses the exact floor-6 pre-mobs boundary and terrain.
- Overall status remains `partial`.

## Next phase

Continue floor 7 from the exact pre-items boundary:

1. Probe the painter RNG immediately before the `(34,2)–(42,10)` PillarsRoom
   in Java and Rust; the geometry implementation itself is source-equivalent.
2. Correct the earliest painter-order/RNG-entry divergence while preserving
   the exact floor-6 boundary and terrain fixtures.
3. Require exact floor-7 mobs, then continue to the earliest item divergence.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
