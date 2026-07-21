/**
 * Upgrade-level color ranks for item names (`+1` … `+4+`).
 * Levels above 4 share the top rank color.
 */

/** Tailwind classes for upgrade rank 1–4 (higher levels clamp to 4). */
export function upgradeLevelClass(level: number): string {
  const rank = Math.min(Math.max(Math.trunc(level), 1), 4)
  switch (rank) {
    case 1:
      return 'font-semibold text-emerald-600 dark:text-emerald-400'
    case 2:
      return 'font-semibold text-sky-600 dark:text-sky-400'
    case 3:
      return 'font-semibold text-violet-600 dark:text-violet-400'
    default:
      return 'font-semibold text-amber-600 dark:text-amber-300'
  }
}

/** Match standalone `+N` tokens in item / quest reward text. */
export const UPGRADE_TOKEN_RE = /(\+\d+)/g
