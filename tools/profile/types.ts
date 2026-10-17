export type Timing = {
  mean_ms: number
  p50_ms: number
  p95_ms: number
  min_ms: number
  max_ms: number
}

export type Memory = {
  peak_heap_bytes: number
  allocations_per_seed: number
  allocated_bytes_per_seed: number
}

export type Workload = {
  name: string
  floors: number
  seeds: number
  generate: Timing
  serialize: Timing
  json_bytes_per_seed: number
  memory: Memory
}

export type Search = {
  name: string
  floors: number
  candidates: number
  seeds_per_second: number
  ms_per_seed: number
  matches: number
  memory: Memory
}

export type NativeResult = {
  seeds: number
  workloads: Workload[]
  searches: Search[]
  binary_bytes: number
  /** Process peak RSS while looping the 26-floor full report. */
  peak_rss_bytes: number | null
}

export type Hotspot = { name: string; samples: number; percent: number }

export type Hotspots = {
  tool: string
  total_samples: number
  self: Hotspot[]
  inclusive: Hotspot[]
}

export type WasmResult = {
  raw_bytes: number
  gzip_bytes: number
  brotli_bytes: number
  wasm_opt: boolean
  full_report_ms: Timing | null
  linear_memory_bytes: number | null
  seeds: number
}

export type PayloadEntry = { path: string; bytes: number }

export type Payload = {
  total_bytes: number
  gzip_bytes: number
  brotli_bytes: number
  entries: PayloadEntry[]
}

export type Environment = {
  date: string
  git_commit: string
  git_dirty: boolean
  cpu: string
  os: string
  rustc: string
}

export type ProfileReport = {
  environment: Environment
  native: NativeResult
  hotspots: Hotspots | null
  hotspot_note: string | null
  wasm: WasmResult | null
  payload: Payload
}
