import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapPreview } from '@/components/FloorMapPreview'
import { FloorAppearanceSection } from '@/components/seed/FloorAppearanceSection'
import { FloorEncounterSection } from '@/components/seed/FloorEncounterSection'
import {
  FloorItemSections,
  visibleItemGroups,
} from '@/components/seed/FloorItemSections'
import { QuestCard } from '@/components/seed/QuestCard'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import type {
  BranchFloorReport,
  FloorReport,
  IdentityMaps,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'

function branchAccessText(branch: BranchFloorReport) {
  const conditions: string[] = []
  if (branch.access.requires_acceptance) {
    conditions.push(
      `accept ${branch.access.quest_id.replaceAll('_', ' ')} quest`
    )
  }
  if (branch.access.required_item) {
    conditions.push(`carry ${branch.access.required_item.replaceAll('_', ' ')}`)
  }
  return conditions.length > 0
    ? conditions.join(' and ')
    : 'No additional access condition'
}

function BlacksmithMineBranch({
  branch,
  identities,
}: {
  branch: BranchFloorReport
  identities: IdentityMaps
}) {
  const showAssumedMap = !branch.map && !!branch.assumed_map
  const displayedMap = branch.map ?? branch.assumed_map ?? null
  const identity = `Floor ${branch.id.depth}, branch ${branch.id.branch}`
  const mapLabel = `Blacksmith Mine ${identity.toLowerCase()} map`

  return (
    <section className="space-y-3 border-l-2 border-amber-700/40 bg-muted/25 px-3 py-3">
      <div className="flex flex-wrap items-center gap-2">
        <h4 className="font-heading text-sm font-medium">Blacksmith Mine</h4>
        <Badge variant="secondary">Objective: {branch.objective}</Badge>
        <Badge variant="outline" className="font-mono text-xs">
          {identity}
        </Badge>
      </div>
      <div className="flex items-start gap-3">
        <div className="min-w-0 flex-1 space-y-2 text-xs">
          <p>
            <span className="text-muted-foreground font-medium">Access:</span>{' '}
            <span className="capitalize">{branchAccessText(branch)}</span>
          </p>
          {branch.rooms.length > 0 && (
            <p className="text-muted-foreground leading-relaxed">
              <span className="font-medium text-foreground">Rooms:</span>{' '}
              {branch.rooms
                .map((room) => room.replace(/Room$/, ''))
                .join(' · ')}
            </p>
          )}
          {showAssumedMap && (
            <Alert variant="warning" className="px-2 py-1.5">
              <AlertTitle className="text-[10px] leading-tight">
                Assumed branch layout
              </AlertTitle>
              <AlertDescription className="text-[9px] leading-tight text-pretty">
                Baseline continuation through unresolved player or meta state.
              </AlertDescription>
            </Alert>
          )}
        </div>
        {displayedMap && (
          <FloorMapPreview
            map={displayedMap}
            identities={identities}
            depth={branch.id.depth}
            mapLabel={mapLabel}
            dialogTitle={`Blacksmith Mine - ${identity}`}
          />
        )}
      </div>
    </section>
  )
}

export function FloorDetail({
  floor,
  identities,
  trinketSelection,
}: {
  floor: FloorReport
  identities: IdentityMaps
  trinketSelection: TrinketSelectionReport
}) {
  const hasQuest = (floor.quests?.length ?? 0) > 0
  const showMap = !!floor.map
  const showAssumedMap = !floor.map && !!floor.assumed_map
  const displayedMap = floor.map ?? floor.assumed_map ?? null
  const visibleItems = visibleItemGroups(floor.items)

  const details = (
    <div className="min-w-0 flex-1 space-y-3">
      {floor.quests && floor.quests.length > 0 && (
        <div className="space-y-2">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            Quests
          </p>
          <div className="space-y-2">
            {floor.quests.map((q, i) => (
              <QuestCard
                key={`${floor.depth}-quest-${i}`}
                quest={q}
                rewards={visibleItems.filter(
                  (item) => item.source === q.contract.rewards.item_source
                )}
                identities={identities}
                depth={floor.depth}
              />
            ))}
          </div>
        </div>
      )}

      <FloorAppearanceSection appearances={floor.guaranteed_appearances} />
      <FloorEncounterSection
        encounters={floor.initial_encounters}
        identities={identities}
      />
      <FloorItemSections
        floor={floor}
        identities={identities}
        trinketSelection={trinketSelection}
      />
    </div>
  )

  return (
    <section className="space-y-3 border-b py-6 first:pt-0 last:border-b-0 last:pb-0">
      <div className="flex flex-wrap items-center gap-2">
        <DepthIcon feeling={floor.feeling} size={20} />
        <h3 className="font-mono text-sm font-medium tabular-nums">
          Floor {floor.depth}
        </h3>
        {floor.rooms && floor.rooms.length > 0 && (
          <Popover>
            <PopoverTrigger asChild>
              <Button variant="outline" size="xs">
                Rooms ({floor.rooms.length})
              </Button>
            </PopoverTrigger>
            <PopoverContent align="start" className="w-72">
              <PopoverHeader>
                <PopoverTitle>Rooms on floor {floor.depth}</PopoverTitle>
              </PopoverHeader>
              <p className="text-sm leading-relaxed">
                {floor.rooms
                  .map((room) => room.replace(/Room$/, ''))
                  .join(' · ')}
              </p>
            </PopoverContent>
          </Popover>
        )}
        {floor.feeling && floor.feeling !== 'none' && (
          <Badge variant="secondary" className="capitalize">
            {floor.feeling}
          </Badge>
        )}
        {floor.builder && (
          <Badge variant="outline" className="font-mono text-xs">
            {floor.builder}
          </Badge>
        )}
        {hasQuest && (
          <Badge variant="default" className="text-xs">
            Quest
          </Badge>
        )}
        {showMap && floor.map && (
          <Badge variant="outline" className="font-mono text-xs">
            {floor.map.width}×{floor.map.height}
          </Badge>
        )}
      </div>

      <div className="flex items-start gap-3">
        {details}
        {displayedMap && (
          <div className="w-32 shrink-0 space-y-1.5">
            <FloorMapPreview
              map={displayedMap}
              identities={identities}
              depth={floor.depth}
            />
            {showAssumedMap && (
              <Alert variant="warning" className="px-1.5 py-1">
                <AlertTitle className="text-[10px] leading-tight">
                  Assumed continuation
                </AlertTitle>
                <AlertDescription className="text-[9px] leading-tight text-pretty">
                  Baseline continuation through unresolved player or meta state.
                  Your floor can differ.
                </AlertDescription>
              </Alert>
            )}
          </div>
        )}
      </div>

      {floor.branches && floor.branches.length > 0 && (
        <div className="space-y-2 pt-1">
          {floor.branches.map((branch) => (
            <BlacksmithMineBranch
              key={`${branch.id.depth}-${branch.id.branch}`}
              branch={branch}
              identities={identities}
            />
          ))}
        </div>
      )}
    </section>
  )
}
