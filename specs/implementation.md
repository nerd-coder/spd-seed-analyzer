# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- Depth-one schema-v3 fixtures pin lifecycle probes, room classes, map bounds,
  final heaps, and final mobs; `HKT-JZN-XQQ` also replays floors 6–8.
- `GFX-PZH-DCH` now pins CrystalPathRoom geometry/reward order and all six exact
  item-to-cell heap associations. That family no longer emits `Room loot`.
- Coverage remains partial; unported room painters and deeper histories can
  still diverge.

## Next phase

Add a second independent replay seed across floors 6–9:

1. Select a seed that exercises room families not dominant in `HKT-JZN-XQQ`.
2. Regenerate every target floor with the pinned Java oracle, preserving all
   prior-floor run state.
3. Pin room classes/bounds, all lifecycle probes, final heaps, final mobs, and
   additive render facts before fixing the first divergence.
4. Port only the divergence reached by that replay and update
   `specs/accuracy.json` in the same change.

After that replay, convert another fixture-backed family that still emits a
legacy `Room loot` marker.

## Known limits

- Uncovered special/secret geometry and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts for
  `AAA-AAA-ACB`.
- VaultLevel branches and player-dependent later-shop bag selection are out of
  the current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
