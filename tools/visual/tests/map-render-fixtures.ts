export type MapRenderFixture = {
  seed: string
  floor: number
}

/**
 * Regular floors with committed structural-layout canvas snapshots.
 */
export const MAP_RENDER_FIXTURES = [
  {
    seed: 'CXG-FJT-BFQ',
    floor: 1,
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 1,
  },
] as const satisfies readonly MapRenderFixture[]
