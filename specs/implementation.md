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
- Floor 7 matches room paint through `LibraryRoom`, including its exact prizes.
- The earliest later mismatch was `StatueRoom.paint` omitting its RNG-bearing
  `center()` call; that call now precedes `Statue.random()` as in pinned Java.
- Exact pre-mobs parity is not restored; status remains `partial`.

## Next phase

Continue the floor-7 trace from the corrected `StatueRoom` boundary:

1. Compare the end of `StatueRoom`, the final `PerimeterRoom`, `paintDoors`,
   and the `Random.Long()` transition into the isolated painter generator.
2. Port the earliest missing or extra call only.
3. Re-enable exact pre-mobs parity if reached, then stop at the next divergence.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
