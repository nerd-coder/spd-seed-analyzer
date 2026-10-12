# Map tiles and assets

Target: **Shattered Pixel Dungeon v4.0.2 @ `57a4e06a4`**. Public maps follow
`MAP-LAYOUT-GOAL`: painter-complete rooms, terrain, doors, transitions, traps,
plants, blobs, and custom tiles/carpets. Not heaps, mobs, or NPCs.

Analyzer map drawing lives in `web/src/lib/tiles.ts`,
`web/src/lib/dungeon-tile-visuals.ts`, `web/src/lib/tiles-overlays.ts`, and
`web/src/lib/map-assets.ts`. Visual fixtures:
`tools/visual/tests/map-render-fixtures.ts`.

## Terrain IDs

`Terrain.java` IDs the renderer must decode. Unlisted slots are unused.

| ID | Constant | Public-map role |
|---|---|---|
| 0–38 | `CHASM` … `HERO_LKD_DR` | Existing terrain IDs. `HERO_LKD_DR` (38) is a runtime locked-door alias, not painter-complete. |
| 39 | `CUSTOM_DECO_WTR` | Solid invisible deco that draws as water (`DungeonTileSheet.java:436`). Blacksmith furnace cell (`BlacksmithRoom.java:72`). |

Direct floor visuals, water/chasm stitching, raised walls/doors/statues, and
alts still come from `DungeonTileSheet.java`. `CUSTOM_DECO` / `CUSTOM_DECO_EMPTY`
stay floor; `CUSTOM_DECO_WTR` is the water-looking hole under a custom overlay.

## Render stack

`GameScene.java:256-338` adds these layers bottom to top. Custom tilemaps are
attached at `GameScene.java:377-387`.

1. scrolling water (`water0.png`…`water4.png`)
2. `DungeonTerrainTilemap` from `tiles_*.png` (`DungeonTerrainTilemap.java:43-117`)
3. `level.customTiles`
4. visual grid
5. **wall–floor occlusion** (`WallOcclusionTilemap`)
6. `TerrainFeaturesTilemap` (`terrain_features.png`: traps, plants, grass,
   embers, barricade, alchemy, statues, region deco, mine crystal/boulder;
   `TerrainFeaturesTilemap.java:56-137`)
7. `level.customTerrain`
8. heaps / mobs (not public maps)
9. **raised grass** (`RaisedTerrainTilemap` on `raised_terrain.png`)
10. `DungeonWallsTilemap` (wall internals + overhangs from `tiles_*.png`)
11. `level.customWalls`

`drawFloorMap` paints scrolling water, then the static bitmap
(`tiles.ts:210-240`). That bitmap is terrain from `tiles_*.png`, a
grass/embers subset of features (`dungeon-tile-visuals.ts:465-472`),
`custom_tiles`, visible traps, known entities, `custom_terrain`, raised grass
from `raised_terrain.png`, walls from `tiles_*.png`, wall occlusion,
`custom_walls`, then markers (`tiles.ts:122-179`). `FloorMap` carries
`custom_tiles`, optional `custom_terrain`, and `custom_walls`
(`spd-wasm.ts:118-120`).

`CityBossLevel.CustomGroundVisuals` (`city_boss.png`, `tileW` 15,
`CityBossLevel.java:455-460`) overrides `allowWater`. The superclass allows
water when the custom image is null (`CustomTilemap.java:130-131`). Otherwise
a curtain tail is allowed only when `i = tileX + tileY*tileW` is greater than
`tileW*32`, the cell is `EMPTY` or `EMPTY_DECO`, and the cell `tileW` above is
`WALL_DECO` (`CityBossLevel.java:620-627`). `setCellToWater` refuses the cell
if any covering custom tilemap returns false (`Level.java:1118-1131`). The
index stride is `tileW`, not `tileH`.

## City carpets — custom tilemap, painter-complete: **yes**

Carpets are **not** a `Terrain` enum. They are `Carpet extends CustomTilemap`
on `level.customTiles`, texture `environment/custom_tiles/carpet.png`
(`Carpet.java:35-38`, `Assets.java:57`). Underneath, rooms still paint
`EMPTY_SP` / `CUSTOM_DECO_EMPTY` / `STATUE` / `STATUE_SP` / `REGION_DECO` /
`REGION_DECO_ALT` / `ENTRANCE_SP` / `EXIT`.

