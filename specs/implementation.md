# SPD Seed Analyzer — Next Implementation Phase

Pinned target: SPD v3.3.8 @ `7b8b845a7`.

Accuracy remains `partial`. `specs/accuracy.json` is the authoritative record
of verified coverage, evidence, and known gaps. Never claim full seed-finder
accuracy while that manifest remains partial.

## Product contract

- Report only seed-determined spawn facts. Exclude runtime-RNG events such as
  combat drops.
- Preserve independently proven properties when identity, placement, or player
  state is uncertain. Represent bounded alternatives and their conditions.
- Public maps are deterministic painter-complete layout snapshots captured
  before NPC, mob, heap, forced-item, Guide Page, and other population.
- Keep Rust reports, WASM, search, map projection, and UI consistent.
- Port pinned Java behavior and RNG call order; generation logic belongs in
  `spd-core`.

## Current boundary

The AAA-AAA-AAA replay is fixture-backed through floor 21. Floor 21 now has
exact room classes and bounds, pre-paint/pre-mobs/pre-items RNG boundaries,
final mobs, and stable final heaps. Its structured Demon Spawner class matches
the Java oracle while the public marker remains human-readable.

Floor-21 Halls painter terrain/decor is not exact. Do not infer terrain,
discoverability, tile variance, transitions, traps, plants, or blobs from the
new population parity. Forced torches remain modeled by the forced-item
contract rather than duplicated into internal map heaps; Rust resolves the
fixture's generic `Seed` item to its deterministic subtype.

## Next phase

Complete AAA-AAA-AAA floor-21 painter parity before moving to floor 22.

1. Diff Rust and Java terrain immediately after each Halls painter stage.
2. Identify the first RNG or cell divergence in Halls decoration, including
   region room painters, DemonSpawnerRoom, doors, water/grass/traps, and Halls
   wall decoration.
3. Port the minimum pinned behavior needed to restore parity; avoid
   fixture-specific corrections.
4. Extend the floor-21 oracle golden incrementally: terrain first, then
   discoverability, tile variance, transitions, traps, plants, and blobs when
   each is exact.
5. Add focused unit tests for every corrected painter rule and update
   `specs/accuracy.json` in the same change.

If floor-21 painter parity exposes a broad missing room family, stop after the
first cohesive source-backed fix and leave the next exact mismatch here.

## Phase completion checklist

- Add or extend `spd-core` tests and pinned Java-oracle evidence.
- Rebuild WASM after Rust changes.
- Run CI parity: `bun run check`, `bun run check:rust`, `bun run test:rust`,
  `bun run build`, and `bun run test:visual:only`.
- Keep `specs/accuracy.json` aligned with the proven boundary.
- Rewrite this file to the next concise handoff, commit with a Conventional
  Commit message, save state, and stop.
