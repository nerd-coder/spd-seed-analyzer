import catalog from '@/lib/item-catalog.json'

/**
 * English item-window `name` and `desc` for SPD v4.0.2 (`57a4e06a4`).
 * Keys are simple class names. Seed entries use the default seed info line
 * plus the plant description.
 */
type GameItemText = {
  name: string
  desc: string
}

const TEXTS: Record<string, GameItemText> = catalog

export function itemGameText(
  className: string | null | undefined
): GameItemText | null {
  if (!className) return null
  return TEXTS[className] ?? null
}

/** Fill a `%s` name template and capitalize the first letter. */
export function gameItemName(name: string, subject?: string | null): string {
  const filled = name.includes('%s')
    ? subject
      ? name.replaceAll('%s', subject)
      : name
          .replaceAll('%s', '')
          .replaceAll(/\s{2,}/g, ' ')
          .trim()
    : name
  if (!filled) return filled
  return filled.charAt(0).toLocaleUpperCase() + filled.slice(1)
}
