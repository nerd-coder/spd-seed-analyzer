export type MapRenderFixture = {
  seed: string
  floor: number
  referenceFile: string
  target: 'regular-floor' | 'quest-branch'
  expectation: 'rendered' | 'intentionally-omitted'
}

/**
 * Canonical input list for manual and automated map-render QA.
 *
 * Keep this registry in lockstep with tools/visual/fixtures/*.png. A fixture
 * only identifies a seed/floor reference; it does not claim pixel parity.
 */
export const MAP_RENDER_FIXTURES = [
  {
    seed: 'CXG-FJT-BFQ',
    floor: 1,
    referenceFile: 'CXG-FJT-BFQ_F1.png',
    target: 'regular-floor',
    expectation: 'intentionally-omitted',
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 1,
    referenceFile: 'HKT-JZN-XQQ_F1.png',
    target: 'regular-floor',
    expectation: 'intentionally-omitted',
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 6,
    referenceFile: 'HKT-JZN-XQQ_F6.png',
    target: 'regular-floor',
    expectation: 'intentionally-omitted',
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 8,
    referenceFile: 'HKT-JZN-XQQ_F8.png',
    target: 'regular-floor',
    expectation: 'intentionally-omitted',
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 12,
    referenceFile: 'HKT-JZN-XQQ_F12.png',
    target: 'regular-floor',
    expectation: 'intentionally-omitted',
  },
  {
    seed: 'HKT-JZN-XQQ',
    floor: 12,
    referenceFile: 'HKT-JZN-XQQ_F12_Q.png',
    target: 'quest-branch',
    expectation: 'intentionally-omitted',
  },
] as const satisfies readonly MapRenderFixture[]

export const AUTOMATED_MAP_RENDER_FIXTURES = MAP_RENDER_FIXTURES.filter(
  (fixture) => fixture.target === 'regular-floor'
)
