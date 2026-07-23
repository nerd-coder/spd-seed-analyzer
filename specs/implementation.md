# SPD Seed Analyzer — Implementation Plan

**Last updated:** 2026-07-23
**Branch:** `main`
**Pinned SPD:** v3.3.8 @ `7b8b845a7`
**Local game source:** `/Users/toan/code/00-Evan/shattered-pixel-dungeon`

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

**Levelgen — broad partial port; seven depth-one fixtures plus HKT replay
floors 6–8 are exact.**
Room init, geometry, both builders (Loop/FigureEight), all connection-room
subclasses, water/grass/trap painter, `paintDoors` merge/Graph, every region's
structural + standard room geometry, special/secret room prize logic, shop
stock, all four quests, crystal rooms, the main `createItems` drop loop, and
floor-map export are implemented. For `AAA-AAA-AAA`, `AAA-AAA-AAD`,
`AAA-AAA-AFO`, `ABC-DEF-GHI`, `GFX-PZH-DCH`, `hello`, and `HKT-JZN-XQQ` at
depth 1, Rust now reaches the exact Java RNG states before painter,
`createMobs`, and `createItems`; matches map bounds, all final mob cells/types,
all final heap cells, and the report-visible item projection. The AAA-AAD fixture adds Garden,
Sacrifice, and Striped room coverage and pins RingRoom's center heap and
forced-queue consumption. The AFO and HKT full-additive fixtures additionally
prove exact final terrain, discoverability, tile variance, transitions, traps,
structured heaps, and structured mobs; HKT also pins trap sprite metadata.
This is deliberately still `partial`: seven room sets do not cover all
depth-one combinations, and deeper-floor lifecycle coverage plus several
room-specific predicates/paint paths remain incomplete. Structured heap facts
are exact only for the covered room families whose paint-time cell association
has been ported; uncovered families retain the legacy `Room loot` marker.

Floor 6 now has an exact pinned lifecycle after replaying floors 1–5. For
`HKT-JZN-XQQ`, Rust matches Java's water feeling, 12-room class set, 48×48
bounds, all three RNG probes, terrain/discoverability/tile variance, 36 heaps
(including 20 shop heaps), 7 mobs, 2 transitions, and 3 traps. This is exact
for the committed fresh-run fixture only; the engine remains `partial` because
other deeper-floor histories and room combinations are not yet covered.

Floor 7 now has a full pinned Java observation after replaying floors 1–6.
For `HKT-JZN-XQQ`, Rust matches the `none` feeling, exact 16-room class set,
41×35 bounds, all three lifecycle probes, terrain/discoverability/tile
variance, 15 heaps, 8 mobs, 2 transitions, 2 traps, empty plant/blob sets, and
the persistent rewards `WandOfPrismaticLight +1` and `WandOfCorrosion +1`.
This is exact for the committed fresh-run fixture only; global status stays
`partial` for other histories and uncovered room combinations.

Floor 8 now has an exact pinned lifecycle after replaying floors 1–7. For
`HKT-JZN-XQQ`, Rust matches the `none` feeling, exact 15-room class set, 30×42
bounds, all three lifecycle probes, terrain/discoverability/tile variance, 11
heaps, 7 mobs, 2 transitions, 14 traps, and the active Alchemy blob. As with
floor 6, this proves the committed fresh-run history only; the engine remains
`partial` for other deeper-floor histories and uncovered room combinations.

**Frontend — HKT floor-one deterministic render composition is matched.**
Analyze + Find-seeds modes, multi-seed session tabs, spoiler toggles, bounded
seed-constraint search, and map rendering with pinned autotiling are present.
The browser now consumes the core's structured discoverability, transition,
trap, heap, and mob facts; HKT floor 1 renders exact trap and entity sprites in
pinned GameScene layer order inside an integer-scaled, discoverability-bounded
viewport. The engine and UI still report `partial`; this browser slice does not
promote uncovered room families or deeper floors.

The renderer can now compose the exact item crops and static idle sprites used
by the floor-6 and floor-8 oracles. Both floors use exact core cells for the
committed HKT replays. Floor 8 adds DM100, Guard, and SpectralNecromancer
alongside its exact AlchemyPage, Crossbow, and EnergyCrystal crops. A dedicated
floor-8 browser comparison has not yet promoted that asset coverage to a
pixel-parity claim.

