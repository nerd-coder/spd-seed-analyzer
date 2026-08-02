import { expect, type Locator, type Page, test } from '@playwright/test'
import { MAP_RENDER_FIXTURES } from './map-render-fixtures'

const APP_STORAGE = {
  mode: 'spd-analyzer-mode',
  testInitialized: 'spd-analyzer-visual-test-initialized',
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
    const tiles = Array(width * height).fill(1)
    for (let y = 4; y < height - 4; y++) {
      for (let x = 4; x < width - 4; x++) tiles[y * width + x] = 2
    }
    const floors = Array.from({ length: 4 }, (_, index) => ({
      depth: index + 1,
      feeling: 'none',
      builder: 'loop',
      rooms: ['SyntheticRoom'],
      guaranteed_appearances:
        index === 2
          ? [
              {
                name: 'Alchemy pot',
                kind: 'alchemy_pot',
                source: 'LaboratoryRoom',
              },
            ]
          : [],
      items: [],
      quests: [],
      map:
        index === 3
          ? {
              width,
              height,
              tileset: 'sewers',
              tiles,
              tile_variance: [],
              discoverable: Array(width * height).fill(true),
              markers: [],
              heaps: [],
              mobs: [],
              transitions: [
                { cell: 5 * width + 5, type: 'REGULAR_ENTRANCE' },
                {
                  cell: (height - 6) * width + (width - 6),
                  type: 'REGULAR_EXIT',
                },
              ],
              traps: [],
              plants: [],
              blobs: [],
              custom_tiles: [
                {
                  class: 'CenterPieceVisuals',
                  texture: 'halls_special',
                  x: 8,
                  y: 8,
                  width: 1,
                  height: 1,
                  static_data: [8],
                },
              ],
              custom_walls: [
                {
                  class: 'CenterPieceWalls',
                  texture: 'halls_special',
                  x: 8,
                  y: 9,
                  width: 1,
                  height: 1,
                  static_data: [-1],
                },
              ],
            }
          : null,
    }))
    ;(floors[3].items as unknown[]).push({
      variants: [
        {
          name: 'Synthetic conditional item',
          quantity: 1,
          class_name: 'ScrollOfTransmutation',
          category: 'scroll',
          level_range: { min: 0, max: 3 },
          cursed: true,
          enchantment: {
            type: 'Blazing',
            conditions: [
              {
                type: 'trinket',
                events: [
                  {
                    before_depth: 4,
                    kind: 'acquired',
                    trinket: 'ParchmentScrap',
                  },
                  { before_depth: 4, kind: 'upgraded' },
                ],
              },
            ],
          },
          prediction: 'exact',
          conditions: [
            {
              type: 'state',
              state_id: 'synthetic_upgrade_route',
            },
          ],
          spawn_conditions: [
            {
              all_of: [
                {
                  type: 'challenge',
                  challenge: 'forbidden_runes',
                  enabled: false,
                },
              ],
            },
          ],
        },
      ],
    })
    ;(floors[3].items as unknown[]).push({
      variants: [
        {
          name: 'Trinket Catalyst',
          quantity: 1,
          class_name: 'TrinketCatalyst',
          category: 'trinket',
          candidate_classes: [
            'MossyClump',
            'MimicTooth',
            'RatSkull',
            'SaltCube',
          ],
          prediction: 'exact',
        },
      ],
    })
    ;(floors[3].items as unknown[]).push({
      source: 'SacrificeRoom',
      variants: [
        {
          name: 'weapon reward',
          quantity: 1,
          category: 'weapon',
          tier_range: { min: 2, max: 5 },
          level_range: { min: 0, max: 3 },
          cursed: true,
          prediction: 'constrained',
        },
        {
          name: 'friendly sickle +2',
          quantity: 1,
          class_name: 'Sickle',
          category: 'weapon',
          level: 2,
          cursed: true,
          prediction: 'baseline',
        },
      ],
    })
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
      trinket_selection: {
        catalyst_depth: 2,
        first_alchemy_pot_depth: 3,
        first_alchemy_pot_is_secret: false,
        selection_depth: 3,
        first_effective_depth: 4,
        catalyst_options: ['MossyClump', 'MimicTooth', 'RatSkull', 'SaltCube'],
        transmutation_sequence: [
          'TrapMechanism',
          'ParchmentScrap',
          'PetrifiedSeed',
          'ExoticCrystals',
          'DimensionalSundial',
          'ThirteenLeafClover',
          'WondrousResin',
          'EyeOfNewt',
          'VialOfBlood',
          'ShardOfOblivion',
          'ChaoticCenser',
          'FerretTuft',
          'CrackedSpyglass',
        ],
      },
      floors,
      analysis_notes: ['Synthetic conditional analysis remains partial.'],
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
    if (sessionStorage.getItem(storage.testInitialized) !== 'true') {
      localStorage.clear()
      localStorage.setItem(storage.mode, 'analyze')
      localStorage.setItem(storage.theme, 'light')
      sessionStorage.setItem(storage.testInitialized, 'true')
    }
  }, APP_STORAGE)

  await page.goto('/')
  await expect(page).toHaveTitle('SPD Seed Analyzer')
  await expect(
    page.getByRole('heading', { name: 'No seeds analyzed yet' })
  ).toBeVisible()

  await page.getByLabel('Enter your seed').fill(seed)
  await page.getByRole('button', { name: 'Analyze', exact: true }).click()
  await expect(page.getByRole('tab', { name: seed, exact: true })).toBeVisible()
  await expect(
    page.getByRole('heading', { name: 'Floor 1', exact: true })
  ).toBeVisible({
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

async function expectSyntheticCustomTile(canvas: Locator) {
  const matchesSource = await canvas.evaluate(async (node) => {
    const mapCanvas = node as HTMLCanvasElement
    const source = new Image()
    source.src = '/assets/environment/custom_tiles/halls_special.png'
    await source.decode()

    const sourceCanvas = document.createElement('canvas')
    sourceCanvas.width = 16
    sourceCanvas.height = 16
    const sourceContext = sourceCanvas.getContext('2d')
    const mapContext = mapCanvas.getContext('2d')
    if (!sourceContext || !mapContext) return false
    // Synthetic layer starts at atlas visual 8 and map cell (8, 8).
    sourceContext.drawImage(source, 0, 16, 16, 16, 0, 0, 16, 16)
    const sourcePixels = sourceContext.getImageData(0, 0, 16, 16).data
    const sourcePixel = Array.from(
      { length: 16 * 16 },
      (_, pixel) => pixel
    ).find((pixel) => sourcePixels[pixel * 4 + 3] === 255)
    if (sourcePixel == null) return false

    const scale = mapCanvas.width / (20 * 16)
    const sourceX = sourcePixel % 16
    const sourceY = Math.floor(sourcePixel / 16)
    const mapPixel = mapContext.getImageData(
      (8 * 16 + sourceX) * scale,
      (8 * 16 + sourceY) * scale,
      1,
      1
    ).data
    return [0, 1, 2, 3].every(
      (channel) => mapPixel[channel] === sourcePixels[sourcePixel * 4 + channel]
    )
  })
  expect(matchesSource).toBe(true)
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

for (const fixture of MAP_RENDER_FIXTURES) {
  test(`${fixture.seed} floor ${fixture.floor} structural layout`, async ({
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

test('mobile map dialog fills the viewport and supports 1x and 2x zoom', async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 })
  await installSyntheticMapReport(page)
  const browserErrors = await openAnalyzer(page, GENERIC_MAP_SEED)
  await page
    .getByRole('button', { name: `Expand floor ${GENERIC_MAP_FLOOR} map` })
    .first()
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
  await expectSyntheticCustomTile(canvas)
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
    .first()
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

test('alchemy pots render as guaranteed non-loot appearances', async ({
  page,
}) => {
  await installSyntheticMapReport(page)
  const browserErrors = await openAnalyzer(page, GENERIC_MAP_SEED)

  await expect(
    page.getByText('Guaranteed appearances', { exact: false })
  ).toBeVisible()
  await expect(page.getByText('Alchemy pot', { exact: true })).toBeVisible()
  await expect(page.getByText('Laboratory', { exact: true })).toBeVisible()

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
    .first()
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

test('Sad Ghost card shows only its resolved target', async ({ page }) => {
  const browserErrors = await openAnalyzer(page, '0')

  await expect(
    page.getByRole('alert').filter({ hasText: 'Other Ghost options' })
  ).toHaveCount(0)
  await expect(
    page.getByText(/^Target: (Fetid Rat|Gnoll Trickster|Great Crab)$/)
  ).toBeVisible()
  await expect(
    page.getByText('target follows spawn floor', { exact: false })
  ).toHaveCount(0)

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})

test('Other items explains fresh baseline equipment rewards', async ({
  page,
}) => {
  const browserErrors = await openAnalyzer(page, 'AAA-AAA-AAA')
  const info = page
    .getByRole('button', { name: 'About fresh baseline items' })
    .first()
  await info.click()
  await expect(
    page.getByText('Fresh baseline items', { exact: true })
  ).toBeVisible()
  await expect(
    page.getByText('not seed-wide guarantees', { exact: false })
  ).toBeVisible()

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})

test('analyzer displays baseline items in their ordinary item group', async ({
  page,
}) => {
  const browserErrors = await openAnalyzer(page, 'RZN-LKU-EFS')
  const baselineItem = page.getByRole('listitem').filter({
    has: page.getByRole('img', { name: 'Ring of Wealth +2' }),
  })
  const otherItems = page
    .getByText(/^Other items/)
    .first()
    .locator('../..')

  await expect(page.getByText(/^Fresh baseline highlights/)).toHaveCount(0)
  await expect(otherItems).toContainText('Ring of Wealth +2')
  await expect(baselineItem).toContainText('Ring of Wealth +2')
  await expect(baselineItem).toContainText('Crystal choice')
  await expect(baselineItem).toContainText('cursed')
  await expect(page.getByText('planning only', { exact: true })).toHaveCount(0)

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})

test('finder result displays only matched constraints', async ({ page }) => {
  const errors: BrowserErrors = { console: [], page: [] }
  page.on('console', (message) => {
    if (message.type() === 'error') errors.console.push(message.text())
  })
  page.on('pageerror', (error) => errors.page.push(error.message))
  await page.emulateMedia({ colorScheme: 'light', reducedMotion: 'reduce' })
  await page.addInitScript((storage) => {
    localStorage.clear()
    localStorage.setItem(storage.mode, 'finder')
    localStorage.setItem(storage.theme, 'light')
  }, APP_STORAGE)

  await page.goto('/')
  await expect(page).toHaveTitle('SPD Seed Analyzer')
  await expect(
    page.getByRole('heading', { name: 'No searches yet' })
  ).toBeVisible()
  await page
    .getByRole('spinbutton', { name: 'Start seed' })
    .fill('3755006876548')
  await page.getByRole('spinbutton', { name: 'Candidates' }).fill('10')
  await page.getByRole('combobox', { name: 'Depth' }).selectOption('17')
  await page.getByRole('spinbutton', { name: 'Results' }).fill('1')
  await page.getByLabel('Item 1 type').selectOption('Food')
  await page.getByRole('button', { name: 'Find seeds' }).click()

  await expect(page.getByText('RZN-LKU-EFS', { exact: true })).toBeVisible({
    timeout: 60_000,
  })
  await expect(
    page.getByText('Matched constraints', { exact: true })
  ).toBeVisible()
  await expect(page.getByText('ration of food', { exact: true })).toBeVisible()
  await expect(
    page.getByText('Fresh baseline highlights', { exact: true })
  ).toHaveCount(0)
  await expect(page.getByText('planning only', { exact: true })).toHaveCount(0)
  await expect(page.getByText('Ring of Wealth +2')).toHaveCount(0)

  expect(errors.console, 'browser console errors').toEqual([])
  expect(errors.page, 'uncaught page errors').toEqual([])
})

test('finder can opt into SacrificeRoom fresh-baseline items', async ({
  page,
}) => {
  const errors: BrowserErrors = { console: [], page: [] }
  page.on('console', (message) => {
    if (message.type() === 'error') errors.console.push(message.text())
  })
  page.on('pageerror', (error) => errors.page.push(error.message))
  await page.emulateMedia({ colorScheme: 'light', reducedMotion: 'reduce' })
  await page.addInitScript((storage) => {
    localStorage.clear()
    localStorage.setItem(storage.mode, 'finder')
    localStorage.setItem(storage.theme, 'light')
  }, APP_STORAGE)

  await page.goto('/')
  await page
    .getByRole('spinbutton', { name: 'Start seed' })
    .fill('3293380032588')
  await page.getByRole('spinbutton', { name: 'Candidates' }).fill('10')
  await page.getByRole('combobox', { name: 'Depth' }).selectOption('4')
  await page.getByRole('spinbutton', { name: 'Results' }).fill('1')
  await page.getByLabel('Item 1 type').selectOption('Sickle')
  await page.getByLabel('Item 1 upgrade level').selectOption('2')

  const includeBaseline = page.getByRole('switch', {
    name: 'Include fresh-baseline matches',
  })
  await expect(includeBaseline).not.toBeChecked()
  await includeBaseline.click()
  await page.getByRole('button', { name: 'Find seeds' }).click()

  await expect(page.getByText('PUB-CLI-VNW', { exact: true })).toBeVisible({
    timeout: 60_000,
  })
  const result = page
    .locator('[data-slot="item"]')
    .filter({ hasText: 'friendly sickle' })
  await expect(result).toContainText('friendly sickle +2')
  await expect(result).toContainText('Sacrifice')
  await expect(
    result.getByText('Fresh baseline', { exact: true })
  ).toBeVisible()

  expect(errors.console, 'browser console errors').toEqual([])
  expect(errors.page, 'uncaught page errors').toEqual([])
})

test('conditional items render inline with their item properties', async ({
  page,
}) => {
  await installSyntheticMapReport(page)
  const browserErrors = await openAnalyzer(page, GENERIC_MAP_SEED)

  await expect(page.getByLabel('Modeled outcome')).toHaveCount(0)
  await expect(
    page.getByRole('switch', { name: 'Forbidden Runes' })
  ).toHaveCount(0)
  await expect(page.getByRole('switch', { name: 'Identities' })).toHaveCount(0)
  await expect(page.getByRole('switch', { name: 'Floor maps' })).toHaveCount(0)
  await expect(page.getByRole('heading', { name: 'Identities' })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Floor 1' })).toBeVisible()
  const syntheticItem = page.getByRole('listitem').filter({
    hasText: 'Synthetic conditional item',
  })
  await expect(syntheticItem).toBeVisible()
  await expect(syntheticItem.getByText('+0…+3', { exact: true })).toBeVisible()
  await expect(
    page.getByRole('button', { name: 'Show upgrade conditions' })
  ).toHaveCount(0)
  await expect(syntheticItem.getByText('cursed', { exact: true })).toBeVisible()
  await expect(
    syntheticItem.getByText('Blazing', { exact: true })
  ).toBeVisible()
  const condition = syntheticItem.getByRole('button', {
    name: 'Forbidden Runes disabled',
  })
  await expect(condition).toBeVisible()
  await condition.click()
  await expect(
    page.getByText('Spawn conditions', { exact: true })
  ).toBeVisible()
  const enhancement = syntheticItem.getByRole('button', {
    name: 'Show enchantment conditions',
  })
  await enhancement.click()
  await expect(
    page.getByText('Enchantment conditions', { exact: true })
  ).toBeVisible()
  await page
    .getByRole('button', { name: 'Show trinket transmutation rotation' })
    .click()
  await expect(
    page.getByText('Trinket transmutation rotation', { exact: true })
  ).toBeVisible()
  await expect(page.getByText('Trap Mechanism', { exact: true })).toBeVisible()
  const sacrificeReward = page.getByRole('listitem').filter({
    hasText: 'weapon reward',
  })
  await expect(sacrificeReward).toHaveCount(1)
  await expect(sacrificeReward).toContainText('friendly sickle +2')
  await expect(sacrificeReward).toContainText('Fresh baseline')
  await expect(
    page.getByRole('button', { name: 'Expand floor 4 map' })
  ).toHaveCount(1)

  expect(browserErrors.console, 'browser console errors').toEqual([])
  expect(browserErrors.page, 'uncaught page errors').toEqual([])
})
