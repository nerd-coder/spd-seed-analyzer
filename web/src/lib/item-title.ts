import { finderItemLabel } from '@/components/finder/finder-items'
import {
  type Enchantment,
  enchantmentClassName,
  type ItemId,
  itemClassName,
  type MobId,
  mobClassName,
} from '@/generated/game-ids'
import { gameObjectText } from '@/lib/game-object-text'
import { gameItemName, itemGameText } from '@/lib/item-text'
import type { ItemVariant } from '@/lib/spd-wasm'

/** Categories whose `level` is not shown as a `+N` suffix in game titles. */
const UNLEVELED_CATEGORIES = new Set([
  'potion',
  'scroll',
  'stone',
  'seed',
  'food',
  'gold',
])

export function variantClassName(
  variant: Pick<ItemVariant, 'item'>
): string | undefined {
  return variant.item == null ? undefined : itemClassName(variant.item)
}

export function variantCandidateClasses(
  variant: Pick<ItemVariant, 'candidates'>
): string[] {
  return (variant.candidates ?? []).map(itemClassName)
}

export function enchantmentName(
  enchantment: Enchantment | null | undefined
): string | undefined {
  return enchantment == null ? undefined : enchantmentClassName(enchantment)
}

const NO_CAPS = new Set(['a', 'an', 'and', 'of', 'by', 'to', 'the', 'x', 'for'])

/** English `Messages.titleCase`: every word capitalized except `NO_CAPS`. */
function titleCase(text: string): string {
  const titled = text.replace(/\S+/g, (word) =>
    NO_CAPS.has(word.toLowerCase().replace(/:|[0-9]/g, ''))
      ? word
      : word.charAt(0).toLocaleUpperCase() + word.slice(1)
  )
  return titled.charAt(0).toLocaleUpperCase() + titled.slice(1)
}

/** Game title for one item class, e.g. `Ring of Wealth`. */
export function itemLabel(item: ItemId | string): string {
  const className = typeof item === 'string' ? item : itemClassName(item)
  const text = itemGameText(className)
  return text ? titleCase(gameItemName(text.name)) : finderItemLabel(className)
}

/**
 * Game `Weapon.name()` / `Armor.name()`: the enchantment template wraps the
 * base name and the `+N` suffix stays last, e.g. `Polarized Quarterstaff +1`.
 */
export function enchantedTitle(
  title: string,
  enchantment: string | null | undefined
): string {
  const text = itemGameText(enchantment)
  if (!text) return title
  const [, base, level = ''] = title.match(/^(.*?)( \+\d+)?$/) ?? ['', title]
  return titleCase(gameItemName(text.name, base)) + level
}

const MOB_TITLES: Record<string, string> = {
  Albino: 'Albino Rat',
  ArmoredStatue: 'Armored Statue',
  CrystalMimic: 'Crystal Mimic',
  GoldenMimic: 'Golden Mimic',
  PhantomPiranha: 'Phantom Piranha',
  DemonSpawner: 'Demon Spawner',
}

/** Title for a mob class, e.g. `Albino Rat`. */
export function mobLabel(mob: MobId): string {
  const className = mobClassName(mob)
  const text = gameObjectText(className)
  if (text) return titleCase(gameItemName(text.name))
  return MOB_TITLES[className] ?? className.replace(/([a-z])([A-Z])/g, '$1 $2')
}

/**
 * Title for a report entry: the item's game title plus its `+N` level, or the
 * engine label when the entry has no single identity.
 */
export function variantTitle(
  variant: Pick<ItemVariant, 'item' | 'label' | 'level' | 'category'>
): string {
  if (variant.item == null) return variant.label ?? 'Unknown item'
  const title = itemLabel(variant.item)
  const level = variant.level ?? 0
  return level > 0 && !UNLEVELED_CATEGORIES.has(variant.category)
    ? `${title} +${level}`
    : title
}
