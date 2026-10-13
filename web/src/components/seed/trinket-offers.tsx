import { finderItemLabel } from '@/components/finder/finder-items'
import { ItemBox } from '@/components/ItemBox'
import { itemSprite } from '@/lib/item-kind'
import type {
  FloorReport,
  GuaranteedAppearance,
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
        if (
          item.class_name === 'TrinketCatalyst' &&
          item.candidate_classes &&
          item.candidate_classes.length > 0
        ) {
          return [...item.candidate_classes]
        }
      }
    }
  }
  return []
}

/** The seed's first alchemy pot, matched by depth and laboratory source. */
export function isFirstAlchemyPot(
  appearance: GuaranteedAppearance,
  depth: number,
  selection: TrinketSelectionReport
) {
  if (appearance.kind !== 'alchemy_pot') return false
  if (depth !== selection.first_alchemy_pot_depth) return false
  const source = selection.first_alchemy_pot_is_secret
    ? 'SecretLaboratoryRoom'
    : 'LaboratoryRoom'
  return appearance.source === source
}

export function trinketOfferDetails(
  index: number,
  total: number,
  sequence: readonly string[]
) {
  const choice = `Catalyst choice ${index + 1} of ${total}.`
  if (sequence.length === 0) return choice
  return (
    <>
      <p>{choice}</p>
      <div className="flex flex-col gap-1">
        <p>Transmutation order</p>
        <div className="flex flex-wrap gap-px">
          {sequence.map((className, orderIndex) => (
            <ItemBox
              key={`${orderIndex}-${className}`}
              scale={1}
              panel={false}
              title={finderItemLabel(className)}
              sprites={itemSprite({ className, category: 'trinket' })}
            />
          ))}
        </div>
      </div>
    </>
  )
}
