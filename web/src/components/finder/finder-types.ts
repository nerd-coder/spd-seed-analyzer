import type {
  SeedSearchConstraint,
  SeedSearchMatch,
  SeedSearchMatchMode,
} from '@/lib/spd-wasm'

export const TOTAL_SEEDS = 5_429_503_678_976
export const MIN_CANDIDATES = 10
export const MAX_CANDIDATES = 10_000
export const MAX_CONSTRAINTS = 32
export const MAX_FLOORS = 26
export const MAX_RESULTS = 100

export function randomStartSeed(): number {
  const values = new Uint32Array(2)
  crypto.getRandomValues(values)
  const random53 = (BigInt(values[0] & 0x1fffff) << 32n) | BigInt(values[1])
  return Number(random53 % BigInt(TOTAL_SEEDS))
}

export type FinderNumericInput = number | ''

export function isIntegerInRange(
  value: FinderNumericInput,
  min: number,
  max: number
) {
  return (
    typeof value === 'number' &&
    Number.isInteger(value) &&
    value >= min &&
    value <= max
  )
}

export type FinderConstraint = SeedSearchConstraint & { id: number }

export type FinderConfig = {
  startSeed: number
  candidateCount: number
  floors: number
  constraints: SeedSearchConstraint[]
  matchMode: SeedSearchMatchMode
  maxMatches: number
  nonStop: boolean
}

export type FinderCompletionReason = 'scanned' | 'result-limit' | 'exhausted'
export type FinderStatus =
  | 'idle'
  | 'running'
  | 'completed'
  | 'cancelled'
  | 'error'

export type FinderRunState = {
  status: FinderStatus
  scanned: number
  requestedCandidates: number
  attemptNumber: number
  attemptStartSeed: number | null
  currentCandidateNumber: number | null
  currentCandidateSeed: number | null
  currentDepth: number | null
  nextSeed: number | null
  exhausted: boolean
  cancelRequested: boolean
  completionReason: FinderCompletionReason | null
  matches: SeedSearchMatch[]
  message: string | null
  error: string | null
  startedAt: number | null
  finishedAt: number | null
}

export const INITIAL_FINDER_RUN: FinderRunState = {
  status: 'idle',
  scanned: 0,
  requestedCandidates: 0,
  attemptNumber: 0,
  attemptStartSeed: null,
  currentCandidateNumber: null,
  currentCandidateSeed: null,
  currentDepth: null,
  nextSeed: null,
  exhausted: false,
  cancelRequested: false,
  completionReason: null,
  matches: [],
  message: null,
  error: null,
  startedAt: null,
  finishedAt: null,
}
