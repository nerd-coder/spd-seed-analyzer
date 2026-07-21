/** Pinned SPD v3.3.8 `DungeonTileSheet` / tilemap visual selection. */

export const TILE_PX = 16
export const SHEET_COLS = 16

export const Terrain = {
  CHASM: 0,
  EMPTY: 1,
  GRASS: 2,
  EMPTY_WELL: 3,
  WALL: 4,
  DOOR: 5,
  OPEN_DOOR: 6,
  ENTRANCE: 7,
  EXIT: 8,
  EMBERS: 9,
  LOCKED_DOOR: 10,
  PEDESTAL: 11,
  WALL_DECO: 12,
  BARRICADE: 13,
  EMPTY_SP: 14,
  HIGH_GRASS: 15,
  SECRET_DOOR: 16,
  SECRET_TRAP: 17,
  TRAP: 18,
  INACTIVE_TRAP: 19,
  EMPTY_DECO: 20,
  LOCKED_EXIT: 21,
  UNLOCKED_EXIT: 22,
  CUSTOM_DECO: 23,
  WELL: 24,
  STATUE: 25,
  STATUE_SP: 26,
  BOOKSHELF: 27,
  ALCHEMY: 28,
  WATER: 29,
  FURROWED_GRASS: 30,
  CRYSTAL_DOOR: 31,
  CUSTOM_DECO_EMPTY: 32,
  REGION_DECO: 33,
  REGION_DECO_ALT: 34,
  MINE_CRYSTAL: 35,
  MINE_BOULDER: 36,
  ENTRANCE_SP: 37,
  HERO_LKD_DR: 38,
} as const

const xy = (x: number, y: number) => x - 1 + SHEET_COLS * (y - 1)

const FLOOR = xy(1, 1)
const CHASM = xy(9, 2)
const WATER = xy(1, 3)
const RAISED_WALL = xy(1, 6)
const RAISED_WALL_DECO = RAISED_WALL + 4
const RAISED_WALL_DOOR = RAISED_WALL + 8
const RAISED_WALL_BOOKSHELF = RAISED_WALL + 12
const RAISED_DOOR = xy(1, 8)
const RAISED_OTHER = xy(9, 8)
const WALL_INTERNAL = xy(1, 10)
const WALL_OVERHANG = xy(1, 13)
const DOOR_OVERHANG = xy(1, 15)
const OTHER_OVERHANG = xy(9, 15)

const direct = new Map<number, number>([
  [Terrain.EMPTY, FLOOR],
  [Terrain.GRASS, FLOOR + 2],
  [Terrain.EMPTY_WELL, FLOOR + 19],
  [Terrain.ENTRANCE, FLOOR + 16],
  [Terrain.EXIT, FLOOR + 17],
  [Terrain.EMBERS, FLOOR + 3],
  [Terrain.PEDESTAL, FLOOR + 20],
  [Terrain.EMPTY_SP, FLOOR + 4],
  [Terrain.ENTRANCE_SP, FLOOR + 22],
  [Terrain.SECRET_TRAP, FLOOR],
  [Terrain.TRAP, FLOOR],
  [Terrain.INACTIVE_TRAP, FLOOR],
  [Terrain.CUSTOM_DECO, FLOOR],
  [Terrain.CUSTOM_DECO_EMPTY, FLOOR],
  [Terrain.EMPTY_DECO, FLOOR + 1],
  [Terrain.LOCKED_EXIT, xy(1, 4) + 13],
  [Terrain.UNLOCKED_EXIT, xy(1, 4) + 12],
  [Terrain.WELL, FLOOR + 18],
])

const commonAlts = new Map<number, number>([
  [FLOOR, FLOOR + 6],
  [FLOOR + 2, FLOOR + 8],
  [FLOOR + 3, FLOOR + 9],
  [FLOOR + 4, FLOOR + 10],
  [FLOOR + 1, FLOOR + 7],
  [RAISED_WALL, RAISED_WALL + 16],
  [RAISED_WALL_DECO, RAISED_WALL_DECO + 16],
  [RAISED_WALL_BOOKSHELF, RAISED_WALL_BOOKSHELF + 16],
  [RAISED_OTHER + 2, RAISED_OTHER + 5],
  [RAISED_OTHER + 3, RAISED_OTHER + 6],
  [RAISED_OTHER + 12, RAISED_OTHER + 13],
  [OTHER_OVERHANG + 2, OTHER_OVERHANG + 5],
  [OTHER_OVERHANG + 3, OTHER_OVERHANG + 6],
  [OTHER_OVERHANG + 18, OTHER_OVERHANG + 21],
  [OTHER_OVERHANG + 19, OTHER_OVERHANG + 22],
  [OTHER_OVERHANG + 12, OTHER_OVERHANG + 13],
])

