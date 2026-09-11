# 03 — Feelings, builder, city rooms, carpets

Status: **open**

Layout RNG and painter-complete city/sewer visuals that v4 actually changed. Quest-room size changes for Wandmaker/Blacksmith live in 04 even though they move prison/caves layouts.

## Why it matters

**MAP-LAYOUT-GOAL / RNG-PARITY.**

v4’s “Mossy clump changing level shape for runs on the same seed” is `Level.create` short-circuit RNG. v3 called `TrapMechanism`’s `Random.Float()` only when Mossy did not override, so holding Mossy Clump shifted every later draw on that floor (builder, paint, items). v4 always draws both floats.

`Builder.findFreeSpace` used to accumulate Manhattan `curDiff` across colliding rooms and left `inside` sticky. v4 resets per room and compares `Point.length()` (Euclidean). That can change which neighbour is “closest” and therefore room graphs on any region — changelog “various rare and minor bugs in level generation”.

City standard rooms now stamp `Carpet` custom tiles and, for hallway entrance/exit, **skip** the `Random.Int(2)` statue-vs-pedestal roll. That is a city floor-stream change, not cosmetics.

## Java sources (what to port)

Feelings:

- `levels/Level.java` `create()` default-feeling branch: `float mossyChance = Random.Float(); float trapMechChance = Random.Float();` then `if (mossyChance < MossyClump.overrideNormalLevelChance())` else trap. `MossyClump.java` itself is unchanged.

Builder:

- `levels/builders/Builder.java` `findFreeSpace` — per-room `curDiff`/`inside`, `closestDiff` is `float`, compare `curDiff.length()`.

City / standard paint (carpets + RNG):

- `levels/rooms/standard/HallwayRoom.java` — carpet on the 3×3; `isEntrance()`/`isExit()` skip `Random.Int(2)` and set `ENTRANCE_SP` / `EXIT` + carpet overrides (`CITY_ENTRANCE` / `SKIP`).
- `levels/rooms/standard/entrance/HallwayEntranceRoom.java`, `exit/HallwayExitRoom.java` — find existing `ENTRANCE_SP`/`EXIT`; no second `Painter.set`.
- `levels/rooms/standard/StatuesRoom.java` — fill `CUSTOM_DECO_EMPTY`, corner `STATUE` (not `STATUE_SP`), carpets with `CITY_STATUE_*`, center `REGION_DECO` + `CITY_PEDESTAL`; entrance/exit 1×1 also place the transition in `paint`.
- `levels/rooms/standard/entrance/StatuesEntranceRoom.java`, `exit/StatuesExitRoom.java` — large rooms (≥11) extra carpet + `ENTRANCE`/`EXIT`; small rooms already placed in `StatuesRoom.paint`.
- `levels/rooms/standard/SegmentedLibraryRoom.java` — successful split `return`s (equivalent to old `tries=0`); failed split places a carpet. Bookshelf RNG should match; carpets are extra, no new Random.
- `levels/rooms/standard/entrance/LibraryRingEntranceRoom.java`, `exit/LibraryRingExitRoom.java` — carpet after transition; existing `Random.Int(2)` corridor jitter unchanged.
- `tiles/custom/Carpet.java` — texture `Assets.Environment.CARPET`, stitch + overrides 80+.
- `levels/Terrain.java` — `CUSTOM_DECO_WTR = 39` (Blacksmith furnace; 04). `DungeonTileSheet` maps it to `WATER`.

Sewer secret (not a quest NPC):

- `levels/rooms/secret/RatKingRoom.java` — **fixed 7×7** (analyzer still `(5,7,5,7)` in `rooms/dimensions.rs`). Statues as `CUSTOM_DECO`, carpet, `rat_king_room.png` deco. Gold still `IntRange(5,20)` on remaining empty cells — count changes.

Boss / last floor terrain (layout IDs, overlays in 06 too):

- `levels/CityBossLevel.java` — throne/entry carpet cells `CUSTOM_DECO_EMPTY` instead of `EMPTY_SP`; extra `CustomTerrainVisuals`; `earnedShop` is 02.
- `levels/LastLevel.java` — flag-map / wall treatment; visual custom tiles. Feeling still goes through `Level.create` (two floats).
- `levels/painters/RegularPainter.java` — extract `hiddenDoorChance(Level)` (vault sets 0 in 02). Main floors unchanged.

Not RegularLevel.java — **no v3.3.8→v4 diff**. Changelog “rare levelgen bugfixes” are the files above, not a RegularLevel rewrite.

## Analyzer files that will need to change

