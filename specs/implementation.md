# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Capture an AAA-AAA-AAA floor 21 oracle fixture, preserving the floor-20 Dwarf
King lifecycle. Compare the first RNG boundary and room layout with pinned
source, fix only the first source-backed divergence, add exact assertions,
update the accuracy manifest, run CI parity, save, commit, and stop.

## Checkpoint

- Floor 19 matches room classes/bounds, RNG boundaries, final mobs/heaps, full
  terrain, discoverability, tile variance, transitions, traps, plants, and the
  LaboratoryRoom Alchemy blob at cell 1223.
- Tile variance matches `DungeonTileSheet.setupVariance` cell-for-cell; no
  generation change was needed.
- Floor 19 retains the exact Wandmaker rewards (`WandOfTransfusion` +1 and
  `WandOfFrost` +2, both uncursed), matching Java's persistent quest state.
- The committed oracle does not capture the Imp's single ring reward. Overall
  accuracy remains `partial`; coverage is fixture-specific.
