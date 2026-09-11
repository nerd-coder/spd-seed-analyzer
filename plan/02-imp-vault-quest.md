# 02 — Imp quest and vault branch

Status: **closed**

Replace the v3.3.8 Ambitious Imp (monks/golems, one cursed +2 ring) with the v4 vault raid. Nest the vault under the Imp spawn floor the same way MiningLevel is nested under the Blacksmith floor.

## Why it matters

**SEED-ANALYSIS / MAP-LAYOUT-GOAL / DECK-FACTS.**

This is the v4 headline. `Imp.Quest.spawn` still runs from `CityLevel.initRooms` (`CityLevel.java:192-193`) on depths 17–19 with `Random.Int(20-depth)==0`. It no longer rolls `alternative` or a cursed ring. It builds `rewardOptions` (six items, take one) and those draws happen **on the city floor stream before the builder**, so city layouts on 17–19 are wrong until this spawn is ported (see 03).

Public facts that are seed-determined:

- Spawn depth (same gate as v3).
- The six `rewardOptions` (artifact/ring, ring, WEP/MIS pair, plate, wand) and their levels/enchants/glyphs, modulo artifact-deck exhaustion (label that condition). They sit on `VaultFinalRoom` pedestals. `EscapeCrystal` only lets the player keep one, and only if score ≥ 500; the +2…+5 pool itself is selectable only at statue score (≥ 4000). Lower scores keep vault T0/T1 loot or consumables (`EscapeCrystal.java:165-197`).
- Vault painter-complete layout for `(seed, depth, branch=1)` after the quest is given.
- Reciprocal `BRANCH_EXIT` / `BRANCH_ENTRANCE` transitions from `AmbitiousImpRoom`.

Not seed-determined:

- Vault score, hazard freebies, mirror use (`Imp.Quest.score`, `hazardFreebies`, `mirrorUsed`).
- `Imp.Quest.earnedShop()` — `completed && score > 2000` (old quest is save-compat only). Floor-20 Imp shop stock is **conditional**, not guaranteed.
- Vault-only enemy AI and `VaultBossElemental` combat (07).
- Which one of the six the player keeps.

Follow MiningLevel: public vault map is layout only; keep entity-rich maps internal for oracle evidence.

## Java sources (what to port)

Quest / spawn / shop gate:

- `actors/mobs/npcs/Imp.java` — `Quest.spawn`, `reset`, `earnedShop`, `complete(int)`, `isOld` (always false for fresh analyzer runs). Do not port `oldQuest` / `oldProcess` / `WndImpOld` except to ignore them.
- `levels/CityLevel.java` — `activateTransition` for `BRANCH_EXIT`: blocked if `isOld() || isCompleted() || !given()` or Ascension/LostInventory.
- `levels/rooms/quest/AmbitiousImpRoom.java` — paint (carpets, `CUSTOM_DECO_EMPTY` cross, `WallBanners` on `customTerrain`, `QuestEntrance` tex width 256), `canPlaceGrass/Water` distance 5 (was 3), transition to `(depth, branch=1)`.
- `levels/rooms/standard/ImpShopRoom.java` — `earnedShop()` gate.
- `levels/CityBossLevel.java` — shop spawn uses `earnedShop()`; terrain `EMPTY_SP` → `CUSTOM_DECO_EMPTY` is 03.

Vault level (MiningLevel analogue):

