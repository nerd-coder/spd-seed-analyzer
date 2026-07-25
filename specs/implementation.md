# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Continue AAA-AAA-AAA floor 19 parity from terrain. Compare the full terrain
array with the committed oracle and pinned City painters, fix only the first
source-backed divergence, add exact assertions, update the accuracy manifest,
run CI parity, save, commit, and stop.

## Checkpoint

- Floor 19 matches room classes/bounds, RNG boundaries, final mobs/heaps,
  transitions, traps, plants, and the LaboratoryRoom Alchemy blob at cell 1223.
- Next unverified facts are terrain, discoverability, and tile variance.
  Overall accuracy remains `partial`; coverage is fixture-specific.
