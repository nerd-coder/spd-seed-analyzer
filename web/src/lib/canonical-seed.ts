const CODE = /^[A-Z]{3}-[A-Z]{3}-[A-Z]{3}$/
const DAILY = /^\d{4}-\d{2}-\d{2}$/

type Kind = 'code' | 'daily'

function kindOf(raw: string): Kind | null {
  const first = raw.toUpperCase().match(/[A-Z0-9]/)?.[0]
  if (!first) return null
  return /[A-Z]/.test(first) ? 'code' : 'daily'
}

function grouped(body: string, sizes: number[]) {
  const parts: string[] = []
  let cursor = 0
  for (const size of sizes) {
    const part = body.slice(cursor, cursor + size)
    if (!part) break
    parts.push(part)
    cursor += size
  }
  return parts.join('-')
}

/**
 * Uppercase a seed code, or keep a daily date numeric, and insert dashes.
 * Letters start an `ABC-DEF-GHI` code. Digits start a `YYYY-MM-DD` date.
 * Anything else is dropped. The result is the stored value.
 */
export function formatCanonicalSeedInput(raw: string): string {
  const kind = kindOf(raw)
  if (kind === 'code') {
    const letters = raw
      .toUpperCase()
      .replace(/[^A-Z]/g, '')
      .slice(0, 9)
    return grouped(letters, [3, 3, 3])
  }
  if (kind === 'daily') {
    const digits = raw.replace(/\D/g, '').slice(0, 8)
    return grouped(digits, [4, 2, 2])
  }
  return ''
}

export function isCompleteCanonicalSeed(value: string): boolean {
  return CODE.test(value) || DAILY.test(value)
}

/** Alphanumeric characters that survive formatting, before the caret. */
export function significantCountBefore(raw: string, cursor: number): number {
  const slice = raw.slice(0, Math.max(0, cursor))
  const kind = kindOf(raw)
  if (kind === 'code') return slice.replace(/[^A-Za-z]/g, '').length
  if (kind === 'daily') return slice.replace(/\D/g, '').length
  return 0
}

export function caretAfterSignificant(
  formatted: string,
  significantCount: number
): number {
  if (significantCount <= 0) return 0
  let seen = 0
  for (let index = 0; index < formatted.length; index++) {
    if (formatted[index] === '-') continue
    seen += 1
    if (seen === significantCount) return index + 1
  }
  return formatted.length
}
