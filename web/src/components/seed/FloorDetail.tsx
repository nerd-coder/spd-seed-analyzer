import { DepthIcon } from '@/components/DepthIcon'
import { FloorMapPreview } from '@/components/FloorMapPreview'
import { ItemIcon } from '@/components/ItemIcon'
import { ItemName } from '@/components/ItemName'
import { QuestCard } from '@/components/seed/QuestCard'
import { Badge } from '@/components/ui/badge'
import { itemAppearance } from '@/lib/identity'
import { formatItemSource, isHighlightSource } from '@/lib/labels'
import type { FloorReport, IdentityMaps } from '@/lib/spd-wasm'

export function FloorDetail({
  floor,
  identities,
  mapSpoilers,
}: {
  floor: FloorReport
  identities: IdentityMaps
  mapSpoilers: boolean
}) {
  const hasQuest = (floor.quests?.length ?? 0) > 0
  const showMap = mapSpoilers && !!floor.map

  const details = (
    <div className="min-w-0 flex-1 space-y-3">
      {floor.quests && floor.quests.length > 0 && (
        <div className="space-y-2">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            Quests
          </p>
          <div className="space-y-2">
            {floor.quests.map((q, i) => (
              <QuestCard key={`${floor.depth}-quest-${i}`} quest={q} />
            ))}
          </div>
        </div>
      )}

      {floor.rooms && floor.rooms.length > 0 && (
        <div className="space-y-1">
          <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
            Rooms
            <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
              ({floor.rooms.length})
            </span>
          </p>
          <p className="text-sm leading-relaxed">
            {floor.rooms.map((r) => r.replace(/Room$/, '')).join(' · ')}
          </p>
        </div>
      )}

      <div className="space-y-1">
        <p className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
          Items
          <span className="ml-1.5 font-mono font-normal tabular-nums normal-case">
            ({floor.items.length})
          </span>
        </p>
        {floor.items.length === 0 ? (
          <p className="text-muted-foreground text-sm">No items listed.</p>
        ) : (
          <ul className="space-y-1.5 text-sm">
            {floor.items.map((item, i) => {
              const sourceLabel = formatItemSource(item.source)
              const highlight = isHighlightSource(item.source)
              return (
                <li
                  key={`${floor.depth}-${i}`}
                  className="flex items-start gap-2"
                >
                  <ItemIcon
                    classNameItem={item.class_name}
                    category={item.category}
                    appearance={itemAppearance(item, identities)}
                    size={16}
                    title={item.name}
                    className="mt-0.5"
                  />
                  <span className="flex min-w-0 flex-wrap items-baseline gap-x-1.5 gap-y-0.5">
                    <ItemName name={item.name} />
                    {item.cursed && (
                      <Badge
                        variant="destructive"
                        className="h-5 px-1.5 py-0 text-[10px] font-normal"
                      >
                        cursed
                      </Badge>
                    )}
                    {sourceLabel && (
                      <Badge
                        variant={highlight ? 'secondary' : 'outline'}
                        className="h-5 px-1.5 py-0 text-[10px] font-normal"
                        title={item.source ?? undefined}
                      >
                        {sourceLabel}
                      </Badge>
                    )}
                  </span>
                </li>
              )
            })}
          </ul>
        )}
      </div>
    </div>
  )

  return (
    <section className="space-y-3 border-b py-6 first:pt-0 last:border-b-0 last:pb-0">
      <div className="flex flex-wrap items-center gap-2">
        <DepthIcon feeling={floor.feeling} size={20} />
        <span className="font-mono text-sm font-medium tabular-nums">
          Floor {floor.depth}
        </span>
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
        {showMap && floor.map && (
          <FloorMapPreview map={floor.map} depth={floor.depth} />
        )}
      </div>
    </section>
  )
}
