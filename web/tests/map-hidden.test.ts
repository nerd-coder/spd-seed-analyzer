import { describe, expect, test } from 'bun:test'
import { mobIdFromClassName } from '@/generated/game-ids'
import { Terrain } from '@/lib/dungeon-tile-visuals'
import { LURKING_OPACITY } from '@/lib/map-entities'
import { concealedMap, hiddenRegions, revealedMap } from '@/lib/map-hidden'
import type { FloorMap } from '@/lib/spd-wasm'

describe('concealedMap', () => {
  test('masks custom art over and overhanging unexplored cells', () => {
    const width = 3
    const map = {
      width,
      height: 3,
      tiles: [
        Terrain.EMPTY,
        Terrain.WALL,
        Terrain.EMPTY,
        Terrain.EMPTY,
        Terrain.STATUE,
        Terrain.EMPTY,
        Terrain.EMPTY,
        Terrain.EMPTY,
        Terrain.EMPTY,
      ],
      heaps: [],
      mobs: [],
      markers: [],
      custom_tiles: [
        {
          class: 'Carpet',
          texture: 'carpet',
          x: 0,
          y: 1,
          width: 3,
          height: 1,
          static_data: [1, 2, 3],
        },
      ],
      custom_walls: [
        {
          class: 'StatueOverhang',
          texture: 'rat_king_room',
          x: 0,
          y: 0,
          width: 3,
          height: 2,
          static_data: [5, 6, 7, 8, 9, 10],
        },
      ],
    } as unknown as FloorMap

    const concealed = concealedMap(map, new Set([1 * width + 1]))
    expect(concealed.custom_tiles[0]?.static_data).toEqual([1, -1, 3])
    expect(concealed.custom_walls[0]?.static_data).toEqual([
      5, -1, 7, 8, -1, 10,
    ])
    expect(map.custom_walls[0]?.static_data).toEqual([5, 6, 7, 8, 9, 10])
  })
})

describe('lurking ebony mimic', () => {
  test('stays faint on a closed door before searching', () => {
    const ebony = mobIdFromClassName('EbonyMimic')
    const map = {
      width: 3,
      height: 2,
      tiles: [
        Terrain.WALL,
        Terrain.WALL,
        Terrain.WALL,
        Terrain.EMPTY,
        Terrain.DOOR,
        Terrain.EMPTY,
      ],
      heaps: [],
      mobs: [{ cell: 4, mob: ebony, state: 'passive' }],
      markers: [],
      traps: [],
      custom_tiles: [],
      custom_walls: [],
      concealed_cells: [4],
    } as unknown as FloorMap

    expect(revealedMap(map).tiles[4]).toBe(Terrain.DOOR)
    expect(revealedMap(map).mobs[0]?.opacity).toBeUndefined()
    expect(concealedMap(map, new Set()).mobs).toEqual([
      { cell: 4, mob: ebony, state: 'passive', opacity: LURKING_OPACITY },
    ])
    expect(hiddenRegions(map).regions).toEqual([[4, 1]])
  })
})
