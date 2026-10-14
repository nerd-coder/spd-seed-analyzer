import type { CSSProperties } from 'react'
import type { Sprite as SpriteData } from '@/lib/sprites'
import { cn } from '@/lib/utils'

/**
 * One sprite crop at a whole-number scale. Sheets are never smoothed; a
 * fractional scale is a bug, so `scale` is rounded.
 */
export function Sprite({
  sprite,
  scale = 2,
  className,
  style,
  label,
}: {
  sprite: SpriteData
  scale?: number
  className?: string
  style?: CSSProperties
  /** Accessible name. Omit when a visible label sits next to the sprite. */
  label?: string
}) {
  const s = Math.max(1, Math.round(scale))
  const crop: CSSProperties = {
    width: sprite.w * s,
    height: sprite.h * s,
    backgroundImage: `url(${sprite.url})`,
    backgroundSize: `${sprite.sheetWidth * s}px ${sprite.sheetHeight * s}px`,
    backgroundPosition: `-${sprite.x * s}px -${sprite.y * s}px`,
    ...style,
  }
  const cls = cn('pixelated inline-block shrink-0', className)
  return label ? (
    <span role="img" aria-label={label} className={cls} style={crop} />
  ) : (
    <span aria-hidden className={cls} style={crop} />
  )
}