const rareAlts = new Map<number, number>([
  [FLOOR, FLOOR + 12],
  [RAISED_OTHER + 12, RAISED_OTHER + 14],
  [OTHER_OVERHANG + 12, OTHER_OVERHANG + 14],
])

const waterStitchable = new Set<number>([
  Terrain.EMPTY,
  Terrain.GRASS,
  Terrain.EMPTY_WELL,
  Terrain.ENTRANCE,
  Terrain.EXIT,
  Terrain.EMBERS,
  Terrain.BARRICADE,
  Terrain.HIGH_GRASS,
  Terrain.FURROWED_GRASS,
  Terrain.SECRET_TRAP,
  Terrain.TRAP,
  Terrain.INACTIVE_TRAP,
  Terrain.EMPTY_DECO,
  Terrain.CUSTOM_DECO,
  Terrain.WELL,
  Terrain.STATUE,
  Terrain.REGION_DECO,
  Terrain.ALCHEMY,
  Terrain.CUSTOM_DECO_EMPTY,
  Terrain.MINE_CRYSTAL,
  Terrain.MINE_BOULDER,
  Terrain.DOOR,
  Terrain.OPEN_DOOR,
  Terrain.LOCKED_DOOR,
  Terrain.HERO_LKD_DR,
  Terrain.CRYSTAL_DOOR,
])

const chasmFloorStitchable = new Set<number>([
  Terrain.EMPTY,
  Terrain.GRASS,
  Terrain.EMBERS,
  Terrain.EMPTY_WELL,
  Terrain.HIGH_GRASS,
  Terrain.FURROWED_GRASS,
  Terrain.EMPTY_DECO,
  Terrain.CUSTOM_DECO,
  Terrain.WELL,
  Terrain.STATUE,
  Terrain.REGION_DECO,
  Terrain.SECRET_TRAP,
  Terrain.INACTIVE_TRAP,
  Terrain.TRAP,
  Terrain.BOOKSHELF,
  Terrain.BARRICADE,
  Terrain.PEDESTAL,
  Terrain.CUSTOM_DECO_EMPTY,
  Terrain.MINE_BOULDER,
  Terrain.MINE_CRYSTAL,
])

const chasmWallStitchable = new Set<number>([
  Terrain.WALL,
  Terrain.DOOR,
  Terrain.OPEN_DOOR,
  Terrain.LOCKED_DOOR,
  Terrain.HERO_LKD_DR,
  Terrain.SECRET_DOOR,
  Terrain.WALL_DECO,
])

export const wallStitchable = (tile: number) =>
  tile === -1 ||
  tile === Terrain.WALL ||
  tile === Terrain.WALL_DECO ||
  tile === Terrain.SECRET_DOOR ||
  tile === Terrain.LOCKED_EXIT ||
  tile === Terrain.UNLOCKED_EXIT ||
  tile === Terrain.BOOKSHELF

const doorTile = (tile: number) =>
  tile === Terrain.DOOR ||
  tile === Terrain.LOCKED_DOOR ||
  tile === Terrain.HERO_LKD_DR ||
  tile === Terrain.CRYSTAL_DOOR ||
  tile === Terrain.OPEN_DOOR

const tileAt = (
  tiles: number[],
  width: number,
  cell: number,
  dx: number,
  dy: number
) => {
  const x = (cell % width) + dx
  const y = Math.floor(cell / width) + dy
  if (x < 0 || x >= width || y < 0 || y >= Math.ceil(tiles.length / width))
    return -1
  return tiles[x + y * width] ?? -1
}

