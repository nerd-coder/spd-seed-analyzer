# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the canonical coverage manifest. Update it with every
generation/parity change. Port only from the pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Current checkpoint

- Java RNG, generator decks, run identities, depth seeds, forced drops, and
  supported shop/quest flows have focused parity coverage.
- Ten depth-one schema-v3 fixtures pin lifecycle RNG boundaries, room classes,
  bounds, final heaps, and final mobs. `HKT-JZN-XQQ` also replays floors 6–8.
- Structured heap capture is fixture-backed for Crypt, Pool, Runestone,
  MagicalFire, Traps, Sentry, Storage, and ToxicGas paths.
- `AAA-AAA-AAA` pins PoolRoom's chest at cell 315 and the complete final
  heap/mob set. The Pool prize early-return and piranha RNG order match Java.
- Full additive map facts are verified only for selected fixtures. Overall
  coverage remains partial; legacy room painters and deeper histories can
  still diverge.

## Next phase

Convert one more RegularLevel room family that still produces a legacy
`Room loot` marker:

1. Find or reuse a compact schema-v3 fixture containing the room; independently
   regenerate it with the pinned Java oracle.
2. Before changing Rust, pin room classes, bounds, all three lifecycle RNG
   probes, every final heap, and every final mob.
3. Port exact geometry, retry predicates, RNG/item order, heap type, and
   item-to-cell association into `spd-core`.
4. Remove the legacy marker only for that covered family; retain it elsewhere.
5. Update `specs/accuracy.json`, oracle notes, and this handoff.

Prefer a family already reachable through `RegularLevel`. Inspect candidates
with:

```bash
rg -n 'heap_occupied|Room loot|PlacedLoot|record_heap' crates/spd-core/src/level
```

After the next room family, prioritize a second replay seed across floors 6–9
to reduce single-history overfitting.

## Known limits

- Uncovered special/secret paint geometry and deeper generation histories may
  diverge even when earlier facts match.
- ToxicGas vents and gas blobs are not exact exported additive facts for
  `AAA-AAA-ACB`.
- VaultLevel-only branches are outside the regular-floor analyzer.
- Later-shop bag selection needs player inventory and Hourglass sandbag input.
- The unseeded early Guidebook page is intentionally out of scope.

## Validation

Oracle usage is documented in `tools/java-oracle/README.md`. Before committing
a Rust phase, run the full CI `check` sequence from `AGENTS.md`:

```bash
mise exec -- bun run check
CARGO_TARGET_DIR=/private/tmp/spd-seed-review-target mise exec -- bun run check:rust
CARGO_TARGET_DIR=/private/tmp/spd-seed-review-target mise exec -- bun run test:rust
CARGO_TARGET_DIR=/private/tmp/spd-seed-review-target mise exec -- bun run build
mise exec -- bun run test:visual:only
```