- `crates/spd-core/src/level/trinkets.rs` — `override_default_feeling` must draw both floats unconditionally. Tests `max_level_mossy_clump_uses_the_persistent_seeded_deck` / same-seed layout.
- `crates/spd-core/src/builders/place.rs` — `find_free_space` currently **deliberately** matches v3 sticky `inside`/`cur_diff` (comment at the `inside` declaration). Port v4; update any free-space trace golden (`tools/java-oracle` `--free-space-trace`, `builders/figure_eight.rs` comments).
- `crates/spd-core/src/level/painter/room_geometry/region_rooms/hallway.rs`
- `crates/spd-core/src/level/painter/room_geometry/region_rooms/statues.rs`
- `crates/spd-core/src/level/painter/room_geometry/region_rooms/segmented_library.rs`
- Library ring paint: `region_rooms/library_ring.rs` (if present) / `region_rooms/mod.rs`
- `crates/spd-core/src/level/painter/room_geometry/region_rooms/city_tests.rs`
- `crates/spd-core/src/level/special_loot/geometry/basic.rs` — `paint_ambitious_imp` carpets / grass distance (coordinates with 02)
- `crates/spd-core/src/rooms/dimensions.rs` — `RatKingRoom` → `(7,7,7,7)`
- RatKing paint (search `RatKingRoom` under `level/special_loot` / `level/painter`)
- `crates/spd-core/src/level/terrain.rs` — `CUSTOM_DECO_WTR = 39`; passable/water flags (`SOLID` like `CUSTOM_DECO`, water-passthrough is renderer)
- `crates/spd-core/src/report.rs` — `FloorMap.custom_terrain` next to `custom_tiles` / `custom_walls` (oracle 00 already captures it)
- `crates/spd-core/src/level/boss_layouts/data/floor20.rs` + `city_overlays.rs` — v4 throne/entry terrain and `CustomTerrainVisuals`
- `crates/spd-core/src/level/boss_layouts/last_level.rs`
- `crates/spd-core/src/level/painter/mod.rs` — hidden door chance hook if vault is generated in 02
- Goldens: `crates/spd-core/tests/java_oracle_goldens/final_heaps/floor_sixteen.rs` … `floor_nineteen.rs`, sewer floors for RatKing, `replay_aaa.rs`
- `web/src/lib/dungeon-tile-visuals.ts` — `CUSTOM_DECO_WTR`; carpet drawing is 06
- `web/src/lib/spd-wasm.ts` — `custom_terrain?: MapCustomTile[]`

## Acceptance

- Same numeric seed, held Mossy Clump +0 vs no trinket: when Mossy *does* override, TrapMechanism’s float is still consumed; builder RNG after feeling matches v4 Java. Test: `override_default_feeling` unit test plus one `--final-heaps-depth` fixture generated once with Mossy forced (new oracle flag or a Rust-only stream probe). Name the file: `crates/spd-core/src/level/trinkets.rs`.
- `find_free_space` Euclidean closest-room: `--free-space-trace` on the existing ABC-DEF-GHI depth-23 Sentry case matches v4 JSON.
- City hallway entrance/exit: no extra `Random.Int(2)` vs ordinary HallwayRoom. Halls-paint-trace (`--halls-paint-trace 16 GFX-PZH-DCH`) matches after 02 spawn RNG is in.
- Statues/Hallway/SegmentedLibrary/LibraryRing public maps include carpet `custom_tiles` with `texture: "carpet"` and the Java static_data.
- RatKingRoom always 7×7; sewer secret goldens updated.
- Floor 20 terrain IDs match regenerated `aaa-aaa-aaa-final-heaps-floor-20.json` (shop spawn still gated — layout without Imp shop).
- `bun run test:rust` for painter/builder/feeling tests. City visual PNGs wait for 06.

## Suggested PRs

1. `fix(core): always roll MossyClump and TrapMechanism feeling floats` — `level/trinkets.rs` + LastLevel’s hardcoded two floats stay consistent.
2. `fix(core): use Euclidean closest-room in findFreeSpace` — `builders/place.rs` + free-space oracle.
3. `feat(core): paint city Hallway/Statues/Library carpets and entrance RNG` — region_rooms + AmbitiousImpRoom paint helpers. Depends on 02 PR 1 for city stream.
4. `fix(core): pin RatKingRoom to 7x7 and update sewer secret paint` — dimensions + paint + sewer goldens.
5. `feat(core): retarget CityBossLevel and LastLevel structural terrain` — `boss_layouts/data/floor20.rs`, `city_overlays.rs`, `last_level.rs`. Shop contents stay 02’s non-goal.

## Dependencies

- **00** (v4 goldens exist).
- **02 PR 1** before any city 17–19 layout claim (Imp spawn draws in `initRooms`).
- City carpets on Imp room overlap 02 paint; share helpers, don’t double-port.
- **04** owns MassGrave/Ritual/Blacksmith sizes — do not “fix” prison/caves goldens here if those rooms spawned.
- **06** renderer/assets for carpets, occlusion, `carpet.png`, `rat_king_room.png`.

## Explicit non-goals

- Ghost/Wandmaker/Blacksmith quest logic (04).
- Vault GridBuilder (02).
- Wall occlusion / `raised_terrain.png` / `terrain_features.png` pixel output (06).
- SentryRoom sprite extraction (`SentrySprite.java`) — no layout RNG.
- ToxicGasRoom `desc()` — text only.
- `Level.invalidHeroPos` / mind-vision OBJECT skip — runtime FOV (07).
