# SPD Seed Analyzer — Implementation Plan

**Last updated:** 2026-07-23
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

**Levelgen — broad partial port; five depth-one lifecycle fixtures are exact.**
Room init, geometry, both builders (Loop/FigureEight), all connection-room
subclasses, water/grass/trap painter, `paintDoors` merge/Graph, every region's
structural + standard room geometry, special/secret room prize logic, shop
stock, all four quests, crystal rooms, the main `createItems` drop loop, and
floor-map export are implemented. For `AAA-AAA-AAA`, `ABC-DEF-GHI`,
`GFX-PZH-DCH`, `hello`, and `HKT-JZN-XQQ` at depth 1, Rust now reaches the
exact Java RNG states before painter, `createMobs`, and `createItems`; matches
map bounds,
all final mob cells/types, all final heap cells, and the report-visible item
projection. The HKT visual fixture additionally proves exact final terrain,
discoverability, tile variance, transitions, traps (including sprite metadata),
structured heaps, and structured mobs. This is deliberately still `partial`:
five room sets do not cover all depth-one combinations, deeper-floor mob
generation and several room-specific predicates/paint paths remain incomplete.
Structured heap facts
are exact only for the covered room families whose paint-time cell association
has been ported; uncovered families retain the legacy `Room loot` marker.

Floor 6 now has a full pinned Java observation after replaying floors 1–5:
48×48 terrain/render arrays, lifecycle probes, 36 heaps, 7 mobs, 2
transitions, and 3 traps. Rust matches the water feeling, exact 12-room class
set, 20 structured shop heaps, Shopkeeper, and fresh-Warrior MagicalHolster
choice. It does **not** yet match the floor lifecycle: Rust is 49×48, has one
fewer ambient mob, and diverges at every recorded RNG boundary and in final
terrain/entity cells.

Floor 8 now has the same full Java observation after replaying floors 1–7:
30×42 render arrays, all three lifecycle probes, 11 heaps, 7 mobs, 2
transitions, 14 traps, and one active blob. Rust matches the `none` feeling,
exact 15-room class set, 30×42 bounds, and the complete pre-paint RNG probe.
It diverges during painting, so terrain, later probes, heap/mob facts,
transition cells, and trap facts remain partial.

**Frontend — HKT floor-one deterministic render composition is matched.**
Analyze + Find-seeds modes, multi-seed session tabs, spoiler toggles, bounded
seed-constraint search, and map rendering with pinned autotiling are present.
The browser now consumes the core's structured discoverability, transition,
trap, heap, and mob facts; HKT floor 1 renders exact trap and entity sprites in
pinned GameScene layer order inside an integer-scaled, discoverability-bounded
viewport. The engine and UI still report `partial`; this browser slice does not
promote uncovered room families or deeper floors.

The renderer can now compose the exact item crops and static idle sprites used
by the floor-6 and floor-8 oracles. Floor 8 adds DM100, Guard, and
SpectralNecromancer alongside its exact AlchemyPage, Crossbow, and
EnergyCrystal crops. This is asset-layer support, not deeper-floor visual
parity: current Rust geometry and entity cells still differ from Java.

**Correctness infra — the tool that will prove parity.**
`tools/java-oracle/` runs the *actual pinned Java source* headlessly and
dumps JSON: schema v1 (run identities), v2 (depth-one pre-build forced-item
queue), v3 (final placed heaps after real `Level.create()`, ordered by cell,
full item/heap facts plus final mob cells/types and lifecycle RNG probes, no
report-shaped filtering). Schema v3 supports direct depth 1 plus sequentially
replayed depths 6 and 8. It can also include additive render facts:
terrain, discoverability, tile variance, transitions, traps, plants, and active
blobs. Regenerate fixtures with
the exact commands in `tools/java-oracle/README.md`. This is the intended
mechanism for closing every gap below — add or extend a fixture, then make
the Rust side match it exactly.

---

## What's lacking for exact parity

