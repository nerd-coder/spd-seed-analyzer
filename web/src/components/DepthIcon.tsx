import { GameSprite } from '@/components/GameSprite'
import { feelingTextKey } from '@/lib/game-object-text'
import { depthIconFrame, UI_ICONS } from '@/lib/ui-icons'

const ICONS = {
  url: UI_ICONS.url,
  width: UI_ICONS.sheetW,
  height: UI_ICONS.sheetH,
} as const

type Props = {
  /** Level feeling (`none`, `water`, …) — picks Icons.DEPTH_* frame. */
  feeling?: string | null
  /**
   * Target display size in CSS px (default 24).
   * Native depth sprites are 6×7 / 7×7; scaled up with nearest-neighbor.
   */
  size?: number
  /** Explicit pixel scale; overrides `size` when set. */
  scale?: number
  className?: string
  title?: string
}

/**
 * SPD MenuPane depth icon from `/assets/interfaces/icons.png`
 * (seeded-run row, feeling-aware).
 */
export function DepthIcon({
  feeling,
  size = 24,
  scale: scaleProp,
  className,
  title,
}: Props) {
  const frame = depthIconFrame(feeling)
  const scale = scaleProp ?? size / Math.max(frame.w, frame.h)
  const label = title ?? (feeling && feeling !== 'none' ? feeling : 'depth')
  return (
    <GameSprite
      sheet={ICONS}
      frames={[frame]}
      scale={scale}
      label={label}
      title={feeling && feeling !== 'none' ? feeling : undefined}
      textKey={feelingTextKey(feeling)}
      className={className}
    />
  )
}
