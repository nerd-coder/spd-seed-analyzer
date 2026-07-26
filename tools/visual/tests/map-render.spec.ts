import { expect, type Locator, type Page, test } from '@playwright/test'
import { AUTOMATED_MAP_RENDER_FIXTURES } from './map-render-fixtures'

const APP_STORAGE = {
  mapSpoilers: 'spd-analyzer-map-spoilers',
  mode: 'spd-analyzer-mode',
  theme: 'spd-analyzer-theme',
} as const

const GENERIC_MAP_SEED = 'VISUAL-MAP'
const GENERIC_MAP_FLOOR = 4

type BrowserErrors = {
  console: string[]
  page: string[]
}

async function installSyntheticMapReport(page: Page) {
  await page.addInitScript(() => {
    const width = 20
    const height = 30
    const tiles = Array.from({ length: width * height }, (_, cell) =>
      cell > width * 3 && cell < width * 7 ? 29 : 1
    )
    const floors = Array.from({ length: 4 }, (_, index) => ({
      depth: index + 1,
      feeling: 'none',
      builder: 'loop',
      rooms: ['SyntheticRoom'],
      items: [],
      quests: [],
      map:
        index === 3
          ? {
              width,
              height,
              tileset: 'sewers',
              tiles,
              tile_variance: Array(width * height).fill(0),
              discoverable: Array(width * height).fill(true),
              markers: [
                { cell: 26, kind: 'item', label: 'Synthetic item' },
                { cell: 93, kind: 'mob', label: 'Synthetic mob' },
              ],
              heaps: [
                {
                  cell: 26,
                  heap_type: 'HEAP',
                  items: [
                    {
                      class: 'StoneOfIntuition',
                      quantity: 1,
                      level: 0,
                      cursed: false,
                    },
                  ],
                },
              ],
              mobs: [{ cell: 93, class: 'Rat' }],
              transitions: [],
              traps: [],
              plants: [],
              blobs: [],
            }
          : null,
    }))
    const report = {
      seed: {
        input: 'VISUAL-MAP',
        numeric: 0,
        code: null,
        formatted: 'VISUAL-MAP',
      },
      spd_version: 'v3.3.8',
      spd_commit: '7b8b845a7',
      floors_requested: 4,
      identities: { potions: [], scrolls: [], rings: [] },
      floors,
      status: 'partial',
      message: 'Synthetic Playwright renderer fixture.',
    }
    class VisualFixtureWorker {
      onmessage: ((event: MessageEvent) => void) | null = null
      onerror: ((event: ErrorEvent) => void) | null = null
      postMessage(message: { type?: string }) {
        if (message.type !== 'analyze') return
        setTimeout(() => {
          this.onmessage?.(
            new MessageEvent('message', {
              data: { type: 'analysis-complete', report },
            })
          )
        }, 0)
      }
      terminate() {}
      addEventListener() {}
      removeEventListener() {}
      dispatchEvent() {
        return true
      }
    }
    Object.assign(window, { Worker: VisualFixtureWorker })
  })
}

const floorRegions = [
  { first: 1, last: 4, name: /^Sewers/ },
  { first: 6, last: 9, name: /^Prison/ },
  { first: 11, last: 14, name: /^Caves/ },
  { first: 16, last: 19, name: /^City/ },
  { first: 21, last: 24, name: /^Halls/ },
] as const

