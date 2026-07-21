/**
 * Map / identity spoiler flags (persisted).
 */

import { persistentAtom } from '@nanostores/persistent'

const MAP_SPOILERS_KEY = 'spd-analyzer-map-spoilers'
const IDENTITY_SPOILERS_KEY = 'spd-analyzer-identity-spoilers'
const LEGACY_ADVANCED_KEY = 'spd-analyzer-advanced-mode'

const boolCodec = {
  encode: (v: boolean) => (v ? '1' : '0'),
  decode: (v: string) => v === '1',
}

/** One-time migrate legacy “advanced mode” → map spoilers. */
function migrateLegacyMapSpoilers() {
  try {
    if (localStorage.getItem(MAP_SPOILERS_KEY) !== null) return
    if (localStorage.getItem(LEGACY_ADVANCED_KEY) === '1') {
      localStorage.setItem(MAP_SPOILERS_KEY, '1')
    }
  } catch {
    /* ignore */
  }
}

migrateLegacyMapSpoilers()

export const $mapSpoilers = persistentAtom<boolean>(
  MAP_SPOILERS_KEY,
  false,
  boolCodec
)

export const $identitySpoilers = persistentAtom<boolean>(
  IDENTITY_SPOILERS_KEY,
  false,
  boolCodec
)

export function setMapSpoilers(value: boolean) {
  $mapSpoilers.set(value)
}

export function setIdentitySpoilers(value: boolean) {
  $identitySpoilers.set(value)
}
