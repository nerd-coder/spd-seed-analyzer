# Dwarven vault rooms — catalogue

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**.

Layout/loot contract: `vault-level.md`. This file lists painter rooms only.

All `VaultRoom`s are rigid 11×11 except long rooms (11×21 or 21×11) and
`VaultFinalRoom` (21×21). Treasure rooms `maxConnections == 1`
(`VaultTreasureRoom.java:36-38`). `createEquipment(n)` / `createConsumabe(n)`
are default-weight vault pools (`VaultLevel.java:241-491`).

### Always present

| Room | Size | Layout / hazards | Generation-time items |
|---|---|---|---|
| `VaultEntranceRoom` | 11×11, `isEntrance` | Carpeted hub, `BRANCH_ENTRANCE` at center | Two `VaultBeacon`; optional `DARKNESS` torch (`VaultEntranceRoom.java:47-147`) |
| `VaultTokensRoom` | long, ≥2 rooms from entrance | Twin diamonds, inner `LOCKED_DOOR` + `VaultTokenDoor`, `VaultMirror` | T3 equipment + T3 consumable beside the door; one non-LARGE wandering mob (`VaultTokensRoom.java:42-154`) |
| `VaultSimpleEnemyTreasureRoom` | 11×11 | Corner bunker, 4 orientations (`Random.Int(4)`) | Rejects T1 mobs; chest = `createEquipment` of the mob's tier (elemental → 3) (`VaultSimpleEnemyTreasureRoom.java:39-125`) |
| `VaultFinalRoom` | 21×21, `isExit`, ≥3 rooms from entrance | Ellipse arena, locked far door, 7 pedestals | Center `ImpStatue`; six `Imp.Quest.rewardOptions` shuffled onto the rest; boss is **not** painted (`VaultFinalRoom.java:140-236`) |

### Chance-table standard rooms (`VaultRoom.java:61-93`)

`setupChances` weights `{2,2,2,2,2, 1,1, 1,1,1}` in this class order. Each
successful pick decrements its weight; empty table refills. `initRooms`
adds rooms until `sizeFactor` sums to 9 (`VaultLevel.java:155-160`).
Hallway and long-rings have `sizeFactor` 2, so this block is 7–9 rooms.
Same class cannot share a grid side (`GridBuilder.java:271-282`).

| Class | Weight | Hazards / mobs | Items |
|---|---|---|---|
| `VaultRingRoom` | 2 | Inner wall ring; 1 wanderer, 4-corner path | none (`VaultRingRoom.java:34-68`) |
| `VaultCrossRoom` | 2 | Cross corridors; center `VaultSentry` (90° / length 4, 3-turn cooldown) | none (`VaultCrossRoom.java:32-61`) |
| `VaultQuadrantsRoom` | 2 | Statue hub, four stubs; 1 wanderer in a door-far corner | chest `createEquipment(mob tier)` (`VaultQuadrantsRoom.java:40-112`) |
| `VaultRingsRoom` | 2 | Four 3×3 pillars; 1 non-LARGE wanderer | none (`VaultRingsRoom.java:37-83`) |
| `VaultEnemyCenterRoom` | 2 | Nested walls + cross aisles; 1 wanderer in the inner 2×2 | center chest `createEquipment(mob tier)` (`VaultEnemyCenterRoom.java:38-100`) |
| `VaultHallwayRoom` | 1, long | Thin hall; 1 wanderer | `findPrizeItem(EquipableItem)` (`VaultHallwayRoom.java:37-80`) |
| `VaultLongRingsRoom` | 1, long | Concentric walls; 2 wanderers | `findPrizeItem(EquipableItem)` at center (`VaultLongRingsRoom.java:34-67`) |
| `VaultCircleRoom` | 1 | Open ring; center `VaultSentry` (`Int(4)` pattern) | none (`VaultCircleRoom.java:33-97`) |
| `VaultAlternatingFireRoom` | 1 | Checker `VaultFlameTrap` (initial CD 0/1, period 2, 1 trigger) | T0 equipment on center pedestal (`VaultAlternatingFireRoom.java:34-63`) |
| `VaultLasersRoom` | 1 | Pedestal lasers on open north/south and east/west walls; CD `IntRange(3,7)` | none (`VaultLasersRoom.java:34-84`) |

