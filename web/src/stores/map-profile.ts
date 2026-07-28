import type { MapProfile } from '@/lib/spd-wasm'

export function defaultMapProfile(): MapProfile {
  return {
    held_trinkets: [],
    meta: 'fresh',
    forbidden_runes: false,
  }
}
