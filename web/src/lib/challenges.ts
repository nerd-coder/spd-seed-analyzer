import type { Challenge } from '@/lib/spd-wasm'

export type ChallengeArt =
  | { kind: 'item'; className: string; category: string }
  | { kind: 'object'; id: 'DM300' | 'Rat' }

export type GenerationChallenge = {
  id: Challenge
  /** The game's name (`challenges.*` in `misc.properties`). */
  name: string
  /** What the challenge changes in generated floors. */
  effect: string
  /** The analyzer replays it exactly; otherwise later floors may differ. */
  modeled: boolean
  art: ChallengeArt
}

/**
 * Challenges that change generated floors, in the game's challenge order.
 * Mirrors `Challenge::generation_effect` in spd-core.
 */
export const GENERATION_CHALLENGES: readonly GenerationChallenge[] = [
  {
    id: 'barren_land',
    name: 'Barren land',
    effect: 'Removes the plants',
    modeled: true,
    art: { kind: 'item', className: 'SungrassSeed', category: 'seed' },
  },
  {
    id: 'into_darkness',
    name: 'Into darkness',
    effect: 'Adds a torch to each floor',
    modeled: true,
    art: { kind: 'item', className: 'Torch', category: 'other' },
  },
  {
    id: 'forbidden_runes',
    name: 'Forbidden runes',
    effect: 'Removes half the upgrade scrolls; later drops move',
    modeled: true,
    art: { kind: 'item', className: 'ScrollOfUpgrade', category: 'scroll' },
  },
  {
    id: 'champion_enemies',
    name: 'Hostile champions',
    effect: 'Moves monsters and items; not modeled yet',
    modeled: false,
    art: { kind: 'object', id: 'Rat' },
  },
  {
    id: 'badder_bosses',
    name: 'Badder bosses',
    effect: 'Twice the exposed wires on floor 15',
    modeled: true,
    art: { kind: 'object', id: 'DM300' },
  },
]

/** Challenges that only change play, by the game's name. */
export const PLAY_ONLY_CHALLENGES = [
  'On diet',
  'Faith is my armor',
  'Pharmacophobia',
  'Swarm intelligence',
] as const

export function challengeName(id: Challenge): string {
  return GENERATION_CHALLENGES.find((entry) => entry.id === id)?.name ?? id
}
