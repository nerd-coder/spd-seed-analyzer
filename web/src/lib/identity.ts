import { type ItemId, itemClassName } from '@/generated/game-ids'
import type { IdentityMaps } from '@/lib/spd-wasm'

export function appearanceDescription(appearance: string): string {
  return appearance.toLowerCase()
}

export function itemAppearance(
  item: { category: string; class_name?: string | null },
  identities: IdentityMaps
): string | undefined {
  const matches = (entry: { item: ItemId }) =>
    itemClassName(entry.item) === item.class_name
  if (item.category === 'potion') {
    return identities.potions.find(matches)?.appearance
  }
  if (item.category === 'scroll') {
    return identities.scrolls.find(matches)?.appearance
  }
  if (item.category === 'ring') {
    return identities.rings.find(matches)?.appearance
  }
  return undefined
}
