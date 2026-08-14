import { expect, type Page, test } from '@playwright/test'

const APP_STORAGE = {
  mode: 'spd-analyzer-mode',
  theme: 'spd-analyzer-theme',
} as const

async function configureNoMatchSearch(page: Page, startSeed: string) {
  await page.getByRole('spinbutton', { name: 'Start seed' }).fill(startSeed)
  await page.getByRole('spinbutton', { name: 'Candidates' }).fill('10')
  await page.getByRole('combobox', { name: 'Depth' }).selectOption('1')
  await page.getByRole('spinbutton', { name: 'Results' }).fill('1')
  await page.getByLabel('Item 1 name').selectOption('RingOfMight')
  await page.getByLabel('Item 1 upgrade level').selectOption('4')
}

async function startAndWait(page: Page) {
  await page.getByRole('button', { name: 'Find' }).click()
  await expect(page.getByRole('button', { name: 'Find' })).toBeVisible({
    timeout: 60_000,
  })
}

test('finder keeps its form and reuses only result-less search tabs', async ({
  page,
}) => {
  const consoleErrors: string[] = []
  const pageErrors: string[] = []
  page.on('console', (message) => {
    if (message.type() === 'error') consoleErrors.push(message.text())
  })
  page.on('pageerror', (error) => pageErrors.push(error.message))
  await page.emulateMedia({ colorScheme: 'light', reducedMotion: 'reduce' })
  await page.addInitScript((storage) => {
    localStorage.clear()
    localStorage.setItem(storage.mode, 'finder')
    localStorage.setItem(storage.theme, 'light')
  }, APP_STORAGE)

  await page.goto('/')
  await configureNoMatchSearch(page, '111')
  await page.getByRole('tab', { name: 'Analyze' }).click()
  await page.getByRole('tab', { name: 'Find' }).click()
  await expect(
    page.getByRole('spinbutton', { name: 'Start seed' })
  ).toHaveValue('111')
  await expect(page.getByLabel('Item 1 name')).toHaveValue('RingOfMight')
  await expect(
    page.getByRole('switch', { name: 'Include fresh-baseline matches' })
  ).toHaveCount(0)

  await startAndWait(page)
  await expect(page.getByRole('tab', { name: '111 (0)' })).toBeVisible()

  await configureNoMatchSearch(page, '222')
  await startAndWait(page)
  await expect(page.getByRole('tab', { name: '222 (0)' })).toBeVisible()
  await expect(page.getByRole('tab', { name: '111 (0)' })).toHaveCount(0)

  await page
    .getByRole('spinbutton', { name: 'Start seed' })
    .fill('3293380032588')
  await page.getByRole('combobox', { name: 'Depth' }).selectOption('4')
  await page.getByLabel('Item 1 name').selectOption('RingOfWealth')
  await page.getByLabel('Item 1 upgrade level').selectOption('any')
  await startAndWait(page)
  await expect(
    page.getByRole('tab', { name: '3293380032588 (1)' })
  ).toBeVisible()

  await configureNoMatchSearch(page, '333')
  await startAndWait(page)
  await expect(page.getByRole('tab', { name: '333 (0)' })).toBeVisible()
  await expect(
    page.getByRole('tab', { name: '3293380032588 (1)' })
  ).toBeVisible()

  expect(consoleErrors, 'browser console errors').toEqual([])
  expect(pageErrors, 'uncaught page errors').toEqual([])
})

test('finder searches successfully when item name is set to any', async ({
  page,
}) => {
  const consoleErrors: string[] = []
  const pageErrors: string[] = []
  page.on('console', (message) => {
    if (message.type() === 'error') consoleErrors.push(message.text())
  })
  page.on('pageerror', (error) => pageErrors.push(error.message))
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
  await page.getByLabel('Item 1 name').selectOption('any')
  await page.getByLabel('Item 1 upgrade level').selectOption('any')

  await startAndWait(page)
  await expect(
    page.getByRole('tab', { name: '3293380032588 (1)' })
  ).toBeVisible()
  const result = page
    .locator('[data-slot="item"]')
    .filter({ hasText: 'Ring of Wealth' })
  await expect(result).toBeVisible()

  expect(consoleErrors, 'browser console errors').toEqual([])
  expect(pageErrors, 'uncaught page errors').toEqual([])
})

test('finder shortcuts randomize the seed and submit the search', async ({
  page,
}) => {
  await page.emulateMedia({ colorScheme: 'light', reducedMotion: 'reduce' })
  await page.addInitScript((storage) => {
    localStorage.clear()
    localStorage.setItem(storage.mode, 'finder')
    localStorage.setItem(storage.theme, 'light')
  }, APP_STORAGE)

  await page.goto('/')
  await configureNoMatchSearch(page, '111')

  const startSeed = page.getByRole('spinbutton', { name: 'Start seed' })
  const randomButton = page.getByRole('button', {
    name: 'Choose a random start seed',
  })
  await expect(randomButton).toHaveAttribute('aria-keyshortcuts', 'Control+R')
  await randomButton.hover()
  await expect(page.getByRole('tooltip')).toContainText('Ctrl + R')

  const seedBefore = await startSeed.inputValue()
  await page.keyboard.press('Control+r')
  await expect(startSeed).not.toHaveValue(seedBefore)

  const findButton = page.getByRole('button', { name: 'Find' })
  await expect(findButton).toHaveAttribute('aria-keyshortcuts', 'Control+F')
  await findButton.focus()
  await expect(
    page.getByRole('tooltip').filter({ hasText: 'Find' })
  ).toContainText('Ctrl + F')

  await page.keyboard.press('Control+f')
  await expect(page.getByText('10 / 10 scanned')).toBeVisible({
    timeout: 60_000,
  })
})

test("Don't let me down retries an empty search from a random seed", async ({
  page,
}) => {
  await page.emulateMedia({ colorScheme: 'light', reducedMotion: 'reduce' })
  await page.addInitScript((storage) => {
    localStorage.clear()
    localStorage.setItem(storage.mode, 'finder')
    localStorage.setItem(storage.theme, 'light')
  }, APP_STORAGE)

  await page.goto('/')
  await configureNoMatchSearch(page, '111')

  const nonStop = page.getByRole('switch', { name: "Don't let me down" })
  await expect(nonStop).toBeVisible()
  await page
    .getByRole('button', { name: "About Don't let me down mode" })
    .hover()
  await expect(page.getByRole('tooltip')).toContainText(
    'until the targeted result count is reached'
  )
  await nonStop.click()
  await page.getByRole('button', { name: 'Find' }).click()

  await expect
    .poll(
      async () => {
        const text = await page.getByText(/^Current:/).textContent()
        return Number(text?.match(/^Current:\s+(\d+)/)?.[1] ?? 0)
      },
      { timeout: 60_000 }
    )
    .toBeGreaterThan(120)
  await expect(page.getByText('Attempt 2')).toBeVisible()
  await expect(page.getByText(/^Start seed \d+$/)).not.toHaveText(
    'Start seed 111'
  )

  await page.getByRole('button', { name: 'Cancel' }).click()
  await expect(page.getByText('Search cancelled')).toBeVisible()
})
