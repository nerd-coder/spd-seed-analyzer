import type {
  SeedSearchConstraint,
  SeedSearchMatch,
  SeedSearchMatchMode,
} from '@/lib/spd-wasm'

export const TOTAL_SEEDS = 5_429_503_678_976
export const MAX_CANDIDATES = 250
export const MAX_CONSTRAINTS = 32
export const MAX_FLOORS = 26
export const MAX_RESULTS = 100

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

export type FinderConstraint = Omit<
  SeedSearchConstraint,
  'minDepth' | 'maxDepth'
> & {
  id: number
  minDepth: FinderNumericInput
  maxDepth: FinderNumericInput
}

export type FinderConfig = {
  startSeed: number
  candidateCount: number
  floors: number
  constraints: SeedSearchConstraint[]
  matchMode: SeedSearchMatchMode
  maxMatches: number
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
  nextSeed: number | null
  exhausted: boolean
  cancelRequested: boolean
  completionReason: FinderCompletionReason | null
  matches: SeedSearchMatch[]
  message: string | null
  error: string | null
}

export const INITIAL_FINDER_RUN: FinderRunState = {
  status: 'idle',
  scanned: 0,
  requestedCandidates: 0,
  nextSeed: null,
  exhausted: false,
  cancelRequested: false,
  completionReason: null,
  matches: [],
  message: null,
  error: null,
}
