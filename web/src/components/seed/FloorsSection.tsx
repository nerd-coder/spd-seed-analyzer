import { FloorDetail } from '@/components/seed/FloorDetail'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { groupFloorsByRegion } from '@/lib/regions'
import type { FloorReport, IdentityMaps } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

export function FloorsSection({
  floors,
  identities,
  mapSpoilers,
}: {
  floors: FloorReport[]
  identities: IdentityMaps
  mapSpoilers: boolean
}) {
  const groups = groupFloorsByRegion(floors)
  if (groups.length === 0) return null

  const defaultRegion = groups[0].region.id

  // Not using Card: its default overflow-hidden kills position:sticky.
  // Region tabs stick under the seed session bar (--seed-tabs-height from App).
  return (
    <section className="bg-card text-card-foreground ring-1 ring-foreground/10">
      <div className="space-y-1 px-4 pt-4">
        <h2 className="font-heading text-sm font-medium">Floors</h2>
        <p className="text-muted-foreground text-xs leading-relaxed">
          Partial levelgen: layout, special/secret rooms, shops, crystal rooms,
          and quest rewards when reported. Boss floors (5 / 10 / 15 / 20 / 25)
          and Last Level (26) are hidden.
          {mapSpoilers
            ? ' Map thumbnails are 128×128 — click to expand.'
            : ' Enable Floor maps to view layout thumbnails.'}
        </p>
      </div>

      <Tabs
        defaultValue={defaultRegion}
        className="block gap-0 overflow-visible"
      >
        <div
          className={cn(
            'sticky z-10 border-b px-4 py-2',
            'bg-card/95 backdrop-blur supports-backdrop-filter:bg-card/90'
          )}
          style={{ top: 'var(--seed-tabs-height, 3rem)' }}
        >
          <TabsList className="h-auto w-full flex-wrap sm:w-auto">
            {groups.map(({ region, floors: regionFloors }) => {
              const lo = regionFloors[0].depth
              const hi = regionFloors[regionFloors.length - 1].depth
              return (
                <TabsTrigger key={region.id} value={region.id}>
                  {region.label}
                  <span className="text-muted-foreground ml-1 hidden text-xs tabular-nums sm:inline">
                    {lo}
                    {hi !== lo ? `–${hi}` : ''}
                  </span>
                </TabsTrigger>
              )
            })}
          </TabsList>
        </div>

        {groups.map(({ region, floors: regionFloors }) => (
          <TabsContent
            key={region.id}
            value={region.id}
            className="space-y-0 px-4 py-4"
          >
            {regionFloors.map((floor) => (
              <FloorDetail
                key={floor.depth}
                floor={floor}
                identities={identities}
                mapSpoilers={mapSpoilers}
              />
            ))}
          </TabsContent>
        ))}
      </Tabs>
    </section>
  )
}
