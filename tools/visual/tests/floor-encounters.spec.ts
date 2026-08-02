import { expect, type Page, test } from '@playwright/test'

const TEST_STORAGE_KEY = 'spd-analyzer-encounter-test-initialized'

async function openFloorOne(page: Page, seed: string) {
  await page.addInitScript((storageKey) => {
    if (sessionStorage.getItem(storageKey) !== 'true') {
      localStorage.clear()
      localStorage.setItem('spd-analyzer-mode', 'analyze')
      localStorage.setItem('spd-analyzer-theme', 'light')
      sessionStorage.setItem(storageKey, 'true')
    }
  }, TEST_STORAGE_KEY)

  await page.goto('/')
  await page.getByLabel('Enter your seed').fill(seed)
  await page.getByRole('button', { name: 'Analyze', exact: true }).click()
  const heading = page.getByRole('heading', { name: 'Floor 1', exact: true })
  await expect(heading).toBeVisible({ timeout: 60_000 })
  return heading.locator('xpath=ancestor::section[1]')
}

test('floor one shows exact room rewards and non-positional encounters', async ({
  page,
}) => {
  const browserErrors: string[] = []
  page.on('console', (message) => {
    if (message.type() === 'error') browserErrors.push(message.text())
  })
  page.on('pageerror', (error) => browserErrors.push(error.message))

  const floor = await openFloorOne(page, 'AAA-AAA-AFU')
  await expect(
    floor.locator('p').filter({ hasText: 'Initial encounters' })
  ).toContainText(/Initial encounters\s*\(8\)/)
  await expect(
    floor.locator('[data-slot="item-title"]').filter({ hasText: 'Rat' })
  ).toContainText(/Rat\s*x6/)
  await expect(
    floor.locator('[data-slot="item-title"]').filter({ hasText: 'Snake' })
  ).toContainText(/Snake\s*x2/)
  await expect(floor.getByText('No seed-determined base drop')).toBeVisible()
  await expect(floor.getByText('25% runtime chance: random seed')).toBeVisible()
  await expect(floor.getByText('Runtime roll')).toBeVisible()
  await expect(floor.getByText(/stone of blast/i)).toBeVisible()
  await expect(floor.getByText(/stone of deep sleep/i)).toBeVisible()
  await expect(floor.getByText('2–3 runestone-room rewards')).toHaveCount(0)

  await page.setViewportSize({ width: 390, height: 844 })
  await floor.scrollIntoViewIfNeeded()
  const viewport = await page.evaluate(() => ({
    clientWidth: document.documentElement.clientWidth,
    scrollWidth: document.documentElement.scrollWidth,
  }))
  expect(viewport.scrollWidth).toBe(viewport.clientWidth)
  await expect(
    floor.locator('p').filter({ hasText: 'Initial encounters' })
  ).toBeVisible()
  expect(browserErrors).toEqual([])
})

test('Shattered Pot uses its in-game item sprite', async ({ page }) => {
  await openFloorOne(page, 'MWH-KAE-DHG')
  const floor = page
    .getByRole('heading', { name: 'Floor 2', exact: true })
    .locator('xpath=ancestor::section[1]')
  const icon = floor.getByRole('img', { name: 'Shattered Pot' })

  await expect(icon).toBeVisible()
  await expect(icon.locator('[aria-hidden="true"]')).toHaveCSS(
    'background-position',
    '-96px -48px'
  )
})
