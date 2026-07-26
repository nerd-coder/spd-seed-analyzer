import { describe, expect, test } from 'bun:test'
import { MAP_RENDER_FIXTURES } from './map-render-fixtures'

describe('structural map-render snapshots', () => {
  test('registers unique regular-floor seed and floor pairs', () => {
    const registeredCases = MAP_RENDER_FIXTURES.map(({ seed, floor }) => {
      expect(seed).toMatch(/^[A-Z]{3}(?:-[A-Z]{3}){2}$/)
      expect(floor).toBeGreaterThan(0)
      return `${seed}:${floor}`
    })

    expect(new Set(registeredCases).size).toBe(registeredCases.length)
  })
})
