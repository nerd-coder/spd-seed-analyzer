import { createContext } from 'react'
import { ItemBox } from '@/components/ItemBox'
import { itemClassName } from '@/generated/game-ids'
import { itemSprite } from '@/lib/item-kind'
import { itemLabel, variantCandidateClasses } from '@/lib/item-title'
import type {
  FloorReport,
  TrinketGenerationEffect,
  TrinketSelectionReport,
} from '@/lib/spd-wasm'

/**
 * Offers copied onto the catalyst item. An empty list stays empty:
 * `trinket_selection.catalyst_options` is not a fallback.
 */
export function catalystOfferClasses(floors: readonly FloorReport[]): string[] {
  for (const floor of floors) {
    for (const group of floor.items) {
      for (const item of group.variants) {
        const offers = variantCandidateClasses(item)
        if (
          item.item != null &&
          itemClassName(item.item) === 'TrinketCatalyst' &&
          offers.length > 0
        ) {
          return offers
        }
      }
    }
  }
  return []
}

/** Upgrade level the analyzer holds a picked trinket at (engine `PICKED_LEVEL`). */
export const PICKED_TRINKET_LEVEL = 3

/**
 * Trinket class a floor (and its branch maps) was generated with; null when
 * the floor is the no-trinket replay.
 */
export const HeldTrinketContext = createContext<string | null>(null)

/** The player's Catalyst pick for one seed session. */
export type TrinketChoice = {
  /** Held trinket class; null shows every outcome as a condition. */
  held: string | null
  /** A regenerated report is on its way. */
  busy: boolean
  select: (trinket: string | null) => void
}

export function trinketEffect(
  className: string,
  selection: TrinketSelectionReport
): TrinketGenerationEffect {
  const index = selection.catalyst_options.findIndex(
    (item) => itemClassName(item) === className
  )
  return selection.catalyst_option_effects[index] ?? 'unmodeled'
}

export function trinketEffectLabel(effect: TrinketGenerationEffect) {
  switch (effect) {
    case 'modeled':
      return 'Changes later floors'
    case 'unmodeled':
      return 'Changes floors, not modeled'
    case 'none':
      return 'No effect on floors'
  }
}

/** Banner mark for floors generated with the held trinket; jumps to the pick. */
export function HeldTrinketChip({ trinket }: { trinket: string }) {
  const label = `Generated with ${itemLabel(trinket)} +${PICKED_TRINKET_LEVEL}. Change trinket`
  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      className="inline-flex cursor-pointer items-center gap-1 rounded-sm px-1 outline-none hover:bg-white/10 focus-visible:ring-2 focus-visible:ring-ring"
      onClick={() => {
        const reduce = window.matchMedia(
          '(prefers-reduced-motion: reduce)'
        ).matches
        document.querySelector('[data-trinket-choice]')?.scrollIntoView({
          block: 'center',
          behavior: reduce ? 'auto' : 'smooth',
        })
      }}
    >
      <ItemBox
        scale={1}
        panel={false}
        sprites={itemSprite({ className: trinket, category: 'trinket' })}
      />
      <span className="text-sm font-semibold text-white/85">
        +{PICKED_TRINKET_LEVEL}
      </span>
    </button>
  )
}
