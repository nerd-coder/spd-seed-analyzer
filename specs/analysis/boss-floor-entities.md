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

Method: `level/boss_layouts/entity_tests.rs` compares the public maps with the
oracle `final_mobs`/`final_heaps` fixtures (depth 5: five seeds; 10: two;
15: four; 26: three) and depth 10 raw terrain, cages and torches included.