Verified against the pinned Java source
(`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`) and seven schema-v3
fixtures. The depth-one suite covers Pool/Runestone, MagicalFire,
CrystalPath/MagicWell, Traps/Treasury, SewerPipe, RegionDecoPatch,
Bridge/Ring/CircleBasin, tunnel, WaterBridge, and HKT's Armory/FigureEight
variants. Every depth-one fixture asserts exact lifecycle RNG probes, map
bounds, final heap cells, final mob cells/types, and report-visible item
projection. The HKT floor-one fixture also asserts every
additive render fact listed above. The floor-six and floor-eight fixtures retain
the same full Java observation, but Rust asserts only each currently exact
subset.

### 0c. ~~Pin HKT floor-six core/render facts~~ — ORACLE EXACT, RUST PARTIAL
For `HKT-JZN-XQQ` floor 6, schema v3 now replays every prior Java floor before
creating the target `PrisonLevel`. The committed oracle records the exact
48×48 map, lifecycle RNG probes, terrain, discoverability, tile variance,
transitions, traps, 36 heaps (20 `for_sale`), and 7 mobs. Regeneration from the
pinned checkout is deterministic; the committed JSON is Biome-formatted after
generation.

The strongest honest Rust projection now matches the water feeling and sorted
room classes: CrystalChoice, four Perimeter rooms, Pillars entrance/standard,
RegionDecoLine exit, SecretMaze, two Segmented rooms, and Shop. Region-specific
room counts and the laboratory-quota draw now occur at their pinned positions.
Shop stock is generated lazily during builder sizing; the room paints 20
clockwise structured sale heaps plus Shopkeeper, and fresh-Warrior inventory
selects MagicalHolster. Floors 2–4 and 6–8 have source-aligned partial
ambient-mob rotations/placement, while only depth 1 remains lifecycle-exact.

This phase intentionally stops at comparison, not parity. Rust currently
builds 49×48, produces five ambient mobs plus Shopkeeper rather than Java's six
plus Shopkeeper, and misses every floor-six lifecycle probe. Terrain,
transitions, traps, heap contents/cells, and mob cells therefore remain open.
The browser can render the oracle's floor-six item/mob sprite families, but its
current composition reflects those partial Rust facts. The reference hero,
post-exploration FOV, and animated-water phase remain outside deterministic
`Level.create()` comparison.

### 0d. ~~Pin HKT floor-eight core/render facts~~ — ORACLE EXACT, RUST PARTIAL
For `HKT-JZN-XQQ` floor 8, schema v3 replays Java floors 1–7 before creating
the target `PrisonLevel`. The committed oracle records the exact 30×42 map,
all lifecycle probes and render arrays, 11 heaps, 7 mobs, 2 transitions, 14
traps, and one Alchemy blob. Its exact rooms are Laboratory, five Perimeter,
Pillars, RegionDecoLine entrance/exit/standard, SecretSummoning, three
Segmented, and Walkway.

The strongest honest Rust projection matches the `none` feeling, 30×42 bounds,
sorted room set, and every pre-paint RNG value. Divergence begins during
painting: Rust has 6 heaps rather than 11, 8 mobs rather than 7, 2 traps rather
than 14, different transition cells, and different pre-mobs/pre-items probes,
terrain, and entity cells. Floor 7 already differs in room structure and
Wandmaker reward state, so this phase intentionally pins and compares floor 8
without claiming downstream parity.

The browser now has exact static crops for every floor-8 heap and mob family,
including DM100, Guard, SpectralNecromancer, AlchemyPage, Crossbow, and
EnergyCrystal. The visual comparison still excludes the reference hero,
post-exploration FOV, and animation phase, and it remains partial because the
core cells differ.

### 0a. ~~HKT depth-one visual-oracle parity~~ — CORE FACTS EXACT
For `HKT-JZN-XQQ`, Rust now matches the pinned Java floor cell-for-cell for
terrain, discoverability, and tile variance, with exact transitions, traps,
structured heaps, and structured mobs. The trap cells are 283, 350, and 567;
their visibility and sprite color/shape metadata also match.

Closing this slice required restoring Java's FigureEight room insertion order,
shifting rooms to level-local coordinates before painter calculations, removing
guessed midpoint doors from the initial terrain map, preserving overlapping
trap candidates with Java's first-match removal semantics, excluding the
depth-one entrance room from trap placement, and flattening tall or furrowed
grass under late item drops. Armory and Treasury painters now retain exact heap
cells and ordered item stacks for the structured report.

