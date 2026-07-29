# Floor layout certainty and run settings — verified audit

Target: **Shattered Pixel Dungeon v3.3.8 @ `7b8b845a7`**. This audit asks
which facts in a painter-complete main-path floor are fixed by the seed, which
are fixed only after choosing a run profile, and which profile inputs are still
missing. Java citations refer to the pinned clone. Rust and web citations refer
to this repository on 2026-07-29.

The public map scope follows `MAP-LAYOUT-GOAL`: room geometry, connections,
doors, transitions, terrain, traps, plants, and blobs after painting, but before
ordinary NPC, mob, heap, and forced-item population. A population step can
still matter to a **later** floor when it mutates persistent generator state.

## Verdict

The updated settings are useful, but they do not yet close the layout problem.

- The profile records all nine challenge choices, chronological trinket
  acquire/upgrade/transmute events, external artifact events, and Parchment
  Scrap claim state. Forbidden Runes plus Mossy Clump, Trap Mechanism, and
  Mimic Tooth already select materially different conditional maps; the other
  map-relevant inputs are stored but not yet replayed.
- The current API still publishes only floor 1 and the dedicated boss/final
  floors as exact. Later regular floors remain `assumed_map` continuations even
  with an explicit profile. That matches the current accuracy manifest.
- Rat Skull, Cracked Spyglass, Barren Land, Badder Bosses, and map-relevant
  artifact-deck events still need generation implementations before they can
  make a painter-complete map exact. The profile deliberately marks their
  affected projections assumed in the meantime.
- Settings alone are insufficient. The separate layout replay stops before
  ordinary population, so it loses automatic persistent deck mutations that
  can affect later floors. The analyzer must snapshot the public map and then
  continue a faithful hidden lifecycle to carry state forward.
- Exact room **selection** can be recovered earlier than exact painted maps.
  The current single `runtime_sensitive_layout` flag unnecessarily hides room
  lists, room bounds, and connections together with later painter uncertainty.

For the bounded scope “first generation of each main-path floor in a custom
seeded descent,” the missing inputs are finite and can be represented at floor
boundaries. “Every map reachable after arbitrary play,” including sealed-floor
Ankh regeneration, needs generation-attempt history as well.

## What the current settings actually narrow

`MapProfile` stores the complete player-facing challenge set plus chronological
trinket and artifact events, without exposing generator counters
(`crates/spd-core/src/trinkets.rs`). A trinket acquire or transmute event starts
a new internal instance while an upgrade preserves its current instance; the
Catalyst/pot calculation prevents events from starting before that seed can
first be affected.

The map pass is deliberately separate. `analyze_seed_with_profile` runs the
full analyzer and a cloned `analyze_layouts_with_profile`, then replaces the
full report's map-facing fields with the layout replay
(`crates/spd-core/src/lib.rs:84-98`). That replay calls the same generator with
`layout_only = true` (`crates/spd-core/src/level/mod.rs:71-82,585-605`). It
captures the map after room painting and isolated decoration, then skips quest
NPCs, ambient mobs, and `createItems`
(`crates/spd-core/src/level/mod.rs:420-495`).

The public projection exposes a map, room list, and builder only while the one
coarse `runtime_sensitive_layout` flag is false; otherwise it moves the map to
`assumed_map` and suppresses all room facts
(`crates/spd-core/src/level/state.rs:529-554`). In the current generated WASM,
AAA-AAA-AAA, GFX-PZH-DCH, and numeric seed 42 all produced the same certainty
pattern under an explicit fresh profile:

```text
exact:   1, 5, 10, 15, 20, 25, 26
assumed: 2–4, 6–9, 11–14, 16–19, 21–24
```

### Analyzer sensitivity measurement

Method: current generated WASM, numeric seeds 0…19, floors 1…24. Each +3
trinket was first held on that seed's reported earliest effective floor. For
each regular floor, the complete serialized `map ?? assumed_map` was compared
with the explicit fresh baseline. This measures present analyzer behavior; it
is **not** Java parity evidence.

| Changed profile | Eligible regular floors | Different maps | Seeds with any difference |
|---|---:|---:|---:|
| Forbidden Runes | 400 | 13 (3.3%) | 7 / 20 |
| Mossy Clump +3 | 336 | 302 (89.9%) | 20 / 20 |
| Trap Mechanism +3 | 336 | 292 (86.9%) | 20 / 20 |
| Mimic Tooth +3 | 336 | 69 (20.5%) | 17 / 20 |

