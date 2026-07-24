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
- Floor-7 painter probes match through `SegmentedRoom`; the first gap was
  `PitRoom` omitting the RNG-bearing `center()` call for its skeleton heap.
- `LibraryRoom` now enters at the pinned RNG state and matches the Java
  Identify/Upgrade prizes at cells 2071/2121. Status remains `partial`.
- The floor-7 pre-mobs probe is still one call ahead after later painters.

## Next phase

Trace the single floor-7 RNG call consumed after `LibraryRoom`:

1. Probe the remaining shuffled rooms, `paintDoors`, and the transition into
   the isolated water/grass/trap generator.
2. Port the earliest missing or extra call only.
3. Re-enable exact pre-mobs parity, then stop at the next lifecycle divergence.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
