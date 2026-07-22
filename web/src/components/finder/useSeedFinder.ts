import { useCallback, useEffect, useRef, useState } from 'react'
import { type SeedSearchMatch, searchSeeds } from '@/lib/spd-wasm'
import {
  type FinderConfig,
  type FinderRunState,
  INITIAL_FINDER_RUN,
  MAX_RESULTS,
} from './finder-types'

const SEARCH_CHUNK_SIZE = 5

function yieldToBrowser(): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, 0)
  })
}

export function useSeedFinder() {
  const [run, setRun] = useState<FinderRunState>(INITIAL_FINDER_RUN)
  const cancelledRef = useRef(false)
  const mountedRef = useRef(true)
  const generationRef = useRef(0)

  useEffect(() => {
    mountedRef.current = true
    return () => {
      mountedRef.current = false
      cancelledRef.current = true
      generationRef.current += 1
    }
  }, [])

  const cancel = useCallback(() => {
    cancelledRef.current = true
    setRun((current) =>
      current.status === 'running'
        ? { ...current, cancelRequested: true }
        : current
    )
  }, [])

  const start = useCallback(async (config: FinderConfig) => {
    const generation = generationRef.current + 1
    generationRef.current = generation
    cancelledRef.current = false

    let scanned = 0
    let cursor: number | null = config.startSeed
    let exhausted = false
    let message: string | null = null
    const matchesBySeed = new Map<number, SeedSearchMatch>()

    const isCurrent = () =>
      mountedRef.current && generationRef.current === generation

    setRun({
      ...INITIAL_FINDER_RUN,
      status: 'running',
      requestedCandidates: config.candidateCount,
      nextSeed: config.startSeed,
    })

    // Let the running state paint before the first synchronous WASM chunk.
    await yieldToBrowser()

    try {
      while (
        scanned < config.candidateCount &&
        matchesBySeed.size < config.maxMatches &&
        cursor !== null &&
        !exhausted
      ) {
        if (cancelledRef.current || !isCurrent()) break

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

        if (!isCurrent()) return
        setRun({
          status: 'running',
          scanned,
          requestedCandidates: config.candidateCount,
          nextSeed: cursor,
          exhausted,
          cancelRequested: cancelledRef.current,
          completionReason: null,
          matches: Array.from(matchesBySeed.values()),
          message,
          error: null,
        })

        if (
          result.candidatesScanned === 0 ||
          matchesBySeed.size >= config.maxMatches ||
          exhausted ||
          cursor === null
        ) {
          break
        }

        // Cancel can take effect and progress can paint between WASM calls.
        await yieldToBrowser()
      }

      if (!isCurrent()) return
      const wasCancelled = cancelledRef.current
      setRun({
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
      if (!isCurrent()) return
      setRun({
        status: cancelledRef.current ? 'cancelled' : 'error',
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
    }
  }, [])

  return { run, start, cancel }
}
