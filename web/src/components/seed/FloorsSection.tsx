import { FloorDetail } from '@/components/seed/FloorDetail'
import { catalystOfferClasses } from '@/components/seed/trinket-offers'
import type {
  FloorReport,
  IdentityMaps,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'

export function FloorsSection({
  floors,
  identities,
  trinketSelection,
}: {
  floors: FloorReport[]
  identities: IdentityMaps
  trinketSelection: TrinketSelectionReport
}) {
  if (floors.length === 0) return null

  const offerClasses = catalystOfferClasses(floors)
  const ordered = [...floors].sort((a, b) => a.depth - b.depth)

  // Ancestors must not set overflow:hidden, or the floor banners cannot stick.
  return (
    <section className="bg-card text-card-foreground rounded-lg border">
      <div className="space-y-1 px-4 py-4">
        <h2 className="font-pixel text-px-1">Floors</h2>
        <p className="text-muted-foreground text-sm">
          Items you may find on each floor.
        </p>
      </div>

      <div>
        {ordered.map((floor) => (
          <FloorDetail
            key={floor.depth}
            floor={floor}
            identities={identities}
            trinketSelection={trinketSelection}
            offerClasses={offerClasses}
          />
        ))}
      </div>
    </section>
  )
}
