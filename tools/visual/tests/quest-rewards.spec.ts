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
              name: 'artifact or ring',
              quantity: 1,
              category: 'artifact',
              cursed: false,
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
              name: 'sandals of nature +2',
              quantity: 1,
              class_name: 'SandalsOfNature',
              category: 'artifact',
              level: 2,
              cursed: false,
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
        spd_version: 'v4.0.0',
        spd_commit: '2bb34a4e9',
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
            possible_rooms: [
              {
                class: 'MassGraveRoom',
                quantity: 1,
                spawn_conditions: [
                  {
                    all_of: [
                      {
                        type: 'trinket',
                        events: [
                          {
                            before_depth: 4,
                            kind: 'acquired',
                            trinket: 'mossy_clump',
                          },
                        ],
                      },
                    ],
                  },
                ],
              },
              {
                class: 'RotGardenRoom',
                quantity: 1,
                spawn_conditions: [
                  {
                    all_of: [
                      {
                        type: 'challenge',
                        challenge: 'forbidden_runes',
                        enabled: true,
                      },
                    ],
                  },
                ],
              },
            ],
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
                  rewards: {
                    item_source: 'Imp.Quest',
                    option_count: 6,
                    selected_count: 1,
                    favor_requirement: 4000,
                  },
                  slots: [
                    {
                      slot: 0,
                      role: 'artifact',
                      baseline_class: 'SandalsOfNature',
                      level_rule: { type: 'transfer_upgrade', transfer: 5 },
                      enchanted: false,
                      scenario_count: 3,
                      candidates: [
                        {
                          class_name: 'SandalsOfNature',
                          drifts: [0],
                          weight: 1,
                        },
                        {
                          class_name: 'HornOfPlenty',
                          drifts: [-1],
                          weight: 1,
                        },
                        { class_name: 'SkeletonKey', drifts: [1], weight: 1 },
                      ],
                    },
                    {
                      slot: 4,
                      role: 'armor',
                      baseline_class: 'PlateArmor',
                      level_rule: { type: 'range', min: 2, max: 4 },
                      enchanted: true,
                      fixed_class: 'PlateArmor',
                      scenario_count: 1,
                      candidates: [],
                    },
                  ],
                },
                baseline: { spawn_depth: 19 },
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

  const floorNine = page
    .getByRole('heading', { name: 'Floor 9', exact: true })
    .locator('xpath=ancestor::section[1]')
  await floorNine.getByRole('button', { name: 'Possible rooms (2)' }).click()
  const possibleRooms = page.getByRole('dialog')
  await expect(possibleRooms).toContainText('Possible rooms')
  await expect(possibleRooms).toContainText('Mass Grave')
  await expect(possibleRooms).toContainText('Mossy Clump is acquired')
  await expect(possibleRooms).toContainText('Rot Garden')
  await expect(possibleRooms).toContainText('Forbidden Runes enabled')

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
  await expect(imp).toContainText('Spawned on depth 19')
  await expect(imp).toContainText(
    'Reward contract: six vault take-out options; the player keeps one.'
  )
  await expect(imp).toContainText(
    'The floor 20/21 Imp shop is not guaranteed. It appears only after completing the vault with score > 2000.'
  )
  await expect(imp).not.toContainText(/monk|golem|token|cursed \+2/i)
  await expect(imp).toContainText(
    'Keeping one of these six requires leaving with the Imp Statue'
  )
  await expect(imp.getByText('Rewards', { exact: true })).toBeVisible()
  await expect(imp.getByText('artifact or ring', { exact: true })).toHaveCount(
    0
  )
  await expect(imp.getByText('OR', { exact: true })).toHaveCount(0)
  await expect(imp).not.toContainText('Per-slot outlook')
  await expect(imp).not.toContainText(/share of ±/)

  await imp.getByRole('button', { name: 'Per-slot reward outlook' }).click()
  const outlook = page
    .getByRole('dialog')
    .filter({ hasText: 'Per-slot outlook' })
  await expect(outlook).toBeVisible()
  await expect(outlook).toContainText('These shares cover a ±1 shift')
  await expect(outlook).toContainText('+5 transferred')
  const artifactRows = outlook
    .getByRole('listitem')
    .first()
    .getByRole('listitem')
  await expect(artifactRows).toHaveCount(3)
  await expect(artifactRows.first()).toContainText('Sandals Of Nature')
  await expect(artifactRows.first()).toContainText('fresh run')
  await expect(artifactRows.first()).toContainText('baseline')
  await expect(artifactRows.nth(1)).toContainText('−1')
  await expect(artifactRows.nth(2)).toContainText('+1')
  // The plate slot has no deck, so it reports a single fixed class.
  await expect(outlook).toContainText('Plate Armor')
  await expect(outlook).toContainText('100%')
  await expect(outlook).toContainText('glyphed')
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

  await page.getByRole('tab', { name: /^City/ }).click()
  const imp = page.locator('[data-quest-type="ambitious_imp"]')
  await expect(imp.getByText('Rewards', { exact: true })).toBeVisible()
  await expect(
    imp.getByRole('button', { name: 'Per-slot reward outlook' })
  ).toBeVisible()
  await expect(imp).toContainText(
    'The floor 20/21 Imp shop is not guaranteed. It appears only after completing the vault with score > 2000.'
  )
  await expect(imp).not.toContainText(/monk|golem|token|cursed \+2/i)
  expect(browserErrors).toEqual([])
})
