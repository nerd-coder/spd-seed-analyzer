# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Verify AAA-AAA-AAA floor-18 `discoverable` and `tile_variance` arrays against
the pinned oracle. Fix only the first semantic difference if either diverges;
otherwise record exact parity. Update `specs/accuracy.json`, run the full CI
`check` sequence from `AGENTS.md`, save this handoff, commit, and stop.

## Checkpoint

- Floor 18 now has full final-terrain parity; the PitRoom entrance is correctly
  projected as `CRYSTAL_DOOR`.
- Room layout, RNG boundaries, mobs, heaps, transitions, traps, plants, and
  GardenRoom foliage blobs already match the committed oracle fixture.
- Discoverability, tile variance, and other unchecked additive facts remain
  unverified.
