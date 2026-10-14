import { SkullIcon } from '@phosphor-icons/react'
import { useEffect, useRef } from 'react'
import { floorItemGroups } from '@/components/seed/floor-items'
import { questTitle } from '@/components/seed/quest-branch'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { usePublishedHeight } from '@/hooks/usePublishedHeight'
import { bossOf, REGIONS, regionOf } from '@/lib/regions'
import type { FloorReport } from '@/lib/spd-wasm'
import { cn } from '@/lib/utils'

type RailFloor = {
  depth: number
  boss?: string
  quests: string[]
  notable: number
}

function railFloor(floor: FloorReport): RailFloor {
  return {
    depth: floor.depth,
    boss: bossOf(floor.depth),
    quests: (floor.quests ?? []).map((quest) => questTitle(quest.type)),
    notable: floorItemGroups(floor).notable.length,
  }
}

function summary(floor: RailFloor) {
  const parts = [`Floor ${floor.depth}, ${regionOf(floor.depth).name}`]
  if (floor.boss) parts.push(`boss: ${floor.boss}`)
  for (const quest of floor.quests) parts.push(`${quest} quest`)
  parts.push(
    floor.notable === 0
      ? 'no notable items'
      : `${floor.notable} notable ${floor.notable === 1 ? 'item' : 'items'}`
  )
  return parts.join(', ')
}

/**
 * Sticky strip of every floor in the report, grouped by region. A skull marks
 * a boss, a square a quest, and each pip a notable item. Clicking a floor
 * scrolls to it; the floor being read is outlined.
 */
export function DescentRail({
  floors,
  current,
  onSelect,
  ref,
}: {
  floors: readonly FloorReport[]
  current: number | null
  onSelect: (depth: number) => void
  ref: { current: HTMLElement | null }
}) {
  const scrollerRef = useRef<HTMLOListElement>(null)
  usePublishedHeight(ref, '--descent-rail-height', true)
  const rail = floors.map(railFloor)

  // Keep the current floor visible when the strip scrolls sideways. Moves
  // only the strip: scrollIntoView would also cancel a smooth page scroll.
  useEffect(() => {
    const scroller = scrollerRef.current
    if (current == null || !scroller) return
    const cell = scroller.querySelector<HTMLElement>(
      `[data-rail-depth="${current}"]`
    )
    if (!cell) return
    const box = scroller.getBoundingClientRect()
    const target = cell.getBoundingClientRect()
    if (target.left < box.left) scroller.scrollLeft -= box.left - target.left
    else if (target.right > box.right)
      scroller.scrollLeft += target.right - box.right
  }, [current])

  return (
    <nav
      ref={ref}
      aria-label="Floors"
      className="sticky z-15 -mx-4 border-b bg-background/95 px-4 py-2 backdrop-blur-sm md:-mx-6 md:px-6"
      style={{ top: 'var(--seed-tabs-height, 3rem)' }}
    >
      <ol ref={scrollerRef} className="flex gap-1.5 overflow-x-auto">
        {REGIONS.map((region) => {
          const band = rail.filter(
            (floor) =>
              floor.depth >= region.floors[0] && floor.depth <= region.floors[1]
          )
          if (band.length === 0) return null
          return (
            <li
              key={region.id}
              className="flex basis-0"
              style={{ flexGrow: band.length }}
            >
              <ol className="flex flex-1 gap-0.5">
                {band.map((floor) => {
                  const label = summary(floor)
                  return (
                    <li key={floor.depth} className="flex min-w-6 flex-1">
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <button
                            type="button"
                            data-rail-depth={floor.depth}
                            aria-current={
                              floor.depth === current ? 'location' : undefined
                            }
                            aria-label={label}
                            onClick={() => onSelect(floor.depth)}
                            className={cn(
                              'flex h-11 w-full flex-col items-center justify-center gap-0.5 text-region-foreground inset-shadow-raised outline-none hover:brightness-110 focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-inset aria-[current=location]:ring-2 aria-[current=location]:ring-foreground aria-[current=location]:ring-inset',
                              region.fill
                            )}
                          >
                            <span className="flex h-2.5 items-center">
                              {floor.boss ? (
                                <SkullIcon weight="fill" className="size-2.5" />
                              ) : floor.quests.length > 0 ? (
                                <span className="bg-region-foreground size-1.5" />
                              ) : null}
                            </span>
                            <span className="font-pixel text-px-1 leading-[20px]">
                              {floor.depth}
                            </span>
                            <span className="flex h-1 gap-0.5">
                              {Array.from(
                                { length: Math.min(floor.notable, 3) },
                                (_, i) => (
                                  <span
                                    key={i}
                                    className="bg-region-foreground/80 h-1 w-1.5"
                                  />
                                )
                              )}
                            </span>
                          </button>
                        </TooltipTrigger>
                        <TooltipContent side="bottom">{label}</TooltipContent>
                      </Tooltip>
                    </li>
                  )
                })}
              </ol>
            </li>
          )
        })}
      </ol>
    </nav>
  )
}
