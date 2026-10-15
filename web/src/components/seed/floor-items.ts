/**
 * Pure grouping of a floor's items: what the report lists, and which of it
 * is notable. Shared by the floor item sections and the floor strip.
 */
import { isNotableSpawn } from '@/lib/item-notable'
import type { FloorReport, ItemGroup, ItemVariant } from '@/lib/spd-wasm'

function splitNotableItems(items: ItemGroup[]) {
  const notable: ItemGroup[] = []
  const other: ItemGroup[] = []
  for (const group of items) {
    const notableVariants = group.variants.filter(isNotableSpawn)
    const otherVariants = group.variants.filter((item) => !isNotableSpawn(item))
    if (notableVariants.length > 0) {
      notable.push({ ...group, variants: notableVariants })
    }
    if (otherVariants.length > 0) {
      other.push({ ...group, variants: otherVariants })
    }
  }
  return { notable, other }
}

function itemSection(
  item: ItemGroup
): 'shop' | 'quest' | 'guaranteed' | 'loot' | 'general' {
  const source = item.source ?? ''
  if (source.includes('ShopRoom')) return 'shop'
  if (
    /Ghost\.Quest|Wandmaker\.Quest|Blacksmith\.Quest|Imp\.Quest/.test(source)
  ) {
    return 'quest'
  }
  if (source === 'guaranteed floor spawn' || source === 'forced') {
    return 'guaranteed'
  }
  if (source === 'heap') return 'loot'
  return 'general'
}

export function visibleItemGroups(items: ItemGroup[]) {
  const exactItems = new Set(
    items.flatMap((group) =>
      group.variants
        .filter((item) => item.prediction === 'exact')
        .map((item) => itemDisplayKey(item, group.source))
    )
  )
  return items
    .map((group) => ({
      ...group,
      variants: group.variants.filter(
        (item) =>
          item.prediction !== 'variable' ||
          !exactItems.has(itemDisplayKey(item, group.source))
      ),
    }))
    .filter((group) => group.variants.length > 0)
}

function itemDisplayKey(item: ItemVariant, source?: string | null) {
  return JSON.stringify([item.item, item.level, source])
}

export function partitionFloorItems(items: ItemGroup[]) {
  return {
    general: items.filter((item) => itemSection(item) === 'general'),
    guaranteed: items.filter((item) => itemSection(item) === 'guaranteed'),
    loot: items.filter((item) => itemSection(item) === 'loot'),
    shop: items.filter((item) => itemSection(item) === 'shop'),
    quest: items.filter((item) => itemSection(item) === 'quest'),
  }
}

/**
 * Notable and other items as the floor list shows them. Shop stock stays off
 * the list and quest rewards stay on the quest card.
 */
export function floorItemGroups(floor: FloorReport) {
  const groups = partitionFloorItems(visibleItemGroups(floor.items))
  return splitNotableItems([
    ...groups.guaranteed,
    ...groups.loot,
    ...groups.general,
  ])
}