`CXG-FJT-BFQ` floor 1 is registered alongside the three HKT screenshots in the
typed map-render QA fixture list. A Playwright harness under `tools/visual/`
drives the real analyzer UI and performs strict zero-difference comparisons of
the deterministic map canvas for all four cases. Its browser baselines are
separate from the gameplay captures because hero, explored-FOV, and animation
state remain outside the deterministic contract; passing the harness is not a
full gameplay pixel-parity claim.

**Correctness infra — the tool that will prove parity.**
`tools/java-oracle/` runs the *actual pinned Java source* headlessly and
dumps JSON: schema v1 (run identities), v2 (depth-one pre-build forced-item
queue), v3 (final placed heaps after real `Level.create()`, ordered by cell,
full item/heap facts plus final mob cells/types and lifecycle RNG probes, no
report-shaped filtering). Schema v3 supports direct depth 1 plus sequentially
replayed depths 6, 7, and 8. It also emits additive render facts:
terrain, discoverability, tile variance, transitions, traps, plants, and active
blobs; lifecycle-only fixtures may intentionally normalize those fields to
`null`. Regenerate fixtures with
the exact commands in `tools/java-oracle/README.md`. This is the intended
mechanism for closing every gap below — add or extend a fixture, then make
the Rust side match it exactly.

---

## What's lacking for exact parity

Verified against the pinned Java source
(`/Users/toan/code/00-Evan/shattered-pixel-dungeon`) and ten schema-v3
fixtures. The depth-one suite covers Pool/Runestone, MagicalFire,
CrystalPath/MagicWell, Traps/Treasury, SewerPipe, RegionDecoPatch,
Bridge/Ring/CircleBasin, Garden/Sacrifice/Striped, tunnel, WaterBridge, Crypt,
and HKT's Armory/FigureEight variants. Every depth-one fixture asserts exact
lifecycle RNG probes, map bounds, final heap cells, final mob cells/types, and
report-visible item projection. The AFO and HKT floor-one fixtures also assert
every additive render fact listed above. The floor-six, floor-seven, and
floor-eight fixtures now assert their complete Java observations and final
render/entity facts for the committed replays.

The supported regular-floor room-placement predicates are now source-exact.
Plants, Aquarium, every StandardBridge family, CavesFissure (including its
entrance/exit subclasses), and RitualSite apply their pinned `canPlaceItem`
and paired `canPlaceCharacter` exclusions without extra predicate RNG beyond
the already-pinned painter setup. VaultLevel branch rooms remain outside the
regular-floor analyzer and are not implied by this coverage.

### 0g. ~~Close CryptRoom paint parity with AAA-AFO~~ — FIXTURE EXACT
For `AAA-AAA-AFO` at depth 1, Rust now matches Java's Crypt, Ring,
RegionDecoPatch exit, SewerPipe, WaterBridge entrance, Tunnel, and Walkway
rooms; 35×33 bounds; all three lifecycle RNG probes; all 1,155 terrain,
discoverability, and tile-variance cells; 2 transitions; 3 traps; 11 heaps;
and 8 mobs. The Crypt tomb is cell 773 and contains the exact cursed
`MailArmor +1`.

Closing this fixture required `CryptRoom.paint()`'s wall/interior geometry,
both `Room.center()` jitter draws before the entrance-specific coordinate is
overridden, all four directional statue/tomb layouts, and structured `tomb`
heap capture. A focused test pins every entrance orientation and the discarded
center-coordinate draw. The fixture retains every additive render fact, but
global status stays `partial` because other special/secret paint families and
deeper histories remain uncovered.

### 0f. ~~Extend depth-one lifecycle coverage with AAA-AAD~~ — FIXTURE EXACT
For `AAA-AAA-AAD` at depth 1, Rust now matches Java's Garden, Sacrifice,
Striped, two Ring, SewerPipe, RegionDecoPatch entrance, WaterBridge exit, and
six Tunnel rooms; 31×44 bounds; all three lifecycle RNG probes; seven final
heap cells; eight final mob cells/types; and report-visible heap projection.

