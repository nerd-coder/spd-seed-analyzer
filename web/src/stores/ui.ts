import {
  DEFAULT_FLOORS,
  type FinderConstraint,
  type FinderNumericInput,
  randomStartSeed,
} from '@/components/finder/finder-types'
import type { SeedSearchMatchMode } from '@/lib/spd-wasm'
import { AppStore, persistentStore } from './store-utils'

export type FinderFormState = {
  attempted: boolean
  cancelCooldown: boolean
  startSeed: FinderNumericInput
  candidateCount: FinderNumericInput
  maxMatches: FinderNumericInput
  matchMode: SeedSearchMatchMode
  nonStop: boolean
  deepSearch: boolean
  constraints: FinderConstraint[]
}

export const $finderForm = new AppStore<FinderFormState>({
  attempted: false,
  cancelCooldown: false,
  startSeed: randomStartSeed(),
  candidateCount: 100,
  maxMatches: 10,
  matchMode: 'all',
  nonStop: false,
  deepSearch: false,
  constraints: [
    {
      id: 1,
      itemGroup: 'ring',
      className: 'RingOfWealth',
      minLevel: null,
      minDepth: 1,
      maxDepth: DEFAULT_FLOORS,
    },
  ],
})

export type MapCanvasState = {
  error: string | null
  reducedMotion: boolean
}

export function createMapCanvasStore(reducedMotion: boolean) {
  return new AppStore<MapCanvasState>({ error: null, reducedMotion })
}

export const $elapsedNow = new AppStore(Date.now())

const VERSION_ALERT_KEY = 'spd-analyzer-version-alert-dismissed'

export const $dismissedVersionAlert = persistentStore<string | null>(
  VERSION_ALERT_KEY,
  null,
  {
    encode: (value) => value ?? '',
    decode: (value) => (value.length > 0 ? value : null),
  }
)

export function dismissVersionAlert(version: string) {
  $dismissedVersionAlert.set(version)
}