- `levels/VaultLevel.java` — `initRooms`, `builder() = GridBuilder`, `painter()` CityPainter with `hiddenDoorChance=0`, `nTraps=0`, `build()` pre-queues 4×`createEquipment(0)` + Dart + 5×`createConsumabe(0)` + 3× food **before** `super.build()`, `createItems` drops that queue, `createMobs` empty (rooms spawn mobs — omit from public map).
- `levels/builders/GridBuilder.java` — v4 rewrite: bounding `maxWidth/maxHeight`, perimeter entrance, first two rooms attach to entrance, extraConnectionChance `0.55`, no same-class shared side, large rooms may poke the bound.
- `levels/rooms/quest/vault/VaultRoom.java` — `setupChances` `{2,2,2,2,2, 1,1, 1,1,1}`, `createRoom`, 11×11.
- Standard vault rooms: `VaultRingRoom`, `VaultCrossRoom`, `VaultQuadrantsRoom`, `VaultRingsRoom`, `VaultEnemyCenterRoom`, `VaultHallwayRoom`, `VaultLongRingsRoom`, `VaultCircleRoom`, `VaultAlternatingFireRoom`, `VaultLasersRoom`, `VaultTokensRoom`, `VaultSimpleEnemyTreasureRoom`, `VaultEntranceRoom`, `VaultLongRoom` (if still referenced).
- Treasure: `VaultTreasureRoom.generateRoomList` T1/T2/T3 shuffle then round-robin; `nextRoom()` ×7. Classes: `VaultFlamePathRoom`, `VaultLaserTreasureRoom`, `VaultCircleScanTreasureRoom`, `VaultSingleEnemyTreasureRoom`, `VaultBookcaseTreasureRoom`, `VaultFlamesTreasureRoom`, `VaultManyScansRoom`, `VaultMultipleEnemyTreasureRoom`, `VaultHardLaserTreasureRoom`.
- `VaultFinalRoom.java` — 21×21, ≥3 rooms from entrance, paints pedestals, `Random.shuffle` leftover spots, drops `Imp.Quest.rewardOptions` then **clears** the list, ImpStatue on center pedestal. Boss spawn on step is runtime (07).
- Hazards that paint terrain/blobs (layout): `actors/blobs/VaultFlameTraps.java`; laser/sentry/mirror NPCs are **not** public-map markers (`MAP-LAYOUT-GOAL`). Capture any terrain they require (doors, `CUSTOM_DECO`, carpets).
- `Dungeon.java` — vault levels now count as generated (`generatedLevels`); `interfloorTeleportAllowed` excludes `VaultLevel`. Analyzer branch id: `depth + 1000*branch` with `branch=1`, same as mining.

## Analyzer files that will need to change

Split new modules (`SMALL-FILES`). Do not grow `quests/imp.rs` past ~300 lines.

- `crates/spd-core/src/quests/imp.rs` + `quests/imp/tests.rs` — spawn gate stays; delete monks/golems/`generate_reward` curse loop; produce `rewardOptions`; `ImpQuestTarget` goes away.
- `crates/spd-core/src/quests/mod.rs`
- New: `crates/spd-core/src/level/vault/` (mirror `level/mining/{mod,rooms,doors,environment,geometry}.rs`)
- New: `crates/spd-core/src/builders/grid.rs` + export from `builders/mod.rs`
- `crates/spd-core/src/level/special_loot/geometry/basic.rs` — `paint_ambitious_imp`
- `crates/spd-core/src/level/special_loot/quest_rooms.rs` — `ambitious_imp_room_npc` (NPC pos is not public-map; still needed for internal parity / door paint)
- `crates/spd-core/src/rooms/dimensions.rs` — AmbitiousImpRoom stays 9×9
- `crates/spd-core/src/report.rs` — `BranchFloorKind::ImpVault` next to `BlacksmithMine`
- `crates/spd-core/src/report/quests.rs` — `AmbitiousImpQuestContract` / `Baseline`: drop `ImpTarget` / `required_tokens`; add reward `option_count: 6`, `selected_count: 1`; shop gate as a labelled condition, not a guaranteed shop.
- `crates/spd-core/src/report/compact.rs` — Imp one-liner
- `crates/spd-core/src/items/model.rs` — replace `QuestRewardRole::ImpRing` with a vault take-one role (or six sourced variants under `source: "Imp.Quest"`)
- `crates/spd-core/src/level/state.rs` — projection for that role
- `crates/spd-core/src/analyze_smoke.rs` — Imp assertions
- `crates/spd-core/src/search/` — finder still matches “a ring in the Imp pool” as one of six options, not the only reward
- `crates/spd-wasm/src/lib.rs` — no new API if JSON shape stays `SeedReport`
- UI: `web/src/lib/spd-wasm.ts` (`ImpTarget`, `BranchFloorReport.kind`), `web/src/components/seed/QuestCard.tsx`, `web/src/components/seed/FloorDetail.tsx` (`BlacksmithMineBranch` is hard-coded — generalize or add `ImpVaultBranch`), `web/src/lib/labels.ts`
- Oracle: new `VaultLevelOracle.java` modelled on `MiningLevelOracle.java` (`tools/java-oracle/src/.../MiningLevelOracle.java`); `tools/java-oracle/run --vault-level DEPTH`; fixtures under `tools/java-oracle/fixtures/vault/`
- Visual: extend `tools/visual/tests/map-render-fixtures.ts` with a city-floor Imp room and a `branch: 1` vault identity (PNG in 06)