`Carpet.create` (`Carpet.java:75-96`) fills a rectangle with 16 stitched edge
tiles at `regionOfs = 16 * ((depth-1)/5)`, then applies overrides. `SKIP = -1`
is a hole (exit cells keep the tilesheet exit). Customs from index 80:
`CITY_STATUE`, `CITY_PEDESTAL`, `CITY_ENTRANCE`, and the four statue / two
pedestal corner pieces (`Carpet.java:43-53`). Shape is rectangular only
(`Carpet.java:33-34`). Implemented city art is the depth-16–20 row.

Paint sites (all `Room.paint`, before `RegularPainter.decorate` at
`RegularPainter.java:124-151`):

- city standards: `HallwayRoom.java:106-122`, `StatuesRoom.java:70-83`,
  `SegmentedLibraryRoom.java:131-133`, `LibraryRingEntranceRoom.java:65-68` /
  `LibraryRingExitRoom.java:65-68`
- `StatuesEntranceRoom` / `StatuesExitRoom`, when `width() >= 11` or
  `height() >= 11`: the center cell is the transition. The carpet rect starts
  as `(c.x-1, c.y-1, c.x+1, c.y+1)`, an inclusive 3×3. An even width or height
  grows that axis by one: `right++` or `bottom++` when `c.x < (left+right)/2f`
  or `c.y < (top+bottom)/2f`, otherwise `left--` or `top--`. Every cell of
  that rect that is not already `STATUE` becomes `EMPTY_DECO`. The carpet
  `setRect` passes `width()+1` and `height()+1` (`Rect.width()` is
  `right-left`; `getPoints()` is inclusive), then is inserted at
  `customTiles` index 0 so later carpets stack on top. Entrance overrides the
  center with `CITY_ENTRANCE`; exit uses `SKIP`. Both rooms then set
  `ENTRANCE` or `EXIT` and add the transition
  (`StatuesEntranceRoom.java:50-91`, `StatuesExitRoom.java:50-91`,
  `Rect.java:48-49,155-160`).
- `AmbitiousImpRoom.java:85-94`, vault rooms
  (`VaultEntranceRoom.java:71-77`, `VaultFinalRoom`, `VaultTokensRoom`)
- secret `RatKingRoom.java:101-103`

`CityPainter.decorate` (`CityPainter.java:36-54`) only scatters `EMPTY_DECO` /
`WALL_DECO`. It does not emit carpets. `HallwayRoom` / `StatuesRoom` weights
are city-only (`StandardRoom.java:144-147,182-183`).

`customTerrain` is a third painter-complete list (`Level.java:189`), drawn
after features (`GameScene.java:294-298,381-383`). Imp banners, mass-grave raised
statues, ritual tables, and rat-king statues live there.

## Wall–floor shadowing — overlay, not a map fact

`WallOcclusionTilemap` (`WallOcclusionTilemap.java:49-124`) maps
`wallStitcheable` neighbours plus door orientation onto
`environment/occlusion_shadows.png`. Nothing is stored in `level.map`. Alchemy
skips the “wall above” case so water still shows through
(`WallOcclusionTilemap.java:71-73`). The renderer must apply this overlay.

Raised high / furrowed grass is the same kind of fact: terrain ID in the map,
extra pixels from `environment/raised_terrain.png` at `region*4`
(`RaisedTerrainTilemap.java:32-67`), then wall overhangs from `tiles_*.png`.

## Vault tileset

`VaultLevel` extends `CityLevel` (`VaultLevel.java:123`). It inherits
`tilesTex()` = `tiles_city.png` and `waterTex()` = `water3.png`
(`CityLevel.java:107-114`). Music is `CITY_TENSE` (`VaultLevel.java:126-128`).
No distinct vault tilesheet; vault rooms add `Carpet` plus `city_quest.png`
overlays. Paint also attaches these `CITY_QUEST` tilemaps: `FinalRoomDoor`
(1×1 on `customTiles`) and `FinalRoomDoorOverhang` (1×2 on `customWalls`,
origin on the cell above the door) (`VaultFinalRoom.java:245-259,473-477,557-562`);
`TokenDoorFloor` (1×1 on `customTiles`)
(`VaultTokensRoom.java:112-114,210-214`).

