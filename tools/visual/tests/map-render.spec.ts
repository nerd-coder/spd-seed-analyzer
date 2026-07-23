import { expect, type Locator, type Page, test } from '@playwright/test'
import { MAP_RENDER_FIXTURES } from './map-render-fixtures'

const APP_STORAGE = {
  mapSpoilers: 'spd-analyzer-map-spoilers',
  mode: 'spd-analyzer-mode',
  theme: 'spd-analyzer-theme',
} as const

type BrowserErrors = {
  console: string[]
  page: string[]
}

const floorRegions = [
  { first: 1, last: 4, name: /^Sewers/ },
  { first: 6, last: 9, name: /^Prison/ },
  { first: 11, last: 14, name: /^Caves/ },
  { first: 16, last: 19, name: /^City/ },
  { first: 21, last: 24, name: /^Halls/ },
] as const

async function openAnalyzer(page: Page, seed: string): Promise<BrowserErrors> {
  const errors: BrowserErrors = { console: [], page: [] }
  page.on('console', (message) => {
    if (message.type() === 'error') errors.console.push(message.text())
  })
  page.on('pageerror', (error) => errors.page.push(error.message))

  await page.emulateMedia({ colorScheme: 'light', reducedMotion: 'reduce' })
  await page.addInitScript((storage) => {
    localStorage.clear()
    localStorage.setItem(storage.mapSpoilers, '1')
    localStorage.setItem(storage.mode, 'analyze')
    localStorage.setItem(storage.theme, 'light')
  }, APP_STORAGE)

  await page.goto('/')
  await expect(page).toHaveTitle('SPD Seed Analyzer')
  await expect(
    page.getByRole('heading', { name: 'No seeds analyzed yet' })
  ).toBeVisible()

  await page.getByLabel('Enter your seed').fill(seed)
  await page.getByRole('button', { name: 'Analyze', exact: true }).click()
  await expect(page.getByRole('tab', { name: seed })).toBeVisible()
  await expect(
    page.getByRole('button', { name: 'Expand floor 1 map' })
  ).toBeVisible({ timeout: 60_000 })

  return errors
}

async function waitForCanvasPaint(canvas: Locator) {
  await expect(canvas).toBeVisible()
  await expect
    .poll(async () =>
      canvas.evaluate((node) => {
        const mapCanvas = node as HTMLCanvasElement
        const context = mapCanvas.getContext('2d')
        if (!context || mapCanvas.width === 0 || mapCanvas.height === 0) {
          return false
        }

        const pixels = context.getImageData(
          0,
          0,
          mapCanvas.width,
          mapCanvas.height
        ).data
        for (let alpha = 3; alpha < pixels.length; alpha += 4) {
          if (pixels[alpha] !== 0) return true
        }
        return false
      })
    )
    .toBe(true)
}

async function enableMarkerLayer(dialog: Locator, name: RegExp) {
  const toggle = dialog.getByRole('switch', { name })
  if ((await toggle.count()) === 0) return false
  await toggle.click()
  await expect(toggle).toHaveAttribute('aria-checked', 'true')
  return true
}

async function snapshotCanvas(
  canvas: Locator,
  snapshot: string,
  markerLayersEnabled: boolean
) {
  if (markerLayersEnabled) {
    await expect(canvas).toHaveAttribute('aria-label', /Visible markers:/)
  }
  const dataUrl = await canvas.evaluate(async (node) => {
    await new Promise<void>((resolve) => {
      requestAnimationFrame(() => requestAnimationFrame(() => resolve()))
    })
    return (node as HTMLCanvasElement).toDataURL('image/png')
  })
  const png = Buffer.from(dataUrl.slice(dataUrl.indexOf(',') + 1), 'base64')
  expect(png).toMatchSnapshot(snapshot, {
    maxDiffPixels: 0,
    threshold: 0,
  })
}

async function captureFloor(page: Page, floor: number, snapshot: string) {
  const region = floorRegions.find(
    ({ first, last }) => floor >= first && floor <= last
  )
  if (!region) throw new Error(`Floor ${floor} is not in a report region`)

  const regionTab = page.getByRole('tab', { name: region.name })
  await regionTab.click()
  await expect(regionTab).toHaveAttribute('aria-selected', 'true')
  await page.getByRole('button', { name: `Expand floor ${floor} map` }).click()

  const dialog = page.getByRole('dialog')
  await expect(dialog).toBeVisible()
  await expect(
    dialog.getByRole('heading', { name: `Floor ${floor}` })
  ).toBeVisible()

  const canvas = dialog.getByRole('img', {
    name: /Shattered Pixel Dungeon floor map/,
  })
  await expect(canvas).toHaveAttribute('data-water-animation', 'paused')
  await waitForCanvasPaint(canvas)
  const itemsEnabled = await enableMarkerLayer(dialog, /^Items/)
  const mobsEnabled = await enableMarkerLayer(dialog, /^Known mobs/)

  await snapshotCanvas(canvas, snapshot, itemsEnabled || mobsEnabled)
}

for (const fixture of MAP_RENDER_FIXTURES) {
  test(`${fixture.seed} floor ${fixture.floor} deterministic map`, async ({
    page,
  }) => {
    const browserErrors = await openAnalyzer(page, fixture.seed)
    await captureFloor(
      page,
      fixture.floor,
      `${fixture.seed}-F${fixture.floor}.png`
    )

    expect(browserErrors.console, 'browser console errors').toEqual([])
    expect(browserErrors.page, 'uncaught page errors').toEqual([])
  })
}
