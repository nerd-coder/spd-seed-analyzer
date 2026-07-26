import type { MapProfile, MapTrinketProfile } from '@/lib/spd-wasm'
import { persistentStore } from './store-utils'

const MAP_TRINKET_KEY = 'spd-analyzer-map-trinket'

const trinkets: readonly MapTrinketProfile[] = [
  'no_map_affecting_trinkets',
  'mossy_clump0',
  'mossy_clump1',
  'mossy_clump2',
  'mossy_clump3',
  'trap_mechanism0',
  'trap_mechanism1',
  'trap_mechanism2',
  'trap_mechanism3',
]

const trinketCodec = {
  encode: (value: MapTrinketProfile) => value,
  decode: (value: string): MapTrinketProfile =>
    trinkets.includes(value as MapTrinketProfile)
      ? (value as MapTrinketProfile)
      : 'no_map_affecting_trinkets',
}

export const $mapTrinket = persistentStore<MapTrinketProfile>(
  MAP_TRINKET_KEY,
  'no_map_affecting_trinkets',
  trinketCodec
)

export function mapProfile(): MapProfile {
  return { trinket: $mapTrinket.get(), meta: 'fresh' }
}

export function setMapTrinket(value: MapTrinketProfile) {
  $mapTrinket.set(value)
}