## Acceptance

- For the five Imp ring-deck seeds, spawn depth matches v4 Java; `RING/ARTIFACT/WAND/WEP_T*/MIS_T*` dropped counters around `initRooms` match the extended oracle.
- Public Imp report: six take-out options, `selected_count: 1`, no monks/golems/token counts. Artifact-vs-ring first option labelled if the artifact deck can be exhausted by prior play.
- Nested `BranchFloorReport` `{ kind: "imp_vault", id: { depth, branch: 1 }, origin: Imp spawn floor, access.requires_acceptance: true }`. No pickaxe. Map is painter-complete (rooms, terrain, doors, transitions, carpets, flame-trap blobs). No vault mobs/heaps on the public map.
- `Imp.Quest.earnedShop` documented as `score > 2000` after completion; floor 20/21 Imp shop is **not** a guaranteed spawn. Do not emit shop stock as `prediction: guaranteed`.
- `VaultLevel` usingDefaults equipment does not change `RING.dropped` (01 oracle).
- `bun run test:rust` covers spawn + one vault layout golden (AAA-AAA-AAA or whichever oracle seed first produces a vault at 17–19).
- UI: QuestCard text no longer says “cursed +2…+4 ring after completing the quest.”

## Suggested PRs

1. `feat(core): generate v4 Imp.Quest rewardOptions at city spawn` — decks + floor RNG only; keep reporting as a multi-option group; leave monks/golems fields unused or deleted. City `initRooms` stream now matches v4 even before vault maps exist.
2. `feat(core): port GridBuilder and VaultLevel.initRooms` — room list, chances, treasure round-robin, no paint yet. Oracle: room-name multiset + bounds.
3. `feat(core): paint vault rooms, carpets, and flame-trap blobs` — `level/vault/` modules; public layout snapshot vs `VaultLevelOracle` at the createMobs boundary (MiningLevelOracle’s trick).
4. `feat(core): nest Imp vault branch reports like MiningLevel` — `BranchFloorKind`, access, reciprocal transitions, `analyze_seed` wiring.
5. `feat(web): Imp vault quest card and nested branch map` — QuestCard, FloorDetail, wasm types. Shop score gate copy. Finder: Imp rewards are a 6-choose-1 pool.

PR 1 is the city-layout RNG dependency for 03. PRs 2–4 can follow without UI.

## Dependencies

- **00**, **01** (artifact fallback, enchant tables, Imp deck oracle).
- **03** city carpets on AmbitiousImpRoom can share paint helpers; spawn RNG must land first.
- **06** vault/city_quest tilesheet and Playwright PNGs.
- Does not wait on 04/05 except enchant names (01/05).

## Explicit non-goals

- `oldQuest` / dwarf-token / `WndImpOld` paths (pre-4.0 saves).
- Vault enemy AI, investigating state, `VaultBossElemental` fight, score formula, hazard freebies, mirror (07).
- Stripping hero gear / `EscapeCrystal` / `ClothArmor` on entry (runtime).
- Guaranteeing the floor-20 Imp shop.
- Public-map vault mobs or interior take-home loot other than the six pedestals (those six are the take-out pool; interior usingDefaults gear is vault-only).
- Swarm Intelligence (07).