Closing this fixture required `SacrificeRoom`'s 7×7 minimum, exact capture of
`RingRoom`'s center heap, and preserving the `:forced` provenance when its
`findPrizeItem()` consumes the pre-build Food. Java reports nested plant seed
classes as the ambiguous simple name `Seed`; the golden normalizes that only at
the oracle boundary, while the public analyzer retains `EarthrootSeed` for
icons and finder constraints. Garden plants remain painter facts rather than
portable items, while blob-held Sacrifice and well rewards stay visible in the
public report. Additive render arrays remain deliberately unasserted for this
lifecycle-only fixture, and global status stays `partial`.

### 0c. ~~HKT floor-six lifecycle parity~~ — FIXTURE EXACT
For `HKT-JZN-XQQ` floor 6, schema v3 now replays every prior Java floor before
creating the target `PrisonLevel`. The committed oracle records the exact
48×48 map, lifecycle RNG probes, terrain, discoverability, tile variance,
transitions, traps, 36 heaps (20 `for_sale`), and 7 mobs. Regeneration from the
pinned checkout is deterministic; the committed JSON is Biome-formatted after
generation.

Rust now matches that complete observation: water feeling; sorted room classes
(CrystalChoice, four Perimeter rooms, Pillars entrance/standard,
RegionDecoLine exit, SecretMaze, two Segmented rooms, and Shop); 48×48 bounds;
all lifecycle RNG probes; every render array; and all heap, mob, transition,
trap, plant, and blob facts. Shop stock remains lazily generated at Java's
builder-sizing boundary and selects the fresh-Warrior MagicalHolster.
Later-shop bag selection now scores the direct main backpack like Java: with
generated heaps left uncollected, the same fresh inventory uniquely selects
PotionBandolier at floor 11 and leaves ScrollHolder for floor 16.

Closing the lifecycle required Java-float builder angles, Java room-list order
before FigureEight branches, `EntranceRoom`/`ExitRoom` connection weighting,
`mobLimit()` draw timing, exact SecretLaboratory/SecretLarder and laboratory
guide-page draw shapes on prior floors, exact CrystalChoice geometry and heap
cells, SecretMaze's prize-to-chest association, cumulative-default generator
semantics, and regular GuidePage nested-generator placement. The reference
hero, post-exploration FOV, and animated-water phase remain outside
deterministic `Level.create()` comparison. Global status stays `partial`; this
result proves only the committed HKT fresh-run floor-6 history.

### 0d. ~~Close HKT floor-seven lifecycle parity~~ — FIXTURE EXACT
For `HKT-JZN-XQQ` floor 7, schema v3 replays Java floors 1–6 before creating
the target `PrisonLevel`. The committed oracle records the full 41×35 Java
observation: 16 room classes, all lifecycle probes and render arrays, 15
heaps, 8 mobs, and persistent Wandmaker reward state.

Rust now matches that complete observation: `none` feeling, sorted room set,
41×35 bounds, every lifecycle probe and render array, all 15 heap cells/stacks,
all 8 mob cells/classes including the Wandmaker, both transitions, both traps,
empty plant/blob sets, and the exact `WandOfPrismaticLight +1` and
`WandOfCorrosion +1` rewards.

The earlier boundary work covered the inherited `StandardRoom.setSizeCat()`
draws for `RitualSiteRoom`, CellBlock's temporary `EmptyRoom`, RitualSite's
jittered center, and Wandmaker placement. Closing the final facts required
source-exact StorageRoom wall/`EMPTY_SP` paint and heap-cell retention, its
`BARRICADE` entrance terrain/flags, and PrisonPainter's Java
`instanceof ChasmBridgeRoom` behavior for entrance/exit subclasses. The
committed fixture was independently regenerated from the pinned checkout with
identical canonical JSON. Global status stays `partial`; this proves only the
committed HKT fresh-run floor-7 history.

### 0e. ~~Close HKT floor-eight lifecycle parity~~ — FIXTURE EXACT
For `HKT-JZN-XQQ` floor 8, schema v3 replays Java floors 1–7 before creating
the target `PrisonLevel`. The committed oracle records the exact 30×42 map,
all lifecycle probes and render arrays, 11 heaps, 7 mobs, 2 transitions, 14
traps, and one Alchemy blob. Its exact rooms are Laboratory, five Perimeter,
Pillars, RegionDecoLine entrance/exit/standard, SecretSummoning, three
Segmented, and Walkway.

