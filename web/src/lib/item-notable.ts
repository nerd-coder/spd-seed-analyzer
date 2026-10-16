import { type ItemId, itemClassName } from '@/generated/game-ids'
import { ItemCategory, parseItemCategory } from '@/lib/item-kind'

/** Notable at any upgrade level. */
const ALWAYS_CATEGORIES = new Set<ItemCategory>([
  ItemCategory.Wand,
  ItemCategory.Ring,
  ItemCategory.Artifact,
  ItemCategory.Trinket,
])

/** Notable only at +1 or better. */
const EQUIPMENT_CATEGORIES = new Set<ItemCategory>([
  ItemCategory.Weapon,
  ItemCategory.Armor,
  ItemCategory.Missile,
])

/**
 * Notable classes whose report category does not say so: progression
 * consumables, and artifacts or trinkets reported as `other`.
 */
const ALWAYS_CLASSES = new Set<string>([
  'PotionOfStrength',
  'PotionOfExperience',
  'ScrollOfUpgrade',
  'ScrollOfTransmutation',
  'StoneOfEnchantment',
  'Stylus',
  'AlchemistsToolkit',
  'CapeOfThorns',
  'ChaliceOfBlood',
  'CloakOfShadows',
  'DriedRose',
  'EtherealChains',
  'HolyTome',
  'HornOfPlenty',
  'LloydsBeacon',
  'MasterThievesArmband',
  'SandalsOfNature',
  'TalismanOfForesight',
  'TimekeepersHourglass',
  'UnstableSpellbook',
  'TrinketCatalyst',
  'ChaoticCenser',
  'CrackedSpyglass',
  'DimensionalSundial',
  'ExoticCrystals',
  'EyeOfNewt',
  'FerretTuft',
  'MimicTooth',
  'MossyClump',
  'ParchmentScrap',
  'PetrifiedSeed',
  'RatSkull',
  'SaltCube',
  'ShardOfOblivion',
  'ThirteenLeafClover',
  'TrapMechanism',
  'VialOfBlood',
  'WondrousResin',
])

function categories(category: string) {
  return category
    .split('/')
    .map((part) => parseItemCategory(part.trim()))
    .filter((part): part is ItemCategory => part != null)
}

function upgraded(item: {
  level?: number | null
  level_range?: { min: number; max: number } | null
}) {
  return Math.max(item.level ?? 0, item.level_range?.max ?? 0) >= 1
}

function notableCategory(category: string, isUpgraded: boolean) {
  const parts = categories(category)
  if (parts.length === 0) return false
  if (parts.every((part) => ALWAYS_CATEGORIES.has(part))) return true
  return (
    isUpgraded &&
    parts.every(
      (part) => ALWAYS_CATEGORIES.has(part) || EQUIPMENT_CATEGORIES.has(part)
    )
  )
}

function notableClass(item: ItemId) {
  return ALWAYS_CLASSES.has(itemClassName(item))
}

/**
 * Wands, rings, artifacts, trinkets, progression consumables (strength,
 * experience, upgrade, transmutation, enchantment, stylus), and weapons,
 * armor, or missiles at +1 or better.
 */
export function isNotableSpawn(item: {
  label?: string
  category: string
  item?: ItemId
  candidates?: ItemId[]
  level?: number | null
  level_range?: { min: number; max: number } | null
}) {
  const isUpgraded = upgraded(item)
  if (notableCategory(item.category, isUpgraded)) return true
  if (isUpgraded && item.label != null && /\bequipment\b/i.test(item.label)) {
    return true
  }
  if (item.item != null && notableClass(item.item)) return true
  return item.candidates?.some(notableClass) ?? false
}
