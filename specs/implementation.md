# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- Preserved `AAA-AAA-AAA` replay now crosses floor 10 and matches floor 11 at
  pre-paint, pre-mobs, and pre-items RNG boundaries.
- Floor 11 matches exact mobs and SecretHoardRoom heaps after replacing its
  approximate painter with the pinned gold/trap lifecycle.
- Other floor-11 heaps are not exact: later-shop bag state and additive
  GuidePage/key/drop placement still differ.

## Next phase

Resolve the earliest floor-11 heap divergence without disturbing the exact RNG
boundaries. Start with the later-shop bag choice (`MagicalHolster` in Java vs
`PotionBandolier` in Rust), then compare GuidePage/key/main-drop placement.

## Known limits

- Fixture-specific room painters and Generator histories may still diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and general player-dependent later-shop bags are out of
  scope; only pinned deterministic inventory profiles can be verified.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