Rust now matches that complete observation: feeling, sorted room set, bounds,
all three RNG probes, every render array, and all heap, mob, transition, trap,
plant, and blob facts. Closing the painter boundary required exact regular
Laboratory geometry and item-to-cell capture, both chapter-two AlchemyPage
placement draws, the Alchemy pot/blob, SecretSummoning's jittered center and
3×4 hidden-trap field, and PrisonPainter's shuffled room-order CHASM ornament
pass. The blob projection also preserves Java's additive cell concentrations
and emits the report contract's deterministic class/cell ordering.

The browser now has exact static crops for every floor-8 heap and mob family,
including DM100, Guard, SpectralNecromancer, AlchemyPage, Crossbow, and
EnergyCrystal. The deterministic core cells are exact, but a dedicated browser
comparison still needs to account for the reference hero, post-exploration FOV,
and animation phase before claiming pixel parity. Global engine status remains
`partial`; this result proves only the committed HKT fresh-run floor-8 history.

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

Committed screenshots in `tools/visual/fixtures/`, named
`<seed>_F<floor>.png`, are visual-regression references.
Their hero positions are post-generation gameplay state, not seed-derived
`Level.create()` output; frontend comparisons must treat the hero as a fixed
reference overlay or exclude it from the deterministic contract. Floor-1
deterministic browser composition is matched and floor-6 core lifecycle parity
is exact for HKT; floor-8 core lifecycle parity is also exact for its committed
replay. Core parity does not by itself prove gameplay-state pixel parity.

`tools/visual/` automates the deterministic side of that comparison with
Playwright Chromium. It enters each registered seed through the public UI,
selects the correct region/floor, enables exact heap and mob layers, captures
the raw canvas backing bitmap with reduced motion, and compares it at zero
pixel tolerance against `tools/visual/snapshots/`. CI runs the suite after the
production WASM/Vite build. Browser baselines intentionally do not replace the
pinned gameplay references above.

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
status. Floor-6 and floor-8 asset coverage and committed HKT core facts are
exact, while dedicated browser comparisons for those deeper floors remain
open.

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

### 1a. ~~Port deeper-floor `createMobs`~~ — SOURCE-PORTED THROUGH FLOOR 24
`level/create_mobs.rs` now follows the pinned `RegularLevel.createMobs` pass on
every regular floor through depth 24: exact `mobLimit` and LARGE-feeling
rounding, weighted and shuffled `StandardRoom` instances, 30-retry placement,
failed-mob retention, the same-room second-spawn roll, recursive
`ShadowCaster`, bounded entrance distance, room character masks,
traps/plants/exits, large-mob `openSpace`, and high/furrowed-grass cleanup.

`level/create_mobs/rotation.rs` ports every regional `MobSpawner` table,
Shaman and Elemental subtype draws, rare additions and alternatives, list
shuffle order, constructor-side draws, and the unconditional champion roll.
Painter-created actors and quest NPCs now occupy their source-derived cells
before the ambient pass: Aquarium piranhas, shopkeepers, MassGrave skeletons,
RotGarden lashers, demon spawners, Ghost, Wandmaker, Blacksmith, and Imp.
Focused tests pin the regional tables, LARGE classes, `openSpace` corner
order, mob-limit rounding, and Blacksmith corridor/NPC behavior. The existing
oracles prove the complete pass for depth 1 and HKT floors 6, 7, and 8. Floors
without lifecycle goldens remain only source-ported because upstream
builder/painter state can still diverge. Global status therefore remains
`partial`.

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
After selecting a regular potion or scroll from a category deck, Java checks
its regular→exotic map and evaluates
`Random.Float() < consumableExoticChance()`. With no ExoticCrystals trinket
the chance is zero, but that Float still advances the restored level stream.
Java's cumulative `defaultProbsTotal` branch is different: it returns directly
after class selection and skips the exotic roll. Rust now preserves both draw
shapes, and focused generator tests pin their distinct RNG tails.

Combined with persistent room order, `AAA-AAA-AAA` depth 1 now generates and
places the exact five main drops (`ScrollOfRage`, `ScrollOfRecharging`,
`PotionOfHealing`, `Gold`, `Gold`) at cells 444, 173, 565, 939, and 903 with
matching heap types. The subsequent `itemsToSpawn` retry stream and final heap
cells also match.

