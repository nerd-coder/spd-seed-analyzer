/**
 * App-wide state via nanostores — public façade.
 *
 * Implementation is split by concern:
 * - `sessions` — seed tabs, analyze, rehydrate
 * - `spoilers` — map / identity spoiler flags
 * - `meta` — SPD version/commit from wasm
 *
 * Consumers should keep importing from `@/stores/app`.
 */

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
