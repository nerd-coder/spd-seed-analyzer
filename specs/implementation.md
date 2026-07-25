# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Continue AAA-AAA-AAA floor 19 parity from blobs. Compare the Alchemy blob at
cell 1223 with the committed oracle and pinned City lifecycle, fix only the
first source-backed divergence, add exact assertions, update the accuracy
manifest, run CI parity, save, commit, and stop.

## Checkpoint

- Floor 19 matches room classes/bounds, RNG boundaries, final mobs/heaps,
  transitions, traps, and PlantsRoom plants.
- PlantsRoom now retains the exact non-Firebloom seed identity consumed by the
  existing generator lifecycle: Starflower at 1419 and Stormvine at 1421.
- Next unverified facts are the Alchemy blob, terrain, discoverability, and tile
  variance. Overall accuracy remains `partial`; coverage is fixture-specific.
