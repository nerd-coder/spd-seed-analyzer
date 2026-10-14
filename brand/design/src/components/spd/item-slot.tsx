import type { CSSProperties } from 'react'
import { Sprite } from '@/components/spd/sprite'
import type { SampleItem } from '@/lib/sample-seed'
import { SPRITES } from '@/lib/sprites'
import { cn } from '@/lib/utils'

/** Upgrade tier text color for `+N`; negative levels read as degraded. */
export function tierClass(level: number) {
  if (level < 0) return 'text-degraded'
  if (level >= 4) return 'text-tier-4'
  if (level === 3) return 'text-tier-3'
  if (level === 2) return 'text-tier-2'
  return 'text-tier-1'
}

export function formatLevel(level: number) {
  return level > 0 ? `+${level}` : `${level}`
}

/**
 * An inventory slot: a recessed square on the 17px journal grid, scaled 2x.
 * Pixel surfaces keep square corners. The frame takes the enchantment glow,
 * or the degraded red for cursed gear. Built from spans so it can sit
 * inside a button.
 */
export function ItemSlot({
  item,
  scale = 2,
  caption,
  className,
}: {
  item: SampleItem
  scale?: number
  /** One short line under the slot, such as an unidentified appearance. */
  caption?: string
  className?: string
}) {
  const size = 17 * scale + 8
  const frame: CSSProperties | undefined = item.glow
    ? { borderColor: item.glow, boxShadow: `inset 0 0 0 1px ${item.glow}66` }
    : undefined
  const label = item.level
    ? `${item.name} ${formatLevel(item.level)}`
    : item.quantity
      ? `${item.quantity} ${item.name}`
      : item.name

  return (
    <span
      className={cn('inline-flex w-fit flex-col items-center gap-1', className)}
    >
      <span
        role="img"
        aria-label={item.cursed ? `${label}, cursed` : label}
        title={label}
        className={cn(
          'relative grid place-items-center border border-border bg-muted inset-shadow-recessed',
          item.cursed && 'border-degraded/80 bg-degraded/10'
        )}
        style={{ width: size, height: size, ...frame }}
      >
        <Sprite sprite={SPRITES[item.sprite]} scale={scale} />
        {item.level ? (
          // `dark` scopes the Torchlit tier colors: labels on art are always
          // outlined in black, as in the game, so they need the bright values.
          <span
            className={cn(
              'dark ink-outline absolute -top-1 right-0.5 font-pixel text-px-1',
              tierClass(item.level)
            )}
          >
            {formatLevel(item.level)}
          </span>
        ) : null}
        {item.quantity ? (
          <span className="ink-outline absolute bottom-0 left-0.5 font-pixel text-px-1 text-white">
            {item.quantity}
          </span>
        ) : null}
      </span>
      {caption ? (
        <span className="max-w-20 truncate text-xs text-muted-foreground">
          {caption}
        </span>
      ) : null}
    </span>
  )
}