async function openAnalyzer(
  page: Page,
  seed: string,
  reducedMotion: 'reduce' | 'no-preference' = 'reduce'
): Promise<BrowserErrors> {
  const errors: BrowserErrors = { console: [], page: [] }
  page.on('console', (message) => {
    if (message.type() === 'error') errors.console.push(message.text())
  })
  page.on('pageerror', (error) => errors.page.push(error.message))

  await page.emulateMedia({ colorScheme: 'light', reducedMotion })
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
  await expect(page.getByText('Floor 1', { exact: true })).toBeVisible({
    timeout: 60_000,
  })

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

async function snapshotCanvas(canvas: Locator, snapshot: string) {
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
  await expect(dialog.getByRole('button', { name: /^Show items/ })).toHaveCount(
    0
  )
  await expect(
    dialog.getByRole('button', { name: /^Show known mobs/ })
  ).toHaveCount(0)

  await snapshotCanvas(canvas, snapshot)
}

for (const fixture of AUTOMATED_MAP_RENDER_FIXTURES) {
  test(`${fixture.seed} floor ${fixture.floor} ${fixture.expectation}`, async ({
    page,
  }) => {
    const browserErrors = await openAnalyzer(page, fixture.seed)
    if (fixture.expectation === 'rendered') {
      await captureFloor(
        page,
        fixture.floor,
        `${fixture.seed}-F${fixture.floor}.png`
      )
    } else {
      const region = floorRegions.find(
        ({ first, last }) => fixture.floor >= first && fixture.floor <= last
      )
      if (!region)
        throw new Error(`Floor ${fixture.floor} is not in a report region`)
      await page.getByRole('tab', { name: region.name }).click()
      await expect(
        page.getByRole('button', { name: `Expand floor ${fixture.floor} map` })
      ).toHaveCount(0)
      const floorSection = page
        .getByText(`Floor ${fixture.floor}`, { exact: true })
        .locator('xpath=ancestor::section[1]')
      await floorSection
        .getByRole('button', { name: 'Render assumed map' })
        .click()
      await expect(
        floorSection.getByText('Assumed continuation', { exact: true })
      ).toBeVisible()
    }

    expect(browserErrors.console, 'browser console errors').toEqual([])
    expect(browserErrors.page, 'uncaught page errors').toEqual([])
  })
}

test('mobile map dialog fills the viewport and supports 1x and 2x zoom', async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 })
  await installSyntheticMapReport(page)
  const browserErrors = await openAnalyzer(page, GENERIC_MAP_SEED)
  await page
    .getByRole('button', { name: `Expand floor ${GENERIC_MAP_FLOOR} map` })
    .click()

  const dialog = page.getByRole('dialog')
  await expect(dialog).toBeVisible()
  await expect
    .poll(() => dialog.boundingBox())
    .toEqual({ x: 0, y: 0, width: 390, height: 844 })

  const canvas = dialog.getByRole('img', {
    name: /Shattered Pixel Dungeon floor map/,
  })
  await waitForCanvasPaint(canvas)
  const oneXWidth = await canvas.evaluate(
    (node) => (node as HTMLCanvasElement).width
  )
  await dialog.getByRole('button', { name: 'Switch map to 2x zoom' }).click()
  await expect
    .poll(() => canvas.evaluate((node) => (node as HTMLCanvasElement).width))
    .toBe(oneXWidth * 2)
  await dialog.getByRole('button', { name: 'Switch map to 1x zoom' }).click()
  await expect
    .poll(() => canvas.evaluate((node) => (node as HTMLCanvasElement).width))
    .toBe(oneXWidth)
  await dialog.getByRole('button', { name: 'Switch map to 2x zoom' }).click()
  await expect
    .poll(() => canvas.evaluate((node) => (node as HTMLCanvasElement).width))
    .toBe(oneXWidth * 2)

  const settingsPanel = dialog.getByTestId('map-settings-panel')
  const scrollContainer = dialog.getByTestId('map-scroll-container')
  await expect(settingsPanel).toHaveClass(/\bdark\b/)
  await expect(settingsPanel).toHaveClass(/bg-background\/30/)
  const panelButtons = settingsPanel.getByRole('button')
  await expect(panelButtons).toHaveCount(1)
  for (const button of await panelButtons.all()) {
    await expect(button).toHaveAttribute('data-variant', 'ghost')
  }
  const panelBounds = await settingsPanel.boundingBox()
  await scrollContainer.evaluate((node) => {
    node.scrollTo({ left: node.scrollWidth, top: node.scrollHeight })
  })
  await expect
    .poll(() => scrollContainer.evaluate((node) => node.scrollLeft))
    .toBeGreaterThan(0)
  await expect
    .poll(() => scrollContainer.evaluate((node) => node.scrollTop))
    .toBeGreaterThan(0)
  expect(await settingsPanel.boundingBox()).toEqual(panelBounds)

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})