const withAlt = (visual: number, cell: number, variance: number[]) => {
  const value = variance[cell] ?? 0
  if (value >= 95)
    return rareAlts.get(visual) ?? commonAlts.get(visual) ?? visual
  if (value >= 50) return commonAlts.get(visual) ?? visual
  return visual
}

const waterCanStitch = (tile: number, tileset: string) =>
  waterStitchable.has(tile) ||
  (tile === Terrain.REGION_DECO_ALT && tileset === 'halls')

export function lowerVisual(
  tiles: number[],
  variance: number[],
  width: number,
  tileset: string,
  cell: number
): number | null {
  const tile = tiles[cell] ?? Terrain.WALL
  const directVisual = direct.get(tile)
  if (directVisual != null) return withAlt(directVisual, cell, variance)

  if (tile === Terrain.WATER) {
    let visual = WATER
    if (waterCanStitch(tileAt(tiles, width, cell, 0, -1), tileset)) visual += 1
    if (waterCanStitch(tileAt(tiles, width, cell, 1, 0), tileset)) visual += 2
    if (waterCanStitch(tileAt(tiles, width, cell, 0, 1), tileset)) visual += 4
    if (waterCanStitch(tileAt(tiles, width, cell, -1, 0), tileset)) visual += 8
    return visual === WATER ? null : visual
  }
  if (tile === Terrain.CHASM) {
    const above = cell > width ? tileAt(tiles, width, cell, 0, -1) : -1
    if (above === Terrain.EMPTY_SP || above === Terrain.STATUE_SP)
      return CHASM + 2
    if (above === Terrain.WATER) return CHASM + 4
    if (chasmWallStitchable.has(above)) return CHASM + 3
    if (above === Terrain.REGION_DECO_ALT) {
      return tileset === 'prison'
        ? CHASM
        : tileset === 'halls'
          ? CHASM + 1
          : CHASM + 2
    }
    return chasmFloorStitchable.has(above) ? CHASM + 1 : CHASM
  }
  if (doorTile(tile)) {
    if (wallStitchable(tileAt(tiles, width, cell, 0, -1)))
      return RAISED_DOOR + 4
    if (tile === Terrain.DOOR) return RAISED_DOOR
    if (tile === Terrain.OPEN_DOOR) return RAISED_DOOR + 1
    if (tile === Terrain.LOCKED_DOOR || tile === Terrain.HERO_LKD_DR)
      return RAISED_DOOR + 2
    return RAISED_DOOR + 3
  }
  if (wallStitchable(tile)) {
    const below = tileAt(tiles, width, cell, 0, 1)
    if (wallStitchable(below)) return null
    let visual = doorTile(below)
      ? RAISED_WALL_DOOR
      : tile === Terrain.WALL || tile === Terrain.SECRET_DOOR
        ? RAISED_WALL
        : tile === Terrain.WALL_DECO
          ? RAISED_WALL_DECO
          : tile === Terrain.BOOKSHELF
            ? RAISED_WALL_BOOKSHELF
            : -1
    if (visual < 0) return null
    visual = withAlt(visual, cell, variance)
    if (!wallStitchable(tileAt(tiles, width, cell, 1, 0))) visual += 1
    if (!wallStitchable(tileAt(tiles, width, cell, -1, 0))) visual += 2
    return visual
  }
  if (tile === Terrain.STATUE) return RAISED_OTHER + 8
  if (tile === Terrain.STATUE_SP) return RAISED_OTHER + 9
  if (tile === Terrain.REGION_DECO) return RAISED_OTHER + 10
  if (tile === Terrain.REGION_DECO_ALT) return RAISED_OTHER + 11
  if (tile === Terrain.MINE_CRYSTAL || tile === Terrain.MINE_BOULDER)
    return withAlt(RAISED_OTHER + 12, cell, variance)
  if (tile === Terrain.ALCHEMY) return RAISED_OTHER
  if (tile === Terrain.BARRICADE) return RAISED_OTHER + 1
  if (tile === Terrain.HIGH_GRASS)
    return withAlt(RAISED_OTHER + 2, cell, variance)
  if (tile === Terrain.FURROWED_GRASS)
    return withAlt(RAISED_OTHER + 3, cell, variance)
  return null
}

