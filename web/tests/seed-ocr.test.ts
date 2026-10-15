import { describe, expect, test } from 'bun:test'
import { SEED_LETTER_GLYPHS } from '../src/lib/seed-ocr/glyphs'
import { recognizeSeedCodes } from '../src/lib/seed-ocr/recognize'

const DASH = ['######', '######']
const DASH_TOP = 7
const SPACING = 2
const BACKGROUND = 60

/** Lay out text on the font-pixel grid, as the game draws it. */
function fontPixels(text: string) {
  const cells: { x: number; y: number }[] = []
  let cursor = 0
  for (const char of text) {
    if (char === ' ') {
      cursor += 12
      continue
    }
    const rows = char === '-' ? DASH : SEED_LETTER_GLYPHS[char]
    const top = char === '-' ? DASH_TOP : 0
    rows.forEach((row, y) => {
      for (let x = 0; x < row.length; x++) {
        if (row[x] === '#') cells.push({ x: cursor + x, y: top + y })
      }
    })
    cursor += rows[0].length + SPACING
  }
  return { cells, width: cursor }
}

/** Box-filter the text into a dark screenshot at a fractional scale. */
function screenshot(lines: string[], scale: number) {
  const margin = 20
  const lineHeight = Math.ceil(24 * scale)
  const laid = lines.map(fontPixels)
  const width =
    Math.ceil(Math.max(...laid.map((l) => l.width)) * scale) + 2 * margin
  const height = lines.length * lineHeight + 2 * margin
  const coverage = new Float32Array(width * height)
  laid.forEach(({ cells }, line) => {
    const originY = margin + line * lineHeight
    for (const { x, y } of cells) {
      const x0 = margin + x * scale
      const y0 = originY + y * scale
      for (let py = Math.floor(y0); py < Math.ceil(y0 + scale); py++) {
        const dy = Math.min(py + 1, y0 + scale) - Math.max(py, y0)
        for (let px = Math.floor(x0); px < Math.ceil(x0 + scale); px++) {
          const dx = Math.min(px + 1, x0 + scale) - Math.max(px, x0)
          coverage[py * width + px] += dx * dy
        }
      }
    }
  })
  const data = new Uint8ClampedArray(width * height * 4)
  coverage.forEach((amount, i) => {
    const value = BACKGROUND + Math.min(1, amount) * (255 - BACKGROUND)
    data.set([value, value, value, 255], i * 4)
  })
  return { width, height, data }
}

const codes = (image: ReturnType<typeof screenshot>) =>
  recognizeSeedCodes(image).map((seed) => seed.code)

describe('recognizeSeedCodes', () => {
  const seeds = ['ABC-DEF-GHI', 'JKL-MNO-PQR', 'STU-VWX-YZA', 'FVX-YTN-SLC']

  for (const scale of [2, 2.8, 3.5, 5]) {
    test(`reads every letter at ${scale}x`, () => {
      for (const seed of seeds) {
        const image = screenshot(['HEALTH', `SEED ${seed}`, 'MAXIMUM'], scale)
        expect(codes(image)).toEqual([seed])
      }
    })
  }

  test('lists every seed top to bottom without duplicates', () => {
    const image = screenshot(
      ['RUN FVX-YTN-SLC', 'RUN ABC-DEF-GHI', 'RUN FVX-YTN-SLC'],
      3
    )
    expect(codes(image)).toEqual(['FVX-YTN-SLC', 'ABC-DEF-GHI'])
  })

  test('bounds the seed text', () => {
    const [seed] = recognizeSeedCodes(screenshot(['ABC-DEF-GHI'], 2))
    expect(seed.box).toEqual({ left: 20, top: 20, right: 235, bottom: 49 })
  })

  test('ignores text without the seed pattern', () => {
    const image = screenshot(['STRENGTH', 'ABC-DEF', 'ABCDEFGHI'], 3)
    expect(codes(image)).toEqual([])
  })
})
