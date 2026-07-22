# SPD Seed Analyzer — Implementation Plan

**Last updated:** 2026-07-22
**Branch:** `main`
**Pinned SPD:** v3.3.8 @ `7b8b845a7`
**Local game source:** `/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`

## Goal

Browser seed analyzer for Shattered Pixel Dungeon — **Bun/Vite/React UI** over
a **Rust (`spd-core`) → WASM (`spd-wasm`)** dungeon-generation engine, ported
from the headless Java algorithm (`Dungeon.init` → `newLevel` per depth).

**The single goal that matters right now: make the Rust port an exact,
call-for-call RNG match of the pinned Java engine**, not an approximation.
Everything below is written to serve that goal — what's already exact, and
precisely what still diverges.

For commands, CI parity, and repo conventions see `AGENTS.md`. This file is
implementation state + the parity punch list, not a command reference.

---

## Repo layout

```text
spd-seed-analyzer/
├── crates/
│   ├── spd-core/              # pure Rust generation logic (this is the focus)
│   │   ├── src/level/          # per-floor build/paint/createItems/createMobs
│   │   ├── src/level/special_loot/  # special/secret room prize RNG
│   │   ├── src/quests/         # Ghost/Wandmaker/Blacksmith/Imp
│   │   └── tests/java_oracle_goldens.rs + fixtures/  # Java-vs-Rust parity tests
│   └── spd-wasm/               # thin wasm-bindgen façade
├── web/                        # Vite app (UI is stable; not the current focus)
├── tools/java-oracle/          # headless Java oracle: dumps ground-truth JSON
│   └── fixtures/                # committed golden fixtures, schema v1/v2/v3
└── specs/implementation.md     # this file
```

---

## Current state

**Foundations — solid, oracle-tested.** `java.util.Random` LCG, watabou
`Random` stack (push/pop, scrambleSeed, chances, shuffle, NormalIntRange),
seed codes, run init (`seed+1`) identities (potion/scroll/ring), generator
decks/tiers, depth seeds / limited drops. Golden fixtures in
`tools/java-oracle/fixtures/` + `crates/spd-core/tests/java_oracle_goldens.rs`
confirm exact identity parity across four seeds.

**Levelgen — broad partial port; four depth-one lifecycle fixtures are exact.**
Room init, geometry, both builders (Loop/FigureEight), all connection-room
subclasses, water/grass/trap painter, `paintDoors` merge/Graph, every region's
structural + standard room geometry, special/secret room prize logic, shop
stock, all four quests, crystal rooms, the main `createItems` drop loop, and
floor-map export are implemented. For `AAA-AAA-AAA`, `ABC-DEF-GHI`,
`GFX-PZH-DCH`, and `hello` at depth 1, Rust now reaches the exact Java RNG
states before painter, `createMobs`, and `createItems`; matches map bounds,
all final mob cells/types, all final heap cells, and the report-visible item
projection. This is deliberately still `partial`: four room sets do not cover
all depth-one combinations, deeper-floor mob generation and several
room-specific predicates/paint paths remain incomplete, and the public report
does not yet retain enough facts for full schema-v3 heap
identity/type/quantity equality.

**Frontend — functionally complete for a `partial` engine, not the current
focus.** Analyze + Find-seeds modes, multi-seed session tabs, spoiler
toggles, map rendering with autotiling, bounded seed-constraint search. See
git history / `AGENTS.md` for UI details; no open frontend work is blocking
parity.

**Correctness infra — the tool that will prove parity.**
`tools/java-oracle/` runs the *actual pinned Java source* headlessly and
dumps JSON: schema v1 (run identities), v2 (depth-one pre-build forced-item
queue), v3 (final placed heaps after real `Level.create()`, ordered by cell,
full item/heap facts plus final mob cells/types and lifecycle RNG probes, no
report-shaped filtering). Regenerate fixtures with
the exact commands in `tools/java-oracle/README.md`. This is the intended
mechanism for closing every gap below — add or extend a fixture, then make
the Rust side match it exactly.

---

## What's lacking for exact parity

