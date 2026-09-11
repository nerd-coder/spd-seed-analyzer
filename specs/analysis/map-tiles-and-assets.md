# Map tiles and assets

Target: **Shattered Pixel Dungeon v4.0.0 @ `2bb34a4e9`**. Public maps follow
`MAP-LAYOUT-GOAL`: painter-complete rooms, terrain, doors, transitions, traps,
plants, and blobs. Not heaps, mobs, or NPCs.

Analyzer map drawing lives in `web/src/lib/tiles.ts`,
`web/src/lib/dungeon-tile-visuals.ts`, and `web/src/lib/map-assets.ts`. Visual
fixtures: `tools/visual/tests/map-render-fixtures.ts`.

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

`GameScene.java:273-336` composites, bottom to top:

1. scrolling water (`water0.png`…`water4.png`)
2. `DungeonTerrainTilemap` from `tiles_*.png` (`DungeonTerrainTilemap.java:43-115`)
3. `level.customTiles`
4. visual grid
5. **wall–floor occlusion** (`WallOcclusionTilemap`)
6. `TerrainFeaturesTilemap` (`terrain_features.png`: traps, plants, grass,
   embers, barricade, alchemy, statues, region deco, mine crystal/boulder;
   `TerrainFeaturesTilemap.java:56-135`)
7. `level.customTerrain`
8. heaps / mobs (not public maps)
9. **raised grass** (`RaisedTerrainTilemap` on `raised_terrain.png`)
10. `DungeonWallsTilemap` (wall internals + overhangs from `tiles_*.png`)
11. `level.customWalls`

The analyzer draws 2, a grass/embers subset of 6, `custom_tiles`, then raised
grass **from `tiles_*.png`**, then walls and `custom_walls`. It has no
occlusion layer, no `customTerrain`, and no `raised_terrain.png`.

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
  `SegmentedLibraryRoom.java:131`, `LibraryRingEntranceRoom` /
  `LibraryRingExitRoom`, `StatuesEntranceRoom.java:59-77` /
  `StatuesExitRoom` (inserted at index 0 so later carpets stack on top)
- `AmbitiousImpRoom.java:85-94`, vault rooms
  (`VaultEntranceRoom.java:71-77`, `VaultFinalRoom`, `VaultTokensRoom`)
- secret `RatKingRoom.java:101-103`

`CityPainter.decorate` (`CityPainter.java:36-54`) only scatters `EMPTY_DECO` /
`WALL_DECO`. It does not emit carpets. `HallwayRoom` / `StatuesRoom` weights
are city-only (`StandardRoom.java:144-147,182-183`).

`customTerrain` is a third painter-complete list (`Level.java:189-190`), drawn
after features (`GameScene.java:295-381`). Imp banners, mass-grave raised
statues, ritual tables, and rat-king statues live there. The analyzer
`FloorMap` has `custom_tiles` / `custom_walls` only (`spd-wasm.ts:118-119`).

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
overlays.

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
  `StatueRaised` on `customTerrain`. (Class `Bones` is pre-v4 save restore
  only.)
- **RitualSiteRoom** (`RitualSiteRoom.java:52-124`): wall cages /
  `REGION_DECO` / `CUSTOM_DECO` tables, 5×5 `RitualMarker` on `customTiles`,
  `Table` on `customTerrain`, all `prison_quest.png`.
- **AmbitiousImpRoom** (`AmbitiousImpRoom.java:52-116`): two carpets, 5×5
  `QuestEntrance` + pulsing `EntranceBarrier` on `customTiles`, `WallBanners`
  on `customTerrain`, all `city_quest.png`.

## Asset inventory

Clone root: `core/src/main/assets/`. Analyzer: `web/public/assets/` (flat).

**Clone-only environment sheets (public-map gaps):**
`occlusion_shadows.png`, `raised_terrain.png`, `custom_tiles/carpet.png`,
`custom_tiles/rat_king_room.png`.

**Same path, different pixels (stale analyzer copies):** every
`tiles_{sewers,prison,caves,caves_crystal,caves_gnoll,city,halls}.png`
(still 256×256); `terrain_features.png` (analyzer 256×128, clone 256×256);
`custom_tiles/{caves_quest,city_quest,prison_quest,caves_boss,city_boss}.png`
(quest sheets also changed size: `caves_quest` 128×64→64×128, `city_quest`
128×128→256×256, `prison_quest` 64×64→256×256).

**Unchanged and already present:** `water0.png`–`water4.png`,
`custom_tiles/{halls_special,prison_exit,sewer_boss,weak_floor}.png`,
`visual_grid.png`, `wall_blocking.png`.

Analyzer `loadMapAssets` never loads carpet, occlusion, raised terrain, or
`rat_king_room`. `customTileImage` keys only
`prison_quest` / `caves_quest` / `city_quest` / `city_boss` / `weak_floor` /
`halls_special` (`tiles.ts:90-98`).

Sprite gaps that are **not** public-map layout: `imp.png`, `sentry.png`
(analyzer still has `red_sentry.png`), `vault_boss_elemental.png`,
`vault_mirror.png`, `vault_tokens_door.png`. Analyzer leftover: `demon.png`.

## Consumable sprites

The analyzer UI draws item icons from `/assets/sprites/items.png`
(`ItemIcon.tsx`, `item-icons.ts`; heap sprites in `map-entities.ts` are not
public maps). `item_icons.png` is unused. Clone `items.png` differs at the
same 256×512 size (`ItemSpriteSheet.java:29-32`): seeds, darts, potions,
elixirs/brews, runestones, scrolls, spells, dewdrop, petrified seed, liquid
metal, blood vial, energy / exotic crystals, dwarf tokens.

## Visual-test snapshots

Committed files in `tools/visual/snapshots/`:

- `CXG-FJT-BFQ-F1.png`
- `HKT-JZN-XQQ-F1.png`
- `AAA-AAA-AAA-F13-B1-Crystal.png`
- `AAA-AAA-AAB-F13-B1-Gnoll.png`

All four are invalid under v4 tiles: sewers and both mining tilesheets plus
`terrain_features.png` and `caves_quest.png` changed, and a correct stack
adds `occlusion_shadows.png` on every floor.
