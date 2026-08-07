import {
  type FinderConfig,
  type FinderRunState,
  INITIAL_FINDER_RUN,
  randomStartSeed,
} from '@/components/finder/finder-types'
import type { SeedSearchMatch } from '@/lib/spd-wasm'
import { searchSeedsInWorker, type WorkerTask } from '@/lib/spd-worker-client'
import { AppStore, derivedStore } from './store-utils'

function mergeMatches(
  existing: SeedSearchMatch[],
  incoming: SeedSearchMatch[]
): SeedSearchMatch[] {
  if (incoming.length === 0) return existing
  if (existing.length === 0) return incoming
  const bySeed = new Map(existing.map((match) => [match.seed.numeric, match]))
  for (const match of incoming) {
    bySeed.set(match.seed.numeric, match)
  }
  return Array.from(bySeed.values())
}

export const MAX_FINDER_SESSIONS = 10

export type FinderSession = {
  id: string
  name: string
  config: FinderConfig
  run: FinderRunState
}

export const $finderSessions = new AppStore<FinderSession[]>([])
export const $activeFinderId = new AppStore<string | null>(null)
export const $finderCloseConfirmationId = new AppStore<string | null>(null)
export const $activeFinderSession = derivedStore(
  [$finderSessions, $activeFinderId] as const,
  () => {
    const sessions = $finderSessions.get()
    const activeId = $activeFinderId.get()
    return (
      sessions.find((session) => session.id === activeId) ?? sessions[0] ?? null
    )
  }
)
export const $finderRunning = derivedStore([$finderSessions] as const, () =>
  $finderSessions.get().some((session) => session.run.status === 'running')
)

const cancelledIds = new Set<string>()
const searchTasks = new Map<string, WorkerTask<unknown>>()
let nextFinderId = 1

function discardSearchTask(id: string) {
  const task = searchTasks.get(id)
  if (!task) return
  cancelledIds.add(id)
  task.cancel()
  searchTasks.delete(id)
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

export function setFinderCloseConfirmation(id: string | null) {
  $finderCloseConfirmationId.set(id)
}

export function closeFinderSession(id: string) {
  discardSearchTask(id)
  if ($finderCloseConfirmationId.get() === id) {
    $finderCloseConfirmationId.set(null)
  }
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
    name: String(config.startSeed),
    config,
    run: {
      ...INITIAL_FINDER_RUN,
      status: 'running',
      requestedCandidates: config.candidateCount,
      attemptNumber: 1,
      attemptStartSeed: config.startSeed,
      currentDepth: config.floors,
      nextSeed: config.startSeed,
      startedAt: Date.now(),
      finishedAt: null,
    },
  }
  const existing = $finderSessions.get()
  const activeId = $activeFinderId.get()
  const activeIndex = existing.findIndex((item) => item.id === activeId)
  const replaceActive =
    activeIndex >= 0 && existing[activeIndex].run.matches.length === 0
  if (replaceActive) {
    const replaced = existing[activeIndex]
    discardSearchTask(replaced.id)
    $finderSessions.set(
      existing.map((item, index) => (index === activeIndex ? session : item))
    )
  } else {
    const dropped = existing.slice(
      0,
      Math.max(0, existing.length + 1 - MAX_FINDER_SESSIONS)
    )
    for (const item of dropped) {
      discardSearchTask(item.id)
    }
    $finderSessions.set([...existing.slice(dropped.length), session])
  }
  $activeFinderId.set(id)
  try {
    let attemptStartSeed = config.startSeed
    let attemptNumber = 1
    let accumulatedMatches: SeedSearchMatch[] = []
    while (true) {
      const remainingMatches = config.maxMatches - accumulatedMatches.length
      const { nonStop: _nonStop, ...searchConfig } = {
        ...config,
        startSeed: attemptStartSeed,
        maxMatches: remainingMatches,
      }
      const task = searchSeedsInWorker(
        searchConfig,
        (result) => {
          const current = $finderSessions.get().find((item) => item.id === id)
          if (current?.run.status !== 'running') return
          patchFinderSession(id, {
            ...current.run,
            scanned: result.candidatesScanned,
            nextSeed: result.nextSeed,
            exhausted: result.exhausted,
            matches: mergeMatches(accumulatedMatches, result.matches),
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

      accumulatedMatches = mergeMatches(accumulatedMatches, result.matches)

      // Non-stop keeps retrying from fresh random seeds until the target
      // result count is filled (or the user cancels).
      if (config.nonStop && accumulatedMatches.length < config.maxMatches) {
        attemptStartSeed = randomStartSeed()
        attemptNumber += 1
        const current = $finderSessions.get().find((item) => item.id === id)
        if (current?.run.status !== 'running') return
        patchFinderSession(id, {
          ...current.run,
          scanned: 0,
          attemptNumber,
          attemptStartSeed,
          currentCandidateNumber: null,
          currentCandidateSeed: null,
          nextSeed: attemptStartSeed,
          exhausted: false,
          matches: accumulatedMatches,
          message: null,
        })
        continue
      }

      patchFinderSession(id, {
        status: 'completed',
        scanned: result.candidatesScanned,
        requestedCandidates: config.candidateCount,
        attemptNumber,
        attemptStartSeed,
        currentCandidateNumber: result.candidatesScanned || null,
        currentCandidateSeed:
          result.candidatesScanned > 0
            ? attemptStartSeed + result.candidatesScanned - 1
            : null,
        currentDepth: config.floors,
        nextSeed: result.nextSeed,
        exhausted: result.exhausted,
        cancelRequested: false,
        completionReason:
          accumulatedMatches.length >= config.maxMatches
            ? 'result-limit'
            : result.exhausted
              ? 'exhausted'
              : 'scanned',
        matches: accumulatedMatches,
        message: result.message,
        error: null,
        startedAt: session.run.startedAt,
        finishedAt: Date.now(),
      })
      break
    }
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
