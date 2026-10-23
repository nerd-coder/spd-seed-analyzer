import { GameSprite } from '@/components/GameSprite'
import { FloorDetail } from '@/components/seed/FloorDetail'
import {
  catalystOfferClasses,
  type TrinketChoice,
} from '@/components/seed/trinket-offers'
import type {
  FloorReport,
  IdentityMaps,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'
import { STAIRS_ICONS, UI_ICONS } from '@/lib/ui-icons'

const ICON_SHEET = {
  url: UI_ICONS.url,
  width: UI_ICONS.sheetW,
  height: UI_ICONS.sheetH,
}

/** Sits above the floor strip; labels the floor list in `FloorsSection`. */
export function FloorsHeading() {
  return (
    <div className="flex items-center gap-2">
      <GameSprite
        sheet={ICON_SHEET}
        frames={[STAIRS_ICONS.none]}
        scale={2}
        label="Stairs"
      />
      <h2 id="floors-title" className="font-pixel text-px-1">
        Floors
      </h2>
      <span className="text-muted-foreground text-sm">
        Items you may find on each floor
      </span>
    </div>
  )
}

export function FloorsSection({
  floors,
  identities,
  trinketSelection,
  trinketChoice,
}: {
  floors: FloorReport[]
  identities: IdentityMaps
  trinketSelection: TrinketSelectionReport
  trinketChoice: TrinketChoice
}) {
  if (floors.length === 0) return null

  const offerClasses = catalystOfferClasses(floors)
  const ordered = [...floors].sort((a, b) => a.depth - b.depth)

  // Ancestors must not set overflow:hidden, or the floor banners cannot stick.
  return (
    <section
      aria-labelledby="floors-title"
      className="bg-card text-card-foreground rounded-lg border"
    >
      <div>
        {ordered.map((floor) => (
          <FloorDetail
            key={floor.depth}
            floor={floor}
            identities={identities}
            trinketSelection={trinketSelection}
            offerClasses={offerClasses}
            trinketChoice={trinketChoice}
          />
        ))}
      </div>
    </section>
  )
}
