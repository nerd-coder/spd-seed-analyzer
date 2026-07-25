# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Complete AAA-AAA-AAA floor 19 map parity: compare transitions, ordinary and
SecretSummoning traps, plants, blobs, terrain, discoverability, and tile
variance with the committed oracle. Fix the first source-backed divergence,
add exact assertions, update the manifest, run CI parity, save, commit, and stop.

## Checkpoint

- Floor 19 matches room classes, normalized bounds, the pre-paint, pre-mobs,
  and pre-items RNG boundaries, and all ten final mob cells/classes.
- Floor 19 now matches exact final heaps. The apparent Generator divergence was
  downstream of missing AmbitiousImpRoom geometry: five absent grass candidates
  shifted the separate painter RNG, placed a trap on the second drop cell, and
  changed later `createItems` draws.
- Terrain, discoverability, tile variance, transitions, traps, plants, and blobs
  remain unverified.
- Overall accuracy remains `partial`; coverage is fixture-specific.
