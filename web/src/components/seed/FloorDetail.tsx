import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapPreview } from '@/components/FloorMapPreview'
import { FloorAppearanceSection } from '@/components/seed/FloorAppearanceSection'
import {
  FloorItemSections,
  partitionFloorItems,
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
import type { FloorReport, IdentityMaps } from '@/lib/spd-wasm'

export function FloorDetail({
  floor,
  identities,
}: {
  floor: FloorReport
  identities: IdentityMaps
}) {
  const hasQuest = (floor.quests?.length ?? 0) > 0
  const questRewards = partitionFloorItems(floor.items).quest
  const showMap = !!floor.map
  const showAssumedMap = !floor.map && !!floor.assumed_map
  const displayedMap = floor.map ?? floor.assumed_map ?? null

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
                rewards={questRewards}
                identities={identities}
                depth={floor.depth}
              />
            ))}
          </div>
        </div>
      )}

      <FloorAppearanceSection appearances={floor.guaranteed_appearances} />
      <FloorItemSections floor={floor} identities={identities} />
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
    </section>
  )
}
