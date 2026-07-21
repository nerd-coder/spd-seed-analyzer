import type { IdentityMaps } from '@/lib/spd-wasm'

export function appearanceDescription(
  category: 'potion' | 'scroll' | 'ring',
  appearance: string
): string {
  const label = appearance.toLowerCase()
  switch (category) {
    case 'potion':
      return `${label} potion`
    case 'scroll':
      return `${label} rune`
    case 'ring':
      return `${label} gem`
  }
}

export function itemAppearance(
  item: { category: string; class_name?: string | null },
  identities: IdentityMaps
): string | undefined {
  if (item.category === 'potion') {
    return identities.potions.find((p) => p.item === item.class_name)
      ?.appearance
  }
  if (item.category === 'scroll') {
    return identities.scrolls.find((s) => s.item === item.class_name)
      ?.appearance
  }
  if (item.category === 'ring') {
    return identities.rings.find((r) => r.item === item.class_name)?.appearance
  }
  return undefined
}

/** Drop "Potion of " / "Scroll of " / "Ring of " — tab already names the category. */
export function shortIdentityName(
  name: string,
  category: 'potion' | 'scroll' | 'ring'
): string {
  const prefix =
    category === 'potion'
      ? 'Potion of '
      : category === 'scroll'
        ? 'Scroll of '
        : 'Ring of '
  return name.startsWith(prefix) ? name.slice(prefix.length) : name
}
