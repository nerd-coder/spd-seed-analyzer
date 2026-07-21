/**
 * SPD terrain → tilesheet index mapping (flat visuals).
 * Tilesheets are 16×16 tiles of 16×16 px (see DungeonTileSheet / DungeonTilemap.SIZE).
 */

export const TILE_PX = 16
export const SHEET_COLS = 16

/** SPD Terrain constants (subset used by analyzer maps). */
export const Terrain = {
  CHASM: 0,
  EMPTY: 1,
  GRASS: 2,
  WALL: 4,
  DOOR: 5,
  OPEN_DOOR: 6,
  ENTRANCE: 7,
  EXIT: 8,
  LOCKED_DOOR: 10,
  WALL_DECO: 12,
  EMPTY_SP: 14,
  HIGH_GRASS: 15,
  SECRET_TRAP: 17,
  TRAP: 18,
  EMPTY_DECO: 20,
  WATER: 29,
} as const

/** 1-based (x,y) on the 16-wide sheet → linear index. */
function xy(x: number, y: number): number {
  return x - 1 + SHEET_COLS * (y - 1)
}

// Flat / simple visuals from DungeonTileSheet
const FLOOR = xy(1, 1) // 0
const FLOOR_SP = xy(1, 1) + 4 // 4
const FLOOR_DECO = xy(1, 1) + 1 // 1 — lightly decorated floor
const GRASS = xy(1, 1) + 2 // 2
const HIGH_GRASS = xy(1, 1) + 3 // 3
const ENTRANCE = xy(1, 1) + 16 // 16
const EXIT = xy(1, 1) + 17 // 17
const FLAT_WALL = xy(1, 4) // 48
const FLAT_WALL_DECO = xy(1, 4) + 1 // 49
const FLAT_DOOR = xy(1, 4) + 8 // 56
const FLAT_DOOR_OPEN = xy(1, 4) + 9 // 57
const FLAT_DOOR_LOCKED = xy(1, 4) + 10 // 58
const CHASM = xy(9, 2) // 24
// water uses animated sheets in-game; fall back to floor tint via grass-ish slot
const WATER_VIS = xy(1, 1) + 2
// traps: small mark on floor
const TRAP_VIS = xy(1, 1) + 5 // 5
const SECRET_TRAP_VIS = FLOOR // invisible until revealed

/** Map SPD Terrain id → tilesheet cell index. */
export function terrainToSheetIndex(terrain: number): number {
  switch (terrain) {
    case Terrain.CHASM:
      return CHASM
    case Terrain.EMPTY:
      return FLOOR
    case Terrain.EMPTY_DECO:
      return FLOOR_DECO
    case Terrain.GRASS:
      return GRASS
    case Terrain.HIGH_GRASS:
      return HIGH_GRASS
    case Terrain.WALL:
      return FLAT_WALL
    case Terrain.WALL_DECO:
      return FLAT_WALL_DECO
    case Terrain.DOOR:
      return FLAT_DOOR
    case Terrain.OPEN_DOOR:
      return FLAT_DOOR_OPEN
    case Terrain.ENTRANCE:
      return ENTRANCE
    case Terrain.EXIT:
      return EXIT
    case Terrain.LOCKED_DOOR:
      return FLAT_DOOR_LOCKED
    case Terrain.EMPTY_SP:
      return FLOOR_SP
    case Terrain.WATER:
      return WATER_VIS
    case Terrain.TRAP:
      return TRAP_VIS
    case Terrain.SECRET_TRAP:
      return SECRET_TRAP_VIS
    default:
      return FLOOR
  }
}

export function tilesetUrl(tileset: string): string {
  const key = ['sewers', 'prison', 'caves', 'city', 'halls'].includes(tileset)
    ? tileset
    : 'sewers'
  return `/assets/environment/tiles_${key}.png`
}

const imageCache = new Map<string, Promise<HTMLImageElement>>()

export function loadTileset(tileset: string): Promise<HTMLImageElement> {
  const url = tilesetUrl(tileset)
  let p = imageCache.get(url)
  if (!p) {
    p = new Promise((resolve, reject) => {
      const img = new Image()
      img.onload = () => resolve(img)
      img.onerror = () => reject(new Error(`Failed to load tileset: ${url}`))
      img.src = url
    })
    imageCache.set(url, p)
  }
  return p
}

export function drawFloorMap(
  ctx: CanvasRenderingContext2D,
  img: HTMLImageElement,
  width: number,
  height: number,
  tiles: number[],
  scale: number
): void {
  const s = TILE_PX * scale
  ctx.imageSmoothingEnabled = false
  ctx.clearRect(0, 0, width * s, height * s)

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const t = tiles[x + y * width] ?? Terrain.WALL
      const sheet = terrainToSheetIndex(t)
      const sx = (sheet % SHEET_COLS) * TILE_PX
      const sy = Math.floor(sheet / SHEET_COLS) * TILE_PX
      ctx.drawImage(img, sx, sy, TILE_PX, TILE_PX, x * s, y * s, s, s)
    }
  }
}