Committed screenshots in `specs/fixtures/visual/`, named
`<seed>_<floor>.png`, are visual-regression references.
Their hero positions are post-generation gameplay state, not seed-derived
`Level.create()` output; frontend comparisons must treat the hero as a fixed
reference overlay or exclude it from the deterministic contract. Floor-1
deterministic browser composition is now matched; floor-6 and floor-8 lifecycle
parity remain open against the pinned oracles above. Core parity does not by
itself prove gameplay-state pixel parity.

### 0b. ~~HKT floor-one browser-render parity~~ — DETERMINISTIC LAYERS MATCHED
For `HKT-JZN-XQQ` floor 1, the browser now consumes every existing structured
render fact instead of reducing heaps and mobs to dots. Visible traps use the
pinned terrain-feature atlas indices (`color + shape*16`); heap containers and
top items use their pinned item frames and seed identity appearances; the eight
Rat/Snake facts use their pinned idle frames. Rendering follows GameScene's
relevant order: lower terrain, terrain features/traps, heaps, mobs, raised
terrain, and walls. Entrance/exit transitions remain terrain-atlas visuals at
their exact structured cells (260 and 804).

The expanded canvas uses a 2× integer backing scale and the deterministic
`cleanWalls()` bounds, retaining one row above for raised overhangs. HKT's
result is 992×1088 rather than the former fractional 981×1040 stretch; its
terrain geometry and asset selection align with the 988×1083 pinned gameplay
capture. Overflow starts at the map's top-left and scrolls instead of centering
oversized content behind the dialog header.

The committed reference still contains fixture-only gameplay state: a hero and
current/post-exploration FOV shading. Those are intentionally excluded because
they are not outputs of `Level.create()`; animated water phase is also not a
fixed pixel contract. Exact heaps/mobs remain opt-in spoiler layers. This closes
only the HKT floor-one browser slice and does not change the engine's `partial`
status. Floor-6 and floor-8 asset coverage is present, but lifecycle/render
parity is open.

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
- Full ambient `createMobs` also feeds map markers — depth one is exact and
  floors 2–4/6–8 have partial source-aligned rotations/placement. Floor 6 is
  still one ambient mob short; floor 8 has 8 mobs versus Java's 7; floors
  9–24 remain absent.
- `SecretLaboratoryRoom` reuses `LaboratoryRoom`'s prize body
  (`laboratory_prizes_shared`); Java gives it its own `paint()` with a
  weighted `potionChances` table (2 potions) — different RNG shape whenever
  the room appears (found while fixing gap 3; no fixture covers it yet).
- Shop stock is builder-timed and fresh-Warrior bag scoring is exact for floor
  6. Later shops still need inventory-sensitive bag modeling.
- `random_deck_item` still has a known exhausted-probability private-generator
  state/push-pop mismatch. None of the six fixtures exercises that rollover;
  fix it with a dedicated Java draw-shape fixture rather than folding it into
  an unrelated room patch.
