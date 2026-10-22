import { ItemBox } from '@/components/ItemBox'
import {
  PICKED_TRINKET_LEVEL,
  type TrinketChoice,
  trinketEffect,
  trinketEffectLabel,
} from '@/components/seed/trinket-offers'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { itemClassName } from '@/generated/game-ids'
import { itemSprite } from '@/lib/item-kind'
import { itemLabel } from '@/lib/item-title'
import type { TrinketSelectionReport } from '@/lib/spd-wasm'

function heldNote(
  held: string | null,
  selection: TrinketSelectionReport
): string {
  const from = selection.first_effective_depth
  if (held == null) {
    return `Floors ${from}+ show every supported trinket outcome as a condition.`
  }
  if (trinketEffect(held, selection) === 'none') {
    return `Floors ${from}+ are generated without a trinket that changes floors.`
  }
  switch (trinketEffect(held, selection)) {
    case 'modeled':
      return `Floors ${from}+ are generated with ${itemLabel(held)} +${PICKED_TRINKET_LEVEL}. Trinket effects are verified through floor 4; deeper floors are spot-checked.`
    case 'unmodeled':
      return `${itemLabel(held)} changes generated floors, but the analyzer does not model that yet. Floors ${from}+ may differ from the game.`
    case 'none':
      return `${itemLabel(held)} only changes play, not generated floors.`
  }
}

/** The Catalyst's offers as a single choice; the pick regenerates later floors. */
export function TrinketChoiceBand({
  offers,
  selection,
  choice,
}: {
  offers: readonly string[]
  selection: TrinketSelectionReport
  choice: TrinketChoice
}) {
  const playOnly = offers.filter(
    (className) => trinketEffect(className, selection) === 'none'
  )
  const value =
    choice.held == null || playOnly.includes(choice.held) ? '' : choice.held
  return (
    <div
      data-trinket-choice
      className="flex flex-col gap-3 rounded-md border bg-muted/30 p-3"
      aria-busy={choice.busy}
    >
      <div className="space-y-1">
        <h4 className="text-sm font-semibold">Trinket Catalyst</h4>
        <p className="text-muted-foreground text-sm">
          Pick the trinket you take to regenerate floors{' '}
          {selection.first_effective_depth}+ with it held. Press it again to
          show every outcome.
        </p>
      </div>

      <ToggleGroup
        type="single"
        variant="outline"
        spacing={2}
        className="flex-wrap items-stretch"
        value={value}
        disabled={choice.busy}
        aria-label="Trinket to take from the Catalyst"
        onValueChange={(next) => {
          // Pressing the held trinket again clears it to no trinket.
          if (next === value) return
          choice.select(next === '' ? null : next)
        }}
      >
        {offers.map((className) => (
          <ToggleGroupItem
            key={className}
            value={className}
            aria-label={itemLabel(className)}
            disabled={playOnly.includes(className)}
            className="h-auto w-32 cursor-pointer flex-col justify-start gap-1 px-2 py-2 whitespace-normal"
          >
            <ItemBox
              scale={3}
              panel={false}
              sprites={itemSprite({ className, category: 'trinket' })}
            />
            <span className="text-foreground text-center leading-tight">
              {itemLabel(className)}
            </span>
            <span className="text-muted-foreground text-center text-xs font-normal leading-tight">
              {trinketEffectLabel(trinketEffect(className, selection))}
            </span>
          </ToggleGroupItem>
        ))}
      </ToggleGroup>

      <p className="text-muted-foreground text-sm" aria-live="polite">
        {choice.busy
          ? 'Regenerating floors…'
          : heldNote(choice.held, selection)}
      </p>

      {selection.transmutation_sequence.length > 0 ? (
        <div data-transmutation-order className="flex flex-col gap-1">
          <p className="text-muted-foreground text-xs">Transmutation order</p>
          <div className="flex flex-wrap gap-px">
            {selection.transmutation_sequence.map((item, index) => (
              <ItemBox
                key={`${index}-${item}`}
                scale={2}
                title={itemLabel(item)}
                sprites={itemSprite({
                  className: itemClassName(item),
                  category: 'trinket',
                })}
              />
            ))}
          </div>
        </div>
      ) : null}
    </div>
  )
}
