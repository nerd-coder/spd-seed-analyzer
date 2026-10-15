import { SEED_LETTER_GLYPHS } from './glyphs'

export interface RgbaImage {
  width: number
  height: number
  data: Uint8ClampedArray | Uint8Array
}

interface Blob {
  id: number
  left: number
  top: number
  right: number
  bottom: number
  area: number
}

interface Template {
  letter: string
  width: number
  height: number
  ink: Float32Array
}

/** Game text is near-white on a dark panel; JPEG halos sit well below this. */
const INK_LUMA = 150
const MIN_LETTER_HEIGHT = 7
/** Mean per-cell difference above which a glyph match is rejected. */
const MAX_LETTER_SCORE = 0.3

const TEMPLATES: Template[] = Object.entries(SEED_LETTER_GLYPHS).map(
  ([letter, rows]) => ({
    letter,
    width: rows[0].length,
    height: rows.length,
    ink: Float32Array.from(rows.join(''), (cell) => (cell === '#' ? 1 : 0)),
  })
)

function inkMask({ width, height, data }: RgbaImage): Uint8Array {
  const mask = new Uint8Array(width * height)
  for (let i = 0, p = 0; i < mask.length; i++, p += 4) {
    const luma = 0.299 * data[p] + 0.587 * data[p + 1] + 0.114 * data[p + 2]
    mask[i] = luma >= INK_LUMA ? 1 : 0
  }
  return mask
}

/** 8-connected components of the ink mask; `labels` holds 1-based blob ids. */
function findBlobs(mask: Uint8Array, width: number, height: number) {
  const labels = new Int32Array(mask.length)
  const blobs: Blob[] = []
  const stack: number[] = []
  for (let start = 0; start < mask.length; start++) {
    if (!mask[start] || labels[start]) continue
    const id = blobs.length + 1
    const blob: Blob = {
      id,
      left: width,
      top: height,
      right: -1,
      bottom: -1,
      area: 0,
    }
    labels[start] = id
    stack.push(start)
    while (stack.length) {
      const index = stack.pop() as number
      const x = index % width
      const y = (index - x) / width
      blob.area++
      if (x < blob.left) blob.left = x
      if (x > blob.right) blob.right = x
      if (y < blob.top) blob.top = y
      if (y > blob.bottom) blob.bottom = y
      for (let dy = -1; dy <= 1; dy++) {
        const ny = y + dy
        if (ny < 0 || ny >= height) continue
        for (let dx = -1; dx <= 1; dx++) {
          const nx = x + dx
          if (nx < 0 || nx >= width) continue
          const next = ny * width + nx
          if (mask[next] && !labels[next]) {
            labels[next] = id
            stack.push(next)
          }
        }
      }
    }
    blobs.push(blob)
  }
  return { labels, blobs }
}

const blobWidth = (blob: Blob) => blob.right - blob.left + 1
const blobHeight = (blob: Blob) => blob.bottom - blob.top + 1

function isLetterShaped(blob: Blob) {
  const height = blobHeight(blob)
  const aspect = blobWidth(blob) / height
  return height >= MIN_LETTER_HEIGHT && aspect > 0.15 && aspect < 1
}

/** A dash sits mid-height on the letter line: 6×2 font pixels vs 15-pixel caps. */
function isDashFor(blob: Blob, letter: Blob) {
  const h = blobHeight(letter)
  const width = blobWidth(blob)
  const height = blobHeight(blob)
  const middle = (blob.top + blob.bottom) / 2 - letter.top
  return (
    width >= h * 0.25 &&
    width <= h * 0.6 &&
    height <= h * 0.25 &&
    width / height >= 1.8 &&
    middle >= h * 0.35 &&
    middle <= h * 0.65
  )
}

function sameLetterLine(blob: Blob, first: Blob) {
  const h = blobHeight(first)
  return (
    isLetterShaped(blob) &&
    Math.abs(blobHeight(blob) - h) <= Math.max(2, h * 0.15) &&
    Math.abs(blob.top - first.top) <= Math.max(2, h * 0.15)
  )
}