### 2b. ~~Category deck rollover private-generator shape~~ — ORACLE EXACT
Pinned `Generator.random(Category)` pushes the category's private generator
once, replays one `Random.Long()` per prior drop, and keeps that same frame
pushed when exhausted probabilities trigger `reset(cat)` and a second
`Random.chances`. Rust now follows that exact one-push/one-pop control flow.

A dedicated pinned-Java `generator_deck_rollover` fixture exhausts the
five-card Food deck on draw five and records draws six through eight after the
reset. The focused Rust test matches every Java item class, dropped count,
remaining deck weight, two reconstructed private-stream integers, and two
active base-stream integers after every draw. It also asserts exactly one
private-frame push/pop per category draw, including the rollover.

This closes a source-shape discrepancy, not a demonstrated historical output
divergence: an exhausted `Random.chances` and `Generator.reset` consume no RNG,
so the removed pop/re-push happened to reconstruct the same observable state
at this pin. Global status remains `partial`; the fixture proves this category
rollover boundary only.

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
  extends `SecretRoom` with its own `paint()` and never pushes an `IronKey`.
  Its EMPTY_SP geometry, alchemy cell, EnergyCrystal stacks, pinned JVM
  HashMap chance order, potion draws, and heap cells are now ported directly.
  Regular `LaboratoryRoom` retains its own prize body, IronKey push, and first
  alchemy-guide placement draw.
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

Focused coverage in `special_loot/tests.rs` includes `*_pushes_iron_key`,
`pool_room_pushes_invisibility`, `secret_runestone_pushes_liquid_flame`,
`secret_laboratory_no_iron_key`, and SecretLarder's exact Pasty/ChargrilledMeat
energy units. The AAA-AAA-AAA final-heaps projection gained
`PotionOfInvisibility` (PoolRoom push now lands; `IronKey` stays report-blacklisted).

### 4. ~~Regular-floor room-placement predicate fidelity~~ — SOURCE-EXACT
The pinned source's supported `RegularLevel` overrides are all represented:

- `PlantsRoom` rejects its planted cells for both items and characters;
  `plant_occupied` also preserves that predicate dynamically for ambient mobs.
- `AquariumRoom` rejects water for both placements, including water introduced
  by the later painter pass through the existing dynamic mob/drop checks.
- `StandardBridgeRoom` uses one shared watabou `Rect.inside` implementation
  (left/top inclusive, right/bottom exclusive) for WaterBridge, ChasmBridge,
  and RegionDecoBridge standard/entrance/exit families. The entire logical
  `spaceRect` stays excluded even where a walkable bridge is painted over it.
- `CavesFissureRoom` and its entrance/exit subclasses reject final
  `EMPTY_SP` cells for items and characters after transition painting.
- `RitualSiteRoom` rejects the exact Chebyshev-radius `< 2` area around its
  jittered ritual center for both placement types.

Focused tests pin the masks, half-open edges, and the absence of predicate RNG
draws. The main-loop `Int(20)` heap roll, mimic float, locked-chest upgrade,
trap destruction, mob/heap occupancy, and exit-cell exclusions remain covered
by the existing Java-transcribed tests and schema-v3 goldens.

`AlternatingTrapsRoom`'s center-only rule and the false-return treasure/final
room overrides belong to the separate `VaultLevel` branch, not
`RegularLevel.randomDropCell`; they must be ported with that branch rather than
guessed into regular-floor generation. This phase therefore does not promote
the global `partial` status or close remaining special-room paint gaps.

### 4a. ~~Inventory-sensitive later-shop bag selection~~ — UNIQUE WINNERS EXACT
Pinned `ShopRoom.ChooseBag` scores every still-available bag from the items in
`Dungeon.hero.belongings.backpack.items`, adds VelvetPouch's built-in score of
one, and replaces its current choice only for a strictly greater score. Rust
now carries the fresh Warrior's direct main-backpack affinities and uses that
same scoring path instead of a fixed limited-drop order.

The existing floor-6 HKT lifecycle continues to pin MagicalHolster as Java's
observed winner of the fresh Warrior's ThrowingStone/Waterskin tie. After that
limited drop is consumed, Waterskin makes PotionBandolier the unique floor-11
winner; after both bags are consumed, ScrollHolder is the remaining floor-16
choice. A standalone pinned-Java fixture calls the real protected
`ShopRoom.ChooseBag` for the untouched later-shop backpack and a synthetic
scroll-heavy backpack, proving PotionBandolier and ScrollHolder as their
respective unique winners.