`VaultLongRoom` is the long-room base, not spawned. `VaultRat` is unused
by `createMob`.

### Treasure rooms (`VaultTreasureRoom.java:63-104`)

Three T1, three T2, three T3. Each tier list is shuffled, then
round-robin T1, T2, T3, … `nextRoom()` is called **seven** times, so
**all three T1**, **two of three T2**, **two of three T3** spawn. One T2
and one T3 class are absent.

`findT2SolveItem` / `findT3SolveItem` look through `itemsToSpawn` for
Blink / Invisibility (`VaultLevel.java:493-526`). Those land in the queue
only if `VaultHardLaserTreasureRoom` or `VaultManyScansRoom` already
painted; otherwise the room uses `createConsumabe`.

#### T1 (all three spawn)

| Class | Hazards | Heaps |
|---|---|---|
| `VaultFlamePathRoom` | Directed flame paths; offset `Int(5)` per lane (`VaultFlamePathRoom.java:135-182`) | T1 equipment chest, T2-solve or T1 consumable, 1 `DwarfToken` (`VaultFlamePathRoom.java:108-125`) |
| `VaultLaserTreasureRoom` | Paired lasers; far row `curCooldown = MAX` (visual only) | same three heaps (`VaultLaserTreasureRoom.java:147-164`) |
| `VaultCircleScanTreasureRoom` | Center sentry, 45° clock/counter-clock | same three heaps (`VaultCircleScanTreasureRoom.java:152-168`) |

#### T2 (two of three)

| Class | Hazards / mobs | Heaps |
|---|---|---|
| `VaultSingleEnemyTreasureRoom` | Ellipse; one `Random.oneOf(T2Mobs)` | T2 equipment chest + T2 consumable; **no** token (`VaultSingleEnemyTreasureRoom.java:38-74`) |
| `VaultBookcaseTreasureRoom` | Bookshelves, two pedestals | `findPrizeItem()` on near pedestal; T2 equipment chest; T3-solve or T2 consumable; 1 token; queues `PotionOfLiquidFlame` (`VaultBookcaseTreasureRoom.java:77-94`) |
| `VaultFlamesTreasureRoom` | Ellipse filled with period-1 flames | T2 equipment chest; T3-solve or T2 consumable + 1 token on the side pedestals; queues `PotionOfPurity` (`VaultFlamesTreasureRoom.java:69-99`) |

#### T3 (two of three)

| Class | Hazards / mobs | Heaps |
|---|---|---|
| `VaultManyScansRoom` | Up to 8 corner/edge sentries aimed at center | T3 equipment chest, T3 consumable, 1 token; queues `PotionOfInvisibility` (`VaultManyScansRoom.java:42-83`) |
| `VaultMultipleEnemyTreasureRoom` | Three `VaultGhoul` (`maxLvl = 0`, no token loot) | T3 equipment chest + T3 consumable; **no** token heap (`VaultMultipleEnemyTreasureRoom.java:54-111`) |
| `VaultHardLaserTreasureRoom` | Dense laser gauntlet; far row visual-only | T3 equipment chest, T3 consumable, 1 token; queues `StoneOfBlink` (`VaultHardLaserTreasureRoom.java:154-170`) |

Generation-time token heaps: 3 (T1) + 1–2 (T2) + 1–2 (T3) = **5–7**.
The token door costs 10 (`VaultTokenDoor.java:72-75`).

### Wandering mob table (`VaultLevel.java:528-563`)

When `mobsToSpawn` is empty, the level queues both T1 classes plus one
extra random T1, all three T2, both T3 plus one extra random T3, then
shuffles. `VaultElemental` resolves through `VaultElemental.random()`:
40% fire, 40% frost, 20% shock (`VaultElemental.java:30-38`). LARGE mobs
(Golem) are drawn-and-returned in cramped rooms (`VaultLevel.java:565-568`).
`createMob` runs during room paint, so identities follow paint order.

| Tier | Classes |
|---|---|
| T1 | `VaultSkeleton`, `VaultDM100` |
| T2 | `VaultShaman`, `VaultDM200`, `VaultGhoul` (“only if solo” in comment; still in the rotation) |
| T3 | `VaultElemental`, `VaultGolem` |

All of these set `loot = DwarfToken`, `lootChance = 1`, `maxLvl = 30`,
`EXP = 0`. Those drops are combat, not generation heaps.
