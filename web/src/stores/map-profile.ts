import type {
  ArtifactEvent,
  ArtifactKind,
  Challenge,
  ClaimState,
  MapProfile,
  TrinketEvent,
  TrinketKind,
} from '@/lib/spd-wasm'
import { persistentStore } from './store-utils'

const RUN_SETTINGS_KEY = 'spd-analyzer-run-settings:v2'
const LEGACY_RUN_SETTINGS_KEY = 'spd-analyzer-run-settings:v1'
const MAX_SAVED_RUN_SETTINGS = 10

const CHALLENGES = new Set<Challenge>([
  'champion_enemies',
  'badder_bosses',
  'on_diet',
  'faith_is_my_armor',
  'pharmacophobia',
  'barren_land',
  'swarm_intelligence',
  'into_darkness',
  'forbidden_runes',
])

const TRINKETS = new Set<TrinketKind>([
  'rat_skull',
  'parchment_scrap',
  'petrified_seed',
  'exotic_crystals',
  'mossy_clump',
  'dimensional_sundial',
  'thirteen_leaf_clover',
  'trap_mechanism',
  'mimic_tooth',
  'wondrous_resin',
  'eye_of_newt',
  'salt_cube',
  'vial_of_blood',
  'shard_of_oblivion',
  'chaotic_censer',
  'ferret_tuft',
  'cracked_spyglass',
])

const ARTIFACTS = new Set<ArtifactKind>([
  'alchemists_toolkit',
  'chalice_of_blood',
  'cloak_of_shadows',
  'dried_rose',
  'ethereal_chains',
  'holy_tome',
  'horn_of_plenty',
  'master_thieves_armband',
  'sandals_of_nature',
  'skeleton_key',
  'talisman_of_foresight',
  'timekeepers_hourglass',
  'unstable_spellbook',
])

type SavedRunSettings = {
  sessionId: string
  profile: MapProfile
}

type LegacyHeldTrinket = {
  trinket:
    | 'no_map_affecting_trinkets'
    | 'mossy_clump'
    | 'trap_mechanism'
    | 'mimic_tooth'
  level: number
  start_depth: number
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value)
}

function isDepth(value: unknown): value is number {
  return (
    Number.isInteger(value) && (value as number) >= 1 && (value as number) <= 26
  )
}

function decodeTrinketEvent(value: unknown): TrinketEvent | null {
  if (
    !isRecord(value) ||
    !isDepth(value.before_depth) ||
    typeof value.kind !== 'string'
  ) {
    return null
  }
  if (value.kind === 'upgraded') {
    return { before_depth: value.before_depth, kind: 'upgraded' }
  }
  if (
    (value.kind !== 'acquired' && value.kind !== 'transmuted') ||
    typeof value.trinket !== 'string' ||
    !TRINKETS.has(value.trinket as TrinketKind)
  ) {
    return null
  }
  return {
    before_depth: value.before_depth,
    kind: value.kind,
    trinket: value.trinket as TrinketKind,
  }
}

function decodeArtifactEvent(value: unknown): ArtifactEvent | null {
  if (
    !isRecord(value) ||
    !isDepth(value.before_depth) ||
    value.before_depth === 1 ||
    (value.kind !== 'obtained' && value.kind !== 'transmuted') ||
    typeof value.artifact !== 'string' ||
    !ARTIFACTS.has(value.artifact as ArtifactKind)
  ) {
    return null
  }
  return {
    before_depth: value.before_depth,
    kind: value.kind,
    artifact: value.artifact as ArtifactKind,
  }
}

function decodeClaimState(value: unknown): ClaimState | null {
  if (!isRecord(value)) return null
  const { parchment_scrap_level } = value
  if (
    parchment_scrap_level !== undefined &&
    parchment_scrap_level !== null &&
    (!Number.isInteger(parchment_scrap_level) ||
      (parchment_scrap_level as number) < 0 ||
      (parchment_scrap_level as number) > 3)
  ) {
    return null
  }
  return {
    parchment_scrap_level: parchment_scrap_level as number | null | undefined,
  }
}

type ActiveTrinket = { trinket: TrinketKind; level: number }

function applyTrinketEvent(
  held: ActiveTrinket | null,
  event: TrinketEvent
): ActiveTrinket | undefined {
  if (event.kind === 'acquired') {
    return held ? undefined : { trinket: event.trinket, level: 0 }
  }
  if (event.kind === 'upgraded') {
    if (!held || held.level >= 3) return undefined
    return { trinket: held.trinket, level: held.level + 1 }
  }
  if (!held || held.trinket === event.trinket) return undefined
  return { trinket: event.trinket, level: held.level }
}

function validTrinketEvents(events: TrinketEvent[]): boolean {
  let previousDepth = 0
  let held: ActiveTrinket | null = null
  for (const event of events) {
    if (event.before_depth < previousDepth) return false
    const next = applyTrinketEvent(held, event)
    if (!next) return false
    held = next
    previousDepth = event.before_depth
  }
  return true
}

