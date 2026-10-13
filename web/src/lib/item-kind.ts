import { type ItemClass, parseItemClass } from '@/lib/item-class'

/** SPD `ItemCategory`, serialized `snake_case` on the report. */
export const ItemCategory = {
  Weapon: 'weapon',
  Armor: 'armor',
  Missile: 'missile',
  Wand: 'wand',
  Ring: 'ring',
  Artifact: 'artifact',
  Potion: 'potion',
  Scroll: 'scroll',
  Stone: 'stone',
  Seed: 'seed',
  Food: 'food',
  Gold: 'gold',
  Trinket: 'trinket',
  Other: 'other',
} as const

export type ItemCategory = (typeof ItemCategory)[keyof typeof ItemCategory]

/** Weapon enchantment, armor glyph, or curse. Values match `glowing()` owners. */
export const ItemEnhancement = {
  Affection: 'Affection',
  Annoying: 'Annoying',
  AntiEntropy: 'AntiEntropy',
  AntiMagic: 'AntiMagic',
  Blazing: 'Blazing',
  Blocking: 'Blocking',
  Blooming: 'Blooming',
  Brimstone: 'Brimstone',
  Bulk: 'Bulk',
  Camouflage: 'Camouflage',
  Chilling: 'Chilling',
  Corrosion: 'Corrosion',
  Corrupting: 'Corrupting',
  Crystal: 'Crystal',
  Dazzling: 'Dazzling',
  Displacement: 'Displacement',
  Displacing: 'Displacing',
  Eldritch: 'Eldritch',
  Elastic: 'Elastic',
  Entanglement: 'Entanglement',
  Explosive: 'Explosive',
  Flow: 'Flow',
  Friendly: 'Friendly',
  Grim: 'Grim',
  Kinetic: 'Kinetic',
  Lucky: 'Lucky',
  Metabolism: 'Metabolism',
  Multiplicity: 'Multiplicity',
  Obfuscation: 'Obfuscation',
  Overgrowth: 'Overgrowth',
  Polarized: 'Polarized',
  Potential: 'Potential',
  Pressurized: 'Pressurized',
  Projecting: 'Projecting',
  Repulsion: 'Repulsion',
  Sacrificial: 'Sacrificial',
  Shocking: 'Shocking',
  Stench: 'Stench',
  Stone: 'Stone',
  Swiftness: 'Swiftness',
  Thorns: 'Thorns',
  Unstable: 'Unstable',
  Vampiric: 'Vampiric',
  Venomous: 'Venomous',
  Viscosity: 'Viscosity',
  Vorpal: 'Vorpal',
  Wayward: 'Wayward',
  Wondrous: 'Wondrous',
} as const

export type ItemEnhancement =
  (typeof ItemEnhancement)[keyof typeof ItemEnhancement]

/** Unidentified potion color, scroll rune, or ring gem. */
export const ItemAppearance = {
  Crimson: 'crimson',
  Amber: 'amber',
  Golden: 'golden',
  Jade: 'jade',
  Turquoise: 'turquoise',
  Azure: 'azure',
  Indigo: 'indigo',
  Magenta: 'magenta',
  Bistre: 'bistre',
  Charcoal: 'charcoal',
  Silver: 'silver',
  Ivory: 'ivory',

  Kaunan: 'KAUNAN',
  Sowilo: 'SOWILO',
  Laguz: 'LAGUZ',
  Yngvi: 'YNGVI',
  Gyfu: 'GYFU',
  Raido: 'RAIDO',
  Isaz: 'ISAZ',
  Mannaz: 'MANNAZ',
  Naudiz: 'NAUDIZ',
  Berkanan: 'BERKANAN',
  Odal: 'ODAL',
  Tiwaz: 'TIWAZ',

  Garnet: 'garnet',
  Ruby: 'ruby',
  Topaz: 'topaz',
  Emerald: 'emerald',
  Onyx: 'onyx',
  Opal: 'opal',
  Tourmaline: 'tourmaline',
  Sapphire: 'sapphire',
  Amethyst: 'amethyst',
  Quartz: 'quartz',
  Agate: 'agate',
  Diamond: 'diamond',
} as const

export type ItemAppearance =
  (typeof ItemAppearance)[keyof typeof ItemAppearance]

/** Which atlas a sprite frame comes from. */
export const ItemSheetKind = {
  Items: 'items',
  ItemIcons: 'item_icons',
} as const

export type ItemSheetKind = (typeof ItemSheetKind)[keyof typeof ItemSheetKind]

export type ItemSpriteSpec = {
  className?: ItemClass | null
  category?: ItemCategory | null
  appearance?: ItemAppearance | null
  /** Atlas index. Skips class resolution when set. */
  index?: number
  sheet?: ItemSheetKind
  sourceWidth?: number
  sourceHeight?: number
}

const CATEGORY_VALUES = new Set<string>(Object.values(ItemCategory))
const ENHANCEMENT_VALUES = new Set<string>(Object.values(ItemEnhancement))
const APPEARANCE_VALUES = new Set<string>(Object.values(ItemAppearance))

export function parseItemCategory(
  value: string | null | undefined
): ItemCategory | undefined {
  const key = value?.toLowerCase()
  if (key != null && CATEGORY_VALUES.has(key)) return key as ItemCategory
  return undefined
}

export function parseItemEnhancement(
  value: string | null | undefined
): ItemEnhancement | undefined {
  if (value != null && ENHANCEMENT_VALUES.has(value)) {
    return value as ItemEnhancement
  }
  return undefined
}

function humanizeClass(name: string): string {
  let out = ''
  for (let index = 0; index < name.length; index += 1) {
    const char = name.charAt(index)
    const upper = char.toLocaleUpperCase()
    if (index > 0 && char === upper && char !== char.toLocaleLowerCase()) {
      out += ' '
    }
    out += index === 0 ? char.toLocaleLowerCase() : char
  }
  return out
}

/**
 * Enchantment prefixed onto a report title (`displacement mail Armor +1`).
 * The structured enchantment field is a separate, sometimes conditional, roll.
 */
export function enhancementInItemName(
  label: string,
  className: string | null | undefined
): ItemEnhancement | undefined {
  if (!className) return undefined
  const item = humanizeClass(className)
  const head = (label.split(',')[0] ?? '').replace(/\s+[+-]\d+$/, '').trim()
  if (!head.endsWith(item) || head.length === item.length) return undefined
  const prefix = head.slice(0, head.length - item.length).trim()
  for (const value of ENHANCEMENT_VALUES) {
    if (humanizeClass(value) === prefix) return value as ItemEnhancement
  }
  return undefined
}

export function parseItemAppearance(
  value: string | null | undefined
): ItemAppearance | undefined {
  if (!value) return undefined
  if (APPEARANCE_VALUES.has(value)) return value as ItemAppearance
  const lower = value.toLowerCase()
  if (APPEARANCE_VALUES.has(lower)) return lower as ItemAppearance
  const upper = value.toUpperCase()
  if (APPEARANCE_VALUES.has(upper)) return upper as ItemAppearance
  return undefined
}

/** Build a sprite spec from report strings. Unknown names are dropped. */
export function itemSprite(input: {
  className?: string | null
  category?: string | null
  appearance?: string | null
  index?: number
  sheet?: ItemSheetKind
  sourceWidth?: number
  sourceHeight?: number
}): ItemSpriteSpec {
  return {
    className: parseItemClass(input.className),
    category: parseItemCategory(input.category),
    appearance: parseItemAppearance(input.appearance),
    index: input.index,
    sheet: input.sheet,
    sourceWidth: input.sourceWidth,
    sourceHeight: input.sourceHeight,
  }
}
