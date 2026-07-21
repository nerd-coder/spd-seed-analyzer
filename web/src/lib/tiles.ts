import {
  featureVisual,
  lowerVisual,
  raisedTerrainVisual,
  SHEET_COLS,
  TILE_PX,
  wallVisual,
} from '@/lib/dungeon-tile-visuals'
import type { FloorMap, MapMarkerKind } from '@/lib/spd-wasm'

export { TILE_PX } from '@/lib/dungeon-tile-visuals'

export type MapAssets = {
  tiles: HTMLImageElement
  terrainFeatures: HTMLImageElement
  water: HTMLImageElement
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
  ]).then(([tiles, terrainFeatures, water]) => ({
    tiles,
    terrainFeatures,
    water,
  }))
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
  for (const marker of map.markers) {
    if (
      !visibility[marker.kind] ||
      marker.cell < 0 ||
      marker.cell >= map.tiles.length
    )
      continue
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
  for (let cell = 0; cell < map.tiles.length; cell++) {
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
    const visual = featureVisual(
      map.tiles[cell],
      map.tileset,
      variance[cell] ?? 0
    )
    if (visual != null) {
      drawSheetTile(ctx, assets.terrainFeatures, visual, cell, map.width, scale)
    }
  }
  for (let cell = 0; cell < map.tiles.length; cell++) {
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
  offset: number
) {
  const tileWidth = water.naturalWidth * scale
  const tileHeight = water.naturalHeight * scale
  const yOffset = (((offset * scale) % tileHeight) + tileHeight) % tileHeight
  for (let y = yOffset - tileHeight; y < height; y += tileHeight) {
    for (let x = 0; x < width; x += tileWidth) {
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
  elapsedSeconds: number
) {
  ctx.imageSmoothingEnabled = false
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height)
  drawWater(
    ctx,
    assets.water,
    ctx.canvas.width,
    ctx.canvas.height,
    scale,
    -5 * elapsedSeconds
  )
  ctx.drawImage(staticMap, 0, 0)
}
