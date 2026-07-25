# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Continue AAA-AAA-AAA floor-18 terrain parity at cell 524 (x=12, y=12):
Rust has `LOCKED_DOOR` (10), while Java has `CRYSTAL_DOOR` (31).
Fix only that next semantic difference, update `specs/accuracy.json`, run the
full CI `check` sequence from `AGENTS.md`, save this handoff, commit, and stop.

## Checkpoint

- Floor 18 matches Java through final blobs and terrain cells 0–523.
- The latest phase source-ported `PitRoom.paint`: ordinary `EMPTY` inset,
  `EMPTY_WELL`, exact well RNG position, water eligibility, and room-local
  grass/trap exclusions.
- Terrain after cell 523, discoverability, tile variance, and later additive
  facts remain unverified.
