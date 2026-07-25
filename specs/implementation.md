# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Continue AAA-AAA-AAA floor 19 parity with AmbitiousImp quest rewards. Compare
the exact reward identities and item state with the committed oracle and pinned
quest source, fix only the first source-backed divergence, add exact assertions,
update the accuracy manifest, run CI parity, save, commit, and stop.

## Checkpoint

- Floor 19 matches room classes/bounds, RNG boundaries, final mobs/heaps, full
  terrain, discoverability, tile variance, transitions, traps, plants, and the
  LaboratoryRoom Alchemy blob at cell 1223.
- Tile variance matches `DungeonTileSheet.setupVariance` cell-for-cell; no
  generation change was needed.
- Next unverified fact is the two AmbitiousImp quest rewards. Overall accuracy
  remains `partial`; coverage is fixture-specific.
