import { InfoIcon } from '@phosphor-icons/react'
import { ItemBox } from '@/components/ItemBox'
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
import { Progress } from '@/components/ui/progress'
import { itemClassName } from '@/generated/game-ids'
import { ItemCategory, itemSprite } from '@/lib/item-kind'
import { itemLabel } from '@/lib/item-title'
import type { WandmakerRewardSlot } from '@/lib/spd-wasm'

/** `+1`, `−2`, or `seed draw` for the no-history draw. */
function driftText(drifts: number[]): string {
  const parts = drifts.map((drift) =>
    drift === 0 ? 'seed draw' : `${drift > 0 ? '+' : '−'}${Math.abs(drift)}`
  )
  return parts.join(' / ')
}

function SlotRow({ slot }: { slot: WandmakerRewardSlot }) {
  const candidates = slot.candidates ?? []
  const label = slot.slot === 0 ? 'Wand 1' : 'Wand 2'

  return (
    <li className="flex flex-col gap-1 border-t pt-2 first:border-t-0 first:pt-0">
      <div className="flex flex-wrap items-baseline gap-x-2 gap-y-0.5">
        <span className="text-sm font-semibold">{label}</span>
        <span className="text-sm font-semibold tabular-nums">
          +{slot.level}
        </span>
      </div>
      <ul className="flex flex-col gap-1">
        {candidates.map((candidate) => {
          const share =
            slot.scenario_count === 0
              ? 0
              : (100 * candidate.weight) / slot.scenario_count
          const isSeedDraw = candidate.drifts.includes(0)
          return (
            <li
              key={candidate.item}
              className="grid grid-cols-[auto_minmax(0,1fr)_auto] items-center gap-x-2"
            >
              <ItemBox
                sprites={itemSprite({
                  className: itemClassName(candidate.item),
                  category: ItemCategory.Wand,
                })}
                title={itemLabel(candidate.item)}
              />
              <span className="flex min-w-0 flex-col gap-0.5">
                <span className="flex min-w-0 flex-wrap items-baseline gap-1.5">
                  <span
                    className={
                      isSeedDraw
                        ? 'truncate text-sm font-medium'
                        : 'truncate text-sm'
                    }
                  >
                    {itemLabel(candidate.item)}
                  </span>
                  {isSeedDraw ? (
                    <Badge variant="outline">seed draw</Badge>
                  ) : null}
                </span>
                <Progress value={share} className="h-1" />
              </span>
              <span className="flex items-baseline gap-1.5 text-xs tabular-nums">
                <span>{share.toFixed(0)}%</span>
                <span className="text-muted-foreground">
                  {driftText(candidate.drifts)}
                </span>
              </span>
            </li>
          )
        })}
      </ul>
    </li>
  )
}

/**
 * Deck-index window for the Old Wandmaker's two ordered wand options.
 *
 * Shares are how many offsets in that window land on a class, not how often
 * players see the wand. The seed-draw row is the no-history pair.
 */
export function WandmakerRewardDistribution({
  slots,
}: {
  slots: WandmakerRewardSlot[]
}) {
  if (slots.length === 0) return null
  const window = Math.max(
    ...slots.flatMap((slot) =>
      (slot.candidates ?? []).flatMap((candidate) =>
        candidate.drifts.map((drift) => Math.abs(drift))
      )
    ),
    0
  )
  const wand2Pinned =
    slots.find((slot) => slot.slot === 1)?.level_pinned ?? true

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="ghost" size="icon-xs" aria-label="Wand reward outlook">
          <InfoIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-96">
        <PopoverHeader>
          <PopoverTitle>Wand reward outlook</PopoverTitle>
          <PopoverDescription>
            The two options are ordered. The player keeps one.
          </PopoverDescription>
          <PopoverDescription>
            Shares are a ±{window} index window around the seed draw, a
            stability measure, not how often players see the wand. The seed-draw
            row is the no-history pair.
          </PopoverDescription>
          <PopoverDescription>
            Wand 1's +N is fixed by the seed.{' '}
            {wand2Pinned
              ? "Wand 2's +N is fixed across this window."
              : "Wand 2's +N can move when a shifted index rejects a different number of duplicate wands."}
          </PopoverDescription>
        </PopoverHeader>
        <ul className="flex max-h-80 flex-col gap-2 overflow-y-auto">
          {slots.map((slot) => (
            <SlotRow key={slot.slot} slot={slot} />
          ))}
        </ul>
      </PopoverContent>
    </Popover>
  )
}
