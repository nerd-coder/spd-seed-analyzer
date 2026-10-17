import { brotliCompressSync, gzipSync } from 'node:zlib'
import type { Payload, PayloadEntry } from './types'

type Json = Record<string, unknown>

const size = (value: unknown) => JSON.stringify(value)?.length ?? 0

function add(totals: Map<string, number>, path: string, value: unknown) {
  totals.set(path, (totals.get(path) ?? 0) + size(value))
}

/** Byte weight of the 26-floor full report, aggregated across floors. */
export function analyzePayload(json: string): Payload {
  const report = JSON.parse(json) as Json
  const totals = new Map<string, number>()
  for (const [key, value] of Object.entries(report)) {
    if (key !== 'floors') add(totals, key, value)
  }
  for (const floor of (report.floors as Json[]) ?? []) {
    for (const [key, value] of Object.entries(floor)) {
      const isMap = (key === 'map' || key === 'assumed_map') && value
      if (!isMap) {
        add(totals, `floors[].${key}`, value)
        continue
      }
      for (const [field, inner] of Object.entries(value as Json)) {
        add(totals, `floors[].${key}.${field}`, inner)
      }
    }
  }
  const entries: PayloadEntry[] = [...totals]
    .map(([path, bytes]) => ({ path, bytes }))
    .filter((entry) => entry.bytes > 0)
    .sort((a, b) => b.bytes - a.bytes)
  const buffer = Buffer.from(json)
  return {
    total_bytes: buffer.length,
    gzip_bytes: gzipSync(buffer, { level: 9 }).length,
    brotli_bytes: brotliCompressSync(buffer).length,
    entries,
  }
}
