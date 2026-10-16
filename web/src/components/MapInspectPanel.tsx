import { forwardRef } from 'react'

import { GameText, ItemDetail } from '@/components/item-detail'
import { Badge } from '@/components/ui/badge'
import {
  mobClassName,
  plantClassName,
  trapClassName,
} from '@/generated/game-ids'
import {
  itemClass,
  itemTitle,
  type MapInspectEntry,
  type MapInspection,
  mobTitle,
  objectText,
  plantTitle,
  transitionTitle,
  trapTitle,
} from '@/lib/map-inspect'
import type { MapMobState } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

const MOB_STATE_NOTE: Record<MapMobState, string | null> = {
  sleeping: 'Asleep when the floor is generated.',
  wandering: 'Awake and wandering when the floor is generated.',
  passive: null,
}

function ObjectSection({
  title,
  desc,
  note,
}: {
  title: string
  desc?: string | null
  note?: string | null
}) {
  return (
    <div className="flex flex-col gap-1">
      <p className="font-semibold text-popover-foreground">{title}</p>
      {note ? <p className="text-sm text-muted-foreground">{note}</p> : null}
      {desc ? (
        <p className="whitespace-pre-wrap text-sm text-muted-foreground">
          <GameText text={desc} />
        </p>
      ) : null}
    </div>
  )
}

function EntrySection({ entry }: { entry: MapInspectEntry }) {
  switch (entry.kind) {
    case 'item':
      return (
        <ItemDetail
          title={itemTitle(entry.item)}
          className={itemClass(entry.item)}
          cursed={entry.item.cursed}
          location={entry.place}
        />
      )
    case 'mob':
      return (
        <ObjectSection
          title={mobTitle(entry.mob)}
          note={MOB_STATE_NOTE[entry.mob.state]}
          desc={objectText(mobClassName(entry.mob.mob))?.desc}
        />
      )
    case 'loot':
      return (
        <ObjectSection
          title={entry.label}
          note="Room loot whose exact item is not modeled at this tile."
        />
      )
    case 'trap':
      return (
        <ObjectSection
          title={trapTitle(entry.trap)}
          note={entry.trap.active ? null : 'Inactive.'}
          desc={objectText(trapClassName(entry.trap.trap))?.desc}
        />
      )
    case 'plant':
      return (
        <ObjectSection
          title={plantTitle(entry.plant)}
          desc={objectText(plantClassName(entry.plant.plant))?.desc}
        />
      )
    case 'transition':
      return <ObjectSection title={transitionTitle(entry.transition)} />
  }
}

type Props = {
  inspection: MapInspection
  pinned: boolean
  style?: React.CSSProperties
}

/** Description panel for one map tile: everything generated on it. */
export const MapInspectPanel = forwardRef<HTMLDivElement, Props>(
  function MapInspectPanel({ inspection, pinned, style }, ref) {
    const { entries, uncertain, hidden, secretDoor } = inspection
    return (
      <div
        ref={ref}
        data-slot="map-inspect"
        role="dialog"
        aria-label="Tile details"
        className={cn(
          'absolute z-20 flex max-h-80 w-80 max-w-[calc(100%-1rem)] flex-col gap-2 overflow-y-auto rounded-lg border bg-popover p-3.5 text-sm text-popover-foreground shadow-window inset-shadow-raised',
          !pinned && 'pointer-events-none'
        )}
        style={style}
      >
        {uncertain || hidden ? (
          <div className="flex flex-wrap gap-1">
            {hidden ? (
              <Badge variant="secondary">
                {secretDoor ? 'Secret door' : 'Hidden'}
              </Badge>
            ) : null}
            {uncertain ? <Badge variant="warning">May differ</Badge> : null}
          </div>
        ) : null}
        {secretDoor && entries.length === 0 ? (
          <p className="text-sm text-muted-foreground">
            Looks like a wall until searched. The room behind it is shown
            pulsing on the map.
          </p>
        ) : null}
        {entries.map((entry, index) => (
          <div
            key={`${entry.kind}-${index}`}
            className={cn(index > 0 && 'border-t pt-2')}
          >
            <EntrySection entry={entry} />
          </div>
        ))}
        {uncertain ? (
          <p className="border-t pt-2 text-xs text-muted-foreground">
            Shown as generated for a fresh run with no trinkets. Items found or
            trinkets held before this floor can change what spawns here.
          </p>
        ) : null}
      </div>
    )
  }
)
