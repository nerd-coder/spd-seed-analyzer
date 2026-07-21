/**
 * App-wide state via nanostores.
 *
 * Persisted (localStorage): spoiler flags, open seed inputs, active tab id.
 * Ephemeral: draft seed input, session runtime (reports), meta, analyzing.
 */

import { persistentAtom } from '@nanostores/persistent'
import { atom, computed } from 'nanostores'

import { analyzeSeed, getSpdMeta, type SeedReport } from '@/lib/spd-wasm'

// —— keys / limits ————————————————————————————————————————————————

const MAP_SPOILERS_KEY = 'spd-analyzer-map-spoilers'
const IDENTITY_SPOILERS_KEY = 'spd-analyzer-identity-spoilers'
const LEGACY_ADVANCED_KEY = 'spd-analyzer-advanced-mode'
const SAVED_SEEDS_KEY = 'spd-analyzer-open-seeds'
const ACTIVE_SEED_KEY = 'spd-analyzer-active-seed'

/** Max open/saved seeds (FIFO when exceeded). */
export const MAX_SAVED_SEEDS = 10

/** Full main-path depth range (SPD clamps to 26). */
export const ANALYZE_FLOORS = 26

/** Delay between sequential re-analyzes after refresh (ms). */
const REANALYZE_GAP_MS = 350

// —— types ————————————————————————————————————————————————————————

export type SessionStatus = 'pending' | 'loading' | 'ready' | 'error'

export type SeedSession = {
  /** Stable tab id (normalized input key). */
  id: string
  /** Original user input used for analyze. */
  input: string
  status: SessionStatus
  report: SeedReport | null
  error: string | null
}

export type SpdMeta = { version: string; commit: string }

// —— helpers ——————————————————————————————————————————————————————

const boolCodec = {
  encode: (v: boolean) => (v ? '1' : '0'),
  decode: (v: string) => v === '1',
}

const stringArrayCodec = {
  encode: (v: string[]) => JSON.stringify(v),
  decode: (v: string): string[] => {
    try {
      const parsed = JSON.parse(v) as unknown
      if (!Array.isArray(parsed)) return []
      return parsed
        .filter((s): s is string => typeof s === 'string')
        .map((s) => s.trim())
        .filter(Boolean)
        .slice(0, MAX_SAVED_SEEDS)
    } catch {
      return []
    }
  },
}

const nullableStringCodec = {
  encode: (v: string | null) => v ?? '',
  decode: (v: string) => (v === '' ? null : v),
}

export function normalizeSeedInput(raw: string): string {
  return raw.trim()
}

export function sessionIdFor(input: string): string {
  return normalizeSeedInput(input).toUpperCase()
}

export function tabLabel(session: SeedSession): string {
  if (session.report) {
    return session.report.seed.code ?? session.report.seed.formatted
  }
  return session.input
}

