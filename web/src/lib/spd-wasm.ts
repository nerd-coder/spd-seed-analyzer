import init, {
  analyze_seed,
  parse_seed,
  spd_commit,
  spd_version,
} from '@/wasm/spd_wasm'

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

let ready: Promise<void> | null = null

export function ensureWasm(): Promise<void> {
  if (!ready) {
    ready = init().then(() => undefined)
  }
  return ready
}

export async function parseSeed(input: string): Promise<SeedInfo> {
  await ensureWasm()
  return parse_seed(input) as SeedInfo
}

export async function analyzeSeed(
  input: string,
  floors: number
): Promise<SeedReport> {
  await ensureWasm()
  return analyze_seed(input, floors) as SeedReport
}

export async function getSpdMeta(): Promise<{
  version: string
  commit: string
}> {
  await ensureWasm()
  return { version: spd_version(), commit: spd_commit() }
}