/** Area-sample the blob's own pixels onto a template grid and compare. */
function classifyLetter(
  blob: Blob,
  labels: Int32Array,
  imageWidth: number
): { letter: string; score: number } {
  const width = blobWidth(blob)
  const height = blobHeight(blob)
  const aspect = width / height
  let best = { letter: '', score: Number.POSITIVE_INFINITY }
  for (const template of TEMPLATES) {
    const aspectPenalty = Math.abs(
      Math.log(aspect / (template.width / template.height))
    )
    if (aspectPenalty > 0.5) continue
    let diff = 0
    for (let ty = 0; ty < template.height; ty++) {
      const y0 = blob.top + Math.floor((ty * height) / template.height)
      const y1 = blob.top + Math.ceil(((ty + 1) * height) / template.height)
      for (let tx = 0; tx < template.width; tx++) {
        const x0 = blob.left + Math.floor((tx * width) / template.width)
        const x1 = blob.left + Math.ceil(((tx + 1) * width) / template.width)
        let ink = 0
        for (let y = y0; y < y1; y++) {
          for (let x = x0; x < x1; x++) {
            if (labels[y * imageWidth + x] === blob.id) ink++
          }
        }
        const coverage = ink / ((y1 - y0) * (x1 - x0))
        diff += Math.abs(coverage - template.ink[ty * template.width + tx])
      }
    }
    const score = diff / template.ink.length + aspectPenalty * 0.2
    if (score < best.score) best = { letter: template.letter, score }
  }
  return best
}

/**
 * Walk right from `first` expecting `LLL-LLL-LLL` with tight glyph spacing.
 * Returns the nine letter blobs, or null when the line does not fit.
 */
function seedLineFrom(first: Blob, sorted: Blob[]): Blob[] | null {
  const h = blobHeight(first)
  const maxGap = h * 0.8
  const letters: Blob[] = [first]
  let previous = first
  let index = sorted.indexOf(first) + 1
  for (let slot = 1; slot < 11; slot++) {
    const wantDash = slot === 3 || slot === 7
    let found: Blob | null = null
    for (; index < sorted.length; index++) {
      const blob = sorted[index]
      if (blob.left <= previous.right) continue
      if (blob.left - previous.right > maxGap) break
      if (wantDash ? isDashFor(blob, first) : sameLetterLine(blob, first)) {
        found = blob
        index++
        break
      }
      if (blob.bottom >= first.top && blob.top <= first.bottom) return null
    }
    if (!found) return null
    if (!wantDash) letters.push(found)
    previous = found
  }
  return letters
}

export interface FoundSeed {
  code: string
  /** Pixel bounds of the seed text in the scanned image (inclusive). */
  box: { left: number; top: number; right: number; bottom: number }
}

/**
 * Find every Shattered Pixel Dungeon seed code (`ABC-DEF-GHI`) rendered in
 * the game's pixel font in a screenshot, top to bottom, without duplicates.
 */
export function recognizeSeedCodes(image: RgbaImage): FoundSeed[] {
  const { width, height } = image
  const { labels, blobs } = findBlobs(inkMask(image), width, height)
  const sorted = blobs
    .filter((blob) => blob.area >= 4 && blobHeight(blob) >= 2)
    .sort((a, b) => a.left - b.left)
  const starts = sorted
    .filter(isLetterShaped)
    .sort((a, b) => a.top - b.top || a.left - b.left)

  const found: FoundSeed[] = []
  const used = new Set<Blob>()
  for (const first of starts) {
    if (used.has(first)) continue
    const letters = seedLineFrom(first, sorted)
    if (!letters) continue
    const matches = letters.map((blob) => classifyLetter(blob, labels, width))
    if (matches.some((match) => match.score > MAX_LETTER_SCORE)) continue
    for (const blob of letters) used.add(blob)
    const letterCode = matches.map((match) => match.letter).join('')
    const code = `${letterCode.slice(0, 3)}-${letterCode.slice(3, 6)}-${letterCode.slice(6)}`
    if (found.some((seed) => seed.code === code)) continue
    found.push({
      code,
      box: {
        left: first.left,
        top: Math.min(...letters.map((blob) => blob.top)),
        right: letters[letters.length - 1].right,
        bottom: Math.max(...letters.map((blob) => blob.bottom)),
      },
    })
  }
  return found
}
