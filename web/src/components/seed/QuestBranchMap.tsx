import { FloorMapPreview } from '@/components/FloorMapPreview'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import type { BranchFloorReport, IdentityMaps } from '@/lib/spd-wasm'
import { branchAccessText, branchTitle } from './quest-branch'

export function QuestBranchMap({
  branch,
  identities,
}: {
  branch: BranchFloorReport
  identities: IdentityMaps
}) {
  const showAssumedMap = !branch.map && !!branch.assumed_map
  const displayedMap = branch.map ?? branch.assumed_map ?? null
  const title = branchTitle(branch.kind)
  const identity = `Floor ${branch.id.depth}, branch ${branch.id.branch}`
  const mapLabel = `${title} ${identity.toLowerCase()} map`

  return (
    <div
      data-branch-kind={branch.kind}
      className="flex flex-wrap items-start gap-3"
    >
      <div className="min-w-[min(100%,12rem)] flex-1 space-y-2 text-xs">
        <div className="flex flex-wrap items-center gap-1.5">
          <Badge variant="secondary">Objective: {branch.objective}</Badge>
          <Badge variant="outline" className="font-mono text-xs">
            {identity}
          </Badge>
        </div>
        <p>
          <span className="text-muted-foreground font-medium">Access:</span>{' '}
          <span className="capitalize">{branchAccessText(branch)}</span>
        </p>
        {showAssumedMap ? (
          <Alert variant="warning" className="px-2 py-1.5">
            <AlertTitle className="text-[10px] leading-tight">
              Assumed branch layout
            </AlertTitle>
            <AlertDescription className="text-[9px] leading-tight text-pretty">
              Baseline continuation through unresolved player or meta state.
            </AlertDescription>
          </Alert>
        ) : null}
      </div>
      {displayedMap ? (
        <FloorMapPreview
          map={displayedMap}
          identities={identities}
          depth={branch.id.depth}
          mapLabel={mapLabel}
          dialogTitle={`${title} - ${identity}`}
        />
      ) : null}
    </div>
  )
}
