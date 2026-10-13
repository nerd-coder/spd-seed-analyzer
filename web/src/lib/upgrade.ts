/**
 * Upgrade-level color ranks for item names (`+1` … `+4+`).
 * Levels above 4 share the top rank color.
 */

/** Color only. Same rank color in light and dark. Negative levels use degraded red. */
export function upgradeLevelColorClass(level: number): string {
  if (level < 0) return 'text-red-300'
  const rank = Math.min(Math.max(Math.trunc(level), 1), 4)
  switch (rank) {
    case 1:
      return 'text-emerald-400'
    case 2:
      return 'text-sky-400'
    case 3:
      return 'text-violet-400'
    default:
      return 'text-amber-300'
  }
}

/** Tailwind classes for upgrade rank 1–4 (higher levels clamp to 4). */
export function upgradeLevelClass(level: number): string {
  return `font-semibold ${upgradeLevelColorClass(level)}`
}

/** Match standalone `+N` tokens in item / quest reward text. */
export const UPGRADE_TOKEN_RE = /(\+\d+)/g
