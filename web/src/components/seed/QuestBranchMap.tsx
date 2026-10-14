import type { ReactNode } from 'react'

import { FloorMapDialog } from '@/components/FloorMapDialog'
import { FloorBanner } from '@/components/seed/FloorBanner'
import type { BranchFloorReport, IdentityMaps } from '@/lib/spd-wasm'
import { branchTitle } from './quest-branch'

/**
 * Quest sub-map in the same frame as a floor banner: strip, name, and target.
 */
export function QuestBranchMap({
  branch,
  identities,
  title,
  target,
}: {
  branch: BranchFloorReport
  identities: IdentityMaps
  title: string
  target?: ReactNode
}) {
  const displayedMap = branch.map ?? branch.assumed_map ?? null
  const identity = `Floor ${branch.id.depth}, branch ${branch.id.branch}`
  const mapName = branchTitle(branch.kind)
  const mapLabel = `${mapName} ${identity.toLowerCase()} map`

  return (
    <div data-branch-kind={branch.kind} className="w-full">
      <FloorBanner
        depth={branch.id.depth}
        tileset={displayedMap?.tileset}
        title={title}
        headingLevel={4}
        showDepth={false}
        meta={`Branch of floor ${branch.id.depth}`}
        action={
          displayedMap ? (
            <FloorMapDialog
              map={displayedMap}
              identities={identities}
              depth={branch.id.depth}
              mapLabel={mapLabel}
              dialogTitle={`${mapName} - ${identity}`}
            />
          ) : null
        }
      >
        {target}
      </FloorBanner>
    </div>
  )
}
