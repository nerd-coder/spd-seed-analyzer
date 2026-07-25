# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Continue AAA-AAA-AAA floor 19 parity with tile variance. Compare the full array
with the committed oracle and pinned `DungeonTileSheet.setupVariance`, fix only
the first source-backed divergence, add exact assertions, update the accuracy
manifest, run CI parity, save, commit, and stop.

## Checkpoint

- Floor 19 matches room classes/bounds, RNG boundaries, final mobs/heaps,
  transitions, traps, plants, and the LaboratoryRoom Alchemy blob at cell 1223.
- Full floor-19 terrain and discoverability match the oracle exactly.
- Next unverified fact is tile variance. Overall accuracy remains `partial`;
  coverage is fixture-specific.
