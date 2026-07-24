import { atom, computed } from 'nanostores'
import { finderItemLabel } from '@/components/finder/finder-items'
import {
  type FinderConfig,
  type FinderRunState,
  INITIAL_FINDER_RUN,
  MAX_RESULTS,
} from '@/components/finder/finder-types'
import { type SeedSearchMatch, searchSeeds } from '@/lib/spd-wasm'

const SEARCH_CHUNK_SIZE = 5
export const MAX_FINDER_SESSIONS = 10

export type FinderSession = {
  id: string
  name: string
  config: FinderConfig
  run: FinderRunState
}

export const $finderSessions = atom<FinderSession[]>([])
export const $activeFinderId = atom<string | null>(null)
export const $activeFinderSession = computed(
  [$finderSessions, $activeFinderId],
  (sessions, activeId) =>
    sessions.find((session) => session.id === activeId) ?? sessions[0] ?? null
)

const cancelledIds = new Set<string>()
let nextFinderId = 1

function searchName(config: FinderConfig): string {
  if (config.constraints.length !== 1) {
    return `Find ${config.constraints.length} items`
  }
  return finderItemLabel(config.constraints[0].className)
}

function patchFinderSession(id: string, run: FinderRunState) {
  const sessions = $finderSessions.get()
  if (!sessions.some((session) => session.id === id)) return false
  $finderSessions.set(
    sessions.map((session) =>
      session.id === id ? { ...session, run } : session
    )
  )
  return true
}

function yieldToBrowser(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0))
}

export function setActiveFinder(id: string | null) {
  $activeFinderId.set(id)
}

export function closeFinderSession(id: string) {
  cancelledIds.add(id)
  const sessions = $finderSessions.get()
  const index = sessions.findIndex((session) => session.id === id)
  if (index < 0) return
  const next = sessions.filter((session) => session.id !== id)
  $finderSessions.set(next)
  if ($activeFinderId.get() === id) {
    $activeFinderId.set(
      (next[index] ?? next[index - 1] ?? next[0] ?? null)?.id ?? null
    )
  }
}

export function cancelFinderSearch(id: string) {
  cancelledIds.add(id)
  const session = $finderSessions.get().find((item) => item.id === id)
  if (session?.run.status === 'running') {
    patchFinderSession(id, { ...session.run, cancelRequested: true })
  }
}

export async function startFinderSearch(config: FinderConfig) {
  const id = `finder-${nextFinderId++}`
  const session: FinderSession = {
    id,
    name: searchName(config),
    config,
    run: {
      ...INITIAL_FINDER_RUN,
      status: 'running',
      requestedCandidates: config.candidateCount,
      nextSeed: config.startSeed,
    },
  }
  const existing = $finderSessions.get()
  const dropped = existing.slice(
    0,
    Math.max(0, existing.length + 1 - MAX_FINDER_SESSIONS)
  )
  for (const item of dropped) cancelledIds.add(item.id)
  $finderSessions.set([...existing.slice(dropped.length), session])
  $activeFinderId.set(id)
  await yieldToBrowser()

  let scanned = 0
  let cursor: number | null = config.startSeed
  let exhausted = false
  let message: string | null = null
  const matchesBySeed = new Map<number, SeedSearchMatch>()

  try {
    while (
      scanned < config.candidateCount &&
      matchesBySeed.size < config.maxMatches &&
      cursor !== null &&
      !exhausted &&
      !cancelledIds.has(id)
    ) {
      const candidateCount = Math.min(
        SEARCH_CHUNK_SIZE,
        config.candidateCount - scanned
      )
      const result = await searchSeeds({
        ...config,
        startSeed: cursor,
        candidateCount,
        maxMatches: Math.min(
          MAX_RESULTS,
          config.maxMatches - matchesBySeed.size
        ),
      })
      scanned += result.candidatesScanned
      cursor = result.nextSeed
      exhausted = result.exhausted
      message = result.message
      for (const match of result.matches) {
        if (!matchesBySeed.has(match.seed.numeric)) {
          matchesBySeed.set(match.seed.numeric, match)
        }
      }
      if (
        !patchFinderSession(id, {
          status: 'running',
          scanned,
          requestedCandidates: config.candidateCount,
          nextSeed: cursor,
          exhausted,
          cancelRequested: cancelledIds.has(id),
          completionReason: null,
          matches: Array.from(matchesBySeed.values()),
          message,
          error: null,
        })
      )
        return
      if (
        result.candidatesScanned === 0 ||
        matchesBySeed.size >= config.maxMatches ||
        exhausted ||
        cursor === null
      )
        break
      await yieldToBrowser()
    }

    const wasCancelled = cancelledIds.has(id)
    patchFinderSession(id, {
      status: wasCancelled ? 'cancelled' : 'completed',
      scanned,
      requestedCandidates: config.candidateCount,
      nextSeed: cursor,
      exhausted,
      cancelRequested: false,
      completionReason: wasCancelled
        ? null
        : exhausted
          ? 'exhausted'
          : matchesBySeed.size >= config.maxMatches
            ? 'result-limit'
            : 'scanned',
      matches: Array.from(matchesBySeed.values()),
      message,
      error: null,
    })
  } catch (error) {
    patchFinderSession(id, {
      status: cancelledIds.has(id) ? 'cancelled' : 'error',
      scanned,
      requestedCandidates: config.candidateCount,
      nextSeed: cursor,
      exhausted,
      cancelRequested: false,
      completionReason: null,
      matches: Array.from(matchesBySeed.values()),
      message,
      error: error instanceof Error ? error.message : String(error),
    })
  } finally {
    cancelledIds.delete(id)
  }
}
