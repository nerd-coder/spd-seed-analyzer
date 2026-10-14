/**
 * Upgrade-level color ranks for item names (`+1` … `+4+`).
 * Levels above 4 share the top rank color.
 */

const TIER_CLASS = [
  'text-tier-1',
  'text-tier-2',
  'text-tier-3',
  'text-tier-4',
] as const

/**
 * Color only, from the `--tier-*` design tokens: deeper in light, brighter in
 * dark. Negative levels use the degraded red.
 */
export function upgradeLevelColorClass(level: number): string {
  if (level < 0) return 'text-degraded'
  const rank = Math.min(Math.max(Math.trunc(level), 1), 4)
  return TIER_CLASS[rank - 1] ?? 'text-tier-4'
}

/** Tailwind classes for upgrade rank 1–4 (higher levels clamp to 4). */
export function upgradeLevelClass(level: number): string {
  return `font-semibold ${upgradeLevelColorClass(level)}`
}

/** Match standalone `+N` tokens in item / quest reward text. */
export const UPGRADE_TOKEN_RE = /(\+\d+)/g
