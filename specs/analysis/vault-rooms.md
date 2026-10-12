# Dwarven vault rooms — catalogue

Target: **Shattered Pixel Dungeon v4.0.2 @ `57a4e06a4`**.

Layout/loot contract: `vault-level.md`. This file lists painter rooms only.

All `VaultRoom`s are rigid 11×11 except long rooms (11×21 or 21×11) and
`VaultFinalRoom` (21×21). Treasure rooms `maxConnections == 1`
(`VaultTreasureRoom.java:36-38`). `createEquipment(n)` / `createConsumabe(n)`
are default-weight vault pools (`VaultLevel.java:241-491`).

### Always present

| Room | Size | Layout / hazards | Generation-time items |
|---|---|---|---|
| `VaultEntranceRoom` | 11×11, `isEntrance` | Carpeted hub, `BRANCH_ENTRANCE` at center | Two `VaultBeacon`; optional `DARKNESS` torch (`VaultEntranceRoom.java:47-147`) |
| `VaultTokensRoom` | long, ≥2 rooms from entrance | Twin diamonds, inner `LOCKED_DOOR` + `VaultTokenDoor`, `TokenDoorFloor` on that door, `VaultMirror` | T3 equipment + T3 consumable beside the door; one non-LARGE wandering mob (`VaultTokensRoom.java:46-163`) |
| `VaultSimpleEnemyTreasureRoom` | 11×11 | Corner bunker, 4 orientations (`Random.Int(4)`) | Rejects T1 mobs; chest = `createEquipment` of the mob's tier (elemental → 3) (`VaultSimpleEnemyTreasureRoom.java:39-125`) |
| `VaultFinalRoom` | 21×21, `isExit`, ≥3 rooms from entrance | Ellipse arena, locked far door, 7 pedestals; `FinalRoomDoor` custom tiles and `FinalRoomDoorOverhang` custom walls on the entry and locked doors | Center `ImpStatue`; six `Imp.Quest.rewardOptions` shuffled onto the rest; boss is **not** painted (`VaultFinalRoom.java:143-259`) |

### Chance-table standard rooms (`VaultRoom.java:61-93`)

`setupChances` weights `{2,2,2,2,2, 1,1, 1,1,1}` in this class order. Each
successful pick decrements its weight; empty table refills. `initRooms`
adds rooms until `sizeFactor` sums to 9 (`VaultLevel.java:155-160`).
Hallway and long-rings have `sizeFactor` 2, so this block is 7–9 rooms.
Same class cannot share a grid side (`GridBuilder.java:271-282`).

| Class | Weight | Hazards / mobs | Items |
|---|---|---|---|
| `VaultRingRoom` | 2 | Inner wall ring; 1 wanderer (T1 if connected to the entrance), 4-corner path | none (`VaultRingRoom.java:40-98`) |
| `VaultCrossRoom` | 2 | Cross corridors; center `VaultSentry` (90° / length 4, 3-turn cooldown) | none (`VaultCrossRoom.java:32-61`) |
| `VaultQuadrantsRoom` | 2 | Statue hub, four stubs; 1 wanderer in a door-far corner (T1 if connected to the entrance) | chest `createEquipment(mob tier)` (`VaultQuadrantsRoom.java:43-141`) |
| `VaultRingsRoom` | 2 | Four 3×3 pillars; 1 non-LARGE wanderer (also T1 if connected to the entrance) | none (`VaultRingsRoom.java:41-101`) |
| `VaultEnemyCenterRoom` | 2 | Nested walls + cross aisles; 1 wanderer in the inner 2×2 (T1 if connected to the entrance) | center chest `createEquipment(mob tier)` (`VaultEnemyCenterRoom.java:43-131`) |
| `VaultHallwayRoom` | 1, long | Thin hall; 1 wanderer (T1 if connected to the entrance) | `findPrizeItem(EquipableItem)` (`VaultHallwayRoom.java:42-123`) |
| `VaultLongRingsRoom` | 1, long | Concentric walls; 2 wanderers; `canConnect` rejects `VaultEntranceRoom` | `findPrizeItem(EquipableItem)` at center (`VaultLongRingsRoom.java:36-89`) |
| `VaultCircleRoom` | 1 | Open ring; center `VaultSentry` (`Int(3)`, `afterScanCooldown` 2) | none (`VaultCircleRoom.java:34-100`) |
| `VaultAlternatingFireRoom` | 1 | Checker `VaultFlameTrap` on `left+2..right-2`, `top+2..bottom-2`, pedestal skipped (initial CD 0/1, period 2, 1 trigger) | T0 equipment on center pedestal (`VaultAlternatingFireRoom.java:35-69`) |
| `VaultLasersRoom` | 1 | Pedestal lasers on open north/south and east/west walls; mob `afterShotCooldown` `IntRange(7, 9)`, `curCooldown` `IntRange(1, afterShotCooldown)` | none (`VaultLasersRoom.java:35-84`) |

`VaultCircleRoom` rolls one `Random.Int(3)` (`VaultCircleRoom.java:51`).
Case 0 is `scanWidth` 90° with eight adjacent single-cell headings
(`VaultCircleRoom.java:52-65`). Cases 1 and 2 are the same 45° pattern,
eight opposite-pair headings (`VaultCircleRoom.java:66-78`). The 22.5°
pattern is not in the switch. `afterScanCooldown` is 2
(`VaultCircleRoom.java:45`). The sentry is a mob, not painter terrain.

