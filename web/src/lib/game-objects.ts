import { ALCHEMY_ICON, depthIconFrame, UI_ICONS } from '@/lib/ui-icons'

/** One cell on a sprite sheet, in source pixels. */
export type SpriteFrame = {
  x: number
  y: number
  w: number
  h: number
}

export type SpriteSheet = {
  url: string
  width: number
  height: number
}

/**
 * Idle clip copied from the object's `*Sprite` class, plus its examine key.
 * Repeated indices hold a pose.
 */
export type GameObjectClip = {
  sheet: SpriteSheet
  frames: readonly SpriteFrame[]
  fps: number
  label: string
  /** `gameObjectText` key. */
  textKey?: string
}

/**
 * TextureFilm index on a uniform grid. `cols` is integer division, matching
 * `TextureFilm(width, height)`.
 */
export function filmFrames(
  sheetWidth: number,
  frameWidth: number,
  frameHeight: number,
  indices: readonly number[]
): SpriteFrame[] {
  const cols = Math.floor(sheetWidth / frameWidth)
  return indices.map((index) => {
    const col = index % cols
    const row = Math.floor(index / cols)
    return {
      x: col * frameWidth,
      y: row * frameHeight,
      w: frameWidth,
      h: frameHeight,
    }
  })
}

function mob(
  file: string,
  [sheetWidth, sheetHeight]: readonly [number, number],
  [frameWidth, frameHeight]: readonly [number, number],
  indices: readonly number[],
  fps: number,
  label: string,
  textKey: string
): GameObjectClip {
  return {
    sheet: {
      url: `/assets/sprites/${file}`,
      width: sheetWidth,
      height: sheetHeight,
    },
    frames: filmFrames(sheetWidth, frameWidth, frameHeight, indices),
    fps,
    label,
    textKey,
  }
}

const ICONS: SpriteSheet = {
  url: UI_ICONS.url,
  width: UI_ICONS.sheetW,
  height: UI_ICONS.sheetH,
}

function feeling(key: string, textKey?: string): GameObjectClip {
  return {
    sheet: ICONS,
    frames: [depthIconFrame(key)],
    fps: 1,
    label: key === 'none' ? 'depth' : key,
    textKey,
  }
}

/** `ImpSprite` idle at 10 fps. */
const IMP_IDLE = [
  0, 1, 2, 3, 0, 1, 2, 3, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 1, 2, 3, 0,
  1, 2, 3, 0, 1, 3, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 4, 4, 4, 4, 4, 4,
  4, 4,
]

export const GAME_OBJECTS = {
  // Quest NPCs.
  Ghost: mob('ghost.png', [128, 16], [14, 15], [0, 1], 5, 'Sad Ghost', 'Ghost'),
  Wandmaker: mob(
    'wandmaker.png',
    [64, 16],
    [12, 14],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 3, 3, 3, 3, 2, 1],
    10,
    'Old Wandmaker',
    'Wandmaker'
  ),
  Blacksmith: mob(
    'blacksmith.png',
    [64, 16],
    [13, 16],
    [0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 3],
    15,
    'Troll Blacksmith',
    'Blacksmith'
  ),
  Imp: mob('imp.png', [64, 16], [12, 14], IMP_IDLE, 10, 'Imp', 'Imp'),

  // Quest targets.
  FetidRat: mob(
    'rat.png',
    [256, 64],
    [16, 15],
    [32, 32, 32, 33],
    2,
    'Fetid Rat',
    'FetidRat'
  ),
  GnollTrickster: mob(
    'gnoll.png',
    [256, 64],
    [12, 15],
    [42, 42, 42, 43, 42, 42, 43, 43],
    2,
    'Gnoll Trickster',
    'GnollTrickster'
  ),
  GreatCrab: mob(
    'crab.png',
    [256, 64],
    [16, 16],
    [32, 33, 32, 34],
    5,
    'Great Crab',
    'GreatCrab'
  ),
  /** The statue offset is not used here. */
  GnollGeomancer: mob(
    'gnoll_geomancer.png',
    [256, 32],
    [12, 16],
    [0, 0, 0, 1, 0, 0, 1, 1],
    2,
    'Gnoll Geomancer',
    'GnollGeomancer'
  ),
  /**
   * Blue full-health cell. The spire's color is `Random.Int(3)` at spawn and
   * is not on the quest report.
   */
  CrystalSpire: mob(
    'crystal_spire.png',
    [128, 128],
    [29, 40],
    [0],
    1,
    'Crystal Spire',
    'CrystalSpire'
  ),

  // Bosses.
  Goo: mob('goo.png', [256, 16], [20, 14], [2, 1, 0, 0, 1], 10, 'Goo', 'Goo'),
  Tengu: mob(
    'tengu.png',
    [256, 16],
    [14, 16],
    [0, 0, 0, 1],
    2,
    'Tengu',
    'Tengu'
  ),
  DM300: mob('dm300.png', [256, 64], [25, 22], [0, 1], 10, 'DM-300', 'DM300'),
  DwarfKing: mob(
    'king.png',
    [256, 16],
    [16, 16],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2],
    12,
    'King of Dwarves',
    'DwarfKing'
  ),
  YogDzewa: mob(
    'yog.png',
    [256, 32],
    [20, 19],
    [0, 1, 2, 2, 1, 0, 3, 4, 4, 3, 0, 5, 6, 6, 5],
    10,
    'Yog-Dzewa',
    'YogDzewa'
  ),

  // Interface marks.
  AlchemyPot: {
    sheet: ICONS,
    frames: [ALCHEMY_ICON],
    fps: 1,
    label: 'Alchemy pot',
    textKey: 'AlchemyPot',
  },
  FeelingNone: feeling('none'),
  FeelingChasm: feeling('chasm', 'FeelingChasm'),
  FeelingWater: feeling('water', 'FeelingWater'),
  FeelingGrass: feeling('grass', 'FeelingGrass'),
  FeelingDark: feeling('dark', 'FeelingDark'),
  FeelingLarge: feeling('large', 'FeelingLarge'),
  FeelingTraps: feeling('traps', 'FeelingTraps'),
  FeelingSecrets: feeling('secrets', 'FeelingSecrets'),
} satisfies Record<string, GameObjectClip>

export type GameObjectId = keyof typeof GAME_OBJECTS

/**
 * Bosses are fixed by depth in v4.0.2: `Dungeon.bossLevel` (Dungeon.java:441)
 * marks 5, 10, 15, 20 and 25, and each builds its region's boss level.
 */
const BOSS_OBJECTS: Record<number, GameObjectId> = {
  5: 'Goo',
  10: 'Tengu',
  15: 'DM300',
  20: 'DwarfKing',
  25: 'YogDzewa',
}

export function bossObjectId(depth: number): GameObjectId | undefined {
  return BOSS_OBJECTS[depth]
}

const FEELING_OBJECTS: Record<string, GameObjectId> = {
  chasm: 'FeelingChasm',
  water: 'FeelingWater',
  grass: 'FeelingGrass',
  dark: 'FeelingDark',
  large: 'FeelingLarge',
  traps: 'FeelingTraps',
  secrets: 'FeelingSecrets',
}

/** Level-feeling mark. `none` and unknown feelings draw the plain depth icon. */
export function feelingObjectId(
  feeling: string | null | undefined
): GameObjectId {
  return FEELING_OBJECTS[(feeling ?? '').toLowerCase()] ?? 'FeelingNone'
}
