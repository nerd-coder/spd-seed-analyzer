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
- Floor 7 now matches the exact pre-mobs RNG boundary through all room paint,
  `paintDoors`, and the isolated painter-generator transition.
- `PitRoom.paint` now marks its entrance `CRYSTAL`, preventing the erroneous
  hidden-door roll that previously left Rust one main-stream draw ahead.
- Overall status remains `partial`.

## Next phase

Continue floor 7 from the restored pre-mobs boundary:

1. Compare `createMobs` and its quest hook against the pinned oracle.
2. Port the earliest missing or extra call only.
3. Re-enable exact pre-items parity if reached, then stop at the next divergence.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
