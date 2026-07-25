# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Trace the preserved Generator general-deck lifecycle before AAA-AAA-AAA floor 19.
Find the missing or extra deck-only draw that makes Rust enter `createItems` one
card later than the pinned game. Fix that source lifecycle divergence, then add
the exact floor-19 final-heap assertion. Update the manifest, run CI parity,
save, commit, and stop.

## Checkpoint

- Floor 19 matches room classes, normalized bounds, the pre-paint, pre-mobs,
  and pre-items RNG boundaries, and all ten final mob cells/classes.
- Exact final-heap comparison fails before placement: the oracle's three regular
  Generator drops are Gold (311, chest), Gold (230), and PotionOfLiquidFlame;
  Rust produces Gold (230), PotionOfLiquidFlame, and PotionOfLevitation. Fixed
  room heaps and SecretSummoning's Bolas already match. Global pre-items RNG
  matches, so this is inherited Generator deck state, not room ordering or
  terrain placement.
- Final heaps, terrain, discoverability, tile variance, transitions, traps,
  plants, and blobs remain unverified.
- Overall accuracy remains `partial`; coverage is fixture-specific.