- Structural-room paint/transition retry loops are capped at 10,000 attempts
  for browser safety (valid layouts shouldn't hit this); `Maze.generate` kept
  its real 2,500-failure limit.
- The early Guidebook page uses an intentionally unseeded Java generator and
  stays outside scope everywhere (oracle and Rust agree on omitting it).

---

## Suggested fix order

1. ~~**HKT browser-render parity.**~~ Rebuilt WASM, rendered
   `HKT-JZN-XQQ` floor 1 in the browser, and compared the frontend's asset
   composition against the corresponding committed visual fixture. Structured
   transitions/traps/heaps/mobs and the cropped integer viewport now compose
   without reimplementing RNG in the UI.
2. ~~**Pin and compare `HKT-JZN-XQQ` floor 6.**~~ Added the sequential Java
   oracle, exact render facts, strongest honest Rust room/shop assertions, and
   floor-6 sprite/icon coverage. The comparison proves lifecycle/render parity
   is still open.
3. **Close HKT floor-6 lifecycle parity.** Match the 48×48 builder boundary,
   painter/terrain facts, sixth ambient mob, final heap/entity cells, and all
   three RNG probes before promoting browser parity.
4. ~~**Pin and compare `HKT-JZN-XQQ` floor 8.**~~ Added the sequential Java
   oracle, exact render facts, strongest honest Rust pre-paint assertions,
   source-aligned floor-7/8 mob rotations, and floor-8 sprite/icon coverage.
   The comparison proves painter/downstream parity is still open.
5. **Close HKT floor-8 painter/lifecycle parity after floor 6.** Match terrain,
   pre-mobs/pre-items probes, 11 heap cells, 7 mob cells, 14 traps, transition
   cells, and the Alchemy blob before promoting browser parity.
6. Extend schema-v3 coverage to still more depth-one room sets. Five exact
   lifecycles are strong regression fixtures, not evidence that every
   depth-one combination is exact.
7. Port deeper-floor `createMobs`: mob limits/rotations, second-room spawns,
   large-mob open-space checks, and quest/NPC occupancy.
8. Close the remaining room-specific `canPlaceItem` predicates and known
   special-room paint gaps.
9. Correct the remaining timing/geometry approximations (SecretLaboratory and
   inventory-sensitive later shops) as new fixtures cover them.
10. Extend exact paint-time heap capture to the remaining room families; keep
   the legacy marker fallback until each family has a pinned cell association.
11. Add multi-depth schema-v3 fixtures and promote each newly covered region
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
  createMobs()      // depth 1 exact; floors 2–4/6–8 partial; 9–24 pending
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
- `search_seeds(request) → SeedSearchResult` (bounded to 250 candidates,
  32 constraints, 100 matches/call)
- `spd_version()` / `spd_commit()`

### `FloorMap` JSON
```json
{
  "width": 40,
  "height": 30,
  "tileset": "sewers",
  "tiles": [4, 4, 1, 5, 7, 8, ...],
  "tile_variance": [12, 68, 97, 3, ...],
  "discoverable": [false, false, true, ...],
  "markers": [{ "cell": 318, "kind": "item", "label": "Potion of Healing" }],
  "heaps": [{ "cell": 318, "heap_type": "heap", "items": [...] }],
  "mobs": [{ "cell": 234, "class": "Rat" }],
  "transitions": [],
  "traps": [],
  "plants": [],
  "blobs": []
}
```
Tiles are SPD `Terrain` values; `tile_variance` is the pinned
`DungeonTileSheet.setupVariance(seedCurDepth)` stream. Marker cells are
row-major, bounds-checked, limited to placements the partial engine actually
knows. Structured arrays retain the render-relevant Java facts known to the
core; all fields are additive and default empty for older serialized data.

---

## Testing

```bash
cargo test -p spd-core
```

`java_oracle_goldens.rs` (+ `java_oracle_goldens/final_heaps.rs`) is the
parity harness: identity maps (schema v1), depth-one forced-item queue
(schema v2), five exact depth-one lifecycle fixtures covering map bounds,
heap cells, mob facts, and report-visible item projection, plus full Java
floor-six and floor-eight observations (schema v3, see above). HKT floor 1
additionally proves the full render-fact projection; floors 6 and 8 currently
assert only their exact Rust subsets. Add
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
2. The `AAA-AAA-AAA`, `ABC-DEF-GHI`, `GFX-PZH-DCH`, `hello`, and
   `HKT-JZN-XQQ` depth-one
   lifecycles are exact at the pre-painter, pre-mobs, and pre-items boundaries
   and for final map bounds, heap cells, mob facts, and report-visible items.
   Keep all five fixtures green.
3. Continue closing `HKT-JZN-XQQ` floor-6 lifecycle parity in suggested fix
   order item 3. Its Java facts are fully pinned, but Rust still differs in
   bounds, all RNG probes, terrain, heaps, and mob cells. Preserve `partial`.
4. Validate against `crates/spd-core/tests/java_oracle_goldens/final_heaps.rs`,
   its focused child modules, and all committed schema-v3 fixtures;
   regenerate/extend fixtures via
   `tools/java-oracle/run` (default source is the pinned clone at
   `/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`).
5. After floor 6 closes, continue floor-8 parity from its exact pre-paint
   boundary using the committed sequential oracle and
   `specs/fixtures/visual/HKT-JZN-XQQ_F8.png`.
6. After Rust changes: `bun run build:wasm` (or `bun run dev`) before treating
   the UI as verified.
7. Dev dump: `cargo run -p spd-core --example dump_seed -- SEED FLOORS`
