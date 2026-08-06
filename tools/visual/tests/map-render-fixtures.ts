export type MainLevelIdentity = {
  kind: 'main'
  depth: number
}

export type BranchLevelIdentity = {
  kind: 'branch'
  depth: number
  branch: number
  objective: 'Crystal' | 'Gnoll' | 'Fungi'
}

export type MapRenderLevelIdentity = MainLevelIdentity | BranchLevelIdentity

export type MapRenderFixture = {
  seed: string
  level: MapRenderLevelIdentity
}

/**
 * Main and branch levels with committed structural-layout canvas snapshots.
 */
export const MAP_RENDER_FIXTURES = [
  {
    seed: 'CXG-FJT-BFQ',
    level: { kind: 'main', depth: 1 },
  },
  {
    seed: 'HKT-JZN-XQQ',
    level: { kind: 'main', depth: 1 },
  },
  {
    seed: 'AAA-AAA-AAA',
    level: { kind: 'branch', depth: 13, branch: 1, objective: 'Crystal' },
  },
  {
    seed: 'AAA-AAA-AAB',
    level: { kind: 'branch', depth: 13, branch: 1, objective: 'Gnoll' },
  },
] as const satisfies readonly MapRenderFixture[]

export function mapRenderIdentityKey(level: MapRenderLevelIdentity) {
  return level.kind === 'main'
    ? `main:${level.depth}`
    : `branch:${level.depth}:${level.branch}:${level.objective}`
}

export function mapRenderSnapshotName(fixture: MapRenderFixture) {
  const { seed, level } = fixture
  return level.kind === 'main'
    ? `${seed}-F${level.depth}.png`
    : `${seed}-F${level.depth}-B${level.branch}-${level.objective}.png`
}

export function mapRenderTestName(fixture: MapRenderFixture) {
  const { seed, level } = fixture
  return level.kind === 'main'
    ? `${seed} floor ${level.depth} structural layout`
    : `${seed} floor ${level.depth} branch ${level.branch} ${level.objective} structural layout`
}
