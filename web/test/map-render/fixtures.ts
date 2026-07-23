export type MapRenderFixture = {
  seed: string
  floor: number
  referenceFile: string
}

/**
 * Canonical input list for manual and automated map-render QA.
 *
 * Keep this registry in lockstep with specs/fixtures/visual/*.png. A fixture
 * only identifies a seed/floor reference; it does not claim pixel parity.
 */
export const MAP_RENDER_FIXTURES = [
  {
    seed: 'CXG-FJT-BFQ',
    floor: 1,
    referenceFile: 'CXG-FJT-BFQ_F1.png',
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 1,
    referenceFile: 'HKT-JZN-XQQ_F1.png',
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 6,
    referenceFile: 'HKT-JZN-XQQ_F6.png',
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 8,
    referenceFile: 'HKT-JZN-XQQ_F8.png',
  },
] as const satisfies readonly MapRenderFixture[]
