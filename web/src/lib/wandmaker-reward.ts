/** One ordered Old Wandmaker option. Slot 0 is wand 1, slot 1 is wand 2. */
export type WandmakerRewardSlot = {
  slot: number
  baseline_class: string
  /** Fresh +N. Wand 2 may move when `level_pinned` is false. */
  baseline_level: number
  level_pinned: boolean
  candidates?: {
    class_name: string
    /** Deck-index offsets from the fresh draw, nearest first. 0 is baseline. */
    drifts: number[]
    weight: number
  }[]
  /** Denominator for candidate weights. Not always 7. */
  scenario_count: number
}
