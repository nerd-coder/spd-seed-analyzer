import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { $ } from 'bun'
import type { Hotspot, Hotspots } from './types'

const TOP = 30

type Demangle = (name: string) => string

/** Readable names for raw sampler symbols, via `spd-profile --demangle`. */
function demangler(bin: string, names: string[]): Demangle {
  const unique = [...new Set(names)]
  const result = Bun.spawnSync([bin, '--demangle'], {
    stdin: Buffer.from(unique.join('\n')),
  })
  const out = result.stdout.toString().split('\n')
  const table = new Map(unique.map((name, i) => [name, out[i] || name]))
  return (name) => table.get(name) ?? name
}

function ranked(counts: Map<string, number>, total: number): Hotspot[] {
  return [...counts]
    .sort((a, b) => b[1] - a[1])
    .slice(0, TOP)
    .map(([name, samples]) => ({
      name,
      samples,
      percent: total ? (samples / total) * 100 : 0,
    }))
}

const CALL_LINE = /^([\s+!:|]*)(\d+)\s+(.+?)\s{2}\(in [^)]+\)/
const TOP_LINE = /^\s+(.+?)\s{2}\(in [^)]+\)\s+(\d+)\s*$/

/** Parse macOS `sample` output: self time and recursion-safe inclusive time. */
export function parseSample(text: string, bin: string): Hotspots {
  const lines = text.split('\n')
  const raw = lines.flatMap((line) => {
    const name = line.match(CALL_LINE)?.[3] ?? line.match(TOP_LINE)?.[1]
    return name ? [name] : []
  })
  const clean = demangler(bin, raw)
  const graphStart = lines.findIndex((l) => l.startsWith('Call graph:'))
  const topStart = lines.findIndex((l) => l.startsWith('Sort by top of stack'))
  const inclusive = new Map<string, number>()
  const stack: { depth: number; name: string }[] = []
  let total = 0
  for (const line of lines.slice(graphStart + 1)) {
    if (line.startsWith('Total number in stack') || line.trim() === '') break
    const match = line.match(CALL_LINE)
    if (!match) continue
    const depth = match[1].length
    const count = Number(match[2])
    const name = clean(match[3])
    if (stack.length === 0 && total === 0) total = count
    while (stack.length && stack[stack.length - 1].depth >= depth) stack.pop()
    if (name.includes('spd_') && !stack.some((frame) => frame.name === name)) {
      inclusive.set(name, (inclusive.get(name) ?? 0) + count)
    }
    stack.push({ depth, name })
  }
  const self = new Map<string, number>()
  for (const line of lines.slice(topStart + 1)) {
    const match = line.match(TOP_LINE)
    if (!match) break
    const name = clean(match[1])
    self.set(name, (self.get(name) ?? 0) + Number(match[2]))
  }
  return {
    tool: 'macOS sample',
    total_samples: total,
    self: ranked(self, total),
    inclusive: ranked(inclusive, total),
  }
}

function parsePerf(text: string, children: boolean, bin: string): Hotspot[] {
  const row = children
    ? /^\s+([\d.]+)%\s+[\d.]+%\s+\[[.k]\]\s+(.+)$/
    : /^\s+([\d.]+)%\s+\[[.k]\]\s+(.+)$/
  const rows = text
    .split('\n')
    .map((line) => line.match(row))
    .filter((m): m is RegExpMatchArray => m !== null)
  const clean = demangler(
    bin,
    rows.map((m) => m[2])
  )
  return rows
    .map((m) => ({ name: clean(m[2]), samples: 0, percent: Number(m[1]) }))
    .filter((h) => !children || h.name.includes('spd_'))
    .slice(0, TOP)
}

async function samplePerf(bin: string, seconds: number, dir: string) {
  const data = join(dir, 'perf.data')
  await $`perf record -F 999 -g -o ${data} -- ${bin} --loop ${seconds}`.quiet()
  const report = (flag: string) =>
    $`perf report -i ${data} --stdio ${flag} --sort symbol -g none --percent-limit 0.3`
      .quiet()
      .text()
  return {
    tool: 'perf',
    total_samples: 0,
    self: parsePerf(await report('--no-children'), false, bin),
    inclusive: parsePerf(await report('--children'), true, bin),
  }
}

async function sampleMac(bin: string, seconds: number, dir: string) {
  const out = join(dir, 'sample.txt')
  const child = Bun.spawn([bin, '--loop', String(seconds + 2)], {
    stdout: 'ignore',
  })
  await Bun.sleep(500)
  await $`/usr/bin/sample ${child.pid} ${seconds} -file ${out}`.quiet()
  child.kill()
  await child.exited
  return parseSample(readFileSync(out, 'utf8'), bin)
}

/** Sample the 26-floor full report loop. Returns a note when no sampler exists. */
export async function collectHotspots(
  bin: string,
  seconds: number,
  dir: string
): Promise<{ hotspots: Hotspots | null; note: string | null }> {
  try {
    if (process.platform === 'darwin') {
      return { hotspots: await sampleMac(bin, seconds, dir), note: null }
    }
    if ((await $`which perf`.quiet().nothrow()).exitCode === 0) {
      return { hotspots: await samplePerf(bin, seconds, dir), note: null }
    }
    return { hotspots: null, note: 'No sampler: install `perf` (Linux).' }
  } catch (error) {
    return { hotspots: null, note: `Sampling failed: ${error}` }
  }
}
