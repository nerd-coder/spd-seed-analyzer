import {
  featureVisual,
  lowerVisual,
  raisedTerrainVisual,
  SHEET_COLS,
  TILE_PX,
  wallVisual,
} from '@/lib/dungeon-tile-visuals'
import {
  drawKnownEntities,
  drawVisibleTraps,
  exactEntityCells,
  type MapEntityAssets,
} from '@/lib/map-entities'
import type { FloorMap, IdentityMaps, MapMarkerKind } from '@/lib/spd-wasm'

export { TILE_PX } from '@/lib/dungeon-tile-visuals'

export type MapAssets = MapEntityAssets & {
  tiles: HTMLImageElement
  water: HTMLImageElement
}

export type MapViewport = {
  x: number
  y: number
  width: number
  height: number
}

type MarkerVisibility = Record<MapMarkerKind, boolean>

const imageCache = new Map<string, Promise<HTMLImageElement>>()

function loadImage(url: string): Promise<HTMLImageElement> {
  let promise = imageCache.get(url)
  if (!promise) {
    promise = new Promise((resolve, reject) => {
      const image = new Image()
      image.onload = () => resolve(image)
      image.onerror = () =>
        reject(new Error(`Failed to load map asset: ${url}`))
      image.src = url
    })
    imageCache.set(url, promise)
  }
  return promise
}

function regionIndex(tileset: string): number {
  const index = ['sewers', 'prison', 'caves', 'city', 'halls'].indexOf(tileset)
  return index < 0 ? 0 : index
}

export function loadMapAssets(tileset: string): Promise<MapAssets> {
  const region = regionIndex(tileset)
  const key = ['sewers', 'prison', 'caves', 'city', 'halls'][region]
  return Promise.all([
    loadImage(`/assets/environment/tiles_${key}.png`),
    loadImage('/assets/environment/terrain_features.png'),
    loadImage(`/assets/environment/water${region}.png`),
    loadImage('/assets/sprites/items.png'),
    loadImage('/assets/sprites/rat.png'),
    loadImage('/assets/sprites/snake.png'),
  ]).then(([tiles, terrainFeatures, water, items, rat, snake]) => ({
    tiles,
    terrainFeatures,
    water,
    items,
    rat,
    snake,
  }))
}

/** Tight deterministic viewport around cells retained by pinned `cleanWalls()`. */
export function mapViewport(map: FloorMap): MapViewport {
  if (map.discoverable.length !== map.tiles.length) {
    return { x: 0, y: 0, width: map.width, height: map.height }
  }
  let left = map.width
  let top = map.height
  let right = -1
  let bottom = -1
  for (let cell = 0; cell < map.discoverable.length; cell++) {
    if (!map.discoverable[cell]) continue
    const x = cell % map.width
    const y = Math.floor(cell / map.width)
    left = Math.min(left, x)
    top = Math.min(top, y)
    right = Math.max(right, x)
    bottom = Math.max(bottom, y)
  }
  if (right >= left) {
    // Raised sprites/walls occupy pixels above their owning cell. Preserve one
    // row of vertical overhang, matching GameScene's camera composition.
    top = Math.max(0, top - 1)
  }
  return right < left
    ? { x: 0, y: 0, width: map.width, height: map.height }
    : {
        x: left,
        y: top,
        width: right - left + 1,
        height: bottom - top + 1,
      }
}

function drawSheetTile(
  ctx: CanvasRenderingContext2D,
  image: HTMLImageElement,
  visual: number,
  cell: number,
  width: number,
  scale: number
) {
  const size = TILE_PX * scale
  ctx.drawImage(
    image,
    (visual % SHEET_COLS) * TILE_PX,
    Math.floor(visual / SHEET_COLS) * TILE_PX,
    TILE_PX,
    TILE_PX,
    (cell % width) * size,
    Math.floor(cell / width) * size,
    size,
    size
  )
}

function drawMarkers(
  ctx: CanvasRenderingContext2D,
  map: FloorMap,
  scale: number,
  visibility: MarkerVisibility
) {
  const size = TILE_PX * scale
  const radius = Math.max(2, 3 * scale)
  ctx.lineWidth = Math.max(1, scale)
  const exactCells = exactEntityCells(map)
  for (const marker of map.markers) {
    if (
      !visibility[marker.kind] ||
      marker.cell < 0 ||
      marker.cell >= map.tiles.length
    )
      continue
    if (exactCells[marker.kind].has(marker.cell)) continue
    const x = (marker.cell % map.width) * size + size / 2
    const y = Math.floor(marker.cell / map.width) * size + size / 2
    ctx.beginPath()
    if (marker.kind === 'item') {
      ctx.moveTo(x, y - radius)
      ctx.lineTo(x + radius, y)
      ctx.lineTo(x, y + radius)
      ctx.lineTo(x - radius, y)
      ctx.closePath()
      ctx.fillStyle = '#f5c451'
    } else {
      ctx.arc(x, y, radius, 0, Math.PI * 2)
      ctx.fillStyle = '#ef6b66'
    }
    ctx.fill()
    ctx.strokeStyle = '#171717'
    ctx.stroke()
  }
}