function sleep(ms: number) {
  return new Promise<void>((resolve) => {
    setTimeout(resolve, ms)
  })
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

// —— persistent stores ————————————————————————————————————————————

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

/** Ordered list of seed inputs to keep open (max {@link MAX_SAVED_SEEDS}). */
export const $savedSeedInputs = persistentAtom<string[]>(
  SAVED_SEEDS_KEY,
  [],
  stringArrayCodec
)

export const $activeSeedId = persistentAtom<string | null>(
  ACTIVE_SEED_KEY,
  null,
  nullableStringCodec
)

// —— ephemeral stores ——————————————————————————————————————————————

/** Draft seed field in the left menu. */
export const $seedInput = atom('')

/** Runtime sessions (reports not persisted — re-analyzed on load). */
export const $sessions = atom<SeedSession[]>([])

export const $analyzing = atom(false)

export const $formError = atom<string | null>(null)

export const $meta = atom<SpdMeta | null>(null)

export const $sessionCount = computed($sessions, (s) => s.length)

// —— internal rehydrate control ————————————————————————————————————

const closedIds = new Set<string>()
let rehydrateGen = 0

// —— pure mutations ————————————————————————————————————————————————

function setSavedInputs(inputs: string[]) {
  const cleaned = inputs
    .map(normalizeSeedInput)
    .filter(Boolean)
    .slice(0, MAX_SAVED_SEEDS)
  // Deduplicate by session id (keep first occurrence order).
  const seen = new Set<string>()
  const unique: string[] = []
  for (const input of cleaned) {
    const id = sessionIdFor(input)
    if (seen.has(id)) continue
    seen.add(id)
    unique.push(input)
  }
  // Cap from the end (keep newest) if still over limit after dedupe.
  $savedSeedInputs.set(
    unique.length > MAX_SAVED_SEEDS
      ? unique.slice(unique.length - MAX_SAVED_SEEDS)
      : unique
  )
}

function patchSession(id: string, patch: Partial<SeedSession>) {
  const prev = $sessions.get()
  if (!prev.some((s) => s.id === id)) return
  $sessions.set(prev.map((s) => (s.id === id ? { ...s, ...patch } : s)))
}

async function runAnalyze(id: string, input: string): Promise<boolean> {
  patchSession(id, { status: 'loading', error: null })
  try {
    const report = await analyzeSeed(input, ANALYZE_FLOORS)
    patchSession(id, { status: 'ready', report, error: null })
    return true
  } catch (err) {
    patchSession(id, {
      status: 'error',
      report: null,
      error: err instanceof Error ? err.message : String(err),
    })
    return false
  }
}

// —— public actions ————————————————————————————————————————————————

export function setSeedInput(value: string) {
  $seedInput.set(value)
}

export function setMapSpoilers(value: boolean) {
  $mapSpoilers.set(value)
}

export function setIdentitySpoilers(value: boolean) {
  $identitySpoilers.set(value)
}

export function setActiveSeed(id: string | null) {
  $activeSeedId.set(id)
}

export function loadSpdMeta() {
  getSpdMeta()
    .then((m) => $meta.set(m))
    .catch((e: unknown) => {
      $formError.set(e instanceof Error ? e.message : String(e))
    })
}

/**
 * Close a seed tab. Removes from runtime sessions + persisted list.
 */
export function closeSeedSession(id: string) {
  closedIds.add(id)
  const prev = $sessions.get()
  const idx = prev.findIndex((s) => s.id === id)
  if (idx < 0) return
  const next = prev.filter((s) => s.id !== id)
  $sessions.set(next)
  setSavedInputs(next.map((s) => s.input))
  if ($activeSeedId.get() === id) {
    const fallback = next[idx] ?? next[idx - 1] ?? next[0] ?? null
    $activeSeedId.set(fallback?.id ?? null)
  }
}

/**
 * Analyze the draft seed input (or re-focus existing tab).
 * Enforces {@link MAX_SAVED_SEEDS} by dropping oldest sessions.
 */
export async function analyzeDraftSeed(): Promise<void> {
  const input = normalizeSeedInput($seedInput.get())
  if (!input) return

  $formError.set(null)
  const id = sessionIdFor(input)
  const sessions = $sessions.get()
  const existing = sessions.find((s) => s.id === id)

  if (existing) {
    $activeSeedId.set(id)
    if (existing.status === 'ready' || existing.status === 'loading') {
      return
    }
    $analyzing.set(true)
    try {
      await runAnalyze(id, existing.input)
    } finally {
      $analyzing.set(false)
    }
    return
  }

  // Cap: drop oldest session(s) so we stay within MAX after adding.
  let nextSessions = [...sessions]
  while (nextSessions.length >= MAX_SAVED_SEEDS) {
    const oldest = nextSessions[0]
    closedIds.add(oldest.id)
    nextSessions = nextSessions.slice(1)
  }

  const session: SeedSession = {
    id,
    input,
    status: 'loading',
    report: null,
    error: null,
  }
  nextSessions = [...nextSessions, session]
  $sessions.set(nextSessions)
  setSavedInputs(nextSessions.map((s) => s.input))
  $activeSeedId.set(id)
  $seedInput.set('')
  $analyzing.set(true)
  try {
    await runAnalyze(id, input)
  } finally {
    $analyzing.set(false)
  }
}

/**
 * Rebuild runtime sessions from persisted seed inputs and re-analyze slowly.
 * Safe under React Strict Mode (cleanup aborts via generation counter).
 *
 * @returns cleanup function that aborts the in-flight loop
 */
export function startSessionRehydrate(): () => void {
  const saved = $savedSeedInputs
    .get()
    .map(normalizeSeedInput)
    .filter(Boolean)
    .slice(0, MAX_SAVED_SEEDS)

  // Normalize / re-cap persisted list if it grew before the limit existed.
  if (
    saved.length !== $savedSeedInputs.get().length ||
    $savedSeedInputs.get().some((s, i) => normalizeSeedInput(s) !== saved[i])
  ) {
    setSavedInputs(saved)
  }

  if (saved.length === 0) {
    $sessions.set([])
    return () => {}
  }

  const restored: SeedSession[] = saved.map((input) => ({
    id: sessionIdFor(input),
    input,
    status: 'pending' as const,
    report: null,
    error: null,
  }))

  closedIds.clear()
  $sessions.set(restored)

  const preferred = $activeSeedId.get()
  const active =
    preferred && restored.some((s) => s.id === preferred)
      ? preferred
      : (restored[0]?.id ?? null)
  $activeSeedId.set(active)

  const gen = ++rehydrateGen
  let cancelled = false

  ;(async () => {
    $analyzing.set(true)
    try {
      for (let i = 0; i < restored.length; i++) {
        if (cancelled || rehydrateGen !== gen) break
        const s = restored[i]
        // Skip only if closed during rehydrate — do not consult $sessions
        // for membership (race with React paint).
        if (closedIds.has(s.id)) continue
        await runAnalyze(s.id, s.input)
        if (i < restored.length - 1 && !cancelled && rehydrateGen === gen) {
          await sleep(REANALYZE_GAP_MS)
        }
      }
    } finally {
      if (!cancelled && rehydrateGen === gen) {
        $analyzing.set(false)
      }
    }
  })()

  return () => {
    cancelled = true
    rehydrateGen += 1
  }
}