This does not claim full floor-11 lifecycle parity. The sequential oracle does
not auto-collect generated heaps, arbitrary gameplay inventory is not an input
to the analyzer, and identified TimekeepersHourglass sandbags remain outside
this phase. Equal-score choices also stay explicitly non-portable because the
pinned Java method iterates identity-hashed `HashMap` keys; Rust retains the
already-oracled floor-6 MagicalHolster result as its stable tie fallback.
Global status therefore remains `partial`.

### Lower-leverage, already-known (from prior disclaimer, still open)
- Full ambient `createMobs` now feeds map markers on every regular floor, but
  exact final cells are only promoted where the preceding lifecycle has a
  matching oracle: depth 1 and the HKT floor-6/floor-7/floor-8 replays.
- Shop stock is builder-timed, and bag scoring is exact for the committed
  floor-6 winner plus the unique-score floor-11/floor-16 fresh-inventory
  progression. Player-collected inventory and TimekeepersHourglass sandbags
  still require an explicit analysis input before they can be exact.
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
   oracle, exact render facts, Rust room/shop assertions, and floor-6
   sprite/icon coverage.
3. ~~**Close HKT floor-6 lifecycle parity.**~~ Rust now matches the 48×48
   builder boundary, all painter/render facts, all three RNG probes, the sixth
   ambient mob, and every final entity cell in the committed fixture.
4. ~~**Pin and compare `HKT-JZN-XQQ` floor 8.**~~ Added the sequential Java
   oracle, exact render facts, strongest honest Rust pre-paint assertions,
   source-aligned floor-7/8 mob rotations, and floor-8 sprite/icon coverage.
   The comparison isolated the painter/downstream work closed in item 6.
5. ~~**Fix the HKT floor-7 Wandmaker replay before floor 8.**~~ Added the
   sequential floor-7 oracle and matched its room classes, pre-paint/pre-mobs
   probes, and `WandOfPrismaticLight +1` / `WandOfCorrosion +1` rewards.
6. ~~**Close HKT floor-8 painter/lifecycle parity after floor 7.**~~ Matched
   terrain and render masks, pre-mobs/pre-items probes, 11 heap cells, 7 mob
   cells, 14 traps, transition cells, and the Alchemy blob. Dedicated browser
   comparison remains a separate follow-up.
7. ~~**Extend schema-v3 coverage to another depth-one room set.**~~ Added the
   exact `AAA-AAA-AAD` lifecycle for Garden, Sacrifice, Striped, and RingRoom
   center-loot behavior. Those six exact lifecycles established a strong
   regression base, not evidence that every depth-one combination is exact.
8. ~~**Port deeper-floor `createMobs`.**~~ Added pinned mob limits and regional
   rotations through floor 24, second-room spawns, constructor/subtype/rare-alt
   draw order, LARGE open-space checks, and painter/quest NPC occupancy. HKT
   floor 7 now also matches its pre-items probe and all eight final mobs.
9. ~~**Close supported regular-floor room-placement predicates.**~~ Audited
   every pinned `canPlaceItem` override reachable from `RegularLevel`,
   centralized StandardBridge's half-open `spaceRect`, and matched the paired
   Plants/Aquarium/CavesFissure/RitualSite character exclusions. Vault-only
   predicates remain scoped to a future VaultLevel port.
10. ~~**Close HKT floor-7 lifecycle and StorageRoom paint parity.**~~ Matched
    all render/entity facts, retained Storage's four exact heap cells, painted
    its barricade terrain, and fixed PrisonPainter's ChasmBridge subclass
    ornament predicate.
11. ~~**Close the next known special-room paint gap with a pinned lifecycle
    fixture.**~~ Added the full-additive `AAA-AAA-AFO` oracle and matched
    CryptRoom geometry, center-jitter timing, all directional statue/tomb
    layouts, exact tomb cell/item facts, and the complete downstream lifecycle.
12. ~~**Close category-deck rollover generator shape.**~~ Added a dedicated
    pinned-Java Food-deck fixture spanning exhaustion and reset, matched exact
    item/private/base RNG observations, and retained one pushed category frame
    across both `Random.chances` calls.