The settings therefore narrow the displayed continuation substantially. They
do not change its certainty label after floor 1 because unresolved state is
still inherited across floors (`crates/spd-core/src/level/mod.rs:83-96,206-221`).

## Generation boundaries and their inputs

`Level.create` pushes the depth-specific RNG, queues guaranteed items, chooses
the feeling, retries `build`, then calls `createMobs` and `createItems`
(`Level.java:215-317`). `RegularLevel.build` chooses the builder, creates and
shuffles rooms, builds the room graph, and finally paints it
(`RegularLevel.java:104-165,176-187`). This creates three useful certainty
boundaries.

| Boundary | Facts fixed there | Player/run inputs that can still matter |
|---|---|---|
| After `initRooms` | Builder kind; room multiset and shuffled room objects | Mossy Clump's short-circuit and persistent feeling instance; generated room/quest deck state |
| After builder | Room bounds and graph connections | Shop-floor artifact state can move builder RNG while lazy stock is generated |
| After painter | Doors, stairs, terrain, traps, plants, blobs, decoration | Forbidden Runes queue, Mossy/Trap feeling, Mimic Tooth, Rat Skull, artifact state, Barren Land; Badder Bosses on floor 15 |

The painter shuffles rooms, then alternates `placeDoors(room)` and
`room.paint(level)` before its final door pass
(`RegularPainter.java:122-133`). A player-sensitive item or mob generated while
painting one room can therefore move the RNG used by a later room's door,
stairs, or terrain. Water, grass, ordinary traps, and regional decoration are
then isolated behind one ambient `Random.Long()`
(`RegularPainter.java:135-153`), which limits but does not erase earlier drift.

### Room facts can be stronger than map facts

The current profile can support a more useful intermediate claim:

- Mossy Clump is the direct current-floor structural trinket. On a normal
  feeling branch, success skips the Trap Mechanism float before builder and
  room selection (`Level.java:255-291`, `MossyClump.java:54-91`).
- Trap Mechanism alone consumes the same Mossy-fail and Trap-check floats as
  the no-trinket route. Its Trap/Chasm result comes from an isolated persistent
  deck and does not trigger the Large/Secrets room-count rules. It changes
  painted terrain and trap visibility, but not that current floor's room
  selection (`Level.java:283-290`, `RegularLevel.java:129-163`,
  `TrapMechanism.java:54-103`).
- Forbidden Runes changes the queued Upgrade Scroll without an RNG call before
  build (`Level.java:228-236`). Mimic Tooth and Rat Skull are consulted during
  room paint or later. They do not change the current room multiset.
- Shop stock is generated lazily from a room-size callback. Artifact
  construction can move the builder stream, so bounds and connections on
  floors 6, 11, and 16 need artifact state even though the already selected
  room list does not (`ShopRoom.java:71-97,332-350`;
  `crates/spd-core/src/level/build.rs:34-53`).

This justifies separate certainty fields such as `room_selection_exact`,
`room_graph_exact`, and `paint_exact`. An unknown paint tail should not erase a
known room multiset.

## Existing inputs that are necessary

### Forbidden Runes

The challenge omits every second guaranteed Scroll of Upgrade but retains the
limited-drop counter (`Level.java:228-236`). A room's `findPrizeItem` either
removes a queued object, consumes a random queue entry, or returns null
(`Level.java:799-826`). Empty-queue fallbacks can generate another item and
move the painter stream. The current toggle is therefore necessary for exact
paint, even though the sensitivity scan found relatively few map differences.

### Mossy Clump and Trap Mechanism

Both class, upgrade level, and effective starting floor are necessary. Their
six-card feeling decks are fields on the concrete trinket object and are saved
with the object (`MossyClump.java:66-119`, `TrapMechanism.java:78-124`). Trap
Mechanism also changes which generated traps are revealed
(`RegularPainter.java:465-493`).

### Mimic Tooth

The Tooth changes room-paint Mimic decisions. A spawned Mimic immediately
generates an additional reward and a held Tooth adds another defaults-based
reward (`Mimic.java:294-358`). Those calls can move the current paint stream
and persistent item decks. The setting must remain.

