import type { MapProfile } from '@/lib/spd-wasm'

export function defaultMapProfile(): MapProfile {
  return {
    trinket: 'no_map_affecting_trinkets',
    meta: 'fresh',
    forbidden_runes: false,
    trinket_start_depth: 1,
  }
}
