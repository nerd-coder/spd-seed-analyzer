# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

**Canonical coverage manifest:** `specs/accuracy.json` — update it in the same
change whenever implementation coverage, parity evidence, known limits, or the
overall accuracy status changes. The sidebar accuracy popover renders this
file.

## Objective

Make `spd-core` a call-for-call RNG and generation match for the pinned Java
engine. Port from the pinned checkout at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`; do not approximate
covered behavior. Keep generation in `spd-core`, with `spd-wasm` and the UI as
thin consumers.

## Verified state

- The seed finder UI keeps up to ten in-memory search instances in nanostores.
  Finder searches and full seed analyses execute in dedicated web workers, so
  synchronous WASM generation cannot block the browser UI. Both flows expose
  live elapsed time and terminate their worker immediately when cancelled.
  Search progress identifies the in-flight numeric seed, one-based candidate
  number, analysis depth, and matches found so far. Cancelling retains the last
  completed progress snapshot and displays any matches already found; finder
  tab captions include their live result count.
  Every execution opens a result tab named for its single requested item or
  `Find N items` for multiple constraints; the oldest instance is dropped at
  the cap. Search parameters live in the sidebar, and finder tabs use the same
  pinned, horizontally scrollable treatment as analyze tabs. Item constraints
  match an exact class and optional minimum upgrade level across the selected
  analysis depth. Candidate count (default 100, UI range 10–10,000), depth,
  and result-limit controls share a dedicated parameter row below the start
  seed and above Item constraints. The first constraint row ends with an
  icon-only add action, while every later row ends with a remove action. The
  All/Any match switch sits left of the Find seeds action. The engine accepts
  bounded chunks up to 10,000 candidates.

- Java `Random`, watabou generator stacks, seed codes, run identities,
  generator decks, depth seeds, forced drops, and supported shop/quest flows
  have focused coverage.
- Ten depth-one schema-v3 fixtures match Java lifecycle RNG boundaries, map
  bounds, room classes, report-visible items, and their fixture-scoped entity
  facts: `AAA-AAA-AAA`, `AAA-AAA-AAD`, `AAA-AAA-AFO`, `AAA-AAA-AFU`,
  `AAA-AAA-AAZ`, `AAA-AAA-ACB`, `ABC-DEF-GHI`, `GFX-PZH-DCH`, `hello`, and
  `HKT-JZN-XQQ`.
- `HKT-JZN-XQQ` replay floors 6, 7, and 8 match their committed Java lifecycle,
  terrain/render facts, heaps, mobs, transitions, traps, and recorded
  quest/blob facts.
- Exact structured heap capture is pinned for Crypt, Runestone, MagicalFire,
  Traps, Sentry, Storage, and ToxicGas room paths covered by the fixtures.
- `AAA-AAA-ACB` pins ToxicGasRoom at its source-exact 7×7 minimum and matches
  all lifecycle probes/final heaps/mobs. Toxic heaps are Gold chests at cells
  947 (×61) and 998 (×71), plus a Gold skeleton at 1096 (×92). Its additive
  terrain/trap/blob fields remain intentionally unverified.
- ToxicGas paint order now matches Java: jittered center/statue, EMPTY-only vent
  retries, eight unique non-statue gold candidates, strict-distance winner,
  skeleton gold, two insertion-order chests, then queued Purity.
- LoopBuilder's closing stitch collision-tests only the current loop, matching
  Java's `placeRoom(loop, ...)` call. Sentry center retries remain exact for
  satisfiable layouts; partial/uncovered geometry uses a no-RNG deterministic
  fallback only when the Java predicate is mathematically unsatisfiable, so
  browser analysis cannot spin forever.
- The deterministic browser map suite passes for CXG floor 1 and HKT floors
  1/6/8. The CXG baseline reflects the corrected ToxicGas minimum size.

## Next phase

Continue fixture-first structured paint capture for one remaining legacy
`Room loot` family:

1. Find a compact depth-one seed containing the target room.
2. Generate a schema-v3 fixture with the pinned Java oracle.
3. Assert room classes, all three lifecycle RNG probes, bounds, final heaps,
   and final mobs before changing Rust.
4. Port exact paint geometry, retry predicates, item generation order, heap
   type, and item-to-cell association into `spd-core`.
5. Retain the legacy marker for every still-uncovered family.
6. Normalize additive render fields to `null` only when unrelated painter gaps
   prevent an honest full-additive claim; document the excluded facts.

Prefer a room already reachable through `RegularLevel` with a legacy marker.
Inspect current candidates with:

```bash
rg -n 'Room loot|PlacedLoot|record_heap' crates/spd-core/src/level
```

After another paint family, add a second replay seed for floors 6–9 to reduce
single-history overfitting. Promote a region only after boundary probes and all
final facts match; deeper mob tables alone are not proof of lifecycle parity.

## Known limits

- Coverage is fixture-specific; uncovered special/secret paint geometry and
  other deeper histories may diverge.
- ToxicGas vents and gas blobs are not yet exported as exact additive render
  facts for `AAA-AAA-ACB`.
- VaultLevel-only branches are outside the regular-floor analyzer.
- Player-collected inventory and Hourglass sandbags require explicit analysis
  input for exact later-shop bag selection.
- The unseeded early Guidebook page remains intentionally outside scope.

## Oracle and validation

Oracle tooling and regeneration commands are in
`tools/java-oracle/README.md`. The main parity consumer is
`crates/spd-core/tests/java_oracle_goldens.rs` with focused modules under
`crates/spd-core/tests/java_oracle_goldens/`.

Before completing a Rust phase, follow `AGENTS.md` CI parity. Use an isolated
target directory if the shared `target/` contains artifacts from another Rust
toolchain:

```bash
CARGO_TARGET_DIR=/private/tmp/spd-seed-review-target mise exec -- bun run check:rust
CARGO_TARGET_DIR=/private/tmp/spd-seed-review-target mise exec -- bun run test:rust
CARGO_TARGET_DIR=/private/tmp/spd-seed-review-target mise exec -- bun run build
mise exec -- bun run check
mise exec -- bun run test:visual:only
```

Useful native inspection:

```bash
CARGO_TARGET_DIR=/private/tmp/spd-seed-review-target \
  mise exec -- cargo run -p spd-core --example dump_seed -- SEED FLOORS
```
