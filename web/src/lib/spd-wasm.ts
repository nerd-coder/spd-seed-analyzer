import init, * as wasmBindings from '@/wasm/spd_wasm'

export type SeedInfo = {
  input: string
  numeric: number
  code: string | null
  formatted: string
}

export type IdentityEntry = {
  item: string
  name: string
  appearance: string
}

export type IdentityMaps = {
  potions: IdentityEntry[]
  scrolls: IdentityEntry[]
  rings: IdentityEntry[]
}

export type ItemEntry = {
  name: string
  /** Java simple class name for icon lookup. */
  class_name?: string | null
  category: string
  /** Present when the item is cursed (chip in item list). */
  cursed?: boolean
  source?: string | null
}

export type FloorMap = {
  width: number
  height: number
  tileset: string
  tiles: number[]
  tile_variance: number[]
  markers: MapMarker[]
}

export type MapMarkerKind = 'item' | 'mob'

export type MapMarker = {
  cell: number
  kind: MapMarkerKind
  label: string
}

export type FloorReport = {
  depth: number
  feeling?: string | null
  builder?: string | null
  rooms?: string[]
  items: ItemEntry[]
  quests: string[]
  map?: FloorMap | null
}

export type SeedReport = {
  seed: SeedInfo
  spd_version: string
  spd_commit: string
  floors_requested: number
  identities: IdentityMaps
  floors: FloorReport[]
  status: string
  message?: string | null
}

export type SeedSearchMatchMode = 'any' | 'all'

export type SeedSearchConstraint = {
  className: string
  minDepth: number
  maxDepth: number
}

export type SeedSearchRequest = {
  startSeed: number
  candidateCount: number
  floors: number
  constraints: SeedSearchConstraint[]
  matchMode: SeedSearchMatchMode
  maxMatches: number
}

export type SeedSearchEvidence = {
  constraintIndex: number
  className: string
  depth: number
  name: string
  source?: string | null
}

export type SeedSearchMatch = {
  seed: SeedInfo
  evidence: SeedSearchEvidence[]
}

export type SeedSearchResult = {
  startSeed: number
  requestedCandidates: number
  candidatesScanned: number
  nextSeed: number | null
  exhausted: boolean
  matchLimitReached: boolean
  matchMode: SeedSearchMatchMode
  matches: SeedSearchMatch[]
  status: string
  message: string
}

type SearchBinding = (request: SeedSearchRequest) => SeedSearchResult

let ready: Promise<void> | null = null

export function ensureWasm(): Promise<void> {
  if (!ready) {
    ready = init().then(() => undefined)
  }
  return ready
}

export async function parseSeed(input: string): Promise<SeedInfo> {
  await ensureWasm()
  return wasmBindings.parse_seed(input) as SeedInfo
}

export async function analyzeSeed(
  input: string,
  floors: number
): Promise<SeedReport> {
  await ensureWasm()
  return wasmBindings.analyze_seed(input, floors) as SeedReport
}

export async function searchSeeds(
  request: SeedSearchRequest
): Promise<SeedSearchResult> {
  await ensureWasm()
  const search = (wasmBindings as unknown as { search_seeds?: SearchBinding })
    .search_seeds
  if (!search) {
    throw new Error('Seed search is unavailable. Rebuild the WASM package.')
  }
  return search(request)
}

export async function getSpdMeta(): Promise<{
  version: string
  commit: string
}> {
  await ensureWasm()
  return {
    version: wasmBindings.spd_version(),
    commit: wasmBindings.spd_commit(),
  }
}
