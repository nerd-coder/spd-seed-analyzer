/**
 * App-wide state via nanostores — public façade.
 *
 * Implementation is split by concern:
 * - `sessions` — seed tabs, analyze, rehydrate
 * - `spoilers` — map / identity spoiler flags
 * - `meta` — SPD version/commit from wasm
 * - `theme` — light / dark / system preference
 * - `mode` — current mode (analyze / finder), persisted
 *
 * Consumers should keep importing from `@/stores/app`.
 */

import { persistentAtom } from '@nanostores/persistent'

export { $meta, loadSpdMeta, type SpdMeta } from './meta'
export {
  $activeSeedId,
  $analyzing,
  $formError,
  $savedSeedInputs,
  $seedInput,
  $sessionCount,
  $sessions,
  ANALYZE_FLOORS,
  analyzeDraftSeed,
  analyzeSeedInput,
  closeSeedSession,
  MAX_SAVED_SEEDS,
  normalizeSeedInput,
  type SeedSession,
  type SessionStatus,
  sessionIdFor,
  setActiveSeed,
  setSeedInput,
  startSessionRehydrate,
  tabLabel,
} from './sessions'
export {
  $identitySpoilers,
  $mapSpoilers,
  setIdentitySpoilers,
  setMapSpoilers,
} from './spoilers'
export {
  $theme,
  applyTheme,
  cycleTheme,
  initTheme,
  resolvedTheme,
  setTheme,
  type Theme,
} from './theme'

/**
 * Mode: analyze or find-seed. Persisted to localStorage via nanostores/persistent.
 */
export type AppMode = 'analyze' | 'finder'

const MODE_KEY = 'spd-analyzer-mode'
const modeCodec = {
  encode: (v: AppMode) => v,
  decode: (v: string): AppMode =>
    v === 'analyze' || v === 'finder' ? v : 'analyze',
}

export const $mode = persistentAtom<AppMode>(MODE_KEY, 'analyze', modeCodec)

export function setMode(value: AppMode) {
  $mode.set(value)
}
