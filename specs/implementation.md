# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Continue AAA-AAA-AAA floor-18 terrain comparison at cell 248 (x=18, y=5):
Rust has `EMPTY_SP` (14), while the Java fixture has `WATER` (29).
Fix only that next semantic difference, update the accuracy manifest, then stop.

## Current checkpoint

- Floor 18 matches Java through blobs and final terrain cells 0–247.
- City decoration now uses SPD's exact wall-stitchable set; the first corrected
  cell was wall 208 above a SegmentedLibraryRoom bookshelf.
- Terrain after cell 247, discoverability, tile variance, and later additive
  facts remain unverified.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
