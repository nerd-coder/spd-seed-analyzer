import { describe, expect, test } from 'bun:test'
import { readdir } from 'node:fs/promises'

import { MAP_RENDER_FIXTURES } from './map-render-fixtures'

const VISUAL_FIXTURE_DIRECTORY = new URL('../fixtures/', import.meta.url)

describe('map-render visual fixtures', () => {
  test('registers every on-disk PNG with matching seed and floor metadata', async () => {
    const registeredFiles: string[] = MAP_RENDER_FIXTURES.map(
      ({ seed, floor, referenceFile }) => {
        expect(seed).toMatch(/^[A-Z]{3}(?:-[A-Z]{3}){2}$/)
        expect(floor).toBeGreaterThan(0)
        expect(String(referenceFile)).toBe(`${seed}_F${floor}.png`)
        return String(referenceFile)
      }
    )

    expect(new Set(registeredFiles).size).toBe(registeredFiles.length)

    const onDiskFiles = (await readdir(VISUAL_FIXTURE_DIRECTORY))
      .filter((file) => file.endsWith('.png'))
      .sort()

    expect([...registeredFiles].sort()).toEqual(onDiskFiles)
  })

  test('includes the CXG-FJT-BFQ floor-one test map', () => {
    expect(MAP_RENDER_FIXTURES).toContainEqual({
      seed: 'CXG-FJT-BFQ',
      floor: 1,
      referenceFile: 'CXG-FJT-BFQ_F1.png',
    })
  })
})
