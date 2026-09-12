import { describe, expect, test } from 'bun:test'
import {
  MAP_RENDER_FIXTURES,
  mapRenderBranchTitle,
  mapRenderIdentityKey,
  mapRenderSnapshotName,
} from './map-render-fixtures'

describe('structural map-render snapshots', () => {
  test('registers valid, unique level identities and snapshot names', () => {
    const registeredCases = MAP_RENDER_FIXTURES.map(({ seed, level }) => {
      expect(seed).toMatch(/^[A-Z]{3}(?:-[A-Z]{3}){2}$/)
      expect(level.depth).toBeGreaterThan(0)
      if (level.kind === 'branch') {
        expect(level.branch).toBeGreaterThan(0)
        expect(['Crystal', 'Gnoll', 'Fungi', 'Vault']).toContain(
          level.objective
        )
      }
      return `${seed}:${mapRenderIdentityKey(level)}`
    })
    const snapshotNames = MAP_RENDER_FIXTURES.map(mapRenderSnapshotName)

    expect(new Set(registeredCases).size).toBe(registeredCases.length)
    expect(new Set(snapshotNames).size).toBe(snapshotNames.length)
    expect(snapshotNames).toEqual([
      'CXG-FJT-BFQ-F1.png',
      'HKT-JZN-XQQ-F1.png',
      'GFX-PZH-DCH-F16.png',
      'AAA-AAA-AAA-F13-B1-Crystal.png',
      'AAA-AAA-AAB-F13-B1-Gnoll.png',
      'AAA-AAA-AAA-F19-B1-Vault.png',
    ])
  })

  test('branch titles follow live UI labels', () => {
    expect(mapRenderBranchTitle('Vault')).toBe('Imp Vault')
    expect(mapRenderBranchTitle('Crystal')).toBe('Blacksmith Mine')
    expect(mapRenderBranchTitle('Gnoll')).toBe('Blacksmith Mine')
    expect(mapRenderBranchTitle('Fungi')).toBe('Blacksmith Mine')
  })
})
