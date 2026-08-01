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
  /** Number of identical items represented by this entry. */
  quantity: number
  /** Java simple class name for icon lookup. */
  class_name?: string | null
  /** Ordered seed-determined identities when run history can shift a deck index. */
  candidate_classes?: string[]
  category: string
  tier?: number | null
  tier_range?: { min: number; max: number } | null
  level?: number | null
  level_range?: { min: number; max: number } | null
  /** Present when the item is cursed (chip in item list). */
  cursed?: boolean | null
  /** Seed-determined enchantment or glyph, possibly subject to a condition. */
  enchantment?: string | null
  prediction: 'exact' | 'constrained'
  /** Alternative dependency clauses; any one clause may produce this variant. */
  spawn_conditions?: ItemSpawnCondition[]
  /** Informational qualifications that do not control item generation. */
  notes?: string[]
  source?: string | null
}

export type Challenge =
  | 'champion_enemies'
  | 'badder_bosses'
  | 'on_diet'
  | 'faith_is_my_armor'
  | 'pharmacophobia'
  | 'barren_land'
  | 'swarm_intelligence'
  | 'into_darkness'
  | 'forbidden_runes'

export type TrinketEvent = {
  before_depth: number
  kind: 'acquired' | 'upgraded' | 'transmuted'
  trinket?: string
}

export type ArtifactEvent = {
  before_depth: number
  kind: 'obtained' | 'transmuted'
  artifact: string
}

export type ItemDependencyCondition =
  | { type: 'challenge'; challenge: Challenge; enabled: boolean }
  | { type: 'trinket'; events?: TrinketEvent[] }
  | { type: 'artifact'; events?: ArtifactEvent[] }

export type ItemSpawnCondition = {
  all_of?: ItemDependencyCondition[]
}

export type FloorMap = {
  width: number
  height: number
  tileset: string
  tiles: number[]
  tile_variance: number[]
  discoverable: boolean[]
  markers: MapMarker[]
  heaps: MapHeap[]
  mobs: MapMob[]
  transitions: MapTransition[]
  traps: MapTrap[]
  plants: MapPlant[]
  blobs: MapBlob[]
  custom_tiles: MapCustomTile[]
  custom_walls: MapCustomTile[]
}

/** A pinned static `CustomTilemap` layer from the game level. */
export type MapCustomTile = {
  class: string
  texture: string
  x: number
  y: number
  width: number
  height: number
  static_data: number[]
}

export type MapMarkerKind = 'item' | 'mob'

export type MapMarker = {
  cell: number
  kind: MapMarkerKind
  label: string
}

export type MapHeap = {
  cell: number
  heap_type: string
  items: MapHeapItem[]
}

export type MapHeapItem = {
  class: string
  quantity: number
  level: number
  cursed: boolean
}

export type MapMob = {
  cell: number
  class: string
}

export type MapTransition = {
  cell: number
  type: string
  left: number
  top: number
  right: number
  bottom: number
  dest_depth: number
  dest_branch: number
  dest_type: string | null
}

export type MapTrap = {
  cell: number
  class: string
  visible: boolean
  active: boolean
  color: number
  shape: number
}

export type MapPlant = {
  cell: number
  class: string
  image: number
}

export type MapBlobCell = {
  cell: number
  value: number
}

export type MapBlob = {
  class: string
  volume: number
  always_visible: boolean
  cells: MapBlobCell[]
}

export type FloorReport = {
  depth: number
  feeling?: string | null
  builder?: string | null
  rooms?: string[]
  guaranteed_appearances?: GuaranteedAppearance[]
  items: ItemEntry[]
  quests: string[]
  map?: FloorMap | null
  assumed_map?: FloorMap | null
}

export type GuaranteedAppearance = {
  name: string
  kind: 'alchemy_pot'
  source?: string | null
}

export type SeedReport = {
  seed: SeedInfo
  spd_version: string
  spd_commit: string
  floors_requested: number
  identities: IdentityMaps
  trinket_selection: TrinketSelectionReport
  floors: FloorReport[]
  analysis_notes?: string[]
  status: string
  message?: string | null
}

export type TrinketSelectionReport = {
  catalyst_depth: number
  first_alchemy_pot_depth: number
  first_alchemy_pot_is_secret: boolean
  selection_depth: number
  first_effective_depth: number
  catalyst_options: string[]
  transmutation_sequence: string[]
}

export type SeedSearchMatchMode = 'any' | 'all'

export type SeedSearchConstraint = {
  className: string
  minLevel: number | null
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
  level: number
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
