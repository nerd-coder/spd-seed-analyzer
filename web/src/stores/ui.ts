import type {
  FinderConstraint,
  FinderNumericInput,
} from '@/components/finder/finder-types'
import type { SeedSearchMatchMode } from '@/lib/spd-wasm'
import { AppStore } from './store-utils'

export type FinderFormState = {
  attempted: boolean
  cancelCooldown: boolean
  startSeed: FinderNumericInput
  candidateCount: FinderNumericInput
  floors: number
  maxMatches: FinderNumericInput
  matchMode: SeedSearchMatchMode
  constraints: FinderConstraint[]
}

export function createFinderFormStore(initialStartSeed: number) {
  return new AppStore<FinderFormState>({
    attempted: false,
    cancelCooldown: false,
    startSeed: initialStartSeed,
    candidateCount: 100,
    floors: 20,
    maxMatches: 10,
    matchMode: 'all',
    constraints: [
      {
        id: 1,
        className: 'RingOfWealth',
        minLevel: null,
        minDepth: 1,
        maxDepth: 20,
      },
    ],
  })
}

export type MapCanvasState = {
  error: string | null
  reducedMotion: boolean
}

export function createMapCanvasStore(reducedMotion: boolean) {
  return new AppStore<MapCanvasState>({ error: null, reducedMotion })
}

export const $elapsedNow = new AppStore(Date.now())