`VaultAlternatingFireRoom` paints traps for
`x = left+2..=right-2`, `y = top+2..=bottom-2`, and skips pedestals, so
the inset-1 ring stays clear (`VaultAlternatingFireRoom.java:53-62`).
The center T0 item is a direct `level.drop`, not a `canPlaceItem` choice
(`VaultAlternatingFireRoom.java:43-49`). `canPlaceItem` is the expression
`super.canPlaceItem(p, l) && p.x == left+1 || p.y == top+1 || p.x == right-1 || p.y == bottom-1`
(`VaultAlternatingFireRoom.java:68-69`). `&&` binds tighter than `||`, so
only the `left+1` edge is conjoined with `super.canPlaceItem`. The
`top+1`, `right-1`, and `bottom-1` edges do not require it.

`VaultLongRingsRoom.canConnect(Room)` returns false for a
`VaultEntranceRoom` (`VaultLongRingsRoom.java:85-89`; more difficult
rooms cannot neighbor the entrance). `connect` tests only the caller
(`Room.java:270-273`), and placement uses `r.connect(n)`
(`GridBuilder.java:286`), so the long-rings room cannot connect itself
to the entrance. The extra-connection pass can still call
`entrance.connect(longRings)` (`GridBuilder.java:300-304`), which does
not use this override. That changes which placement edges succeed.

Laser cooldowns are mob fields set at paint, not painter terrain:
`afterShotCooldown = Random.IntRange(7, 9)` and
`curCooldown = Random.IntRange(1, afterShotCooldown)`
(`VaultLasersRoom.java:59-60,80-81`).

`VaultFinalRoom` sets the entry cell to `DOOR` and the far cell to
`LOCKED_DOOR` (`VaultFinalRoom.java:179-180`), then adds `FinalRoomDoor`
custom tiles on both and `FinalRoomDoorOverhang` custom walls one tile
above each (`VaultFinalRoom.java:245-259`). Those overlays do not write
terrain. `VaultTokensRoom` sets the token door to `LOCKED_DOOR` and adds
a `TokenDoorFloor` custom tile on that cell
(`VaultTokensRoom.java:107-114`). The tile only picks a sprite from the
current terrain (`VaultTokensRoom.java:227-234`).

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

When `mobsToSpawn` is empty, `createMob` queues both T1 classes plus one
`Random.oneOf` T1, all three T2, both T3 plus one `Random.oneOf` T3, then
`Random.shuffle` (`VaultLevel.java:549-556`). The fill does not run while
the list still has classes. Drawing `VaultElemental.class` calls
`VaultElemental.random()` before the instance is built
(`VaultLevel.java:558-562`): one `Random.Float()`, 40% fire, 40% frost,
20% shock (`VaultElemental.java:30-38`). `returnMob` inserts the class
it is given at index 0 (`VaultLevel.java:566-568`).

A wanderer room that is directly connected to the entrance rejects any
mob that is not `VaultSkeleton` or `VaultDM100`. It loops `createMob`
until a T1 remains, holding rejects aside, then
`returnMob(enemy.getClass())` for each. Inserts are at index 0, so the
last rejected class is drawn next. Rejects are not back in the deck
during the loop, so a refill in that loop cannot see them yet.
Rooms: `VaultEnemyCenterRoom` (`VaultEnemyCenterRoom.java:58-82`),
`VaultHallwayRoom` (`VaultHallwayRoom.java:86-110`),
`VaultQuadrantsRoom` (`VaultQuadrantsRoom.java:73-98`, and only when a
door-far corner still exists), `VaultRingRoom`
(`VaultRingRoom.java:50-97`), `VaultRingsRoom`
(`VaultRingsRoom.java:54-100`). `VaultRingsRoom` still rejects `LARGE`
when it is not next to the entrance (`VaultRingsRoom.java:67-70`).
`VaultTokensRoom` still returns `LARGE` the same way
(`VaultTokensRoom.java:129-161`).

A rejected elemental has already consumed `random()` on the main stream
during that room's paint. `returnMob` stores `enemy.getClass()`, the
fire, frost, or shock subclass, not `VaultElemental.class`, so a later
draw of that subclass does not roll again. If the deck empties inside
the reject loop, the `oneOf` / `shuffle` refill runs before those
classes are returned, and another `VaultElemental.class` draw rolls
`random()` again. `VaultRingRoom` and `VaultRingsRoom` call `returnMob`
after their wander rolls; the other three call it before.
Wandering mobs stay off the public map.

`createMob` runs during room paint, so identities follow paint order.

| Tier | Classes |
|---|---|
| T1 | `VaultSkeleton`, `VaultDM100` |
| T2 | `VaultShaman`, `VaultDM200`, `VaultGhoul` (“only if solo” in comment; still in the rotation) |
| T3 | `VaultElemental`, `VaultGolem` |

All of these set `loot = DwarfToken`, `lootChance = 1`, `maxLvl = 30`,
`EXP = 0`. Those drops are combat, not generation heaps.
