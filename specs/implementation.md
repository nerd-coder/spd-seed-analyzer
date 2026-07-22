# SPD Seed Analyzer — Implementation Plan

**Last updated:** 2026-07-22
**Branch:** `main`
**Pinned SPD:** v3.3.8 @ `7b8b845a7`
**Local game source:** `/Users/toan/code/00-Evan/shattered-pixel-dungeon`
  (⚠ `tools/java-oracle/run`'s `DEFAULT_SOURCE` still points at the stale
  `/Users/toan/code/repos/00-Evan/...` path — pass `SPD_SOURCE=...` or fix the
  default before relying on it)

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
│   │   ├── src/level/          # per-floor build/paint/createItems/createMobs(missing)
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

**Levelgen — ported broadly, diverges in RNG-stream order.** Room init,
geometry, both builders (Loop/FigureEight), all connection-room subclasses,
water/grass/trap painter, `paintDoors` merge/Graph, every region's
structural + standard room geometry, special/secret room prize logic, shop
stock, all four quests, crystal rooms, the main `createItems` drop loop, and
floor-map export are all implemented. The known problem is **RNG call order
during and after `createItems`** — see gaps below. This is why the schema-v3
final-heaps oracle test (`java_oracle_goldens/final_heaps.rs`) explicitly
documents a mismatch instead of asserting equality.

**Frontend — functionally complete for a `partial` engine, not the current
focus.** Analyze + Find-seeds modes, multi-seed session tabs, spoiler
toggles, map rendering with autotiling, bounded seed-constraint search. See
git history / `AGENTS.md` for UI details; no open frontend work is blocking
parity.

**Correctness infra — the tool that will prove parity.**
`tools/java-oracle/` runs the *actual pinned Java source* headlessly and
dumps JSON: schema v1 (run identities), v2 (depth-one pre-build forced-item
queue), v3 (final placed heaps after real `Level.create()`, ordered by cell,
full item/heap facts, no report-shaped filtering). Regenerate fixtures with
the exact commands in `tools/java-oracle/README.md`. This is the intended
mechanism for closing every gap below — add or extend a fixture, then make
the Rust side match it exactly.

---

## What's lacking for exact parity

Verified against the pinned Java source
(`/Users/toan/code/00-Evan/shattered-pixel-dungeon`) and against the existing
schema-v3 fixture (`aaa-aaa-aaa-final-heaps-floor-1.json`, seed
`AAA-AAA-AAA` depth 1), whose Java-visible projection
`[Food, PotionOfHealing, PotionOfInvisibility, ScaleArmor, ScrollOfRage,
ScrollOfRecharging, StoneOfAggression, StoneOfBlink]` currently comes out of
Rust as `[Food, PotionOfInvisibility, StoneOfAggression, StoneOfBlink,
StoneOfDeepSleep, ThrowingHammer]` (post-gap-3). These bugs compound on the
same RNG stream — fixing one alone will not flip that test green, but each is
independently correct and unit-testable.

### 1. `createMobs()` is completely unported — the root desync (Large)
Java's per-floor order is `build() → createMobs() → createItems()`
(`Level.java:313-314`). `RegularLevel.createMobs()`
(`RegularLevel.java:219-306`) unconditionally burns RNG *before* `createItems`
ever runs: `Random.shuffle(stdRooms)` weighted by `mobSpawnWeight()`, then per
mob a `createMob()` (rotation + champion roll) and a retry loop (up to 30
tries) gated by `ShadowCaster` entrance FOV, `PathFinder` distance map,
`canPlaceCharacter`, traps/plants/open-space. Depth 1 always spawns exactly 8
mobs. Rust's `level/mod.rs` jumps straight from special-room loot / quests to
`create_items::create_items_main` (`level/mod.rs:350`) — so the very first
RNG draw inside `createItems` (the `n_items` roll) is already reading from the
wrong position. **Nothing downstream can match until this is ported**, even
if it only needs to *burn* the correct RNG shape rather than render mobs.

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
  marked with `// TODO(gap 1):` at its exact conjunct position.

Tests moved to `level/create_items/tests.rs` (SMALL-FILES split; same module
path). New coverage drives a parallel `JavaRandom` twin oracle that
transcribes the Java method (JDK `Collections.shuffle` loop + watabou
`IntRange` semantics) and asserts both the selected cell AND the total draw
count (post-call `next_int` sync): full-list reshuffle per try, exit-room
hosting minus the exit cell, entrance-wastes-try, degenerate zero-draw pick,
and no-match→-1-after-one-shuffle. The AAA-AAA-AAA final-heaps projection
was re-characterized (now shares 6 of 8 Java items; still `assert_ne` —
gap 1 `createMobs` RNG shape remains).

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
- Full ambient `createMobs` also feeds map markers — only exact known cells
  (room-painted heaps/mimics, RotGarden, DemonSpawner) are shown today.
