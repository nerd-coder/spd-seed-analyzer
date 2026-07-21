import { useCallback, useRef } from 'react'

import { FloorDetail } from '@/components/seed/FloorDetail'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { groupFloorsByRegion } from '@/lib/regions'
import type { FloorReport, IdentityMaps } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

/** Small gap between sticky region tabs and the first floor after scroll. */
const SCROLL_GAP_PX = 8

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
  const stickyBarRef = useRef<HTMLDivElement>(null)
  const regionContentRefs = useRef(new Map<string, HTMLElement>())

  const scrollRegionIntoView = useCallback((regionId: string) => {
    // Double rAF: wait for Radix to show the new TabsContent and for layout.
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        const content = regionContentRefs.current.get(regionId)
        const sticky = stickyBarRef.current
        if (!content || !sticky) return

        const stickyBottom = sticky.getBoundingClientRect().bottom
        const contentTop = content.getBoundingClientRect().top
        const delta = contentTop - stickyBottom - SCROLL_GAP_PX
        if (Math.abs(delta) < 1) return

        window.scrollBy({ top: delta, behavior: 'smooth' })
      })
    })
  }, [])

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
        onValueChange={scrollRegionIntoView}
      >
        <div
          ref={stickyBarRef}
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
            ref={(el) => {
              if (el) regionContentRefs.current.set(region.id, el)
              else regionContentRefs.current.delete(region.id)
            }}
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
