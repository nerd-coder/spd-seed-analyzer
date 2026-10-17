import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { brotliCompressSync, gzipSync } from 'node:zlib'
import { $ } from 'bun'
import { ROOT } from './native'
import type { Timing, WasmResult } from './types'

const WASM_DIR = join(ROOT, 'web/src/wasm')

type WasmModule = {
  default: (options: {
    module_or_path: Uint8Array
  }) => Promise<{ memory: WebAssembly.Memory }>
  analyze_seed: (input: string, floors: number) => unknown
}

function timing(samples: number[]): Timing {
  const sorted = [...samples].sort((a, b) => a - b)
  const pick = (q: number) => sorted[Math.round((sorted.length - 1) * q)]
  return {
    mean_ms: sorted.reduce((a, b) => a + b, 0) / sorted.length,
    p50_ms: pick(0.5),
    p95_ms: pick(0.95),
    min_ms: sorted[0],
    max_ms: sorted[sorted.length - 1],
  }
}

function wasmOptEnabled(): boolean {
  const toml = readFileSync(join(ROOT, 'crates/spd-wasm/Cargo.toml'), 'utf8')
  return !/wasm-opt\s*=\s*false/.test(toml)
}

export async function buildWasm() {
  await $`bun run build:wasm`.cwd(ROOT)
}

/** Binary weight plus the browser path: WASM generation + JSON hand-off to JS. */
export async function measureWasm(seeds: string[]): Promise<WasmResult> {
  const bytes = readFileSync(join(WASM_DIR, 'spd_wasm_bg.wasm'))
  const wasm = (await import(join(WASM_DIR, 'spd_wasm.js'))) as WasmModule
  const exports = await wasm.default({ module_or_path: bytes })
  wasm.analyze_seed(seeds[0], 26)
  const samples = seeds.map((seed) => {
    const start = performance.now()
    wasm.analyze_seed(seed, 26)
    return performance.now() - start
  })
  return {
    raw_bytes: bytes.length,
    gzip_bytes: gzipSync(bytes, { level: 9 }).length,
    brotli_bytes: brotliCompressSync(bytes).length,
    wasm_opt: wasmOptEnabled(),
    full_report_ms: timing(samples),
    linear_memory_bytes: exports.memory.buffer.byteLength,
    seeds: seeds.length,
  }
}
