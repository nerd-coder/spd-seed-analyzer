import {
  CLASS_ICON,
  POTION_CLASSES,
  RING_CLASSES,
  SCROLL_CLASSES,
} from '@/lib/item-icons'

export type FinderItemOption = {
  className: string
  label: string
  group: FinderItemGroup
}

export type FinderItemGroup =
  | 'Potions'
  | 'Scrolls'
  | 'Rings'
  | 'Weapons'
  | 'Armor'
  | 'Missiles & darts'
  | 'Wands'
  | 'Artifacts'
  | 'Trinkets'
  | 'Seeds'
  | 'Runestones'
  | 'Supplies & other'

const GROUP_ORDER: FinderItemGroup[] = [
  'Potions',
  'Scrolls',
  'Rings',
  'Weapons',
  'Armor',
  'Missiles & darts',
  'Wands',
  'Artifacts',
  'Trinkets',
  'Seeds',
  'Runestones',
  'Supplies & other',
]

const POTION_SET = new Set<string>(POTION_CLASSES)
const SCROLL_SET = new Set<string>(SCROLL_CLASSES)
const RING_SET = new Set<string>(RING_CLASSES)

const ARTIFACTS = new Set([
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
])

const TRINKETS = new Set([
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

function words(value: string): string {
  return value.replace(/([a-z])([A-Z])/g, '$1 $2')
}

export function finderItemLabel(className: string): string {
  if (className.startsWith('PotionOf')) {
    return `Potion of ${words(className.slice('PotionOf'.length))}`
  }
  if (className.startsWith('ScrollOf')) {
    return `Scroll of ${words(className.slice('ScrollOf'.length))}`
  }
  if (className.startsWith('RingOf')) {
    return `Ring of ${words(className.slice('RingOf'.length))}`
  }
  return words(className)
}

function itemGroup(className: string): FinderItemGroup {
  if (POTION_SET.has(className)) return 'Potions'
  if (SCROLL_SET.has(className)) return 'Scrolls'
  if (RING_SET.has(className)) return 'Rings'
  if (className.endsWith('Armor')) return 'Armor'
  if (className.startsWith('WandOf')) return 'Wands'
  if (className.endsWith('Seed')) return 'Seeds'
  if (className.startsWith('StoneOf')) return 'Runestones'
  if (ARTIFACTS.has(className)) return 'Artifacts'
  if (TRINKETS.has(className)) return 'Trinkets'

  const icon = CLASS_ICON[className]
  if (icon >= 96 && icon <= 134) return 'Weapons'
  if (icon >= 145 && icon <= 172) return 'Missiles & darts'
  return 'Supplies & other'
}

const allOptions = Array.from(
  new Set<string>([
    ...Object.keys(CLASS_ICON),
    ...POTION_CLASSES,
    ...SCROLL_CLASSES,
    ...RING_CLASSES,
  ])
).map((className) => ({
  className,
  label: finderItemLabel(className),
  group: itemGroup(className),
}))

export const FINDER_ITEM_GROUPS = GROUP_ORDER.map((label) => ({
  label,
  items: allOptions
    .filter((item) => item.group === label)
    .sort((a, b) => a.label.localeCompare(b.label)),
})).filter((group) => group.items.length > 0)
