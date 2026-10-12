import type { IdentityMaps } from '@/lib/spd-wasm'

export function appearanceDescription(appearance: string): string {
  return appearance.toLowerCase()
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

/** Drop "Potion of " / "Scroll of " / "Ring of " regardless of case — the tab already names the category. */
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
  return name.toLowerCase().startsWith(prefix.toLowerCase())
    ? name.slice(prefix.length)
    : name
}
