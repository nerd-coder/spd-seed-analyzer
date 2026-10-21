/**
 * Per-seed Trinket Catalyst choice, keyed by session id. Persisted so a
 * restored tab replays the same held trinket.
 */

import { persistentStore } from './store-utils'

const TRINKET_CHOICES_KEY = 'spd-analyzer-trinket-choices'

type TrinketChoices = Record<string, string>

const choicesCodec = {
  encode: (v: TrinketChoices) => JSON.stringify(v),
  decode: (v: string): TrinketChoices => {
    try {
      const parsed = JSON.parse(v) as unknown
      if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
        return {}
      }
      return Object.fromEntries(
        Object.entries(parsed).filter(
          (entry): entry is [string, string] => typeof entry[1] === 'string'
        )
      )
    } catch {
      return {}
    }
  },
}

/** Session id → held trinket class name. */
export const $trinketChoices = persistentStore<TrinketChoices>(
  TRINKET_CHOICES_KEY,
  {},
  choicesCodec
)

export function trinketChoiceFor(id: string): string | null {
  return $trinketChoices.get()[id] ?? null
}

export function setTrinketChoice(id: string, trinket: string | null) {
  const { [id]: _previous, ...rest } = $trinketChoices.get()
  $trinketChoices.set(trinket == null ? rest : { ...rest, [id]: trinket })
}
