/**
 * Per-seed enabled challenges, keyed by session id. Persisted so a restored
 * tab replays the same run settings.
 */

import { GENERATION_CHALLENGES } from '@/lib/challenges'
import type { Challenge } from '@/lib/spd-wasm'
import { persistentStore } from './store-utils'

const CHALLENGE_CHOICES_KEY = 'spd-analyzer-challenge-choices'

type ChallengeChoices = Record<string, Challenge[]>

const known = new Set<string>(GENERATION_CHALLENGES.map((entry) => entry.id))

/** Known challenges, deduplicated, in the order the picker shows them. */
export function normalizeChallenges(values: readonly string[]): Challenge[] {
  return GENERATION_CHALLENGES.map((entry) => entry.id).filter((id) =>
    values.includes(id)
  )
}

const choicesCodec = {
  encode: (v: ChallengeChoices) => JSON.stringify(v),
  decode: (v: string): ChallengeChoices => {
    try {
      const parsed = JSON.parse(v) as unknown
      if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
        return {}
      }
      const choices: ChallengeChoices = {}
      for (const [id, value] of Object.entries(parsed)) {
        if (!Array.isArray(value)) continue
        const names = value.filter(
          (name): name is string => typeof name === 'string' && known.has(name)
        )
        const challenges = normalizeChallenges(names)
        if (challenges.length > 0) choices[id] = challenges
      }
      return choices
    } catch {
      return {}
    }
  },
}

/** Session id → enabled generation-affecting challenges. */
export const $challengeChoices = persistentStore<ChallengeChoices>(
  CHALLENGE_CHOICES_KEY,
  {},
  choicesCodec
)

export function challengeChoiceFor(id: string): Challenge[] {
  return $challengeChoices.get()[id] ?? []
}

export function setChallengeChoice(id: string, challenges: readonly string[]) {
  const { [id]: _previous, ...rest } = $challengeChoices.get()
  const next = normalizeChallenges(challenges)
  $challengeChoices.set(next.length === 0 ? rest : { ...rest, [id]: next })
}
