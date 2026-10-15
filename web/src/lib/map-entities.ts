import { type ItemId, itemClassName, mobClassName } from '@/generated/game-ids'
import { SHEET_COLS, TILE_PX } from '@/lib/dungeon-tile-visuals'
import { ITEM_SHEET, resolveItemIconIndex } from '@/lib/item-icons'
import { itemSpriteSize } from '@/lib/item-sprite-rects'
import type { MapEntityAssets, MobAssetKey } from '@/lib/map-assets'
import type { FloorMap, IdentityMaps, MapHeap, MapMob } from '@/lib/spd-wasm'

type Visibility = { item: boolean; mob: boolean }

const CONTAINER_FRAMES: Record<string, number> = {
  chest: 36,
  locked_chest: 37,
  crystal_chest: 38,
  tomb: 34,
  skeleton: 32,
  remains: 33,
}

type MobFrame = {
  asset: MobAssetKey
  width: number
  height: number
  sourceX?: number
  sourceY?: number
}

/** Pinned v4.0.2 idle frames from the matching Sprite classes. */
const MOB_FRAMES: Record<string, MobFrame> = {
  Rat: { asset: 'rat', width: 16, height: 15 },
  Snake: { asset: 'snake', width: 12, height: 11 },
  Skeleton: { asset: 'skeleton', width: 12, height: 15 },
  Swarm: { asset: 'swarm', width: 16, height: 16 },
  Thief: { asset: 'thief', width: 12, height: 13 },
  // ShopkeeperSprite's idle animation starts on frame 1, then blinks to 0.
  Shopkeeper: {
    asset: 'shopkeeper',
    width: 14,
    height: 14,
    sourceX: 14,
  },
  DM100: { asset: 'dm100', width: 16, height: 14 },
  Guard: { asset: 'guard', width: 12, height: 16 },
  // SpectralNecromancerSprite starts at frame 16, the second atlas row.
  SpectralNecromancer: {
    asset: 'necromancer',
    width: 16,
    height: 16,
    sourceY: 16,
  },
}

function validCell(map: FloorMap, cell: number) {
  return cell >= 0 && cell < map.tiles.length
}

function isDiscoverable(map: FloorMap, cell: number) {
  return map.discoverable.length !== map.tiles.length || map.discoverable[cell]
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

export function drawVisibleTraps(
  ctx: CanvasRenderingContext2D,
  assets: MapEntityAssets,
  map: FloorMap,
  scale: number
) {
  for (const trap of map.traps) {
    if (!trap.visible || !validCell(map, trap.cell)) continue
    if (!isDiscoverable(map, trap.cell)) continue
    const color = trap.active ? trap.color : 8
    drawSheetTile(
      ctx,
      assets.terrainFeatures,
      color + trap.shape * SHEET_COLS,
      trap.cell,
      map.width,
      scale
    )
  }
}

function appearanceFor(item: ItemId, identities: IdentityMaps) {
  const className = itemClassName(item)
  if (className.startsWith('PotionOf')) {
    return identities.potions.find((entry) => entry.item === item)?.appearance
  }
  if (className.startsWith('ScrollOf')) {
    return identities.scrolls.find((entry) => entry.item === item)?.appearance
  }
  if (className.startsWith('RingOf')) {
    return identities.rings.find((entry) => entry.item === item)?.appearance
  }
  return undefined
}

function itemFrame(heap: MapHeap, identities: IdentityMaps) {
  const container = CONTAINER_FRAMES[heap.heap_type]
  if (container != null) {
    const [width, height] = itemSpriteSize(container)
    return [container, width, height] as const
  }
  const item = heap.items[0]
  if (!item) {
    const [width, height] = itemSpriteSize(0)
    return [0, width, height] as const
  }
  const index = resolveItemIconIndex(itemClassName(item.item), {
    appearance: appearanceFor(item.item, identities),
  })
  const [width, height] = itemSpriteSize(index)
  return [index, width, height] as const
}

function drawHeap(
  ctx: CanvasRenderingContext2D,
  image: HTMLImageElement,
  map: FloorMap,
  heap: MapHeap,
  identities: IdentityMaps,
  scale: number
) {
  const [index, width, height] = itemFrame(heap, identities)
  const cellX = heap.cell % map.width
  const cellY = Math.floor(heap.cell / map.width)
  const raise = height < 8 ? 13 - height : 5
  const x = cellX * TILE_PX + (TILE_PX - width) / 2
  const y = (cellY + 1) * TILE_PX - height - raise
  ctx.drawImage(
    image,
    (index % ITEM_SHEET.cols) * ITEM_SHEET.size,
    Math.floor(index / ITEM_SHEET.cols) * ITEM_SHEET.size,
    width,
    height,
    x * scale,
    y * scale,
    width * scale,
    height * scale
  )
}

function drawMob(
  ctx: CanvasRenderingContext2D,
  assets: MapEntityAssets,
  map: FloorMap,
  mob: MapMob,
  scale: number
) {
  const frame = MOB_FRAMES[mobClassName(mob.mob)]
  if (!frame) return false
  const image = assets[frame.asset]
  const cellX = mob.cell % map.width
  const cellY = Math.floor(mob.cell / map.width)
  const x = cellX * TILE_PX + (TILE_PX - frame.width) / 2
  const y = (cellY + 1) * TILE_PX - frame.height - 6
  ctx.drawImage(
    image,
    frame.sourceX ?? 0,
    frame.sourceY ?? 0,
    frame.width,
    frame.height,
    x * scale,
    y * scale,
    frame.width * scale,
    frame.height * scale
  )
  return true
}

export function drawKnownEntities(
  ctx: CanvasRenderingContext2D,
  assets: MapEntityAssets,
  map: FloorMap,
  identities: IdentityMaps,
  scale: number,
  visibility: Visibility
) {
  if (visibility.item) {
    for (const heap of map.heaps) {
      if (!validCell(map, heap.cell) || !isDiscoverable(map, heap.cell))
        continue
      drawHeap(ctx, assets.items, map, heap, identities, scale)
    }
  }
  if (visibility.mob) {
    for (const mob of map.mobs) {
      if (!validCell(map, mob.cell) || !isDiscoverable(map, mob.cell)) continue
      drawMob(ctx, assets, map, mob, scale)
    }
  }
}

export function exactEntityCells(map: FloorMap) {
  return {
    item: new Set(map.heaps.map((heap) => heap.cell)),
    mob: new Set(
      map.mobs
        .filter((mob) => MOB_FRAMES[mobClassName(mob.mob)] != null)
        .map((mob) => mob.cell)
    ),
  }
}