test('floor rooms open from a title chip and desktop maps use a large dialog', async ({
  page,
}) => {
  await page.setViewportSize({ width: 1440, height: 1200 })
  await installSyntheticMapReport(page)
  const browserErrors = await openAnalyzer(page, GENERIC_MAP_SEED)

  const rooms = page
    .getByRole('button', { name: /^Rooms \(\d+\)$/ })
    .nth(GENERIC_MAP_FLOOR - 1)
  await rooms.click()
  await expect(
    page.getByText(new RegExp(`^Rooms on floor ${GENERIC_MAP_FLOOR}$`))
  ).toBeVisible()

  await page
    .getByRole('button', { name: `Expand floor ${GENERIC_MAP_FLOOR} map` })
    .click()
  const dialog = page.getByRole('dialog')
  await expect(dialog).toBeVisible()
  await expect
    .poll(async () => (await dialog.boundingBox())?.width)
    .toBeGreaterThan(1200)
  await expect
    .poll(async () => (await dialog.boundingBox())?.height)
    .toBeGreaterThan(1100)
  await expect(
    dialog.getByRole('button', { name: /Switch map to [12]x zoom/ })
  ).toBeVisible()

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})

test('map dialog initially focuses its container instead of a control', async ({
  page,
}) => {
  await installSyntheticMapReport(page)
  const browserErrors = await openAnalyzer(page, GENERIC_MAP_SEED)
  await page
    .getByRole('button', { name: `Expand floor ${GENERIC_MAP_FLOOR} map` })
    .click()

  const dialog = page.getByRole('dialog')
  await expect(dialog).toBeVisible()
  await expect(dialog).toBeFocused()
  await expect(
    dialog.getByRole('button', { name: /Switch map/ })
  ).not.toBeFocused()
  await expect(dialog.getByRole('button', { name: 'Close' })).not.toBeFocused()

  await page.keyboard.press('Tab')
  await expect(dialog.getByRole('button', { name: /Switch map/ })).toBeFocused()

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})

test('accuracy details use a responsive modal and restore trigger focus', async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 600 })
  const browserErrors = await openAnalyzer(page, '2')
  const trigger = page.getByRole('button', { name: 'View accuracy details' })
  await trigger.click()

  const dialog = page.getByRole('dialog', { name: 'Accuracy details' })
  await expect(dialog).toBeVisible()
  await expect(
    dialog.getByText(/Last reviewed .+ for v?3\.3\.8\./)
  ).toBeVisible()

  const bounds = await dialog.boundingBox()
  expect(bounds).not.toBeNull()
  expect(bounds?.x).toBeGreaterThanOrEqual(16)
  expect(bounds?.y).toBeGreaterThanOrEqual(16)
  expect(bounds?.width).toBeLessThanOrEqual(358)
  expect(bounds?.height).toBeLessThanOrEqual(568)

  const scrollArea = dialog.getByTestId('accuracy-details-scroll')
  await expect
    .poll(() =>
      scrollArea.evaluate(
        (node) =>
          node.scrollHeight > node.clientHeight &&
          node.scrollWidth > node.clientWidth
      )
    )
    .toBe(true)

  await page.keyboard.press('Escape')
  await expect(dialog).toBeHidden()
  await expect(trigger).toBeFocused()

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})

test('animated liquid advances on the pixel-aligned canvas path', async ({
  page,
}) => {
  await installSyntheticMapReport(page)
  const browserErrors = await openAnalyzer(
    page,
    GENERIC_MAP_SEED,
    'no-preference'
  )
  await page
    .getByRole('button', { name: `Expand floor ${GENERIC_MAP_FLOOR} map` })
    .click()

  const canvas = page.getByRole('dialog').getByRole('img', {
    name: /Shattered Pixel Dungeon floor map/,
  })
  await waitForCanvasPaint(canvas)
  await expect(canvas).toHaveAttribute('data-water-animation', 'running')
  const firstFrame = await canvas.evaluate((node) =>
    (node as HTMLCanvasElement).toDataURL('image/png')
  )
  await page.waitForTimeout(300)
  const laterFrame = await canvas.evaluate((node) =>
    (node as HTMLCanvasElement).toDataURL('image/png')
  )
  expect(laterFrame).not.toBe(firstFrame)

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})
