import { SkullIcon } from '@phosphor-icons/react'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { isBossFloor, REGIONS } from '@/lib/regions'
import type { SampleFloor } from '@/lib/sample-seed'
import { cn } from '@/lib/utils'

function floorSummary(floor: SampleFloor) {
  const parts = [`Floor ${floor.depth}`]
  if (floor.quest) parts.push(`${floor.quest.name} quest`)
  const notable = floor.notable.length
  parts.push(
    notable === 0
      ? 'no notable items'
      : `${notable} notable ${notable === 1 ? 'item' : 'items'}`
  )
  return parts.join(', ')
}

/**
 * The run as one strip: 26 floors in five region bands. Marks on a cell
 * are facts about that floor: a skull for the boss, a square for a quest,
 * one pip per notable item.
 */
export function DescentRail({
  floors,
  selected,
  onSelect,
}: {
  floors: readonly SampleFloor[]
  selected: number
  onSelect: (depth: number) => void
}) {
  return (
    <nav aria-label="Floors" className="-mx-1 overflow-x-auto px-1 pt-3 pb-1">
      <ol className="grid min-w-[880px] grid-cols-[5fr_5fr_5fr_5fr_6fr] gap-3">
        {REGIONS.map((region) => {
          const [first, last] = region.floors
          const band = floors.filter(
            (floor) => floor.depth >= first && floor.depth <= last
          )
          return (
            <li key={region.id} className="flex flex-col gap-2">
              <ol className="flex gap-0.5">
                {band.map((floor) => {
                  const active = floor.depth === selected
                  return (
                    <li key={floor.depth} className="flex-1">
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <button
                            type="button"
                            aria-current={active ? 'true' : undefined}
                            aria-label={floorSummary(floor)}
                            onClick={() => onSelect(floor.depth)}
                            className={cn(
                              'relative flex h-16 w-full flex-col items-center justify-between py-1.5 text-region-foreground inset-shadow-raised transition-transform duration-100 outline-none hover:-translate-y-0.5 focus-visible:z-10 focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:ring-offset-background',
                              region.fill,
                              active &&
                                '-translate-y-1.5 ring-2 ring-foreground ring-offset-2 ring-offset-background hover:-translate-y-1.5'
                            )}
                          >
                            <span className="flex h-3 items-center">
                              {isBossFloor(floor.depth) ? (
                                <SkullIcon weight="fill" className="size-3" />
                              ) : floor.quest ? (
                                <span className="size-1.5 bg-region-foreground" />
                              ) : null}
                            </span>
                            <span className="font-pixel text-px-1 leading-none">
                              {floor.depth}
                            </span>
                            <span className="flex h-1 gap-0.5">
                              {floor.notable.slice(0, 3).map((item) => (
                                <span
                                  key={item.name}
                                  className="h-1 w-1.5 bg-region-foreground/80"
                                />
                              ))}
                            </span>
                          </button>
                        </TooltipTrigger>
                        <TooltipContent side="top">
                          {floorSummary(floor)}
                        </TooltipContent>
                      </Tooltip>
                    </li>
                  )
                })}
              </ol>
              <p className="flex items-baseline justify-between gap-2 text-sm">
                <span className="font-semibold">{region.name}</span>
                <span className="text-muted-foreground tabular-nums">
                  {first}–{last}
                </span>
              </p>
            </li>
          )
        })}
      </ol>
    </nav>
  )
}
