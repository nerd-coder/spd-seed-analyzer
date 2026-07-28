import type {
  HeldTrinketProfile,
  MapProfile,
  MapTrinketProfile,
} from '@/lib/spd-wasm'
import { persistentStore } from './store-utils'

const RUN_SETTINGS_KEY = 'spd-analyzer-run-settings:v1'
const MAX_SAVED_RUN_SETTINGS = 10
const MAX_HELD_TRINKETS = 12
const MAP_TRINKETS = new Set<MapTrinketProfile>([
  'no_map_affecting_trinkets',
  'mossy_clump',
  'trap_mechanism',
  'mimic_tooth',
])

type SavedRunSettings = {
  sessionId: string
  profile: MapProfile
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value)
}

function decodeHeldTrinket(value: unknown): HeldTrinketProfile | null {
  if (!isRecord(value)) return null
  const { trinket, level, start_depth } = value
  if (
    typeof trinket !== 'string' ||
    !MAP_TRINKETS.has(trinket as MapTrinketProfile) ||
    !Number.isInteger(level) ||
    (level as number) < 0 ||
    (level as number) > 3 ||
    !Number.isInteger(start_depth) ||
    (start_depth as number) < 1 ||
    (start_depth as number) > 26
  ) {
    return null
  }
  return {
    trinket: trinket as MapTrinketProfile,
    level: level as number,
    start_depth: start_depth as number,
  }
}

function decodeMapProfile(value: unknown): MapProfile | null {
  if (
    !isRecord(value) ||
    value.meta !== 'fresh' ||
    typeof value.forbidden_runes !== 'boolean' ||
    !Array.isArray(value.held_trinkets) ||
    value.held_trinkets.length > MAX_HELD_TRINKETS
  ) {
    return null
  }

  const held_trinkets: HeldTrinketProfile[] = []
  for (const rawState of value.held_trinkets) {
    const state = decodeHeldTrinket(rawState)
    const previous = held_trinkets.at(-1)
    if (
      !state ||
      (previous &&
        (state.start_depth <= previous.start_depth ||
          state.level < previous.level))
    ) {
      return null
    }
    held_trinkets.push(state)
  }

  return {
    held_trinkets,
    meta: 'fresh',
    forbidden_runes: value.forbidden_runes,
  }
}

const savedRunSettingsCodec = {
  encode: (value: SavedRunSettings[]) => JSON.stringify(value),
  decode: (value: string): SavedRunSettings[] => {
    try {
      const parsed = JSON.parse(value) as unknown
      if (!Array.isArray(parsed)) return []
      const decoded: SavedRunSettings[] = []
      const seen = new Set<string>()
      for (const entry of parsed) {
        if (!isRecord(entry) || typeof entry.sessionId !== 'string') continue
        const sessionId = entry.sessionId.trim().toUpperCase()
        const profile = decodeMapProfile(entry.profile)
        if (!sessionId || !profile || seen.has(sessionId)) continue
        seen.add(sessionId)
        decoded.push({ sessionId, profile })
        if (decoded.length >= MAX_SAVED_RUN_SETTINGS) break
      }
      return decoded
    } catch {
      return []
    }
  },
}

const $savedRunSettings = persistentStore<SavedRunSettings[]>(
  RUN_SETTINGS_KEY,
  [],
  savedRunSettingsCodec
)

export function defaultMapProfile(): MapProfile {
  return {
    held_trinkets: [],
    meta: 'fresh',
    forbidden_runes: false,
  }
}

export function savedMapProfile(sessionId: string): MapProfile | null {
  return (
    $savedRunSettings.get().find((saved) => saved.sessionId === sessionId)
      ?.profile ?? null
  )
}

export function saveMapProfile(sessionId: string, profile: MapProfile) {
  const saved = $savedRunSettings
    .get()
    .filter((entry) => entry.sessionId !== sessionId)
  const isDefault =
    profile.meta === 'fresh' &&
    !profile.forbidden_runes &&
    profile.held_trinkets.length === 0
  $savedRunSettings.set(isDefault ? saved : [...saved, { sessionId, profile }])
}

export function pruneSavedMapProfiles(sessionIds: string[]) {
  const allowed = new Set(sessionIds)
  const saved = $savedRunSettings.get()
  const next = saved.filter((entry) => allowed.has(entry.sessionId))
  if (next.length !== saved.length) $savedRunSettings.set(next)
}
