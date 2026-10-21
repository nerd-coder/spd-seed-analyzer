import { expect, type Page, test } from '@playwright/test'
import { GAME_IDS } from './game-ids'

const SEED = 'QUE-STB-ASE'

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

async function installQuestReport(page: Page, includeVariable: boolean) {
  await page.addInitScript(
    ({ includeVariable, seed, ids: { Enchantment, ItemId, RoomId } }) => {
      const wandItems: Record<string, unknown>[] = [
        {
          source: 'Wandmaker.Quest',
          variants: [
            {
              label: 'wand reward',
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
              label: 'wand reward',
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
              label: 'artifact or ring',
              quantity: 1,
              category: 'artifact',
              cursed: false,
              prediction: 'constrained',
            },
          ],
        },
      ]
      if (includeVariable) {
        wandItems.push(
          {
            source: 'Wandmaker.Quest',
            variants: [
              {
                quantity: 1,
                item: ItemId.WandOfBlastWave,
                category: 'wand',
                level: 2,
                cursed: false,
                prediction: 'variable',
              },
            ],
          },
          {
            source: 'Wandmaker.Quest',
            variants: [
              {
                quantity: 1,
                item: ItemId.WandOfCorrosion,
                category: 'wand',
                level: 1,
                cursed: false,
                prediction: 'variable',
              },
            ],
          }
        )
        impItems.push({
          source: 'Imp.Quest',
          variants: [
            {
              quantity: 1,
              item: ItemId.SandalsOfNature,
              category: 'artifact',
              level: 2,
              cursed: false,
              prediction: 'variable',
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
        spd_version: 'v4.0.2',
        spd_commit: '57a4e06a4',
        floors_requested: 19,
        identities: { potions: [], scrolls: [], rings: [] },
        trinket_selection: {
          catalyst_depth: 2,
          first_alchemy_pot_depth: 3,
          first_alchemy_pot_is_secret: false,
          selection_depth: 3,
          first_effective_depth: 4,
          catalyst_options: [],
          catalyst_option_effects: [],
          transmutation_sequence: [],
        },
        floors: [
          {
            depth: 9,
            possible_rooms: [
              {
                room: RoomId.MassGraveRoom,
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
                room: RoomId.RotGardenRoom,
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
                variable: { objective: 'rotberry' },
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
                      item: ItemId.SandalsOfNature,
                      level_rule: { type: 'transfer_upgrade', transfer: 5 },
                      enchanted: false,
                      level: 2,
                      scenario_count: 3,
                      candidates: [
                        {
                          item: ItemId.SandalsOfNature,
                          drifts: [0],
                          weight: 1,
                        },
                        {
                          item: ItemId.HornOfPlenty,
                          drifts: [-1],
                          weight: 1,
                        },
                        {
                          item: ItemId.SkeletonKey,
                          drifts: [1],
                          weight: 1,
                        },
                      ],
                    },
                    {
                      slot: 4,
                      role: 'armor',
                      item: ItemId.PlateArmor,
                      level_rule: { type: 'range', min: 2, max: 4 },
                      enchanted: true,
                      level: 4,
                      enchantment: Enchantment.Viscosity,
                      fixed_item: ItemId.PlateArmor,
                      scenario_count: 1,
                      candidates: [],
                    },
                  ],
                  upgrades_pinned: true,
                  total_upgrade_value: 21,
                },
                variable: { spawn_depth: 19 },
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
    { includeVariable, seed: SEED, ids: GAME_IDS }
  )
}

async function openQuestReport(page: Page, includeVariable: boolean) {
  await installQuestReport(page, includeVariable)
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

test('quest cards prefer concrete variable rewards', async ({ page }) => {
  const browserErrors = collectBrowserErrors(page)
  await openQuestReport(page, true)

  const floorNine = page
    .getByRole('heading', { name: 'Floor 9', exact: true })
    .locator('xpath=ancestor::section[1]')
  await expect(
    floorNine.getByRole('button', { name: /^Possible rooms/ })
  ).toHaveCount(0)
  await expect(
    floorNine.getByRole('button', { name: /^Rooms \(/ })
  ).toHaveCount(0)

  const wandmaker = page.locator('[data-quest-type="old_wandmaker"]')
  await expect(wandmaker.getByLabel('Rotberry', { exact: true })).toBeVisible()
  await expect(wandmaker).not.toContainText(/baseline/i)
  await expect(
    wandmaker.getByText('Rewards (vary with gameplay)')
  ).toBeVisible()
  await expect(wandmaker.getByRole('listitem')).toHaveCount(2)
  await expect(wandmaker.getByText('wand reward', { exact: true })).toHaveCount(
    0
  )
  await expect(wandmaker.getByText('OR', { exact: true })).toHaveCount(0)

  const imp = page.locator('[data-quest-type="ambitious_imp"]')
  await expect(imp).not.toContainText('Spawned on depth')
  await expect(imp.getByRole('img', { name: 'Imp', exact: true })).toBeVisible()
  await expect(imp).not.toContainText(/baseline/i)
  await expect(imp).not.toContainText(/monk|golem|token|cursed \+2/i)
  await expect(imp).not.toContainText('Keeping one of these six')
  await imp.getByRole('button', { name: 'About the Imp Statue' }).click()
  const statue = page.getByRole('dialog').filter({ hasText: 'Imp Statue' })
  await expect(statue).toBeVisible()
  await expect(statue).toContainText(
    'Keeping one of these six requires leaving with the Imp Statue: every option is +2 or better, and lower vault scores can only take out an item at +0/+1.'
  )
  await page.keyboard.press('Escape')
  await imp.getByRole('img', { name: 'Imp', exact: true }).click()
  const examine = page.locator('[data-slot="item-detail"]')
  await expect(examine).toBeVisible()
  await expect(
    examine.getByText('Ambitious imp', { exact: true })
  ).toBeVisible()
  await expect(examine).toContainText(
    'This little red person look like an Imp. Imps are lesser demons'
  )
  await page.keyboard.press('Escape')
  await expect(examine).toHaveCount(0)
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
  await expect(artifactRows.first()).toContainText('Sandals of Nature')
  await expect(artifactRows.first()).toContainText('seed draw')
  await expect(artifactRows.nth(1)).toContainText('−1')
  await expect(artifactRows.nth(2)).toContainText('+1')
  // The plate slot has no deck, so it reports a single fixed class.
  await expect(outlook).toContainText('Plate Armor')
  await expect(outlook).toContainText('100%')

  // Upgrades are seed-pinned, so each slot shows its concrete +N (the artifact
  // shows its transfer amount, not its cap-scaled stored level) and the glyph
  // is named rather than described.
  await expect(outlook).toContainText('+5')
  await expect(outlook).toContainText('of +5 transferred')
  await expect(outlook).toContainText('+4')
  await expect(outlook).toContainText('of +2…+4')
  await expect(outlook).toContainText('Viscosity')
  await expect(outlook).toContainText(
    'upgrades and enchants are fixed by the seed'
  )
  await expect(outlook).toContainText('21 upgrades across the six options')
  await expect(outlook).not.toContainText('undefined')
  await expect(outlook).not.toContainText('NaN')
  expect(browserErrors).toEqual([])
})

test('quest cards fall back to the universal reward entries without variable rewards', async ({
  page,
}) => {
  const browserErrors = collectBrowserErrors(page)
  await openQuestReport(page, false)

  const wandmaker = page.locator('[data-quest-type="old_wandmaker"]')
  await expect(wandmaker.getByText('Rewards', { exact: true })).toBeVisible()
  await expect(wandmaker.getByText('Rewards (vary with gameplay)')).toHaveCount(
    0
  )
  await expect(
    wandmaker.getByLabel('wand reward', { exact: true })
  ).toHaveCount(2)

  const imp = page.locator('[data-quest-type="ambitious_imp"]')
  await expect(imp.getByRole('img', { name: 'Imp', exact: true })).toBeVisible()
  await expect(imp).not.toContainText('Spawned on depth')
  await expect(imp).not.toContainText('Keeping one of these six')
  await expect(
    imp.getByRole('button', { name: 'About the Imp Statue' })
  ).toBeVisible()
  await expect(imp.getByText('Rewards', { exact: true })).toBeVisible()
  await expect(
    imp.getByRole('button', { name: 'Per-slot reward outlook' })
  ).toBeVisible()
  await expect(imp).not.toContainText(/monk|golem|token|cursed \+2/i)
  expect(browserErrors).toEqual([])
})
