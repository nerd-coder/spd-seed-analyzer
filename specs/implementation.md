# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Continue AAA-AAA-AAA floor 19 map parity from plants. Port the exact City
planting lifecycle and identities, compare with the committed oracle, fix only
the first source-backed divergence, add exact assertions, update the manifest,
run CI parity, save, commit, and stop.

## Checkpoint

- Floor 19 matches room classes, normalized bounds, the pre-paint, pre-mobs,
  and pre-items RNG boundaries, and all ten final mob cells/classes.
- Floor 19 now matches exact final heaps. The apparent Generator divergence was
  downstream of missing AmbitiousImpRoom geometry: five absent grass candidates
  shifted the separate painter RNG, placed a trap on the second drop cell, and
  changed later `createItems` draws.
- Exact transitions now match, including AmbitiousImpRoom's center
  `BRANCH_EXIT` to branch 1; TerrainMap retains explicit quest branch exits.
- All ordinary and SecretSummoning traps match exactly.
- The next mismatch is plants: Rust records none; the oracle has Starflower at
  cell 1419 and Stormvine at cell 1421. Blobs, terrain, discoverability, and
  tile variance remain unverified.
- Overall accuracy remains `partial`; coverage is fixture-specific.
