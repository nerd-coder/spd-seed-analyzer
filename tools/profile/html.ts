import type { Hotspot, PayloadEntry, ProfileReport, Workload } from './types'

const esc = (s: string) =>
  s.replace(
    /[&<>"]/g,
    (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c] ?? c
  )
const ms = (v: number) => (v >= 100 ? v.toFixed(0) : v.toFixed(2))
const num = (v: number) => Math.round(v).toLocaleString('en-US')
function bytes(v: number | null | undefined): string {
  if (v == null) return '—'
  if (v >= 1e6) return `${(v / 1e6).toFixed(2)} MB`
  if (v >= 1e3) return `${(v / 1e3).toFixed(1)} KB`
  return `${v} B`
}

/** Signed change vs the previous run; green when it moved the better way. */
function delta(
  cur?: number | null,
  prev?: number | null,
  lowerIsBetter = true
) {
  if (cur == null || prev == null || prev === 0) return ''
  const pct = ((cur - prev) / prev) * 100
  if (Math.abs(pct) < 1) return '<span class="d same">±0%</span>'
  const good = lowerIsBetter ? pct < 0 : pct > 0
  return `<span class="d ${good ? 'good' : 'bad'}">${pct > 0 ? '+' : ''}${pct.toFixed(0)}%</span>`
}

function card(label: string, value: string, change: string, hint = '') {
  return `<div class="card"><div class="label">${label}</div><div class="value">${value} ${change}</div><div class="hint">${hint}</div></div>`
}

function bar(percent: number) {
  return `<div class="bar"><span style="width:${Math.min(100, percent).toFixed(1)}%"></span></div>`
}

const key = (w: Workload) => `${w.name}/${w.floors}`

function workloadRows(report: ProfileReport, prev: ProfileReport | null) {
  const before = new Map(prev?.native.workloads.map((w) => [key(w), w]))
  return report.native.workloads
    .map((w) => {
      const p = before.get(key(w))
      return `<tr><td>${esc(w.name)}</td><td>${w.floors}</td>
<td>${ms(w.generate.mean_ms)} ${delta(w.generate.mean_ms, p?.generate.mean_ms)}</td>
<td>${ms(w.generate.p50_ms)}</td><td>${ms(w.generate.p95_ms)}</td>
<td>${ms(w.serialize.mean_ms)}</td>
<td>${bytes(w.json_bytes_per_seed)} ${delta(w.json_bytes_per_seed, p?.json_bytes_per_seed)}</td>
<td>${bytes(w.memory.peak_heap_bytes)} ${delta(w.memory.peak_heap_bytes, p?.memory.peak_heap_bytes)}</td>
<td>${num(w.memory.allocations_per_seed)} ${delta(w.memory.allocations_per_seed, p?.memory.allocations_per_seed)}</td>
<td>${bytes(w.memory.allocated_bytes_per_seed)}</td></tr>`
    })
    .join('')
}

function searchRows(report: ProfileReport, prev: ProfileReport | null) {
  return report.native.searches
    .map((s) => {
      const p = prev?.native.searches.find((x) => x.floors === s.floors)
      return `<tr><td>${esc(s.name)}</td><td>${s.floors}</td><td>${num(s.candidates)}</td>
<td>${num(s.seeds_per_second)} ${delta(s.seeds_per_second, p?.seeds_per_second, false)}</td>
<td>${ms(s.ms_per_seed)}</td><td>${s.matches}</td>
<td>${bytes(s.memory.peak_heap_bytes)}</td><td>${num(s.memory.allocations_per_seed)}</td></tr>`
    })
    .join('')
}

function payloadRows(entries: PayloadEntry[], total: number) {
  return entries
    .slice(0, 25)
    .map((e) => {
      const pct = (e.bytes / total) * 100
      return `<tr><td><code>${esc(e.path)}</code></td><td>${bytes(e.bytes)}</td><td>${pct.toFixed(1)}%</td><td class="w">${bar(pct)}</td></tr>`
    })
    .join('')
}

function hotspotRows(list: Hotspot[]) {
  return list
    .map(
      (h) =>
        `<tr><td title="${esc(h.name)}"><code>${esc(h.name.length > 90 ? `${h.name.slice(0, 87)}…` : h.name)}</code></td><td>${h.percent.toFixed(1)}%</td><td class="w">${bar(h.percent)}</td></tr>`
    )
    .join('')
}

function hotspotSection(report: ProfileReport) {
  const h = report.hotspots
  if (!h)
    return `<p class="muted">${esc(report.hotspot_note ?? 'Skipped.')}</p>`
  return `<p class="muted">${esc(h.tool)}${h.total_samples ? `, ${num(h.total_samples)} samples` : ''} of the 26-floor full report loop.</p>
<div class="grid2"><div><h3>Self time</h3><table><tr><th>Function</th><th>%</th><th></th></tr>${hotspotRows(h.self)}</table></div>
<div><h3>Inclusive (spd crates)</h3><table><tr><th>Function</th><th>%</th><th></th></tr>${hotspotRows(h.inclusive)}</table></div></div>`
}

export function renderHtml(report: ProfileReport, prev: ProfileReport | null) {
  const env = report.environment
  const full = report.native.workloads.find(
    (w) => w.name === 'full report' && w.floors === 26
  )
  const prevFull = prev?.native.workloads.find(
    (w) => w.name === 'full report' && w.floors === 26
  )
  const search = report.native.searches.find((s) => s.floors === 26)
  const prevSearch = prev?.native.searches.find((s) => s.floors === 26)
  const wasm = report.wasm
  const pw = prev?.wasm
  const pay = report.payload
  const cards = [
    card(
      'Full report, 26 floors (native)',
      `${ms(full?.generate.mean_ms ?? 0)} ms`,
      delta(full?.generate.mean_ms, prevFull?.generate.mean_ms),
      'mean per seed'
    ),
    card(
      'Full report, 26 floors (WASM)',
      wasm?.full_report_ms ? `${ms(wasm.full_report_ms.mean_ms)} ms` : '—',
      delta(wasm?.full_report_ms?.mean_ms, pw?.full_report_ms?.mean_ms),
      'incl. JSON hand-off to JS'
    ),
    card(
      'Peak heap (full, 26 floors)',
      bytes(full?.memory.peak_heap_bytes),
      delta(full?.memory.peak_heap_bytes, prevFull?.memory.peak_heap_bytes),
      `process RSS ${bytes(report.native.peak_rss_bytes)}`
    ),
    card(
      'Report JSON',
      bytes(pay.total_bytes),
      delta(pay.total_bytes, prev?.payload.total_bytes),
      `gzip ${bytes(pay.gzip_bytes)} · br ${bytes(pay.brotli_bytes)}`
    ),
    card(
      'WASM binary (brotli)',
      bytes(wasm?.brotli_bytes),
      delta(wasm?.brotli_bytes, pw?.brotli_bytes),
      wasm
        ? `raw ${bytes(wasm.raw_bytes)} · gzip ${bytes(wasm.gzip_bytes)}`
        : 'skipped'
    ),
    card(
      'Search, 26 floors',
      `${num(search?.seeds_per_second ?? 0)}/s`,
      delta(search?.seeds_per_second, prevSearch?.seeds_per_second, false),
      'seeds per second, one core'
    ),
  ].join('')
  const compared = prev
    ? `Compared with ${esc(prev.environment.date)} (${esc(prev.environment.git_commit)}${prev.environment.git_dirty ? ', dirty' : ''}).`
    : 'No previous run to compare with.'
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>spd-core profile · ${esc(env.date)}</title><style>
:root{color-scheme:dark;--bg:#14161b;--panel:#1d2027;--line:#2c3039;--text:#e6e8ee;--muted:#8d93a1;--accent:#e0a84a;--good:#5cc98a;--bad:#ef6b6b}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:14px/1.5 ui-sans-serif,system-ui,sans-serif}
main{max-width:1180px;margin:0 auto;padding:32px 20px 64px}h1{margin:0 0 4px;font-size:24px}h2{margin:40px 0 12px;font-size:18px;color:var(--accent)}h3{font-size:14px;margin:16px 0 8px}
.muted,.hint,.label{color:var(--muted)}.cards{display:grid;grid-template-columns:repeat(auto-fill,minmax(250px,1fr));gap:12px;margin-top:24px}
.card{background:var(--panel);border:1px solid var(--line);border-radius:10px;padding:14px 16px}.value{font-size:22px;font-weight:600;margin:4px 0}.label,.hint{font-size:12px}
table{width:100%;border-collapse:collapse;background:var(--panel);border:1px solid var(--line);border-radius:10px;overflow:hidden}
th,td{padding:7px 10px;border-bottom:1px solid var(--line);text-align:left;white-space:nowrap;font-variant-numeric:tabular-nums}th{color:var(--muted);font-weight:500;font-size:12px}
td code{font-size:12px}.w{width:30%}.bar{height:8px;background:var(--line);border-radius:4px;overflow:hidden}.bar span{display:block;height:100%;background:var(--accent)}
.d{font-size:12px;font-weight:600;margin-left:4px}.good{color:var(--good)}.bad{color:var(--bad)}.same{color:var(--muted)}
.grid2{display:grid;grid-template-columns:1fr 1fr;gap:16px}.scroll{overflow-x:auto}@media(max-width:900px){.grid2{grid-template-columns:1fr}}
</style></head><body><main>
<h1>spd-core engine profile</h1>
<div class="muted">${esc(env.date)} · ${esc(env.git_commit)}${env.git_dirty ? ' (dirty)' : ''} · ${esc(env.cpu)} · ${esc(env.os)} · ${esc(env.rustc)}</div>
<div class="muted">${compared} Deltas: <span class="good">better</span> / <span class="bad">worse</span>.</div>
<div class="cards">${cards}</div>
<h2>Native workloads</h2><p class="muted">${report.native.seeds} seeds, single thread, release + LTO profile with line tables. Heap counters come from a counting allocator.</p>
<div class="scroll"><table><tr><th>Workload</th><th>Floors</th><th>Generate mean (ms)</th><th>p50</th><th>p95</th><th>JSON (ms)</th><th>JSON size</th><th>Peak heap</th><th>Allocs / seed</th><th>Alloc bytes / seed</th></tr>${workloadRows(report, prev)}</table></div>
<h2>Seed search</h2><div class="scroll"><table><tr><th>Query</th><th>Floors</th><th>Candidates</th><th>Seeds / s</th><th>ms / seed</th><th>Matches</th><th>Peak heap</th><th>Allocs / seed</th></tr>${searchRows(report, prev)}</table></div>
<h2>WASM</h2>${
    wasm
      ? `<table><tr><th>wasm-opt</th><th>Raw</th><th>gzip</th><th>brotli</th><th>Full report 26 floors mean</th><th>p95</th><th>Linear memory</th></tr>
<tr><td>${wasm.wasm_opt ? 'on' : 'off'}</td><td>${bytes(wasm.raw_bytes)} ${delta(wasm.raw_bytes, pw?.raw_bytes)}</td><td>${bytes(wasm.gzip_bytes)}</td><td>${bytes(wasm.brotli_bytes)}</td>
<td>${wasm.full_report_ms ? `${ms(wasm.full_report_ms.mean_ms)} ms ${delta(wasm.full_report_ms.mean_ms, pw?.full_report_ms?.mean_ms)}` : '—'}</td><td>${wasm.full_report_ms ? `${ms(wasm.full_report_ms.p95_ms)} ms` : '—'}</td><td>${bytes(wasm.linear_memory_bytes)}</td></tr></table>
<p class="muted">Measured in Bun (JavaScriptCore) over ${wasm.seeds} seeds; browsers differ. Linear memory never shrinks, so it is the high-water mark.</p>`
      : '<p class="muted">Skipped.</p>'
  }
<h2>Report payload weight</h2><p class="muted">26-floor full report for one seed, ${bytes(pay.total_bytes)} total; per-floor fields summed across floors.</p>
<table><tr><th>Field</th><th>Bytes</th><th>Share</th><th></th></tr>${payloadRows(pay.entries, pay.total_bytes)}</table>
<h2>Hotspots</h2>${hotspotSection(report)}
</main></body></html>`
}
