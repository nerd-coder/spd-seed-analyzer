/**
 * Illustrative floor data for the preview. Not output of the analyzer:
 * the preview never runs the engine.
 */
import type { SpriteName } from '@/lib/sprites'

export const SAMPLE_SEED = 'ABC-DEF-GHI'

export type SampleItem = {
  sprite: SpriteName
  name: string
  level?: number
  quantity?: number
  cursed?: boolean
  /** Enchantment or glyph glow, from the game's `ItemSprite.Glowing`. */
  glow?: string
}

export type SampleQuest = {
  giver: SpriteName
  name: string
  rewards: SampleItem[]
}

export type SampleFloor = {
  depth: number
  quest?: SampleQuest
  notable: SampleItem[]
  other: SampleItem[]
}

const potion = (sprite: SpriteName, name: string): SampleItem => ({
  sprite,
  name,
})

const OTHER: SampleItem[] = [
  { sprite: 'gold', name: 'Gold', quantity: 41 },
  potion('potionAmber', 'Amber potion'),
  potion('scrollRaido', 'Scroll with the rune Raido'),
  { sprite: 'pasty', name: 'Pasty' },
  { sprite: 'seedSungrass', name: 'Sungrass seed' },
]

const FLOORS: Record<number, Omit<SampleFloor, 'depth'>> = {
  1: {
    notable: [{ sprite: 'wandLightning', name: 'Wand of Lightning', level: 2 }],
    other: OTHER,
  },
  2: {
    notable: [{ sprite: 'ringGarnet', name: 'Garnet ring', level: 1 }],
    other: OTHER.slice(0, 4),
  },
  3: {
    quest: {
      giver: 'ghost',
      name: 'Sad Ghost',
      rewards: [
        { sprite: 'runicBlade', name: 'Runic Blade', level: 2 },
        { sprite: 'scaleArmor', name: 'Scale Armor', level: 2 },
      ],
    },
    notable: [],
    other: OTHER.slice(1, 4),
  },
  4: {
    notable: [
      {
        sprite: 'longsword',
        name: 'Longsword of Blazing',
        level: 3,
        glow: '#ff4400',
      },
      { sprite: 'hornOfPlenty', name: 'Horn of Plenty' },
    ],
    other: OTHER,
  },
  5: { notable: [], other: OTHER.slice(0, 2) },
  6: {
    notable: [
      { sprite: 'mailArmor', name: 'Mail Armor', level: -1, cursed: true },
    ],
    other: OTHER,
  },
  8: {
    quest: {
      giver: 'wandmaker',
      name: 'Old Wandmaker',
      rewards: [
        { sprite: 'wandFrost', name: 'Wand of Frost', level: 1 },
        { sprite: 'wandCorruption', name: 'Wand of Corruption', level: 1 },
      ],
    },
    notable: [],
    other: OTHER.slice(0, 3),
  },
  9: {
    notable: [{ sprite: 'ringSapphire', name: 'Sapphire ring', level: 4 }],
    other: OTHER,
  },
  13: {
    quest: {
      giver: 'blacksmith',
      name: 'Troll Blacksmith',
      rewards: [{ sprite: 'warHammer', name: 'War Hammer', level: 2 }],
    },
    notable: [{ sprite: 'cloakOfShadows', name: 'Cloak of Shadows' }],
    other: OTHER,
  },
  17: {
    quest: {
      giver: 'imp',
      name: 'Ambitious Imp',
      rewards: [{ sprite: 'ringEmerald', name: 'Emerald ring', level: 3 }],
    },
    notable: [],
    other: OTHER.slice(0, 4),
  },
  22: {
    notable: [{ sprite: 'chaliceOfBlood', name: 'Chalice of Blood' }],
    other: OTHER,
  },
}

export const SAMPLE_FLOORS: SampleFloor[] = Array.from(
  { length: 26 },
  (_, i) => {
    const depth = i + 1
    const floor = FLOORS[depth] ?? { notable: [], other: OTHER.slice(0, 3) }
    return { depth, ...floor }
  }
)

export function sampleFloor(depth: number): SampleFloor {
  return SAMPLE_FLOORS[depth - 1] ?? (SAMPLE_FLOORS[0] as SampleFloor)
}
