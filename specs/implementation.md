# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` preserves exact Java room bounds on floors 6–9; floor 6 still
  matches through pre-items.
- Floor 7 now has exact pre-mobs/pre-items boundaries and exact mobs. The
  Guard placement mismatch was caused before `PillarsRoom`: constructing the
  PitRoom's `UnstableSpellbook` must randomize its scroll list with 11
  `Random.chances` draws.
- `PillarsRoom (34,2)–(42,10)` now enters at the pinned RNG state and paints
  the Guard-blocking pillars exactly.
- Overall accuracy remains `partial`.

## Next phase

Continue `AAA-AAA-AAA` floor 7 from the exact pre-items boundary:

1. Compare final structured heaps/items and locate the earliest mismatch.
2. Port the responsible pinned item/placement lifecycle behavior.
3. Preserve exact floor-6 fixtures and exact floor-7 mobs while advancing
   toward full floor-7 terrain and heap parity.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