Verified against the pinned Java source
(`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`) and four schema-v3
fixtures. The suite covers Pool/Runestone, MagicalFire, CrystalPath/MagicWell,
Traps/Treasury, SewerPipe, RegionDecoPatch, Bridge/Ring/CircleBasin, tunnel,
and WaterBridge variants. Every fixture asserts exact lifecycle RNG probes,
map bounds, final heap cells, final mob cells/types, and report-visible item
projection. Remaining gaps are outside these covered lifecycles or require
richer report facts.

### 0. ~~Broaden depth-one schema-v3 room coverage~~ — FIXED FOR FOUR FIXTURES
Three representative fixtures were added beside the original AAA regression:
`ABC-DEF-GHI`, `GFX-PZH-DCH`, and `hello`. Closing them required exact ports of
the following pinned behavior:

- MagicalFire's 7×7 minimum, temporary `EmptyRoom` constructor draw, fire and
  behind-fire geometry, heap retries/cells, and forced-prize queue removal;
- CrystalPath's six temporary-room constructor draws, zero-chance exotic draw,
  center jitter, center-only connection policy, six-room layout, crystal
  doors, paint masks, and exact heap cells; MagicWell's center/water draws and
  non-heap well-water blob;
- TrapsRoom's 6…8 dimensions, trap/chasm paint, opposite safe row, chest cell,
  and immediate return when `findPrizeItem` succeeds;
- Treasury's center jitter, item-before-position order, EMPTY/heap/mob retry
  predicates, and six small-gold placements that may merge; plus the main
  `createItems` locked-chest GoldenKey enqueue.

Focused tests pin the affected RNG tails and on-map facts. This closes the
current four-fixture slice only; it is not a full depth-one accuracy claim.

### 1. ~~Depth-one builder/painter boundary was 66 LCG steps behind~~ — FIXED
The schema-v3 lifecycle probes recover the exact 48-bit Java LCG state before
`RegularPainter.paint`, `createMobs`, and `createItems`. The fixture now proves:

- builder placement arrives at the exact pre-painter Java state and produces
  the exact 40×30 bounds;
- painter consumes exactly 190 raw LCG advances, matching Java;
- depth-one `createMobs` consumes exactly 116 raw advances and ends at the
  exact pre-`createItems` state.

The root builder bug was `findFreeSpace`: Java keeps `inside` and `curDiff`
across the collision-room loop, while Rust reset them per room. PoolRoom and
RunestoneRoom also require 6×6 minimum dimensions rather than the generic
5×5 special-room minimum.

The painter gap closed by porting RunestoneRoom's wall/chasm/interior geometry
and EMPTY/no-existing-heap retry predicate, and by preventing
`RegionDecoPatchEntranceRoom` merges through depth 2. Java also shuffles its
actual `rooms` list during painting; Rust now passes that paint order into both
population phases instead of returning to builder order.

### 1a. Depth-one `createMobs` is exact for the fixture; deeper floors remain (Large)
`level/create_mobs.rs` now ports the fixed eight-mob depth-one pass: weighted
and shuffled `StandardRoom` instances, rat/snake rotations and rare-alt rolls,
the unconditional champion roll, exact 30-retry semantics, recursive
`ShadowCaster`, bounded eight-neighbour entrance distance, room character
masks, traps/plants, existing mob occupancy, exits, and high-grass cleanup.
Pool/Aquarium piranhas are retained as real occupied cells and map markers.
The boundary and schema-v3 tests now assert exact draw shape and all 11 final
mob cells/types. Floors 2–24 still need `mobLimit`, region rotations (including
Shaman/Elemental subtype rolls and rare alts), large-mob properties, second
mob room rolls, quest/NPC occupancy, and exact markers.

### 2. ~~`random_drop_cell` shuffles the wrong-sized list (Medium)~~ — FIXED
Java's `randomRoom(type)` (`RegularLevel.java:707-710`) shuffles the level's
**entire** `rooms` list (every room: entrance, exit, standard, special,
secret, shop, connectors, **and set-empty rooms**) on *every call*, then
linear-scans for the first room of the requested type (`randomDropCell`,
`RegularLevel.java:736-766`). Fisher-Yates cost scales with total room count.
Rust's `random_drop_cell` now does exactly that: full index-list
`Random::shuffle_list` per try (the `special_loot::special_room_loot`
pattern), first `StandardRoom`-instance match in shuffled order.

Verified-against-source details now ported:

