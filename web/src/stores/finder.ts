import { atom, computed } from 'nanostores'
import { finderItemLabel } from '@/components/finder/finder-items'
import {
  type FinderConfig,
  type FinderRunState,
  INITIAL_FINDER_RUN,
} from '@/components/finder/finder-types'
import { searchSeedsInWorker, type WorkerTask } from '@/lib/spd-worker-client'

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
const searchTasks = new Map<string, WorkerTask<unknown>>()
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

export function setActiveFinder(id: string | null) {
  $activeFinderId.set(id)
}

export function closeFinderSession(id: string) {
  cancelledIds.add(id)
  searchTasks.get(id)?.cancel()
  searchTasks.delete(id)
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

export function cancelFinderSearch(id?: string) {
  const targetIds = id
    ? [id]
    : $finderSessions
        .get()
        .filter((session) => session.run.status === 'running')
        .map((session) => session.id)
  for (const targetId of targetIds) {
    cancelledIds.add(targetId)
    searchTasks.get(targetId)?.cancel()
    searchTasks.delete(targetId)
    const session = $finderSessions.get().find((item) => item.id === targetId)
    if (session?.run.status !== 'running') continue
    patchFinderSession(targetId, {
      ...session.run,
      status: 'cancelled',
      cancelRequested: false,
      finishedAt: Date.now(),
    })
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
      currentDepth: config.floors,
      nextSeed: config.startSeed,
      startedAt: Date.now(),
      finishedAt: null,
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
  try {
    const task = searchSeedsInWorker(
      config,
      (result) => {
        const current = $finderSessions.get().find((item) => item.id === id)
        if (current?.run.status !== 'running') return
        patchFinderSession(id, {
          ...current.run,
          scanned: result.candidatesScanned,
          nextSeed: result.nextSeed,
          exhausted: result.exhausted,
          matches: result.matches,
          message: result.message,
        })
      },
      ({ candidateNumber, seed, depth }) => {
        const current = $finderSessions.get().find((item) => item.id === id)
        if (current?.run.status !== 'running') return
        patchFinderSession(id, {
          ...current.run,
          currentCandidateNumber: candidateNumber,
          currentCandidateSeed: seed,
          currentDepth: depth,
        })
      }
    )
    searchTasks.set(id, task)
    const result = await task.promise
    if (cancelledIds.has(id)) return
    patchFinderSession(id, {
      status: 'completed',
      scanned: result.candidatesScanned,
      requestedCandidates: config.candidateCount,
      currentCandidateNumber: result.candidatesScanned || null,
      currentCandidateSeed:
        result.candidatesScanned > 0
          ? config.startSeed + result.candidatesScanned - 1
          : null,
      currentDepth: config.floors,
      nextSeed: result.nextSeed,
      exhausted: result.exhausted,
      cancelRequested: false,
      completionReason: result.exhausted
        ? 'exhausted'
        : result.matches.length >= config.maxMatches
          ? 'result-limit'
          : 'scanned',
      matches: result.matches,
      message: result.message,
      error: null,
      startedAt: session.run.startedAt,
      finishedAt: Date.now(),
    })
  } catch (error) {
    if (cancelledIds.has(id)) return
    patchFinderSession(id, {
      ...($finderSessions.get().find((item) => item.id === id)?.run ??
        session.run),
      status: 'error',
      cancelRequested: false,
      completionReason: null,
      error: error instanceof Error ? error.message : String(error),
      finishedAt: Date.now(),
    })
  } finally {
    searchTasks.delete(id)
    cancelledIds.delete(id)
  }
}
