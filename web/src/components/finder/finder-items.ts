import { CLASS_ICON, RING_CLASSES } from '@/lib/item-icons'

export type FinderItemOption = {
  className: string
  label: string
  group: FinderItemGroup
}

export type FinderItemGroup = 'Rings' | 'Wands' | 'Artifacts'

const GROUP_ORDER: FinderItemGroup[] = ['Rings', 'Wands', 'Artifacts']

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

function words(value: string): string {
  return value.replace(/([a-z])([A-Z])/g, '$1 $2')
}

export function finderItemLabel(className: string): string {
  if (className.startsWith('RingOf')) {
    return `Ring of ${words(className.slice('RingOf'.length))}`
  }
  if (className.startsWith('WandOf')) {
    return `Wand of ${words(className.slice('WandOf'.length))}`
  }
  return words(className)
}

function itemGroup(className: string): FinderItemGroup | null {
  if (RING_SET.has(className)) return 'Rings'
  if (className.startsWith('WandOf')) return 'Wands'
  if (ARTIFACTS.has(className)) return 'Artifacts'
  return null
}

const allOptions = Array.from(
  new Set<string>([...Object.keys(CLASS_ICON), ...RING_CLASSES])
)
  .map((className) => {
    const group = itemGroup(className)
    if (!group) return null
    return {
      className,
      label: finderItemLabel(className),
      group,
    }
  })
  .filter((item): item is FinderItemOption => item !== null)

export const FINDER_ITEM_GROUPS = GROUP_ORDER.map((label) => ({
  label,
  items: allOptions
    .filter((item) => item.group === label)
    .sort((a, b) => a.label.localeCompare(b.label)),
})).filter((group) => group.items.length > 0)

const UPGRADEABLE_GROUPS = new Set<FinderItemGroup>(['Rings', 'Wands'])

const ITEM_GROUP_BY_CLASS = new Map(
  allOptions.map((item) => [item.className, item.group])
)

export function isFinderItemUpgradeable(className: string): boolean {
  const group = ITEM_GROUP_BY_CLASS.get(className)
  return group !== undefined && UPGRADEABLE_GROUPS.has(group)
}

export const FINDER_GROUP_ORDER = GROUP_ORDER

export function toCoreCategory(group: FinderItemGroup): string {
  switch (group) {
    case 'Rings':
      return 'ring'
    case 'Wands':
      return 'wand'
    case 'Artifacts':
      return 'artifact'
  }
}

export function fromCoreCategory(core: string): FinderItemGroup {
  switch (core) {
    case 'wand':
      return 'Wands'
    case 'artifact':
      return 'Artifacts'
    default:
      return 'Rings'
  }
}

export function isFinderItemGroupUpgradeable(group: FinderItemGroup): boolean {
  return UPGRADEABLE_GROUPS.has(group)
}

export function itemsForGroup(group: FinderItemGroup) {
  return FINDER_ITEM_GROUPS.find((g) => g.label === group)?.items || []
}
