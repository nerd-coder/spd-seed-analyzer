import {
  type IconResolveOpts,
  itemIconStyle,
  resolveItemIconIndex,
} from '@/lib/item-icons'
import { cn } from '@/lib/utils'

type Props = {
  className?: string
  /** Java simple class name, e.g. `Sword`, `PotionOfHealing`. */
  classNameItem?: string | null
  category?: string | null
  appearance?: string | null
  /** Display size in CSS pixels (default 16 = native sprite). */
  size?: number
  /** Visible frame size within the 16×16 atlas cell. */
  sourceWidth?: number
  sourceHeight?: number
  scaleSource?: boolean
  title?: string
}

/**
 * Renders an SPD item sprite from `/assets/sprites/items.png`.
 */
export function ItemIcon({
  className,
  classNameItem,
  category,
  appearance,
  size = 16,
  sourceWidth,
  sourceHeight,
  scaleSource,
  title,
}: Props) {
  const opts: IconResolveOpts = { category, appearance }
  const index = resolveItemIconIndex(classNameItem, opts)
  return (
    <span
      role="img"
      aria-label={title ?? classNameItem ?? 'item'}
      title={title ?? classNameItem ?? undefined}
      className={cn(
        'inline-grid shrink-0 place-items-center align-middle',
        className
      )}
      style={{ width: size, height: size }}
    >
      <span
        aria-hidden="true"
        style={itemIconStyle(
          index,
          size,
          sourceWidth,
          sourceHeight,
          scaleSource
        )}
      />
    </span>
  )
}
