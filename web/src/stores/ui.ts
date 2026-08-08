import {
  type FinderConstraint,
  type FinderNumericInput,
  randomStartSeed,
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
  nonStop: boolean
  constraints: FinderConstraint[]
}

export const $finderForm = new AppStore<FinderFormState>({
  attempted: false,
  cancelCooldown: false,
  startSeed: randomStartSeed(),
  candidateCount: 100,
  floors: 20,
  maxMatches: 10,
  matchMode: 'all',
  nonStop: false,
  constraints: [
    {
      id: 1,
      itemGroup: 'ring',
      className: 'RingOfWealth',
      minLevel: null,
      minDepth: 1,
      maxDepth: 20,
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

export type ReportNavigationState = {
  selectedRegion: string | null
  identitiesTab: 'potions' | 'scrolls' | 'rings'
}

export const $reportNavigation = new AppStore<ReportNavigationState>({
  selectedRegion: null,
  identitiesTab: 'potions',
})

export function setSelectedRegion(region: string) {
  $reportNavigation.set({
    ...$reportNavigation.get(),
    selectedRegion: region,
  })
}

export function setIdentitiesTab(tab: ReportNavigationState['identitiesTab']) {
  $reportNavigation.set({
    ...$reportNavigation.get(),
    identitiesTab: tab,
  })
}

export const $elapsedNow = new AppStore(Date.now())
