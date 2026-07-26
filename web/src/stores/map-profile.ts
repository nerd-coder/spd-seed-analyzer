import { persistentAtom } from '@nanostores/persistent'

import type { MapProfile } from '@/lib/spd-wasm'

const booleanCodec = {
  encode: (value: boolean) => String(value),
  decode: (value: string) => value === 'true',
}

export const $assumeNoMapTrinkets = persistentAtom(
  'spd-analyzer-map-no-trinkets-v2',
  false,
  booleanCodec
)
export const $assumeFreshMeta = persistentAtom(
  'spd-analyzer-map-fresh-meta-v2',
  false,
  booleanCodec
)

export function mapProfile(): MapProfile | undefined {
  if (!$assumeNoMapTrinkets.get() || !$assumeFreshMeta.get()) return undefined
  return {
    trinket: 'no_map_affecting_trinkets',
    meta: 'fresh',
    floors: [],
  }
}

export function setAssumeNoMapTrinkets(value: boolean) {
  $assumeNoMapTrinkets.set(value)
}

export function setAssumeFreshMeta(value: boolean) {
  $assumeFreshMeta.set(value)
}