function decodeMapProfile(value: unknown): MapProfile | null {
  if (
    !isRecord(value) ||
    !Array.isArray(value.challenges) ||
    !Array.isArray(value.trinket_events) ||
    !Array.isArray(value.artifact_events)
  ) {
    return null
  }
  const challenges = value.challenges.filter(
    (challenge): challenge is Challenge =>
      typeof challenge === 'string' && CHALLENGES.has(challenge as Challenge)
  )
  if (
    challenges.length !== value.challenges.length ||
    new Set(challenges).size !== challenges.length
  ) {
    return null
  }
  const trinket_events = value.trinket_events.map(decodeTrinketEvent)
  const artifact_events = value.artifact_events.map(decodeArtifactEvent)
  const claim_state = decodeClaimState(value.claim_state)
  if (
    trinket_events.some((event) => event === null) ||
    artifact_events.some((event) => event === null) ||
    !claim_state
  ) {
    return null
  }
  const resolvedTrinketEvents = trinket_events as TrinketEvent[]
  const resolvedArtifactEvents = artifact_events as ArtifactEvent[]
  if (
    !validTrinketEvents(resolvedTrinketEvents) ||
    resolvedArtifactEvents.some(
      (event, index) =>
        index > 0 &&
        event.before_depth < resolvedArtifactEvents[index - 1].before_depth
    )
  ) {
    return null
  }
  return {
    challenges,
    trinket_events: resolvedTrinketEvents,
    artifact_events: resolvedArtifactEvents,
    claim_state,
  }
}

function decodeLegacyHeldTrinket(value: unknown): LegacyHeldTrinket | null {
  if (!isRecord(value)) return null
  const { trinket, level, start_depth } = value
  if (
    (trinket !== 'no_map_affecting_trinkets' &&
      trinket !== 'mossy_clump' &&
      trinket !== 'trap_mechanism' &&
      trinket !== 'mimic_tooth') ||
    !Number.isInteger(level) ||
    (level as number) < 0 ||
    (level as number) > 3 ||
    !isDepth(start_depth)
  ) {
    return null
  }
  return {
    trinket,
    level: level as number,
    start_depth,
  }
}

function decodeLegacyMapProfile(value: unknown): MapProfile | null {
  if (
    !isRecord(value) ||
    value.meta !== 'fresh' ||
    typeof value.forbidden_runes !== 'boolean' ||
    !Array.isArray(value.held_trinkets)
  ) {
    return null
  }
  const states = value.held_trinkets.map(decodeLegacyHeldTrinket)
  if (states.some((state) => state === null)) return null
  const trinket_events: TrinketEvent[] = []
  let held: LegacyHeldTrinket | null = null
  for (const state of states as LegacyHeldTrinket[]) {
    if (state.trinket === 'no_map_affecting_trinkets') return null
    if (!held) {
      trinket_events.push({
        before_depth: state.start_depth,
        kind: 'acquired',
        trinket: state.trinket,
      })
    } else if (held.trinket !== state.trinket) {
      trinket_events.push({
        before_depth: state.start_depth,
        kind: 'transmuted',
        trinket: state.trinket,
      })
    }
    for (let level = held?.level ?? 0; level < state.level; level += 1) {
      trinket_events.push({ before_depth: state.start_depth, kind: 'upgraded' })
    }
    held = state
  }
  if (!validTrinketEvents(trinket_events)) return null
  return {
    challenges: value.forbidden_runes ? ['forbidden_runes'] : [],
    trinket_events,
    artifact_events: [],
    claim_state: {},
  }
}

function decodeSavedRunSettings(
  value: string,
  decodeProfile: (profile: unknown) => MapProfile | null
): SavedRunSettings[] {
  try {
    const parsed = JSON.parse(value) as unknown
    if (!Array.isArray(parsed)) return []
    const decoded: SavedRunSettings[] = []
    const seen = new Set<string>()
    for (const entry of parsed) {
      if (!isRecord(entry) || typeof entry.sessionId !== 'string') continue
      const sessionId = entry.sessionId.trim().toUpperCase()
      const profile = decodeProfile(entry.profile)
      if (!sessionId || !profile || seen.has(sessionId)) continue
      seen.add(sessionId)
      decoded.push({ sessionId, profile })
      if (decoded.length >= MAX_SAVED_RUN_SETTINGS) break
    }
    return decoded
  } catch {
    return []
  }
}

const savedRunSettingsCodec = {
  encode: (value: SavedRunSettings[]) => JSON.stringify(value),
  decode: (value: string) => decodeSavedRunSettings(value, decodeMapProfile),
}

function legacySavedRunSettings(): SavedRunSettings[] {
  try {
    const value = localStorage.getItem(LEGACY_RUN_SETTINGS_KEY)
    return value === null
      ? []
      : decodeSavedRunSettings(value, decodeLegacyMapProfile)
  } catch {
    return []
  }
}

const $savedRunSettings = persistentStore<SavedRunSettings[]>(
  RUN_SETTINGS_KEY,
  legacySavedRunSettings(),
  savedRunSettingsCodec
)

export function defaultMapProfile(): MapProfile {
  return {
    challenges: [],
    trinket_events: [],
    artifact_events: [],
    claim_state: {},
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
    profile.challenges.length === 0 &&
    profile.trinket_events.length === 0 &&
    profile.artifact_events.length === 0 &&
    profile.claim_state.parchment_scrap_level === undefined
  $savedRunSettings.set(isDefault ? saved : [...saved, { sessionId, profile }])
}

export function pruneSavedMapProfiles(sessionIds: string[]) {
  const allowed = new Set(sessionIds)
  const saved = $savedRunSettings.get()
  const next = saved.filter((entry) => allowed.has(entry.sessionId))
  if (next.length !== saved.length) $savedRunSettings.set(next)
}
