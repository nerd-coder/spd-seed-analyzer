import {
  doorTile,
  Terrain,
  TILE_PX,
  wallStitchable,
} from '@/lib/dungeon-tile-visuals'
import type { MapAssets } from '@/lib/map-assets'
import type { FloorMap, MapCustomTile } from '@/lib/spd-wasm'

const CLEARED = 0
const DOOR_VERT = 6
const DOOR_HORIZ = 7
const WALL_ABOVE = 40
const WALL_LEFT_BELOW = 1
const WALL_LEFT = 2
const WALL_LEFT_ABOVE = 3
const WALL_LEFT_BOTH = 4
const WALL_RIGHT_BELOW = 8
const WALL_RIGHT = 16
const WALL_RIGHT_ABOVE = 24
const WALL_RIGHT_BOTH = 32

function customTileStem(texture: string) {
  const file = texture.slice(texture.lastIndexOf('/') + 1)
  return file.endsWith('.png') ? file.slice(0, -4) : file
}

function customTileImage(assets: MapAssets, texture: string) {
  const images: Record<string, HTMLImageElement> = {
    prison_quest: assets.customTiles.prisonQuest,
    caves_quest: assets.customTiles.cavesQuest,
    city_quest: assets.customTiles.cityQuest,
    city_boss: assets.customTiles.cityBoss,
    weak_floor: assets.customTiles.weakFloor,
    halls_special: assets.customTiles.hallsSpecial,
    carpet: assets.customTiles.carpet,
    rat_king_room: assets.customTiles.ratKingRoom,
  }
  return images[customTileStem(texture)] ?? null
}

export function drawSheetTile(
  ctx: CanvasRenderingContext2D,
  image: HTMLImageElement,
  visual: number,
  cell: number,
  width: number,
  scale: number
) {
  const size = TILE_PX * scale
  const sheetColumns = image.naturalWidth / TILE_PX
  ctx.drawImage(
    image,
    (visual % sheetColumns) * TILE_PX,
    Math.floor(visual / sheetColumns) * TILE_PX,
    TILE_PX,
    TILE_PX,
    (cell % width) * size,
    Math.floor(cell / width) * size,
    size,
    size
  )
}

export function drawCustomTiles(
  ctx: CanvasRenderingContext2D,
  assets: MapAssets,
  map: FloorMap,
  layers: MapCustomTile[] | undefined,
  scale: number
) {
  for (const layer of layers ?? []) {
    const image = customTileImage(assets, layer.texture)
    if (!image || layer.width <= 0) continue
    for (let index = 0; index < layer.static_data.length; index++) {
      const visual = layer.static_data[index]
      if (visual == null || visual < 0) continue
      const x = layer.x + (index % layer.width)
      const y = layer.y + Math.floor(index / layer.width)
      if (x < 0 || y < 0 || x >= map.width || y >= map.height) continue
      const cell = y * map.width + x
      if (
        map.discoverable.length === map.tiles.length &&
        !map.discoverable[cell]
      )
        continue
      drawSheetTile(ctx, image, visual, cell, map.width, scale)
    }
  }
}

function insideMap(width: number, length: number, cell: number) {
  return (
    cell >= width &&
    cell < length - width &&
    cell % width !== 0 &&
    cell % width !== width - 1
  )
}

function wallAt(tiles: number[], cell: number) {
  if (cell < 0 || cell >= tiles.length) return false
  return wallStitchable(tiles[cell] ?? -1)
}

/** Pinned `WallOcclusionTilemap.getTileVisual`. Returns null when CLEARED/EMPTY. */
export function occlusionVisual(
  tiles: number[],
  width: number,
  cell: number,
  discoverable: boolean[]
): number | null {
  if (!insideMap(width, tiles.length, cell)) return null
  if (discoverable.length === tiles.length && !discoverable[cell]) return null

  const tile = tiles[cell] ?? Terrain.WALL
  if (doorTile(tile)) {
    return wallAt(tiles, cell - 1) && wallAt(tiles, cell + 1)
      ? DOOR_HORIZ
      : DOOR_VERT
  }
  if (wallStitchable(tile) || tile === Terrain.CHASM) return null

  let curr = CLEARED
  const above = wallAt(tiles, cell - width)
  if (above && tile !== Terrain.ALCHEMY) {
    curr += WALL_ABOVE
    if (wallAt(tiles, cell - 1)) curr += WALL_LEFT
    else if (wallAt(tiles, cell + width - 1)) curr += WALL_LEFT_BELOW
    if (wallAt(tiles, cell + 1)) curr += WALL_RIGHT
    else if (wallAt(tiles, cell + width + 1)) curr += WALL_RIGHT_BELOW
  } else {
    if (wallAt(tiles, cell - 1)) {
      curr += WALL_LEFT
    } else if (wallAt(tiles, cell + width - 1)) {
      curr += wallAt(tiles, cell - width - 1) ? WALL_LEFT_BOTH : WALL_LEFT_BELOW
    } else if (wallAt(tiles, cell - width - 1)) {
      curr += WALL_LEFT_ABOVE
    }
    if (wallAt(tiles, cell + 1)) {
      curr += WALL_RIGHT
    } else if (wallAt(tiles, cell + width + 1)) {
      curr += wallAt(tiles, cell - width + 1)
        ? WALL_RIGHT_BOTH
        : WALL_RIGHT_BELOW
    } else if (wallAt(tiles, cell - width + 1)) {
      curr += WALL_RIGHT_ABOVE
    }
  }
  return curr === CLEARED ? null : curr
}

export function drawWallOcclusion(
  ctx: CanvasRenderingContext2D,
  assets: MapAssets,
  map: FloorMap,
  scale: number
) {
  for (let cell = 0; cell < map.tiles.length; cell++) {
    const visual = occlusionVisual(map.tiles, map.width, cell, map.discoverable)
    if (visual == null) continue
    drawSheetTile(ctx, assets.occlusionShadows, visual, cell, map.width, scale)
  }
}
