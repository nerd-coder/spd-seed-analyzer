# 04 — Existing quests (Ghost, Wandmaker, Blacksmith, Mining)

Status: **closed**

Only v4 deltas that change generation, layout RNG, or painter-complete visuals. Quest contracts (Ghost targets, Wandmaker objective, Blacksmith Crystal/Gnoll) are unchanged in Java NPC classes.

## Why it matters

**MAP-LAYOUT-GOAL / SEED-ANALYSIS / RNG-PARITY.**

`Ghost.java` and `Wandmaker.java` have **no** v3.3.8→v4 quest-logic diff. Rewards still roll the same decks; enchant *names* follow 01/05 tables.

What did change is the rooms those quests inject, plus Mining gold placement:

- MassGrave / RitualSite grew and consume more paint RNG → prison layouts and later floor-stream draws on that depth move.
- BlacksmithRoom is larger, places heaps on fixed pedestals (no `random()` loops), and forbids grass/water/traps/merge. CavesPainter skips it for wall-deco/gold. That moves caves layouts and BlacksmithRoom item identities (still `Generator.random` ARMOR/WEAPON/MISSILE).
- MiningLevelPainter gold candidates must have a non-wall neighbour **inside the room**. MineEntrance uses `ENTRANCE_SP`. Crystal/gnoll tilesheets gained frames (06).

Sad Ghost is otherwise untouched. Do not reopen `specs/analysis/quest-sad-ghost.md` here.

## Java sources (what to port)

Ghost: none for spawn/rewards. Enchant table is 01/05.

Wandmaker rooms:

- `levels/rooms/quest/MassGraveRoom.java` — size **fixed 11×10** (was min 7×7). `canConnect(direction)` bottom only; `canConnect(Point)` `|x-center|<=2`. Paint: extra walls/statues, `MassGraveDeco` on `prison_quest` tex width 256, `StatueRaised` on `customTerrain`. Skeleton/item loops use `random(1)` with y clamps (`p.y<=top+2` pulls x inward; reject `p.y>top+3` / `>top+5`). Loot table unchanged (corpse dust, gold rolls, `Generator.random()`, armor).
- `levels/rooms/quest/RitualSiteRoom.java` — min size **10** (was 9). Top-row cages: `validTopRowCells`, `Random.Int(2)` cageRow, `Random.IntRange(1,2)` offsets, leftover cage `Random.IntRange(1,9)` up to 100 tries. RitualMarker 5×5, center.y++. `Table` on `customTerrain`. `canConnect` forbids `top` at `left+3` and `left+6`. Four ceremonial candles still queued.

Blacksmith:

- `actors/mobs/npcs/Blacksmith.java` — only pre-3.1 `freePickaxe` save conversion removed (07). `Quest.spawn` / `generateRewards` unchanged.
- `levels/rooms/quest/BlacksmithRoom.java` — min **8×8** (analyzer `max(6)`). NPC at `(left+3, top+3)`. Two `Generator.random(oneOf(ARMOR,WEAPON,MISSILE))` on pedestals at `(right-3, bottom-3)` (y+1 if height==8). `Random.Int(2)` entrance side, overridden by a top door. `CUSTOM_DECO` / `CUSTOM_DECO_WTR` furnace. `SmithyVisuals` + `FurnaceOverhang`. `maxConnections(TOP)=1`; `canConnect` only `top` at `left+1` or `right-1`, never `y==top+1`; no grass/water/trap/item-on-sp/merge; `canPlaceCharacter` false.
- `levels/painters/CavesPainter.java` — skip `BlacksmithRoom` in the StandardRoom wall-deco loop (`!(room instanceof StandardRoom) || room instanceof BlacksmithRoom`).

Mining:

- `levels/painters/MiningLevelPainter.java` — gold wall candidates: neighbour `map[i+j] != WALL` **and** `r.inside(cellToPoint(i+j))`.
- `levels/rooms/quest/MineEntrance.java` — entrance `ENTRANCE_SP`, neighbours `EMPTY_SP`; `TEX_WIDTH` 64 (visual).
- `levels/rooms/quest/MineGiantRoom.java` / others — no logic diff in the v3.3.8→v4 stat besides entrance/tilesheets.

## Analyzer files that will need to change