- **Persistent list order**: Java shuffles the actual `rooms` ArrayList, so
  the permutation left by one drop is the starting order for the next. Rust
  carries one mutable index permutation through all generated drops and the
  entire `itemsToSpawn` queue instead of rebuilding identity order per item.
- **Instanceof set**: `EntranceRoom`/`ExitRoom` extend `StandardRoom`
  (v3.3.8), so Rust kinds `Entrance`, `Exit`, AND `Standard` all match the
  scan. Entrance picked → try wasted (`room != roomEntrance`), zero further
  draws. Exit room CAN host drops — only the exact exit cell is excluded.
- **Exit cell**: Java `exit()` is the `REGULAR_EXIT` transition cell = the
  single cell painted `Terrain.EXIT` by the exit room. Rust resolves it as
  `map.map.iter().position(|&t| t == EXIT)` once per call (no `TerrainMap`
  field needed; the whole exit room is NOT excluded).
- **Degenerate IntRange correction**: watabou `Int(max)` returns 0 with **no
  RNG draw** when `max <= 0` (`Random.java:120-124`), so a set-empty room's
  `IntRange(left+1, right-1)` = `IntRange(1, -1)` burns **zero** draws (not
  one per coordinate) and yields point (1, 1), which then fails
  passable/solid (wall padding in practice). Rust `Random::int_max` already
  mirrored this; the old `width() <= 2` pre-skip (which burned nothing and
  never drew) is gone — Java always evaluates `Room.random()` for
  non-entrance matches.
- **No fallback**: scan finds no `StandardRoom` instance → return -1 on the
  FIRST try after exactly one full-list shuffle. The old non-Java
  "non-entrance rooms" fallback is dropped.
- Kept as-is: `occupied` mask as the `heaps.get(pos) == null` analog,
  `item_allowed` mask + `AquariumRoom` water override (gap 4),
  `trap_destroys_items`, passable/solid, 100-try cap. `findMob(pos) == null`
  is now covered by folding room-painted and depth-one ambient mob occupancy
  into the same mask before item placement.

Tests moved to `level/create_items/tests.rs` (SMALL-FILES split; same module
path). New coverage drives a parallel `JavaRandom` twin oracle that
transcribes the Java method (JDK `Collections.shuffle` loop + watabou
`IntRange` semantics) and asserts both the selected cell AND the total draw
count (post-call `next_int` sync): full-list reshuffle per try, exit-room
hosting minus the exit cell, entrance-wastes-try, degenerate zero-draw pick,
and no-match→-1-after-one-shuffle. The schema-v3 golden additionally pins the
cross-item permutation behavior through exact final heap-cell equality.

### 2a. ~~Consumable generation skipped the exotic-conversion draw~~ — FIXED
After selecting a regular potion or scroll, Java checks its regular→exotic
map and always evaluates `Random.Float() < consumableExoticChance()`. With no
ExoticCrystals trinket the chance is zero, but the Float still advances the
restored level-generation stream after the private category deck is popped.
Rust now preserves this draw in both category-deck and default-selection
paths. Focused generator tests pin the extra level-stream advance.

Combined with persistent room order, `AAA-AAA-AAA` depth 1 now generates and
places the exact five main drops (`ScrollOfRage`, `ScrollOfRecharging`,
`PotionOfHealing`, `Gold`, `Gold`) at cells 444, 173, 565, 939, and 903 with
matching heap types. The subsequent `itemsToSpawn` retry stream and final heap
cells also match.

### 3. ~~Eight special/secret rooms drop their trailing forced-item push~~ — FIXED
All 9 missing `addItemToSpawn` pushes are now ported. Push positions were
verified per-room against Java (they are **not** all at the end of `paint()`):
Crypt/Statue push right after `entrance.set(LOCKED)` (before prize RNG),
PoolRoom pushes after the pedestal prize but before piranha placement,
SecretRunestone pushes before its stone drops, the rest (Runestone, Library,
Treasury, Laboratory, Armory) push last. Constructors consume zero RNG; the
pushes only reorder `items_to_spawn` contents.

Two Java-source corrections to what this section previously claimed:

