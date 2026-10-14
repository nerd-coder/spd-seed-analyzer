/** The five dungeon regions. Each owns one hue in the system. */

export type RegionId = 'sewers' | 'prison' | 'caves' | 'city' | 'halls'

export type Region = {
  id: RegionId
  name: string
  floors: readonly [number, number]
  /** Tailwind background utility for the region fill. */
  fill: string
  /** CSS custom property holding the region color. */
  token: string
  /** Where the hue comes from in the game. */
  source: string
}

export const REGIONS: readonly Region[] = [
  {
    id: 'sewers',
    name: 'Sewers',
    floors: [1, 5],
    fill: 'bg-sewers',
    token: '--region-sewers',
    source: 'Moss on the sewer flagstones.',
  },
  {
    id: 'prison',
    name: 'Prison',
    floors: [6, 10],
    fill: 'bg-prison',
    token: '--region-prison',
    source: 'Torchlight in the prison halls.',
  },
  {
    id: 'caves',
    name: 'Caves',
    floors: [11, 15],
    fill: 'bg-caves',
    token: '--region-caves',
    source: 'Cold night air over the mines.',
  },
  {
    id: 'city',
    name: 'Dwarven City',
    floors: [16, 20],
    fill: 'bg-city',
    token: '--region-city',
    source: 'Rose stone of the dwarven walls.',
  },
  {
    id: 'halls',
    name: 'Demon Halls',
    floors: [21, 26],
    fill: 'bg-halls',
    token: '--region-halls',
    source: 'Lava running through the hex floor.',
  },
]

export function regionOf(depth: number): Region {
  const found = REGIONS.find(
    (region) => depth >= region.floors[0] && depth <= region.floors[1]
  )
  return found ?? (REGIONS[REGIONS.length - 1] as Region)
}

/** Boss floors close each region. */
export function isBossFloor(depth: number) {
  return depth % 5 === 0 && depth <= 25
}

export function textureUrl(id: RegionId) {
  return `/textures/${id}.png`
}