- `SecretLaboratoryRoom` reuses `LaboratoryRoom`'s prize body
  (`laboratory_prizes_shared`); Java gives it its own `paint()` with a
  weighted `potionChances` table (2 potions) — different RNG shape whenever
  the room appears (found while fixing gap 3; no fixture covers it yet).
- Sewer room-count tables are reused for all regions.
- Shop stock is generated post-build instead of mid-`setSize`; bag choice is
  hero-less.
- CrystalPath placement geometry is approximate (prize generation itself is
  exact).
- Structural-room paint/transition retry loops are capped at 10,000 attempts
  for browser safety (valid layouts shouldn't hit this); `Maze.generate` kept
  its real 2,500-failure limit.
- The early Guidebook page uses an intentionally unseeded Java generator and
  stays outside scope everywhere (oracle and Rust agree on omitting it).

---

## Suggested fix order

The remaining two bugs share one RNG stream, so verify each in isolation with
a narrow unit test, but expect the `java_oracle_goldens/final_heaps.rs`
mismatch test to stay red until **both** land:

1. ~~Gap 3~~ — **done** (9 forced-item pushes + per-room push ordering).
2. ~~Gap 2~~ — **done** (`random_drop_cell` full-list reshuffle per try,
   Java instanceof semantics for Entrance/Exit/Standard, exact exit-cell
   exclusion, zero-draw degenerate IntRange, no fallback).
3. **Gap 1** (`createMobs`) — largest lift. Decide up front whether to fully
   port `ShadowCaster`/`PathFinder`/mob placement (enables future mob map
   markers) or build a reduced path that burns *identical* RNG shape without
   real placement (faster to parity, defers mob rendering). Either way this
   must consume the exact same `Random` calls Java does before `createItems`
   begins.
4. Once both are in, flip
   `depth_one_final_heaps_characterize_known_analyzer_mismatch` from
   `assert_ne!` to exact fact equality (cell, heap type, quantity, level,
   curse — the v3 fixture already carries all of it; the Rust report
   currently only retains class+cursed, so the report/analyzer model likely
   needs to start retaining cell/quantity/level/heap-type too).
5. Extend schema-v3 fixtures to more seeds/depths once depth-one matches, to
   catch anything depth-1-specific (e.g. the "8 mobs on depth 1" special
   case) that wouldn't show up on other floors.

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
  paint_minimal → FloorMap
  createMobs()      // <-- NOT YET PORTED (gap 1); Java runs this before createItems
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
  "width": 41,
  "height": 43,
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

If gap-1/2/3 fixes start retaining cell/quantity/level/heap-type on
`SeedReport` items (needed for step 4 above), this contract will need a
matching update — check `web/src/lib/` consumers before changing shape.

---

## Testing

```bash
cargo test -p spd-core
```

`java_oracle_goldens.rs` (+ `java_oracle_goldens/final_heaps.rs`) is the
parity harness: identity maps (schema v1), depth-one forced-item queue
(schema v2), and the known final-heap mismatch (schema v3, see above). Add
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
2. ~~Gap 3~~ — done. ~~Gap 2~~ — done
   (`level/create_items.rs` `random_drop_cell` now shuffles the full per-floor
   room list per try with Java `StandardRoom` instanceof semantics).
3. Then gap 1 (`level/mod.rs` + new `createMobs` port) — the only remaining
   RNG-stream gap before `createItems`; `random_drop_cell` already carries a
   `// TODO(gap 1):` marker where the `findMob(pos) == null` conjunct lands.
4. Validate against `crates/spd-core/tests/java_oracle_goldens/final_heaps.rs`
   and the `aaa-aaa-aaa-final-heaps-floor-1.json` fixture; regenerate/extend
   fixtures via `tools/java-oracle/run` (set `SPD_SOURCE=/Users/toan/code/00-Evan/shattered-pixel-dungeon`).
5. After Rust changes: `bun run build:wasm` (or `bun run dev`) before treating
   the UI as verified.
6. Dev dump: `cargo run -p spd-core --example dump_seed -- SEED FLOORS`
