/**
 * Dungeon regions by depth, and the floor strip each banner draws.
 * Region hues are design tokens (`bg-sewers` …); see brand/design/DESIGN.md.
 */

export type RegionId = 'sewers' | 'prison' | 'caves' | 'city' | 'halls'

export type Region = {
  id: RegionId
  name: string
  floors: readonly [number, number]
  /** Tailwind background utility for the region fill. */
  fill: string
}

export const REGIONS: readonly Region[] = [
  { id: 'sewers', name: 'Sewers', floors: [1, 5], fill: 'bg-sewers' },
  { id: 'prison', name: 'Prison', floors: [6, 10], fill: 'bg-prison' },
  { id: 'caves', name: 'Caves', floors: [11, 15], fill: 'bg-caves' },
  { id: 'city', name: 'Dwarven City', floors: [16, 20], fill: 'bg-city' },
  { id: 'halls', name: 'Demon Halls', floors: [21, 26], fill: 'bg-halls' },
]

export function regionOf(depth: number): Region {
  const found = REGIONS.find(
    (region) => depth >= region.floors[0] && depth <= region.floors[1]
  )
  return found ?? (REGIONS[REGIONS.length - 1] as Region)
}

/**
 * Bosses are fixed by depth in v4.0.2: `Dungeon.bossLevel` (Dungeon.java:441)
 * marks 5, 10, 15, 20 and 25, and each builds its region's boss level.
 */
const BOSSES: Record<number, string> = {
  5: 'Goo',
  10: 'Tengu',
  15: 'DM-300',
  20: 'Dwarf King',
  25: 'Yog-Dzewa',
}

export function bossOf(depth: number): string | undefined {
  return BOSSES[depth]
}

const STRIP_TILESETS = new Set([
  'sewers',
  'prison',
  'caves',
  'caves_crystal',
  'caves_gnoll',
  'city',
  'halls',
])

/**
 * 128×32 floor strip cut from `tiles_<tileset>.png`: the plain floor tile and
 * its two decorated variants. Falls back to the region of `depth`.
 */
export function floorStripUrl(depth: number, tileset?: string | null) {
  const name = tileset && STRIP_TILESETS.has(tileset) ? tileset : null
  return `/assets/environment/strip_${name ?? regionOf(depth).id}.png`
}
