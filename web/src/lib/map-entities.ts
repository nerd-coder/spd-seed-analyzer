import { SHEET_COLS, TILE_PX } from '@/lib/dungeon-tile-visuals'
import { ITEM_SHEET, resolveItemIconIndex } from '@/lib/item-icons'
import type { FloorMap, IdentityMaps, MapHeap, MapMob } from '@/lib/spd-wasm'

export type MapEntityAssets = {
  terrainFeatures: HTMLImageElement
  items: HTMLImageElement
  rat: HTMLImageElement
  snake: HTMLImageElement
  skeleton: HTMLImageElement
  swarm: HTMLImageElement
  thief: HTMLImageElement
  shopkeeper: HTMLImageElement
}

type Visibility = { item: boolean; mob: boolean }

const CONTAINER_FRAMES: Record<string, [number, number, number]> = {
  chest: [36, 16, 14],
  locked_chest: [37, 16, 14],
  crystal_chest: [38, 16, 14],
  tomb: [34, 14, 15],
  skeleton: [32, 14, 11],
  remains: [33, 14, 11],
}

const ITEM_FRAME_SIZES: Record<string, [number, number]> = {
  Alchemize: [10, 15],
  Ankh: [10, 16],
  Bomb: [10, 13],
  CrystalKey: [8, 14],
  Dirk: [13, 14],
  Food: [16, 12],
  Gold: [15, 13],
  GuidePage: [10, 11],
  HandAxe: [12, 14],
  HealingDart: [15, 15],
  IronKey: [8, 14],
  LeatherArmor: [14, 13],
  MagicalHolster: [15, 16],
  MailArmor: [14, 12],
  Pasty: [16, 11],
  Scimitar: [13, 16],
  Shuriken: [12, 12],
  SmallRation: [14, 11],
  StoneOfIntuition: [14, 12],
}

type MobAsset = keyof Pick<
  MapEntityAssets,
  'rat' | 'snake' | 'skeleton' | 'swarm' | 'thief' | 'shopkeeper'
>

type MobFrame = {
  asset: MobAsset
  width: number
  height: number
  sourceX?: number
}

/** Pinned v3.3.8 idle frames from the matching Sprite classes. */
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

function appearanceFor(className: string, identities: IdentityMaps) {
  if (className.startsWith('PotionOf')) {
    return identities.potions.find((entry) => entry.item === className)
      ?.appearance
  }
  if (className.startsWith('ScrollOf')) {
    return identities.scrolls.find((entry) => entry.item === className)
      ?.appearance
  }
  if (className.startsWith('RingOf')) {
    return identities.rings.find((entry) => entry.item === className)
      ?.appearance
  }
  return undefined
}

function itemFrame(heap: MapHeap, identities: IdentityMaps) {
  const container = CONTAINER_FRAMES[heap.heap_type]
  if (container) return container
  const item = heap.items[0]
  if (!item) return [0, 8, 13] as const
  const index = resolveItemIconIndex(item.class, {
    appearance: appearanceFor(item.class, identities),
  })
  const [width, height] = item.class.startsWith('PotionOf')
    ? [12, 14]
    : item.class.startsWith('ScrollOf')
      ? [15, 14]
      : item.class.startsWith('RingOf')
        ? [8, 10]
        : item.class.startsWith('WandOf')
          ? [14, 14]
          : item.class.startsWith('StoneOf')
            ? [14, 12]
            : (ITEM_FRAME_SIZES[item.class] ?? [16, 16])
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
  const frame = MOB_FRAMES[mob.class]
  if (!frame) return false
  const image = assets[frame.asset]
  const cellX = mob.cell % map.width
  const cellY = Math.floor(mob.cell / map.width)
  const x = cellX * TILE_PX + (TILE_PX - frame.width) / 2
  const y = (cellY + 1) * TILE_PX - frame.height - 6
  ctx.drawImage(
    image,
    frame.sourceX ?? 0,
    0,
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
        .filter((mob) => MOB_FRAMES[mob.class] != null)
        .map((mob) => mob.cell)
    ),
  }
}
