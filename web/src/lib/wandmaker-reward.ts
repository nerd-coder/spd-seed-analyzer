import type { ItemId } from '@/generated/game-ids'

/** One ordered Old Wandmaker option. Slot 0 is wand 1, slot 1 is wand 2. */
export type WandmakerRewardSlot = {
  slot: number
  /** The seed-drawn wand. */
  item: ItemId
  /** Seed-drawn +N. Wand 2 may move when `level_pinned` is false. */
  level: number
  level_pinned: boolean
  candidates?: {
    item: ItemId
    /** Deck-index offsets from the seed draw, nearest first. 0 is the seed draw. */
    drifts: number[]
    weight: number
  }[]
  /** Denominator for candidate weights. Not always 7. */
  scenario_count: number
}
