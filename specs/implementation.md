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
Rust as `[Food, StoneOfAggression, StoneOfBlink, StoneOfDeepSleep,
ThrowingHammer]`. These three bugs compound on the same RNG stream — fixing
one alone will not flip that test green, but each is independently correct
and unit-testable.

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

### 2. `random_drop_cell` shuffles the wrong-sized list (Medium)
Java's `randomRoom(type)` (`RegularLevel.java:707-710`) shuffles the level's
**entire** `rooms` list (every room: entrance, exit, standard, special,
secret, shop, connectors) on *every call*, then linear-scans for the first
room of the requested type (`randomDropCell`, `RegularLevel.java:736-766`).
Fisher-Yates cost scales with total room count. Rust's `random_drop_cell`
(`crates/spd-core/src/level/create_items.rs:198-258`) instead pre-filters to
`RoomKind::Standard` rooms *before* shuffling, then reshuffles that smaller
list on each of its 100 tries — a different RNG-call count on literally every
invocation (main loop ×3-5, `itemsToSpawn` placement, trailing
torch/rose/guide draws). Fix: shuffle the full per-floor room list (a
precedent already exists in `special_loot::special_room_loot`, which shuffles
a full index list the same way Java does), then take the first
`RoomKind::Standard` match.

### 3. Eight special/secret rooms drop their trailing forced-item push (Small, precise)
Java's `paint()` queues one more item via `level.addItemToSpawn(...)` at the
very end of these rooms; the Rust port of the same function omits it, so
`itemsToSpawn` is one entry short and every subsequent `randomDropCell` +
per-drop `pushGenerator(Random.Long())` in the loop is shifted:

| Room | Missing push | Rust location | Java location |
|---|---|---|---|
| RunestoneRoom | `IronKey` | `special_loot/special_rooms/consumable.rs:147` `runestone_prizes` | `RunestoneRoom.java:64` |
| LibraryRoom | `IronKey` | `consumable.rs:12` `library_prizes` | `LibraryRoom.java:65` |
| TreasuryRoom | `IronKey` | `consumable.rs:55` `treasury_prizes` | `TreasuryRoom.java:76` |
| LaboratoryRoom | `IronKey` | `consumable.rs:175` `laboratory_prizes` (also inherited by `secret_rooms.rs:82` `secret_laboratory`) | `LaboratoryRoom.java:121` |
| CryptRoom | `IronKey` | `special_rooms/equip.rs:13` `crypt_prize` (doesn't take `items_to_spawn` yet) | `CryptRoom.java:51` |
| StatueRoom | `IronKey` | `equip.rs:130` `statue_weapon` (doesn't take `items_to_spawn` yet) | `StatueRoom.java:46` |
| ArmoryRoom | `IronKey` | `equip.rs:33` `armory_prizes` | `ArmoryRoom.java:78` |
| PoolRoom | `PotionOfInvisibility` | `equip.rs:89` `pool_prize` | `PoolRoom.java:91` |
| SecretRunestoneRoom | `PotionOfLiquidFlame` | `secret_rooms.rs:39` `secret_runestone` | `SecretRunestoneRoom.java:64` |

(`trap_rooms.rs`, `pit_secrets.rs`, `quest_rooms.rs`, `gardens.rs`,
`crystal.rs` already push their forced items correctly — bug is isolated to
the 9 sites above.) `crypt_prize` and `statue_weapon` need
`items_to_spawn: &mut Vec<GeneratedItem>` threaded through from
`special_loot/mod.rs` call sites first.

The AAA-AAA-AAA fixture's expected `PotionOfInvisibility` (cell 454, `heap`)
and `IronKey` (cell 941) directly confirm this floor has both a PoolRoom and
a RunestoneRoom hitting this bug.

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

These three bugs share one RNG stream, so verify each in isolation with a
narrow unit test, but expect the `java_oracle_goldens/final_heaps.rs` mismatch
test to stay red until **all three** land:

1. **Gap 3 first** (small, isolated, no plumbing changes beyond two function
   signatures) — add the 9 missing pushes. Cheapest to verify: rerun
   `cargo test -p spd-core` and diff the changed `itemsToSpawn` shape.
2. **Gap 2** (`random_drop_cell` shuffle scope) — needs the full per-floor
   room list passed in instead of a pre-filtered `Vec`; reuse the shuffle
   pattern already in `special_loot::special_room_loot`.
3. **Gap 1** (`createMobs`) — largest lift. Decide up front whether to fully
   port `ShadowCaster`/`PathFinder`/mob placement (enables future mob map
   markers) or build a reduced path that burns *identical* RNG shape without
   real placement (faster to parity, defers mob rendering). Either way this
   must consume the exact same `Random` calls Java does before `createItems`
   begins.
4. Once all three are in, flip
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
2. Start with gap 3 (`level/special_loot/special_rooms/{consumable,equip}.rs`,
   `level/special_loot/secret_rooms.rs`) — smallest, most isolated, most
   verifiable.
3. Then gap 2 (`level/create_items.rs:198-258`), then gap 1
   (`level/mod.rs:350` + new `createMobs` port).
4. Validate against `crates/spd-core/tests/java_oracle_goldens/final_heaps.rs`
   and the `aaa-aaa-aaa-final-heaps-floor-1.json` fixture; regenerate/extend
   fixtures via `tools/java-oracle/run` (set `SPD_SOURCE=/Users/toan/code/00-Evan/shattered-pixel-dungeon`).
5. After Rust changes: `bun run build:wasm` (or `bun run dev`) before treating
   the UI as verified.
6. Dev dump: `cargo run -p spd-core --example dump_seed -- SEED FLOORS`