## Recorded inputs not yet replayed

### 1. Rat Skull — direct current-floor paint input

Rat Skull changes Statue versus Armored Statue construction
(`Statue.java:198-212`) and the Crystal Vault's chest versus Crystal Mimic
branch (`CrystalVaultRoom.java:53-82`). The alternate mob construction and
immediate Mimic reward consume a different painter tail before later rooms and
doors. The profile records its chronological acquire/upgrade/transmute history.
Its paint behavior still needs to be ported before an affected map is exact.

### 2. Cracked Spyglass — deterministic later-floor state input

After ordinary item population, Cracked Spyglass generates extra hidden loot
under an isolated floor RNG (`RegularLevel.java:679-689`). Its count is 0.375 ×
`(level + 1)` before the final integer roll (`CrackedSpyglass.java:52-61`). The
isolated RNG protects the ambient floor tail, but `Generator.randomUsingDefaults()`
can still select ARTIFACT, and artifact deck mutation survives the pop. That
can change later special-room paint, shop-floor builder RNG, and floor-20 City
decoration. Cracked Spyglass is recorded as a held trinket event, but its
post-population artifact mutation still needs lifecycle replay.

This new deck fact is also recorded in `specs/generator-decks.md`, where
persistent deck behavior is canonical.

### 3. Artifact-deck history — cross-floor and shop-floor input

This is already established in `specs/generator-decks.md`, §11. Runtime
artifact requests always use the ARTIFACT deck
(`Generator.java:743-751,854-878`). A different remaining class can introduce
the Unstable Spellbook constructor tail (`UnstableSpellbook.java:84-103`), and
deck exhaustion falls through to a Ring with a different ambient randomization
shape (`Generator.java:698-709`; `Artifact.java:217-225`; `Ring.java:258-277`).

Artifact state affects maps in three places:

1. special/secret room prizes generated between painted rooms;
2. lazy regular-shop stock generated while the builder is placing rooms;
3. `ImpShopRoom.paint` before the floor-20 `CityPainter` consumes its isolated
   decoration seed (`CityBossLevel.java:190-201`, `ShopRoom.java:332-350`,
   `RegularPainter.java:135-153`).

A custom seeded/challenged run excludes prior-run Bones loot
(`Bones.java:198-205`), so no Bones setting is needed for the analyzer's normal
seeded scope. Runtime artifact draws, artifact transmutation, and automatic
level-generation draws still need to be replayed.

The player-facing input is chronological events, not internal deck counters:
“artifact generated/obtained or transmuted before first generating floor N.”
The profile now records them; the lifecycle must still derive and apply the
resulting deck state from the seed.

### 4. Trinket instance identity/reset

SPD owns each feeling deck on the trinket instance. Transmutation constructs a
new trinket and transfers level and knowledge, not the old instance's
`levelFeels`/`shuffles`
(`ScrollOfTransmutation.java:322-333`).

Therefore Mossy → another trinket → a newly acquired Mossy must restart the
Mossy deck, while upgrading the same Mossy must preserve it. The profile now
derives that identity from acquire/transmute actions and the Rust Mossy and Trap
state resets on a new instance (`crates/spd-core/src/trinkets.rs`,
`crates/spd-core/src/level/trinkets.rs`). Full lifecycle replay remains needed
before those persistent decks can make later maps exact.

### 5. Barren Land and Badder Bosses

Only three of the nine challenge bits alter the initial public map in this
pinned version:

- Forbidden Runes (`NO_SCROLLS`) is already supported.
- Barren Land (`NO_HERBALISM`) makes `Level.plant` stop before creating the
  plant object, after preserving the terrain/RNG work (`Challenges.java:29-38`;
  `Level.java:1021-1044`). Plants remain part of `FloorMap.into_layout_only`,
  which removes markers/heaps/mobs but deliberately retains traps, plants, and
  blobs (`crates/spd-core/src/report.rs:25-58,84-91`).
- Badder Bosses (`STRONGER_BOSSES`) changes floor 15's initial inactive-trap
  roll from 1/8 to 1/4 (`CavesBossLevel.java:116-140`). The Rust boss builder
  currently hard-codes 1/8 (`crates/spd-core/src/level/boss_layouts/caves.rs:24-42`).

