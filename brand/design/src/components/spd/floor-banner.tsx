import type { ReactNode } from 'react'
import { isBossFloor, regionOf, textureUrl } from '@/lib/regions'
import { cn } from '@/lib/utils'

const BOSSES: Record<number, string> = {
  5: 'Goo',
  10: 'Tengu',
  15: 'DM-300',
  20: 'Dwarf King',
  25: 'Yog-Dzewa',
}

/**
 * Floor header: a region-colored depth block, then a window onto the
 * region's floor tiles. The tile strip stays dark in both themes, so its
 * label always uses the game's outlined white text.
 */
export function FloorBanner({
  depth,
  children,
  action,
  className,
}: {
  depth: number
  /** Extra marks after the title, such as an alchemy pot. */
  children?: ReactNode
  /** Trailing control, such as the map button. */
  action?: ReactNode
  className?: string
}) {
  const region = regionOf(depth)
  const boss = isBossFloor(depth) ? BOSSES[depth] : undefined

  return (
    <div
      className={cn(
        'flex min-h-16 items-stretch border-y border-border',
        className
      )}
    >
      <div
        className={cn(
          'grid w-16 shrink-0 place-items-center text-region-foreground inset-shadow-raised',
          region.fill
        )}
      >
        <span className="font-pixel text-px-2" aria-hidden>
          {depth}
        </span>
      </div>
      <div
        className="pixelated relative flex min-w-0 flex-1 items-center gap-3 overflow-hidden px-4 py-2"
        style={{
          backgroundImage: `url(${textureUrl(region.id)})`,
          backgroundSize: '256px 64px',
        }}
      >
        <div
          aria-hidden
          className="absolute inset-0 bg-linear-to-r from-black/70 via-black/45 to-black/25"
        />
        <div className="relative flex min-w-0 flex-wrap items-baseline gap-x-3 gap-y-0.5">
          <h3 className="ink-outline font-pixel text-px-1 text-white">
            Floor {depth}
          </h3>
          <span className="text-sm font-semibold text-white/85">
            {region.name}
            {boss ? `, boss: ${boss}` : null}
          </span>
        </div>
        {children ? (
          <div className="relative flex items-center gap-2">{children}</div>
        ) : null}
        {action ? <div className="relative ml-auto">{action}</div> : null}
      </div>
    </div>
  )
}