export function raisedTerrainVisual(
  tile: number,
  variance: number[],
  cell: number
) {
  if (tile === Terrain.HIGH_GRASS)
    return withAlt(OTHER_OVERHANG + 18, cell, variance)
  if (tile === Terrain.FURROWED_GRASS)
    return withAlt(OTHER_OVERHANG + 19, cell, variance)
  return null
}

export function wallVisual(
  tiles: number[],
  variance: number[],
  width: number,
  cell: number
): number | null {
  const tile = tiles[cell] ?? Terrain.WALL
  const below = tileAt(tiles, width, cell, 0, 1)
  if (wallStitchable(tile)) {
    if (below !== -1 && !wallStitchable(below)) {
      if (below === Terrain.DOOR) return DOOR_OVERHANG + 3
      if (below === Terrain.LOCKED_DOOR || below === Terrain.HERO_LKD_DR) {
        return DOOR_OVERHANG + 4
      }
      if (below === Terrain.CRYSTAL_DOOR) return DOOR_OVERHANG + 5
      if (below === Terrain.OPEN_DOOR) return null
    } else {
      let visual =
        tile === Terrain.BOOKSHELF || below === Terrain.BOOKSHELF
          ? WALL_INTERNAL + 32
          : WALL_INTERNAL
      if (!wallStitchable(tileAt(tiles, width, cell, 1, 0))) visual += 1
      if (!wallStitchable(tileAt(tiles, width, cell, 1, 1))) visual += 2
      if (!wallStitchable(tileAt(tiles, width, cell, -1, 1))) visual += 4
      if (!wallStitchable(tileAt(tiles, width, cell, -1, 0))) visual += 8
      return visual
    }
  }

  if (tile === Terrain.LOCKED_EXIT || tile === Terrain.UNLOCKED_EXIT)
    return DOOR_OVERHANG + 6
  if (below !== -1 && wallStitchable(below)) {
    let visual = below === Terrain.BOOKSHELF ? WALL_OVERHANG + 8 : WALL_OVERHANG
    if (!wallStitchable(tileAt(tiles, width, cell, 1, 1))) visual += 1
    if (!wallStitchable(tileAt(tiles, width, cell, -1, 1))) visual += 2
    return visual
  }
  if (
    below === Terrain.DOOR ||
    below === Terrain.LOCKED_DOOR ||
    below === Terrain.HERO_LKD_DR
  ) {
    return DOOR_OVERHANG
  }
  if (below === Terrain.OPEN_DOOR) return DOOR_OVERHANG + 1
  if (below === Terrain.CRYSTAL_DOOR) return DOOR_OVERHANG + 2
  if (below === Terrain.STATUE) return OTHER_OVERHANG + 8
  if (below === Terrain.STATUE_SP) return OTHER_OVERHANG + 9
  if (below === Terrain.REGION_DECO) return OTHER_OVERHANG + 10
  if (below === Terrain.REGION_DECO_ALT) return OTHER_OVERHANG + 11
  if (below === Terrain.MINE_CRYSTAL || below === Terrain.MINE_BOULDER)
    return withAlt(OTHER_OVERHANG + 12, cell + width, variance)
  if (below === Terrain.ALCHEMY) return OTHER_OVERHANG
  if (below === Terrain.BARRICADE) return OTHER_OVERHANG + 1
  if (below === Terrain.HIGH_GRASS)
    return withAlt(OTHER_OVERHANG + 2, cell + width, variance)
  if (below === Terrain.FURROWED_GRASS)
    return withAlt(OTHER_OVERHANG + 3, cell + width, variance)
  return null
}

export function featureVisual(tile: number, tileset: string, variance: number) {
  const stage = ['sewers', 'prison', 'caves', 'city', 'halls'].indexOf(tileset)
  const alt = variance >= 50 ? 1 : 0
  if (tile === Terrain.HIGH_GRASS) return 9 + 16 * Math.max(0, stage) + alt
  if (tile === Terrain.FURROWED_GRASS) return 11 + 16 * Math.max(0, stage) + alt
  if (tile === Terrain.GRASS) return 13 + 16 * Math.max(0, stage) + alt
  if (tile === Terrain.EMBERS) return 9 + 16 * 5 + alt
  return null
}
