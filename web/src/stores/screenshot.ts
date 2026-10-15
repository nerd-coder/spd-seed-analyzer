/**
 * Reading seed codes from a game screenshot, from the open file through the
 * player's choice of which seeds to analyze. Ephemeral.
 */

import {
  firstImageFile,
  readSeedsFromImage,
  type ScreenshotSeed,
} from '@/lib/seed-ocr/read-image'
import { analyzeSeedInput } from './sessions'
import { AppStore } from './store-utils'

export type ScreenshotScan =
  | { status: 'dropping' }
  | { status: 'reading' }
  | { status: 'found'; seeds: ScreenshotSeed[] }
  | { status: 'error'; title: string; message: string }

/** The screenshot dialog is open while this is non-null. */
export const $screenshotScan = new AppStore<ScreenshotScan | null>(null)

/** Bumped on every scan and dismissal so a stale read cannot reopen the dialog. */
let scanGeneration = 0
/** What the dialog showed before a drag turned it into a drop target. */
let beforeDrop: ScreenshotScan | null = null

/** A file drag entered the page: show the dialog as a drop target. */
export function showScreenshotDropTarget() {
  const current = $screenshotScan.get()
  if (current?.status === 'dropping' || current?.status === 'reading') return
  beforeDrop = current
  $screenshotScan.set({ status: 'dropping' })
}

/** The drag left the page without dropping: put the dialog back. */
export function hideScreenshotDropTarget() {
  if ($screenshotScan.get()?.status !== 'dropping') return
  $screenshotScan.set(beforeDrop)
  beforeDrop = null
}

export async function dropScreenshot(files: FileList): Promise<void> {
  const image = firstImageFile(files)
  if (image) return scanScreenshot(image)
  scanGeneration++
  beforeDrop = null
  $screenshotScan.set({
    status: 'error',
    title: 'That file is not an image',
    message: 'Drop a PNG or JPEG screenshot of the game.',
  })
}

export async function scanScreenshot(image: Blob): Promise<void> {
  const generation = ++scanGeneration
  beforeDrop = null
  $screenshotScan.set({ status: 'reading' })
  let next: ScreenshotScan
  try {
    const seeds = await readSeedsFromImage(image)
    next =
      seeds.length > 0
        ? { status: 'found', seeds }
        : {
            status: 'error',
            title: 'No seed found',
            message:
              'Use a full-size screenshot that shows the Dungeon Seed in the hero stats window.',
          }
  } catch {
    next = {
      status: 'error',
      title: 'Could not open that image',
      message: 'Try a PNG or JPEG screenshot.',
    }
  }
  if (generation === scanGeneration) $screenshotScan.set(next)
}

export function dismissScreenshotScan() {
  scanGeneration++
  beforeDrop = null
  $screenshotScan.set(null)
}

/** Analyze the chosen codes in order; the last one ends up as the active tab. */
export async function analyzeScreenshotSeeds(codes: string[]): Promise<void> {
  dismissScreenshotScan()
  for (const code of codes) {
    await analyzeSeedInput(code)
  }
}
