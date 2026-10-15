import { type FoundSeed, recognizeSeedCodes } from './recognize'

/** Caps pixel work on huge photos; game text stays several pixels per font pixel. */
const MAX_SIDE = 3200
/** Crop height, so every seed preview reads at the same size (2× for HiDPI). */
const PREVIEW_HEIGHT = 72

export interface ScreenshotSeed {
  code: string
  /** PNG data URL of the seed text as it appears in the screenshot. */
  preview: string
}

export function firstImageFile(files: FileList | null | undefined) {
  return Array.from(files ?? []).find((file) => file.type.startsWith('image/'))
}

function cropPreview(source: HTMLCanvasElement, { box }: FoundSeed): string {
  const pad = Math.round((box.bottom - box.top + 1) * 0.4)
  const left = Math.max(0, box.left - pad)
  const top = Math.max(0, box.top - pad)
  const width = Math.min(source.width, box.right + pad + 1) - left
  const height = Math.min(source.height, box.bottom + pad + 1) - top
  const scale = PREVIEW_HEIGHT / height
  const crop = document.createElement('canvas')
  crop.width = Math.round(width * scale)
  crop.height = PREVIEW_HEIGHT
  const context = crop.getContext('2d')
  if (!context) return ''
  context.imageSmoothingQuality = 'high'
  context.drawImage(
    source,
    left,
    top,
    width,
    height,
    0,
    0,
    crop.width,
    crop.height
  )
  return crop.toDataURL('image/png')
}

/** Decode an image file and find every seed code in it. */
export async function readSeedsFromImage(
  image: Blob
): Promise<ScreenshotSeed[]> {
  const bitmap = await createImageBitmap(image)
  try {
    const scale = Math.min(1, MAX_SIDE / Math.max(bitmap.width, bitmap.height))
    const canvas = document.createElement('canvas')
    canvas.width = Math.round(bitmap.width * scale)
    canvas.height = Math.round(bitmap.height * scale)
    const context = canvas.getContext('2d', { willReadFrequently: true })
    if (!context) throw new Error('Canvas 2D is unavailable')
    context.imageSmoothingQuality = 'high'
    context.drawImage(bitmap, 0, 0, canvas.width, canvas.height)
    const found = recognizeSeedCodes(
      context.getImageData(0, 0, canvas.width, canvas.height)
    )
    return found.map((seed) => ({
      code: seed.code,
      preview: cropPreview(canvas, seed),
    }))
  } finally {
    bitmap.close()
  }
}
