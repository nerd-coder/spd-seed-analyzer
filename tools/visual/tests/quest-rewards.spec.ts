import { expect, type Page, test } from '@playwright/test'

const SEED = 'QUEST-BASELINE'

function collectBrowserErrors(page: Page) {
  const errors: string[] = []
  page.on('console', (message) => {
    if (message.type() === 'error' || message.type() === 'warning') {
      errors.push(message.text())
    }
  })
  page.on('pageerror', (error) => errors.push(error.message))
  return errors
}

async function installQuestReport(page: Page, includeBaselines: boolean) {
  await page.addInitScript(
    ({ includeBaselines, seed }) => {
      const wandItems: Record<string, unknown>[] = [
        {
          source: 'Wandmaker.Quest',
          variants: [
            {
              name: 'wand reward',
              quantity: 1,
              category: 'wand',
              level_range: { min: 1, max: 3 },
              cursed: false,
              prediction: 'constrained',
            },
          ],
        },
        {
          source: 'Wandmaker.Quest',
          variants: [
            {
              name: 'wand reward',
              quantity: 1,
              category: 'wand',
              level_range: { min: 1, max: 3 },
              cursed: false,
              prediction: 'constrained',
            },
          ],
        },
      ]
      const impItems: Record<string, unknown>[] = [
        {
          source: 'Imp.Quest',
          variants: [
            {
              name: 'ring reward',
              quantity: 1,
              category: 'ring',
              level_range: { min: 2, max: 4 },
              cursed: true,
              prediction: 'constrained',
            },
          ],
        },
      ]
      if (includeBaselines) {
        wandItems.push(
          {
            source: 'Wandmaker.Quest',
            variants: [
              {
                name: 'wand of blast wave +2',
                quantity: 1,
                class_name: 'WandOfBlastWave',
                category: 'wand',
                level: 2,
                cursed: false,
                prediction: 'baseline',
              },
            ],
          },
          {
            source: 'Wandmaker.Quest',
            variants: [
              {
                name: 'wand of corrosion +1',
                quantity: 1,
                class_name: 'WandOfCorrosion',
                category: 'wand',
                level: 1,
                cursed: false,
                prediction: 'baseline',
              },
            ],
          }
        )
        impItems.push({
          source: 'Imp.Quest',
          variants: [
            {
              name: 'ring of haste +3',
              quantity: 1,
              class_name: 'RingOfHaste',
              category: 'ring',
              level: 3,
              cursed: true,
              prediction: 'baseline',
            },
          ],
        })
      }

      const report = {
        seed: {
          input: seed,
          numeric: 0,
          code: null,
          formatted: seed,
        },
        spd_version: 'v3.3.8',
        spd_commit: '7b8b845a7',
        floors_requested: 19,
        identities: { potions: [], scrolls: [], rings: [] },
        trinket_selection: {
          catalyst_depth: 2,
          first_alchemy_pot_depth: 3,
          first_alchemy_pot_is_secret: false,
          selection_depth: 3,
          first_effective_depth: 4,
          catalyst_options: [],
          transmutation_sequence: [],
        },
        floors: [
          {
            depth: 9,
            items: wandItems,
            quests: [
              {
                type: 'old_wandmaker',
                contract: {
                  spawn_depth_range: { min: 7, max: 9 },
                  objective_options: [
                    'corpse_dust',
                    'elemental_embers',
                    'rotberry',
                  ],
                  rewards: {
                    item_source: 'Wandmaker.Quest',
                    option_count: 2,
                    selected_count: 1,
                  },
                },
                baseline: { objective: 'rotberry' },
              },
            ],
          },
          {
            depth: 19,
            items: impItems,
            quests: [
              {
                type: 'ambitious_imp',
                contract: {
                  spawn_depth_range: { min: 17, max: 19 },
                  target_rules: [
                    {
                      spawn_depth: 19,
                      target: 'golem',
                      required_tokens: 4,
                    },
                  ],
                  rewards: {
                    item_source: 'Imp.Quest',
                    option_count: 1,
                    selected_count: 1,
                  },
                },
                baseline: { target: 'golem', required_tokens: 4 },
              },
            ],
          },
        ],
        status: 'partial',
      }

      class QuestFixtureWorker {
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

      Object.assign(window, { Worker: QuestFixtureWorker })
    },
    { includeBaselines, seed: SEED }
  )
}

async function openQuestReport(page: Page, includeBaselines: boolean) {
  await installQuestReport(page, includeBaselines)
  await page.addInitScript(() => {
    localStorage.clear()
    localStorage.setItem('spd-analyzer-mode', 'analyze')
    localStorage.setItem('spd-analyzer-theme', 'light')
  })
  await page.goto('/')
  await page.getByLabel('Enter your seed').fill(SEED)
  await page.getByRole('button', { name: 'Analyze', exact: true }).click()
  await expect(
    page.getByRole('heading', { name: 'Floor 9', exact: true })
  ).toBeVisible()
}

test('quest cards prefer concrete baselines and keep the universal warning visible', async ({
  page,
}) => {
  const browserErrors = collectBrowserErrors(page)
  await openQuestReport(page, true)

  const wandmaker = page.locator('[data-quest-type="old_wandmaker"]')
  await expect(wandmaker).toContainText('Baseline target: Rotberry')
  await expect(wandmaker).toContainText(
    'Reward contract: two distinct uncursed +1…+3 wands'
  )
  await expect(wandmaker.getByText('Baseline rewards')).toBeVisible()
  await expect(wandmaker.getByRole('listitem')).toHaveCount(2)
  await expect(wandmaker.getByText('wand reward', { exact: true })).toHaveCount(
    0
  )
  await expect(wandmaker.getByText('OR', { exact: true })).toHaveCount(0)

  await page.getByRole('tab', { name: /^City/ }).click()
  const imp = page.locator('[data-quest-type="ambitious_imp"]')
  await expect(imp).toContainText('Baseline target: Golem (4 tokens)')
  await expect(imp).toContainText(
    'Reward contract: one cursed +2…+4 ring after completing the quest.'
  )
  await expect(imp.getByText('Baseline rewards')).toBeVisible()
  await expect(imp.getByRole('listitem')).toHaveCount(1)
  await expect(imp.getByText('ring reward', { exact: true })).toHaveCount(0)
  await expect(imp.getByText('OR', { exact: true })).toHaveCount(0)
  expect(browserErrors).toEqual([])
})

test('quest cards fall back to the universal reward entries without a baseline', async ({
  page,
}) => {
  const browserErrors = collectBrowserErrors(page)
  await openQuestReport(page, false)

  const wandmaker = page.locator('[data-quest-type="old_wandmaker"]')
  await expect(wandmaker.getByText('Rewards', { exact: true })).toBeVisible()
  await expect(wandmaker.getByText('Baseline rewards')).toHaveCount(0)
  await expect(wandmaker.getByText('wand reward', { exact: true })).toHaveCount(
    2
  )
  expect(browserErrors).toEqual([])
})