- `SecretLaboratoryRoom` does **not** inherit `LaboratoryRoom.paint` — it
  extends `SecretRoom` with its own `paint()` (weighted `potionChances`
  table) and never pushes an `IronKey`. Rust now splits
  `laboratory_prizes` (push) from `laboratory_prizes_shared` (no push, used
  by `secret_laboratory`). **Still open:** the shared body itself is
  LaboratoryRoom's prize logic, not SecretLaboratoryRoom's own paint —
  listed under lower-leverage gaps.
- Push-order fidelity matters beyond count: `PoolRoom`'s unfiltered
  `findPrizeItem` would consume its own `PotionOfInvisibility` if the push
  landed before prize selection.

Report-model note: `create_items` no longer blanket-tags every
`itemsToSpawn` placement `source = "forced"`. Pre-build forced drops keep
`"forced"`; room-paint additions are tagged `"items_to_spawn"` (both still
merge into the report's forced list). This keeps the schema-v2 oracle —
which snapshots Java's queue at the **pre-build** boundary
(`FloorOracle.RecordingSewerLevel.build()`) — comparable after room pushes
started surviving into `createItems`.

Unit coverage: 10 tests in `special_loot/tests.rs` (`*_pushes_iron_key`,
`pool_room_pushes_invisibility`, `secret_runestone_pushes_liquid_flame`,
`secret_laboratory_no_iron_key`). The AAA-AAA-AAA final-heaps projection
gained `PotionOfInvisibility` (PoolRoom push now lands; `IronKey` stays
report-blacklisted).

### 4. `canPlaceItem` fidelity gaps (Medium)
Only a generic `item_allowed` mask + trap-destroys-items filter +
`AquariumRoom`'s water override are ported. Still missing room-specific
exclusions: `PlantsRoom` (plant-occupied cells, `PlantsRoom.java:112-113`),
`StandardBridgeRoom`/`CavesFissureRoom` (bridge/fissure exclusion rects),
`RitualSiteRoom` (≥2 cells from candles), Vault-style rooms (center-only
drop). The `Int(20)` heap-type roll / mimic-float / locked-chest-upgrade
logic in the main loop was verified call-for-call correct against Java
already — this gap is specifically the room-shape predicate.

### Lower-leverage, already-known (from prior disclaimer, still open)
- Full ambient `createMobs` also feeds map markers — depth-one ambient mobs
  and represented room-painted mobs are shown, but deeper-floor ambient mobs
  are still absent.
- `SecretLaboratoryRoom` reuses `LaboratoryRoom`'s prize body
  (`laboratory_prizes_shared`); Java gives it its own `paint()` with a
  weighted `potionChances` table (2 potions) — different RNG shape whenever
  the room appears (found while fixing gap 3; no fixture covers it yet).
- Sewer room-count tables are reused for all regions.
- Shop stock is generated post-build instead of mid-`setSize`; bag choice is
  hero-less.
- `random_deck_item` still has a known exhausted-probability private-generator
  state/push-pop mismatch. None of the four fixtures exercises that rollover;
  fix it with a dedicated Java draw-shape fixture rather than folding it into
  an unrelated room patch.
- Structural-room paint/transition retry loops are capped at 10,000 attempts
  for browser safety (valid layouts shouldn't hit this); `Maze.generate` kept
  its real 2,500-failure limit.
- The early Guidebook page uses an intentionally unseeded Java generator and
  stays outside scope everywhere (oracle and Rust agree on omitting it).

---

## Suggested fix order

1. **Next phase — HKT visual-oracle parity.** Add a depth-one schema-v3 fixture
   for `HKT-JZN-XQQ`, extend the oracle additively with full terrain/transition
   facts needed for exact comparison, and make Rust layout, tiles, doors,
   transitions, heaps, mobs, and other objects match. Only after the core map
   is exact, compare the frontend's asset composition against
   `specs/HKT-JZN-XQQ_F1.png` (user-owned reference; do not modify or commit it).
2. Extend schema-v3 coverage to still more depth-one room sets. Four exact
   lifecycles are strong regression fixtures, not evidence that every
   depth-one combination is exact.
3. Port deeper-floor `createMobs`: mob limits/rotations, second-room spawns,
   large-mob open-space checks, and quest/NPC occupancy.
4. Close the remaining room-specific `canPlaceItem` predicates and known
   special-room paint gaps.
5. Correct the remaining timing/geometry approximations (SecretLaboratory,
   region room counts, shop `setSize`) as new fixtures cover them.
6. Extend the analyzer/report model with item-to-cell, quantity, level, and
   heap type so schema-v3 can assert full heap facts rather than the currently
   exact cell set plus report-visible item projection.
7. Add multi-depth schema-v3 fixtures and promote each newly covered region
   only after its lifecycle boundary probes and final facts match.

---

## Key algorithms / sequences to preserve

### `Dungeon.init` (run)
```text
Random.pushGenerator(seed + 1)
  Scroll.initLabels / Potion.initColors / Ring.initGems
  SpecialRoom.initForRun / SecretRoom.initForRun
  Generator.fullReset
Random.resetGenerators
```

### Per floor (`Level.create`)
```text
Random.pushGenerator(seedForDepth(seed, depth, 0))
  forced drops (food, SoU, PoS, …) + feeling
  builder()  // loop vs figure-eight + curve params
  initRooms() + shuffle
  retry builder.build until success
  RegularPainter.paint()  // shuffles the actual rooms list in place
  createMobs()      // depth-one fixture exact; floors 2–24 pending
  createItems main loop
Random.popGenerator
depth++
```

### `seedForDepth`
```text
pushGenerator(seed); Long() × depth; result = Long(); pop
```

---

## Frontend contracts (stable — reference only)

### WASM
- `parse_seed(input) → SeedInfo`
- `analyze_seed(input, floors) → SeedReport`
- `search_seeds(request) → SeedSearchResult` (bounded to 250 candidates, 32 constraints, 100 matches/call)
- `spd_version()` / `spd_commit()`

### `FloorMap` JSON
```json
{
  "width": 40,
  "height": 30,
  "tileset": "sewers",
  "tiles": [4, 4, 1, 5, 7, 8, ...],
  "tile_variance": [12, 68, 97, 3, ...],
  "markers": [{ "cell": 318, "kind": "item", "label": "Potion of Healing" }]
}
```
Tiles are SPD `Terrain` values; `tile_variance` is the pinned
`DungeonTileSheet.setupVariance(seedCurDepth)` stream. Marker cells are
row-major, bounds-checked, limited to placements the partial engine actually
knows.

If parity fixes start retaining item-to-cell/quantity/level/heap-type on
`SeedReport` items (needed for step 5 above), this contract will need a
matching update — check `web/src/lib/` consumers before changing shape.

---

## Testing

```bash
cargo test -p spd-core
```

`java_oracle_goldens.rs` (+ `java_oracle_goldens/final_heaps.rs`) is the
parity harness: identity maps (schema v1), depth-one forced-item queue
(schema v2), and four exact depth-one lifecycle fixtures covering map bounds,
heap cells, mob facts, and report-visible item projection (schema v3, see
above). Add
tightly-scoped oracle fixtures before writing new Rust behavior — regenerate
via `tools/java-oracle/run` (see `tools/java-oracle/README.md`).

---

## License

SPD is GPL-3.0. This project ports generation logic → treat as
**GPL-3.0-or-later** when publishing. Assets are from SPD and under the same
license constraints.

---

## How to resume (clean context)

1. Read this file, specifically **"What's lacking for exact parity"** above.
2. The `AAA-AAA-AAA`, `ABC-DEF-GHI`, `GFX-PZH-DCH`, and `hello` depth-one
   lifecycles are exact at the pre-painter, pre-mobs, and pre-items boundaries
   and for final map bounds, heap cells, mob facts, and report-visible items.
   Keep all four fixtures green.
3. Continue with the `HKT-JZN-XQQ` visual-oracle phase in suggested fix order
   item 1. Preserve `partial` status while broader depth-one and deeper-floor
   parity remain incomplete.
4. Validate against `crates/spd-core/tests/java_oracle_goldens/final_heaps.rs`
   and all `*-final-heaps-floor-1.json` fixtures; regenerate/extend fixtures
   via `tools/java-oracle/run` (default source is the pinned clone at
   `/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`).
5. After Rust changes: `bun run build:wasm` (or `bun run dev`) before treating
   the UI as verified.
6. Dev dump: `cargo run -p spd-core --example dump_seed -- SEED FLOORS`
