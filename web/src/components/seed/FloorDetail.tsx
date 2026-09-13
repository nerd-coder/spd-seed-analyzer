import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapPreview } from '@/components/FloorMapPreview'
import { FloorAppearanceSection } from '@/components/seed/FloorAppearanceSection'
import { FloorEncounterSection } from '@/components/seed/FloorEncounterSection'
import {
  FloorItemSections,
  visibleItemGroups,
} from '@/components/seed/FloorItemSections'
import { SpawnConditionDetails } from '@/components/seed/ItemConditionDetails'
import { QuestBranchMap } from '@/components/seed/QuestBranchMap'
import { QuestCard } from '@/components/seed/QuestCard'
import {
  branchTitle,
  matchingQuestBranch,
  unclaimedBranches,
} from '@/components/seed/quest-branch'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from '@/components/ui/popover'
import type {
  BranchFloorReport,
  FloorReport,
  IdentityMaps,
  PossibleRoom,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'

function roomLabel(className: string) {
  return className.replace(/Room$/, '').replace(/([a-z])([A-Z])/g, '$1 $2')
}

function PossibleRoomsPopover({ rooms }: { rooms: PossibleRoom[] }) {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline" size="xs">
          Possible rooms ({rooms.length})
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80">
        <PopoverHeader>
          <PopoverTitle>Possible rooms</PopoverTitle>
          <PopoverDescription>
            Alternate room counts under modeled player and trinket profiles.
          </PopoverDescription>
        </PopoverHeader>
        <ul className="flex flex-col gap-2 text-sm">
          {rooms.map((room, index) => (
            <li
              key={`${room.class}-${room.quantity}-${index}`}
              className="flex min-w-0 items-center justify-between gap-2"
            >
              <span>
                {roomLabel(room.class)}
                {room.quantity > 1 ? ` ×${room.quantity}` : ''}
              </span>
              <SpawnConditionDetails conditions={room.spawn_conditions} />
            </li>
          ))}
        </ul>
      </PopoverContent>
    </Popover>
  )
}

function branchBorderClass(kind: BranchFloorReport['kind']) {
  switch (kind) {
    case 'blacksmith_mine':
      return 'border-amber-700/40'
    case 'imp_vault':
      return 'border-rose-700/40'
  }
}

function NestedBranchFloor({
  branch,
  identities,
}: {
  branch: BranchFloorReport
  identities: IdentityMaps
}) {
  return (
    <section
      className={`space-y-3 border-l-2 bg-muted/25 px-3 py-3 ${branchBorderClass(branch.kind)}`}
    >
      <h4 className="font-heading text-sm font-medium">
        {branchTitle(branch.kind)}
      </h4>
      <QuestBranchMap branch={branch} identities={identities} />
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
  const leftoverBranches = unclaimedBranches(floor.branches, floor.quests)

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
                branch={matchingQuestBranch(q, floor.branches)}
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
        {floor.possible_rooms && floor.possible_rooms.length > 0 && (
          <PossibleRoomsPopover rooms={floor.possible_rooms} />
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

      {leftoverBranches.length > 0 && (
        <div className="space-y-2 pt-1">
          {leftoverBranches.map((branch) => (
            <NestedBranchFloor
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
