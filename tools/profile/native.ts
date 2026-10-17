import { statSync } from 'node:fs'
import { join } from 'node:path'
import { $ } from 'bun'
import type { NativeResult } from './types'

export const ROOT = join(import.meta.dir, '../..')

export async function buildProfiler(): Promise<string> {
  await $`cargo build --profile profiling -p spd-cli --bin spd-profile`.cwd(
    ROOT
  )
  const meta = await $`cargo metadata --format-version 1 --no-deps`
    .cwd(ROOT)
    .quiet()
    .json()
  return join(meta.target_directory as string, 'profiling', 'spd-profile')
}

/** Peak RSS of a child process via `/usr/bin/time` (BSD `-l` or GNU `-v`). */
async function peakRss(bin: string, args: string[]): Promise<number | null> {
  const flag = process.platform === 'darwin' ? '-l' : '-v'
  const result = await $`/usr/bin/time ${flag} ${bin} ${args}`.quiet().nothrow()
  const text = result.stderr.toString()
  const bsd = text.match(/(\d+)\s+maximum resident set size/)
  if (bsd) return Number(bsd[1])
  const gnu = text.match(/Maximum resident set size \(kbytes\):\s*(\d+)/)
  return gnu ? Number(gnu[1]) * 1024 : null
}

export async function runNative(
  bin: string,
  seeds: number,
  dumpPath: string
): Promise<NativeResult> {
  const output =
    await $`${bin} --seeds ${seeds} --dump-report ${dumpPath}`.json()
  return {
    ...output,
    binary_bytes: statSync(bin).size,
    peak_rss_bytes: await peakRss(bin, ['--loop', '1']),
  }
}