/** Build all non-water layers once; animated frames only composite this bitmap. */
export function renderStaticMap(
  assets: MapAssets,
  map: FloorMap,
  identities: IdentityMaps,
  scale: number,
  visibility: MarkerVisibility
): HTMLCanvasElement {
  const canvas = document.createElement('canvas')
  canvas.width = map.width * TILE_PX * scale
  canvas.height = map.height * TILE_PX * scale
  const ctx = canvas.getContext('2d')
  if (!ctx) return canvas
  ctx.imageSmoothingEnabled = false

  const variance = map.tile_variance ?? []
  const hasDiscoverability = map.discoverable.length === map.tiles.length
  for (let cell = 0; cell < map.tiles.length; cell++) {
    if (hasDiscoverability && !map.discoverable[cell]) {
      const size = TILE_PX * scale
      ctx.fillStyle = '#000'
      ctx.fillRect(
        (cell % map.width) * size,
        Math.floor(cell / map.width) * size,
        size,
        size
      )
      continue
    }
    const visual = lowerVisual(
      map.tiles,
      variance,
      map.width,
      map.tileset,
      cell
    )
    if (visual != null)
      drawSheetTile(ctx, assets.tiles, visual, cell, map.width, scale)
  }
  for (let cell = 0; cell < map.tiles.length; cell++) {
    if (hasDiscoverability && !map.discoverable[cell]) continue
    const visual = featureVisual(
      map.tiles[cell],
      map.tileset,
      variance[cell] ?? 0
    )
    if (visual != null) {
      drawSheetTile(ctx, assets.terrainFeatures, visual, cell, map.width, scale)
    }
  }
  drawVisibleTraps(ctx, assets, map, scale)
  drawKnownEntities(ctx, assets, map, identities, scale, visibility)
  for (let cell = 0; cell < map.tiles.length; cell++) {
    if (hasDiscoverability && !map.discoverable[cell]) continue
    const raised = raisedTerrainVisual(map.tiles[cell], variance, cell)
    if (raised != null)
      drawSheetTile(ctx, assets.tiles, raised, cell, map.width, scale)
    const wall = wallVisual(map.tiles, variance, map.width, cell)
    if (wall != null)
      drawSheetTile(ctx, assets.tiles, wall, cell, map.width, scale)
  }
  drawMarkers(ctx, map, scale, visibility)
  return canvas
}

function drawWater(
  ctx: CanvasRenderingContext2D,
  water: HTMLImageElement,
  width: number,
  height: number,
  scale: number,
  offset: number,
  originX: number,
  originY: number
) {
  const tileWidth = water.naturalWidth * scale
  const tileHeight = water.naturalHeight * scale
  const xOffset = -(((originX * scale) % tileWidth) + tileWidth) % tileWidth
  const yOffset =
    ((((offset - originY) * scale) % tileHeight) + tileHeight) % tileHeight
  for (let y = yOffset - tileHeight; y < height; y += tileHeight) {
    for (let x = xOffset; x < width; x += tileWidth) {
      ctx.drawImage(water, x, y, tileWidth, tileHeight)
    }
  }
}

/** Pinned GameScene water speed is 5 world pixels per second. */
export function drawFloorMap(
  ctx: CanvasRenderingContext2D,
  assets: MapAssets,
  staticMap: HTMLCanvasElement,
  scale: number,
  elapsedSeconds: number,
  viewport: MapViewport
) {
  ctx.imageSmoothingEnabled = false
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height)
  drawWater(
    ctx,
    assets.water,
    ctx.canvas.width,
    ctx.canvas.height,
    scale,
    -5 * elapsedSeconds,
    viewport.x * TILE_PX,
    viewport.y * TILE_PX
  )
  ctx.drawImage(
    staticMap,
    viewport.x * TILE_PX * scale,
    viewport.y * TILE_PX * scale,
    viewport.width * TILE_PX * scale,
    viewport.height * TILE_PX * scale,
    0,
    0,
    ctx.canvas.width,
    ctx.canvas.height
  )
}
