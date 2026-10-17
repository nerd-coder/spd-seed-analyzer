#!/usr/bin/env bun
/**
 * Engine profiler: `bun run profile [--seeds N] [--sample-seconds S]
 * [--no-hotspots] [--no-wasm] [--no-wasm-build] [--open]`.
 *
 * Writes `profile-report/index.html`, `profile-report/report.json`, and a
 * timestamped copy under `profile-report/history/` used for run-over-run deltas.
 */
import { existsSync, mkdirSync, readdirSync, readFileSync } from 'node:fs'
import { cpus, release, type } from 'node:os'
import { join } from 'node:path'
import { parseArgs } from 'node:util'
import { $ } from 'bun'
import { collectHotspots } from './hotspots'
import { renderHtml } from './html'
import { buildProfiler, ROOT, runNative } from './native'
import { analyzePayload } from './payload'
import type { Environment, ProfileReport } from './types'
import { buildWasm, measureWasm } from './wasm'

const { values: args } = parseArgs({
  options: {
    seeds: { type: 'string', default: '100' },
    'sample-seconds': { type: 'string', default: '8' },
    'no-hotspots': { type: 'boolean', default: false },
    'no-wasm': { type: 'boolean', default: false },
    'no-wasm-build': { type: 'boolean', default: false },
    open: { type: 'boolean', default: false },
  },
})

const OUT = join(ROOT, 'profile-report')
const HISTORY = join(OUT, 'history')
const seeds = Number(args.seeds)
const step = (label: string) => console.log(`\n▸ ${label}`)

async function environment(): Promise<Environment> {
  const text = async (cmd: ReturnType<typeof $>) =>
    (await cmd.quiet().nothrow().text()).trim()
  return {
    date: new Date().toISOString().replace(/\.\d+Z$/, 'Z'),
    git_commit: await text($`git rev-parse --short HEAD`.cwd(ROOT)),
    git_dirty: (await text($`git status --porcelain`.cwd(ROOT))).length > 0,
    cpu: cpus()[0]?.model ?? 'unknown',
    os: `${type()} ${release()}`,
    rustc: await text($`rustc --version`.cwd(ROOT)),
  }
}

function previousRun(): ProfileReport | null {
  if (!existsSync(HISTORY)) return null
  const last = readdirSync(HISTORY)
    .filter((f) => f.endsWith('.json'))
    .sort()
    .at(-1)
  return last ? JSON.parse(readFileSync(join(HISTORY, last), 'utf8')) : null
}

mkdirSync(HISTORY, { recursive: true })
const env = await environment()

step('Building spd-profile (profiling profile)')
const bin = await buildProfiler()

step(`Native benchmarks (${seeds} seeds)`)
const dumpPath = join(OUT, 'sample-report.json')
const native = await runNative(bin, seeds, dumpPath)
const payload = analyzePayload(readFileSync(dumpPath, 'utf8'))

let hotspots: ProfileReport['hotspots'] = null
let hotspotNote: string | null = 'Skipped (--no-hotspots).'
if (!args['no-hotspots']) {
  step(`Sampling hotspots (${args['sample-seconds']}s)`)
  const sampled = await collectHotspots(
    bin,
    Number(args['sample-seconds']),
    OUT
  )
  hotspots = sampled.hotspots
  hotspotNote = sampled.note
}

let wasm: ProfileReport['wasm'] = null
if (!args['no-wasm']) {
  if (!args['no-wasm-build']) {
    step('Building WASM')
    await buildWasm()
  }
  step('WASM size + speed')
  wasm = await measureWasm(
    Array.from({ length: Math.min(seeds, 30) }, (_, i) =>
      String(i * 7_919_993 + 1)
    )
  )
}

const report: ProfileReport = {
  environment: env,
  native,
  hotspots,
  hotspot_note: hotspotNote,
  wasm,
  payload,
}
const previous = previousRun()
const html = renderHtml(report, previous)
const json = JSON.stringify(report, null, 2)
await Bun.write(join(OUT, 'index.html'), html)
await Bun.write(join(OUT, 'report.json'), json)
await Bun.write(join(HISTORY, `${env.date.replace(/[:]/g, '-')}.json`), json)

const full = native.workloads.find(
  (w) => w.name === 'full report' && w.floors === 26
)
console.log(`
Full report (26 floors): ${full?.generate.mean_ms.toFixed(1)} ms/seed native${
  wasm?.full_report_ms
    ? `, ${wasm.full_report_ms.mean_ms.toFixed(1)} ms/seed WASM`
    : ''
}
Peak heap: ${((full?.memory.peak_heap_bytes ?? 0) / 1e6).toFixed(2)} MB · JSON: ${(payload.total_bytes / 1e3).toFixed(0)} KB${
  wasm ? ` · WASM: ${(wasm.brotli_bytes / 1e3).toFixed(0)} KB brotli` : ''
}
Report: ${join(OUT, 'index.html')}`)

if (args.open) {
  await $`${process.platform === 'darwin' ? 'open' : 'xdg-open'} ${join(OUT, 'index.html')}`.nothrow()
}
