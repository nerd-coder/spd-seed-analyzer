import { depthIconFrame, uiIconStyle } from '@/lib/ui-icons'
import { cn } from '@/lib/utils'

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
  // Fit the longer edge to `size` so the icon stays within a size×size box.
  const scale = scaleProp ?? size / Math.max(frame.w, frame.h)
  const label = title ?? (feeling && feeling !== 'none' ? feeling : 'depth')
  return (
    <span
      role="img"
      aria-label={label}
      title={feeling && feeling !== 'none' ? feeling : undefined}
      className={cn('inline-block align-middle', className)}
      style={uiIconStyle(frame, scale)}
    />
  )
}