- `crates/spd-core/src/rooms/dimensions.rs` — MassGrave `(11,11,10,10)`; RitualSite `max(10)`; Blacksmith `max(8)`.
- `crates/spd-core/src/rooms/room.rs` — MassGrave `can_connect` (bottom + x-band); Blacksmith top-corner connections (`can_connect_point`).
- `crates/spd-core/src/level/special_loot/quest_rooms.rs` — `mass_grave_prizes`, `ritual_site_setup`, `blacksmith_room_prizes` (still v3 paint: TRAP fill, `random()` heaps).
- `crates/spd-core/src/level/special_loot/geometry/` — MassGrave/Ritual/Blacksmith geometry if split from prizes.
- `crates/spd-core/src/level/painter/decorate/caves.rs` — `fill_room_corners` / gold: skip `room.name == "BlacksmithRoom"`.
- `crates/spd-core/src/level/mining/environment.rs` — `generate_gold` / `cardinal_neighbour_is_not_wall` must require the empty neighbour inside the current room.
- `crates/spd-core/src/level/mining/rooms.rs` — MineEntrance `ENTRANCE_SP`
- `crates/spd-core/src/quests/ghost.rs`, `quests/wandmaker.rs`, `quests/blacksmith.rs` — **no contract change**. Re-run existing tests; prison/caves goldens will move because rooms changed.
- Tests: `crates/spd-core/src/quests/blacksmith/tests.rs`, `analyze_smoke.rs` BlacksmithRoom items, `java_oracle_goldens/final_heaps/floor_seven.rs` / `floor_eight.rs` / `floor_thirteen` mining fixtures `tools/java-oracle/fixtures/mining/`
- Visual fixtures already include `AAA-AAA-AAA-F13-B1-Crystal.png` and `AAA-AAA-AAB-F13-B1-Gnoll.png` — PNG refresh in 06; Rust mining layout goldens here.

## Acceptance

- MassGraveRoom never sizes below 11×10; connection only on the bottom three-center tiles. Prison `--final-heaps-depth 7..9` terrain + custom_tiles match v4 for seeds that spawn Corpse Dust.
- RitualSiteRoom min 10; top-row cage RNG matches `--halls-paint-trace`/`final-heaps` on a Ritual seed (use existing `aaa-aaa-aa*` quest-npc fixtures; regenerate in 00).
- BlacksmithRoom min 8; two equipment heaps sit on pedestals, not `random()` EMPTY_SP. `Generator.random` category still ARMOR/WEAPON/MISSILE. CavesPainter does not put `WALL_DECO` gold in that room.
- Mining gold cells: every `WALL_DECO` produced by `generate_gold` has a non-wall cardinal neighbour inside the same room. `MiningLevelOracle` JSON for crystal+gnoll matches Rust at the createMobs boundary.
- MineEntrance transition cell is `ENTRANCE_SP`.
- Ghost/Wandmaker/Blacksmith *quest reports* still serialize as today (`fetid_rat` / `corpse_dust` / `crystal` …). Reward option counts unchanged.
- `bun run test:rust` including mining + prison/caves goldens.

## Suggested PRs

1. `feat(core): retarget MassGraveRoom size, connections, and paint` — dimensions, `rooms/room.rs`, `quest_rooms.rs` loot loops, prison goldens.
2. `feat(core): retarget RitualSiteRoom cages and marker` — paint RNG + custom tiles/terrain.
3. `feat(core): retarget BlacksmithRoom paint and skip it in CavesPainter deco` — pedestals, connections, `decorate/caves.rs`.
4. `fix(core): restrict MiningLevel gold to in-room walls and use ENTRANCE_SP` — `mining/environment.rs`, `mining/rooms.rs`, mining fixtures.
5. `test(core): refresh Wandmaker/Blacksmith/Mining java-oracle goldens` — if 00 did not already, or any leftover Rust expected blobs.

PR 3+4 are the caves/mining visual dependency for 06’s Crystal/Gnoll snapshots.

## Dependencies

- **00** goldens.
- **01** only if a BlacksmithRoom weapon enchant name is asserted (tables).
- **03** Builder/feeling should land first so prison/caves graph diffs are not mixed with MassGrave size diffs.
- **06** `prison_quest.png`, `caves_quest.png`, mining tilesheet alts (`DungeonTileSheet.updateAltVariants`).

## Explicit non-goals

- Re-auditing Ghost player-state routes (`specs/analysis/quest-sad-ghost.md`).
- Blacksmith `freePickaxe` save conversion (07).
- Mining reset/unmodeled pre-quest state (already called out in `specs/implementation.md`).
- Vault (02).
- Fungi mining objective — still not a v4 Blacksmith type.
- Quest NPC sprites / smithy examine text except as custom-tile static_data for maps.
