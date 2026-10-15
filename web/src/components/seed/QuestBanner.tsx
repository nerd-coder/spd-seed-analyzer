import type { ReactNode } from 'react'

import { FloorMapDialog } from '@/components/FloorMapDialog'
import { FloorBanner } from '@/components/seed/FloorBanner'
import type { BranchFloorReport, IdentityMaps } from '@/lib/spd-wasm'
import { branchTitle } from './quest-branch'

/**
 * Quest header in the same frame as a floor banner: NPC, name, and target.
 * A quest with its own branch floor adds the map button.
 */
export function QuestBanner({
  depth,
  title,
  npc,
  target,
  branch,
  identities,
}: {
  depth: number
  title: string
  npc?: ReactNode
  target?: ReactNode
  branch?: BranchFloorReport | null
  identities: IdentityMaps
}) {
  const displayedMap = branch
    ? (branch.map ?? branch.assumed_map ?? null)
    : null
  let action: ReactNode = null
  if (branch && displayedMap) {
    const identity = `Floor ${branch.id.depth}, branch ${branch.id.branch}`
    const mapName = branchTitle(branch.kind)
    action = (
      <FloorMapDialog
        map={displayedMap}
        identities={identities}
        depth={branch.id.depth}
        mapLabel={`${mapName} ${identity.toLowerCase()} map`}
        dialogTitle={`${mapName} - ${identity}`}
      />
    )
  }

  return (
    <div data-branch-kind={branch?.kind} className="w-full">
      <FloorBanner
        depth={depth}
        tileset={displayedMap?.tileset}
        title={title}
        headingLevel={4}
        showDepth={false}
        leading={npc}
        meta={null}
        action={action}
      >
        {target}
      </FloorBanner>
    </div>
  )
}
