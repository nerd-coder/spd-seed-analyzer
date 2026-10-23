# Boss floor entities

Pinned v4.0.2 level-creation entities on boss floors. Every one is placed
before any Generator draw on its floor, so none depends on run history. Bones
(`Bones.get()`) is absent for custom seeds; its child seed is still drawn.

| Depth | Entity | Placement |
|-------|--------|-----------|
| 5 | Goo | `boss.pos = pointToCell(center())` at the end of each Goo room paint (`DiamondGooRoom.java:64-66`, same in the Walled/ThinPillars/ThickPillars rooms) |
| 5 | Rat King | `RatKingRoom` center, `state = SLEEPING` (`RatKingRoom.java:117-118`, `RatKing.java:46`) |
| 5 | Gold | `Gold(IntRange(5, 20))` on every EMPTY/EMPTY_SP room cell except the center, column-major (`RatKingRoom.java:121-125`) |
| 10 | Cages | `addCagesToCells`: child generator on `seedCurDepth`, 5× `randomPrisonCellPos`, `REGION_DECO` when a NEIGHBOURS4 cell is `WALL` (torches are `WALL_DECO`, so they do not count) (`PrisonBossLevel.java:383-399`) |
| 10 | IronKey | `createItems`: `Random.Long()` for the Bones child, then `randomPrisonCellPos` until not solid (`PrisonBossLevel.java:607-626`, `:643-648`) |
| 15 | Pylons | `pylonPositions` (`CavesBossLevel.java:112`, `:208-214`); each instance draws `targetNeighbor = Random.Int(8)` (`Pylon.java:69`) |
| 26 | Amulet | `drop(new Amulet(), AMULET_POS)`, `AMULET_POS = 12*WIDTH + MID` (`LastLevel.java:120`, `:188-190`) |

Depths 20 and 25 create no level entities; Tengu is held off-level until
the fight (`PrisonBossLevel.java:598-600`).

## Conditional Imp shop (depth 20)

`ImpShopRoom.paint` only generates the stock during level creation
(`ImpShopRoom.java:63-67`). `spawnShop` runs on level load after the Imp quest
and the Dwarf King (`CityBossLevel.java:129`, `:378`; `ImpShopRoom.java:154`).
Placement is then deterministic: room `(3,12)-(11,20)` inclusive
(`CityBossLevel.java:194`), grown downward while
`spacesNeeded >= 7*(height-2)` (`:395-396`); the ImpShopkeeper stands on the
build pedestal at `(7,16)`; with no connections `entrance()` is
`((left+right)/2+1, bottom-1)` (`ImpShopRoom.java:96`), so `ShopRoom.placeItems`
fills rings clockwise from `(6,19)`, skipping each ring's entry inset (45 cells
for the 9×9 room). The map publishes the keeper and FOR_SALE heaps in
`uncertain_cells`.

## Boss floor terrain and custom layers

- Sewer (5) and Caves (15) raw terrain, decorations included, matches the
  oracles. `CavesPainter.generateGold` turns walls into `WALL_DECO` only above
  `DungeonTileSheet.floorTile` cells: water, or a `directVisuals` entry below
  `CHASM` (`CavesPainter.java:135-147`, `DungeonTileSheet.java:178`, `:419-444`).
  Statues, high grass and region deco do not count; `CUSTOM_DECO` (the gate)
  and secret traps do.
- Depth 5 adds `GooNest` (`GooBossRoom.java:61`), `SewerExit` and
  `SewerExitOverhang` (`SewerBossExitRoom.java:71`, `:75`).
- Depth 15 adds `CityEntrance`, `EntranceTerrain`, `EntranceOverhang` and
  `ArenaVisuals` (`CavesBossLevel.java:652-886`), in their unlocked state:
  pylon cells are `-1`, adjacent pads `54 + Δ(x+8y)`, wires 37, and the
  solid gate 40–44.
- Depth 25 keeps its exit as `WALL_DECO` until `unseal` sets `EXIT`
  (`HallsBossLevel.java:292`). The public map shows the opened exit and the
  matching `updateState` patches (`:442`, `:481`) so the stairs are visible.
- Depth 26 `CustomFloor` (`LastLevel.java:160`, `:276-324`) paints the 7×7
  candles from `AMULET_POS - 3 - 3*width` (rows 9–15), bumps 46 to 47 at tile
  variance ≥ 50, marks floor-over-chasm edges 6, and maps EMPTY to 19 and
  EMPTY_DECO to 27. The oracle dumps only the static `CANDLES` array.

Method: `level/boss_layouts/entity_tests.rs` compares the public maps with the
oracle `final_mobs`/`final_heaps` fixtures (depth 5: five seeds; 10: two;
15: four; 26: three) and depth 10 raw terrain, cages and torches included.
`imp_shop_is_published_as_conditional_entities` checks depth 20 against the
two oracles outside the shop cells. `boss_layouts/tests.rs` compares raw
terrain (5, 15, 25) and custom layer rects (5, 15, 26) with the oracles.
