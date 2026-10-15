import { ItemBox } from '@/components/ItemBox'
import { type ItemId, itemClassName, RoomId } from '@/generated/game-ids'
import { itemSprite } from '@/lib/item-kind'
import { itemLabel, variantCandidateClasses } from '@/lib/item-title'
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

/** The seed's first alchemy pot, matched by depth and laboratory source. */
export function isFirstAlchemyPot(
  appearance: GuaranteedAppearance,
  depth: number,
  selection: TrinketSelectionReport
) {
  if (appearance.kind !== 'alchemy_pot') return false
  if (depth !== selection.first_alchemy_pot_depth) return false
  const source = selection.first_alchemy_pot_is_secret
    ? RoomId.SecretLaboratoryRoom
    : RoomId.LaboratoryRoom
  return appearance.source === source
}

export function trinketOfferDetails(
  index: number,
  total: number,
  sequence: readonly ItemId[]
) {
  const choice = `Catalyst choice ${index + 1} of ${total}.`
  if (sequence.length === 0) return choice
  return (
    <>
      <p>{choice}</p>
      <div className="flex flex-col gap-1">
        <p>Transmutation order</p>
        <div className="flex flex-wrap gap-px">
          {sequence.map((item, orderIndex) => (
            <ItemBox
              key={`${orderIndex}-${item}`}
              scale={1}
              panel={false}
              title={itemLabel(item)}
              sprites={itemSprite({
                className: itemClassName(item),
                category: 'trinket',
              })}
            />
          ))}
        </div>
      </div>
    </>
  )
}