## Quest-room layout visuals

All of these attach during `paint` (painter-complete). NPCs and heaps on the
same rooms are not public-map content.

- **BlacksmithRoom** (`BlacksmithRoom.java:54-123`): `EMPTY_SP` smithy,
  `CUSTOM_DECO` / `CUSTOM_DECO_WTR` furnace, `caves_quest.png` `QuestEntrance`
  + `SmithyVisuals`, `FurnaceOverhang` on `customWalls`.
- **MineEntrance** (`MineEntrance.java:68-94`): `ENTRANCE_SP` + 8× `EMPTY_SP`,
  3×3 `QuestExit` from `caves_quest.png`.
- **MassGraveRoom** (`MassGraveRoom.java:55-83`): `CUSTOM_DECO_EMPTY` floor,
  9×9 `MassGraveDeco` from `prison_quest.png` on `customTiles`,
  `StatueRaised` on `customTerrain`. (`Bones` is the old-save tilemap, not a
  paint attachment, `MassGraveRoom.java:162-163`.)
- **RitualSiteRoom** (`RitualSiteRoom.java:52-124`): wall cages /
  `REGION_DECO` / `CUSTOM_DECO` tables, 5×5 `RitualMarker` on `customTiles`,
  `Table` on `customTerrain`, all `prison_quest.png`.
- **AmbitiousImpRoom** (`AmbitiousImpRoom.java:52-116`): two carpets, 5×5
  `QuestEntrance` + pulsing `EntranceBarrier` on `customTiles`, `WallBanners`
  on `customTerrain`, all `city_quest.png`.

## Asset inventory

Clone root: `core/src/main/assets/`. Analyzer: `web/public/assets/` (flat).

`loadMapAssets` loads, and the clone bytes match, the seven `tiles_*.png`
sheets (256×256), `terrain_features.png` (256×256), the active
`water{region}.png`, `occlusion_shadows.png`, `raised_terrain.png`, and
`custom_tiles/{carpet,rat_king_room,caves_quest,prison_quest,city_boss,halls_special,weak_floor}.png`
(`map-assets.ts:67-120`). `water0.png`–`water4.png` are all present and match.
Also present and matching, and not requested by that loader:
`visual_grid.png`, `wall_blocking.png`,
`custom_tiles/{caves_boss,prison_exit,sewer_boss}.png`.

`custom_tiles/city_quest.png` (256×256) and `sprites/items.png` (256×512,
`ItemSpriteSheet.java:29-32`) match the clone. `item_icons.png` matches the
clone and is unused by the map UI.

`customTileImage` resolves `prison_quest`, `caves_quest`, `city_quest`,
`city_boss`, `weak_floor`, `halls_special`, `carpet`, and `rat_king_room`
(`tiles-overlays.ts:28-39`).

Sprite gaps that are **not** public-map layout: `imp.png`, `sentry.png`,
`vault_boss_elemental.png`, `vault_mirror.png`, `vault_tokens_door.png`.
Analyzer leftovers: `red_sentry.png`, `demon.png`.

## Consumable sprites

The analyzer UI draws item icons from `/assets/sprites/items.png`
(`ItemIcon.tsx`, `item-icons.ts`; heap sprites in `map-entities.ts` are not
public maps). `item_icons.png` is unused. `items.png` matches the clone
(256×512, `ItemSpriteSheet.java:29-32`).

## Visual-test snapshots

Committed files in `tools/visual/snapshots/`:

- `CXG-FJT-BFQ-F1.png`
- `HKT-JZN-XQQ-F1.png`
- `AAA-AAA-AAA-F13-B1-Crystal.png`
- `AAA-AAA-AAB-F13-B1-Gnoll.png`
- `AAA-AAA-AAA-F19-B1-Vault.png`
- `GFX-PZH-DCH-F16.png`
