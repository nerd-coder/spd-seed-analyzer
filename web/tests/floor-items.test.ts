import { describe, expect, test } from 'bun:test'
import { visibleItemGroups } from '@/components/seed/floor-items'
import { summarizeItemContract } from '@/lib/item-contract'
import type { ItemGroup } from '@/lib/spd-wasm'

describe('visibleItemGroups', () => {
  test('prefers a variable sacrifice sample and keeps its contract', () => {
    const group: ItemGroup = {
      source: 'SacrificeRoom',
      variants: [
        {
          label: 'weapon reward',
          quantity: 1,
          category: 'weapon',
          cursed: true,
          prediction: 'constrained',
          tier_range: { min: 2, max: 5 },
          level_range: { min: 0, max: 3 },
        },
        {
          quantity: 1,
          item: 164,
          category: 'weapon',
          level: 1,
          cursed: true,
          enchantment: { type: 31, conditions: [] },
          prediction: 'variable',
        },
      ],
    }

    const visible = visibleItemGroups([group])
    expect(visible).toHaveLength(1)
    expect(visible[0]?.variants).toHaveLength(1)
    expect(visible[0]?.variants[0]?.prediction).toBe('variable')
    expect(visible[0]?.variants[0]?.item).toBe(164)
    const contract = visible[0]?.contract
    expect(contract?.prediction).toBe('constrained')
    expect(contract?.label).toBe('weapon reward')
    expect(contract && summarizeItemContract(contract)).toEqual({
      summary: 'weapon reward',
      facts: ['Cursed', 'Tier 2–5', 'Upgrade +0–+3'],
    })
  })

  test('hides a blank constrained stub without a variable sample', () => {
    const group: ItemGroup = {
      source: 'SacrificeRoom',
      variants: [
        {
          label: 'weapon reward',
          quantity: 1,
          category: 'weapon',
          cursed: true,
          prediction: 'constrained',
        },
      ],
    }

    expect(visibleItemGroups([group])).toHaveLength(0)
  })

  test('keeps a constrained entry that lists candidates', () => {
    const group: ItemGroup = {
      source: 'SecretHoneypotRoom:bomb',
      variants: [
        {
          label: 'Bomb reward',
          quantity: 1,
          candidates: [20, 52],
          category: 'other',
          prediction: 'constrained',
        },
      ],
    }

    const visible = visibleItemGroups([group])
    expect(visible[0]?.variants).toHaveLength(1)
    expect(visible[0]?.contract).toBeUndefined()
  })

  test('summarizes a seeded quantity range', () => {
    expect(
      summarizeItemContract({
        quantity: 6,
        quantity_range: { min: 6, max: 10 },
        item: 66,
        category: 'other',
        cursed: false,
        prediction: 'exact',
      }).facts
    ).toEqual(['Quantity 6–10', 'Uncursed'])
  })
})
