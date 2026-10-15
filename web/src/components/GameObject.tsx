import type { ReactNode } from 'react'
import { GameSprite } from '@/components/GameSprite'
import { GAME_OBJECTS, type GameObjectId } from '@/lib/game-objects'

/** Longer edge of a mob sprite, in CSS pixels. Fits a floor banner. */
const LONG_EDGE = 36

type GameObjectProps = {
  id: GameObjectId
  /** Fit the first frame's longer edge to this many CSS pixels. Default 36. */
  size?: number
  /** Source pixels to CSS pixels. Overrides `size`. */
  scale?: number
  /** Accessible name. Defaults to the catalog label. */
  label?: string
  /** Analyzer notes, shown under the game description. */
  details?: ReactNode
  className?: string
}

/**
 * Animated in-game object: a mob, NPC, boss, or interface mark, playing its
 * SPD idle clip. Hover opens the game description; click pins it.
 */
export function GameObject({
  id,
  size = LONG_EDGE,
  scale,
  label,
  details,
  className,
}: GameObjectProps) {
  const clip = GAME_OBJECTS[id]
  const name = label ?? clip.label
  return (
    <GameSprite
      sheet={clip.sheet}
      frames={clip.frames}
      fps={clip.fps}
      size={size}
      scale={scale}
      label={name}
      title={name}
      textKey={clip.textKey}
      details={details}
      className={className}
    />
  )
}

export type { GameObjectId, GameObjectProps }