The source audit found no initial painter-complete terrain hook for the other
six challenges. Darkness changes view distance, while the remaining hooks are
combat, mob, item-use, or post-generation behavior. The complete challenge
mask is stored, but Barren Land and Badder Bosses still need their pinned map
behavior before an affected projection is exact.

### 6. Generation attempts, only if regenerated boss floors are in scope

Using an unblessed Ankh while a floor is sealed calls `Dungeon.newLevel()` and
regenerates it (`InterlevelScene.java:749-765`). A regenerated floor 20 reaches
shop generation again with already-mutated persistent decks, so its decoration
need not match the first attempt. The dormant `InterlevelScene.Mode.RESET` has
no call site in the pinned tree and should not be presented as a normal regular-
floor route.

The clean product boundary is to claim **first generation of each main-path
floor**. If post-death boss maps are desired, add a per-depth generation-attempt
history instead of silently folding them into the first-generation profile.

## Inputs without a direct initial map effect

- Parchment Scrap changes equipment effects inside an isolated item generator;
  the profile records its claim-only level, while the ambient stream pays the
  same one `Random.Long()` either way
  (`Weapon.java:432-446`, `Armor.java:667-681`).
- Exotic Crystals keeps the potion/scroll conversion roll in both the absent
  and held cases (`Generator.java:729-767`). Its profile event changes excluded
  item identity, not the painter call count or terrain.
- Petrified Seed, Eye of Newt, Ferret Tuft, Salt Cube, Vial of Blood, Wondrous
  Resin, Shard of Oblivion, Thirteen-leaf Clover, Chaotic Censer, and
  Dimensional Sundial have no direct initial painter hook. Runtime consequences
  that eventually request artifacts are captured by artifact events.
- Shop bag identity and Hourglass sand quantity do not change regular shop
  room size. `spacesNeeded` removes actual sandbags and adds four fixed slots
  (`ShopRoom.java:81-97`). In the once-generated main path, four bag types cover
  the three regular shops plus the fixed-size Imp shop.
- Quest acceptance, completion, and reward choice occur after the room was
  selected. The floor-20 Imp shop stock is generated during build regardless
  of whether it is later placed (`CityBossLevel.java:190-201`).
- Ascension reloads already generated main-path floors; it does not generate a
  second ordinary layout for the same descent.

## Recommended profile and replay design

### Profile scope

Make the supported claim explicit:

```text
scope = first_generation_main_path
meta = no_runtime_artifact_events | chronological_artifact_events
challenges = forbidden_runes + barren_land + badder_bosses
held_trinket_instances = floor-boundary class/level/instance history
```

Add Rat Skull and Cracked Spyglass to the directly modeled trinkets. Preserve a
generic “other trinket” state only after source-auditing it as having no direct
or automatic cross-floor map effect.

### Replay

Use one faithful lifecycle. At the painter-complete boundary, clone/snapshot
the layout-only public map, then continue hidden NPC/mob/item generation solely
to preserve persistent state for the next floor. Apply user artifact events at
their floor boundary. Do not replace a full replay's maps with a second replay
that skips population.

### Certainty projection

Track at least:

1. room selection exactness;
2. room bounds/connection exactness;
3. painter-complete map exactness;
4. later population/loot exactness, independently.

This lets the UI publish exact room names and possibly the room graph on many
floors where only doors, stairs, or terrain remain conditional. When an input
is unknown, show the invariant intersection and label each map variant with its
condition rather than suppressing every structural fact.

### Verification required before promotion

Extend the Java oracle with paired fixtures for:

- each held trinket at +0 and +3 from a fixed effective floor;
- Mossy/Trap upgrade versus transmute-away-and-reacquire instance histories;
- Forbidden Runes and Barren Land regular floors;
- normal versus Badder Bosses floor 15;
- Rat Skull Crystal Vault/Statue paths;
- Cracked Spyglass followed by a later artifact-sensitive floor;
- artifact events before a special room, regular shop, and floor-20 shop;
- first versus sealed-floor regenerated floor 20 if regeneration enters scope.

Only after those paths match room selection, bounds, connections, doors,
terrain, transitions, traps, plants, blobs, and post-snapshot persistent state
should later regular floors move from “assumed continuation” to exact.