13. ~~**Correct inventory-sensitive later-shop bag selection.**~~ Added a
    standalone pinned-Java `ChooseBag` fixture with unique-score fresh and
    scroll-heavy floor-11 profiles, ported direct-backpack scoring, and fixed
    the uncollected fresh-run progression to MagicalHolster, PotionBandolier,
    then ScrollHolder. Equal-score player inventories and hourglass sandbags
    remain outside this phase.
14. Extend exact paint-time heap capture to the remaining room families; keep
    the legacy marker fallback until each family has a pinned cell association.
15. Add multi-depth schema-v3 fixtures and promote each newly covered region
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
  createMobs()      // source-ported through 24; fixtures exact at 1 and HKT 6/7/8
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
bun run test:map-render
bun run test:visual
```

`java_oracle_goldens.rs` (+ `java_oracle_goldens/final_heaps.rs`) is the
parity harness: identity maps (schema v1), depth-one forced-item queue
(schema v2), seven exact depth-one lifecycle fixtures covering map bounds,
heap cells, mob facts, and report-visible item projection, plus full Java
floor-six, floor-seven, and floor-eight observations (schema v3, see above).
AFO floor 1 and HKT floors 1, 6, 7, and 8 prove their complete render-fact
projections. Add tightly-scoped oracle fixtures before writing new Rust
behavior — regenerate via `tools/java-oracle/run` (see
`tools/java-oracle/README.md`).

The separate Food deck-rollover fixture under
`tools/java-oracle/fixtures/generator/` pins item sequence and private/base RNG
state across its first reset; its consumer is the focused `spd-core` unit test,
not the floor-schema integration harness.

The map-render registry test keeps the source gameplay PNGs and typed cases in
lockstep. The Playwright suite builds the app and checks the deterministic
browser canvases; use `bun run install:visual-browser` once locally and
`bun run test:visual:update` only after reviewing an intentional renderer
change. Failures retain actual, expected, diff, and trace artifacts under the
gitignored `tools/visual/test-results/` directory.

---

## License

SPD is GPL-3.0. This project ports generation logic → treat as
**GPL-3.0-or-later** when publishing. Assets are from SPD and under the same
license constraints.

---

## How to resume (clean context)

1. Read this file, specifically **"What's lacking for exact parity"** above.
2. The `AAA-AAA-AAA`, `AAA-AAA-AAD`, `AAA-AAA-AFO`, `ABC-DEF-GHI`,
   `GFX-PZH-DCH`, `hello`, and `HKT-JZN-XQQ` depth-one
   lifecycles are exact at the pre-painter, pre-mobs, and pre-items boundaries
   and for final map bounds, heap cells, mob facts, and report-visible items.
   Keep all seven fixtures green. AFO and HKT additionally pin every additive
   render fact.
3. Keep the exact `HKT-JZN-XQQ` floor-6 lifecycle golden green. It matches
   bounds, all RNG probes, terrain/render masks, heaps, mobs, transitions, and
   traps, but this single replay does not change global `partial` status.
4. Keep the exact `HKT-JZN-XQQ` floor-7 lifecycle golden green. It matches
   bounds, all RNG probes and render masks, 15 heaps, 8 mobs, 2 transitions,
   2 traps, and both Wandmaker rewards. This replay still leaves global status
   `partial`.
5. Keep the exact `HKT-JZN-XQQ` floor-8 lifecycle golden green. It matches all
   three RNG probes, terrain/render masks, heaps, mobs, transitions, traps, and
   the Alchemy blob; this replay also leaves global status `partial`.
6. Validate against `crates/spd-core/tests/java_oracle_goldens/final_heaps.rs`,
   its focused child modules, and all committed schema-v3 fixtures;
   regenerate/extend fixtures via `tools/java-oracle/run --source
   /Users/toan/code/00-Evan/shattered-pixel-dungeon ...`.
7. Keep the source-exact regular-floor item/character placement predicates,
   CryptRoom paint, Food deck-rollover oracle, and unique-winner shop-bag
   oracle green. Next, extend exact paint-time heap capture to another room
   family with a pinned lifecycle fixture; source-ported deeper mob
   populations are not a substitute for goldens. VaultLevel-only predicates
   remain part of a future branch port.
8. After Rust changes: `bun run build:wasm` (or `bun run dev`) before treating
   the UI as verified.
9. Dev dump: `cargo run -p spd-core --example dump_seed -- SEED FLOORS`
