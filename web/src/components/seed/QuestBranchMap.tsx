import type { ReactNode } from 'react'

import { FloorMapPreview } from '@/components/FloorMapPreview'
import type { BranchFloorReport, IdentityMaps } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'
import { branchTitle } from './quest-branch'

/**
 * Quest sub-map in the same frame as a floor banner: map, name, and target.
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
    <div
      data-branch-kind={branch.kind}
      className="relative isolate w-full overflow-hidden bg-black"
    >
      {displayedMap ? (
        <FloorMapPreview
          variant="banner"
          map={displayedMap}
          identities={identities}
          depth={branch.id.depth}
          mapLabel={mapLabel}
          dialogTitle={`${mapName} - ${identity}`}
        />
      ) : null}
      <div className="pointer-events-none absolute inset-0 z-10 bg-white/70 dark:bg-black/50" />
      <div
        className={cn(
          'relative z-20 flex w-full flex-wrap items-center gap-2 px-3 py-2',
          displayedMap && 'pr-12'
        )}
      >
        <h4 className="spd-floor-label font-mono text-sm font-bold text-white">
          {title}
        </h4>
        {target}
      </div>
    </div>
  )
}
