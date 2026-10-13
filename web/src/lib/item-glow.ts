import { ItemEnhancement } from '@/lib/item-kind'

/**
 * Resting `ItemSprite.Glowing` colors for SPD v4.0.2 enchantments, glyphs, and curses.
 * Curses are black, which darkens the sprite instead of adding a hue.
 */
const ENHANCEMENT_COLOR: Record<ItemEnhancement, string> = {
  [ItemEnhancement.Affection]: '#ff4488',
  [ItemEnhancement.Annoying]: '#000000',
  [ItemEnhancement.AntiEntropy]: '#000000',
  [ItemEnhancement.AntiMagic]: '#88eeff',
  [ItemEnhancement.Blazing]: '#ff4400',
  [ItemEnhancement.Blocking]: '#0000ff',
  [ItemEnhancement.Blooming]: '#008800',
  [ItemEnhancement.Brimstone]: '#ff4400',
  [ItemEnhancement.Bulk]: '#000000',
  [ItemEnhancement.Camouflage]: '#448822',
  [ItemEnhancement.Chilling]: '#00ffff',
  [ItemEnhancement.Corrosion]: '#000000',
  [ItemEnhancement.Corrupting]: '#440066',
  [ItemEnhancement.Crystal]: '#0088ff',
  [ItemEnhancement.Dazzling]: '#000000',
  [ItemEnhancement.Displacement]: '#000000',
  [ItemEnhancement.Displacing]: '#000000',
  [ItemEnhancement.Eldritch]: '#222222',
  [ItemEnhancement.Elastic]: '#ff00ff',
  [ItemEnhancement.Entanglement]: '#663300',
  [ItemEnhancement.Explosive]: '#000000',
  [ItemEnhancement.Flow]: '#0000ff',
  [ItemEnhancement.Friendly]: '#000000',
  [ItemEnhancement.Grim]: '#000000',
  [ItemEnhancement.Kinetic]: '#ffff00',
  [ItemEnhancement.Lucky]: '#00ff00',
  [ItemEnhancement.Metabolism]: '#000000',
  [ItemEnhancement.Multiplicity]: '#000000',
  [ItemEnhancement.Obfuscation]: '#888888',
  [ItemEnhancement.Overgrowth]: '#000000',
  [ItemEnhancement.Polarized]: '#000000',
  [ItemEnhancement.Potential]: '#ffffff',
  [ItemEnhancement.Pressurized]: '#000000',
  [ItemEnhancement.Projecting]: '#8844cc',
  [ItemEnhancement.Repulsion]: '#ffffff',
  [ItemEnhancement.Sacrificial]: '#000000',
  [ItemEnhancement.Shocking]: '#ffffff',
  [ItemEnhancement.Stench]: '#000000',
  [ItemEnhancement.Stone]: '#222222',
  [ItemEnhancement.Swiftness]: '#ffff00',
  [ItemEnhancement.Thorns]: '#660022',
  [ItemEnhancement.Unstable]: '#999999',
  [ItemEnhancement.Vampiric]: '#660022',
  [ItemEnhancement.Venomous]: '#4400aa',
  [ItemEnhancement.Viscosity]: '#8844cc',
  [ItemEnhancement.Vorpal]: '#aa6666',
  [ItemEnhancement.Wayward]: '#000000',
  [ItemEnhancement.Wondrous]: '#000000',
}

/** CSS color for an enchantment, glyph, or curse. */
export function enhancementColor(
  enhancement: ItemEnhancement | null | undefined
): string | null {
  if (!enhancement) return null
  return ENHANCEMENT_COLOR[enhancement] ?? null
}

/** Weapon and armor curses. Grim glows black and is not a curse. */
const CURSE_ENHANCEMENTS = new Set<ItemEnhancement>([
  ItemEnhancement.Annoying,
  ItemEnhancement.AntiEntropy,
  ItemEnhancement.Bulk,
  ItemEnhancement.Corrosion,
  ItemEnhancement.Dazzling,
  ItemEnhancement.Displacement,
  ItemEnhancement.Displacing,
  ItemEnhancement.Explosive,
  ItemEnhancement.Friendly,
  ItemEnhancement.Metabolism,
  ItemEnhancement.Multiplicity,
  ItemEnhancement.Overgrowth,
  ItemEnhancement.Polarized,
  ItemEnhancement.Pressurized,
  ItemEnhancement.Sacrificial,
  ItemEnhancement.Stench,
  ItemEnhancement.Wayward,
  ItemEnhancement.Wondrous,
])

const CURSE_BORDER = '#d64545'

/** Red frame used for cursed equipment. */
export function curseBorderColor(): string {
  return CURSE_BORDER
}

function channel(hex: string, index: number): number {
  return Number.parseInt(hex.slice(index, index + 2), 16)
}

function mix(hex: string, toward: number, amount: number): string {
  const mixed = [1, 3, 5].map((index) => {
    const value = channel(hex, index)
    return Math.round(value + (toward - value) * amount)
      .toString(16)
      .padStart(2, '0')
  })
  return `#${mixed.join('')}`
}

function luminance(hex: string): number {
  return (
    (0.2126 * channel(hex, 1) +
      0.7152 * channel(hex, 3) +
      0.0722 * channel(hex, 5)) /
    255
  )
}

/**
 * Slot border for an enchantment, glyph, or curse.
 * Curses use red. Other colors follow the sprite glow, shifted when the
 * glow is too dark or too light to read on the slot.
 */
export function enhancementBorderColor(
  enhancement: ItemEnhancement | null | undefined
): string | null {
  if (!enhancement) return null
  if (CURSE_ENHANCEMENTS.has(enhancement)) return CURSE_BORDER
  const glow = ENHANCEMENT_COLOR[enhancement]
  if (!glow) return null
  const lum = luminance(glow)
  if (lum < 0.18) return mix(glow, 255, 0.72)
  if (lum > 0.85) return mix(glow, 0x22, 0.45)
  return glow
}
