import type { ReactNode } from 'react'
import { bossOf, floorStripUrl, regionOf } from '@/lib/regions'
import { cn } from '@/lib/utils'

/**
 * Floor header: a region-colored depth block, then a strip of the floor's own
 * tiles, darkened so the outlined white title reads in either theme.
 * Kept under 64px tall so a sticky stack of banners stays compact.
 */
export function FloorBanner({
  depth,
  tileset,
  title,
  headingLevel = 3,
  showDepth = true,
  meta,
  children,
  action,
}: {
  depth: number
  /** Map tileset; picks the strip. Falls back to the depth's region. */
  tileset?: string | null
  /** Defaults to `Floor {depth}`. */
  title?: string
  headingLevel?: 3 | 4
  /** The region-colored depth block. Off for branches inside a quest card. */
  showDepth?: boolean
  /** Short text after the title. Defaults to the region, and boss if any. */
  meta?: ReactNode
  /** Game marks after the title, such as the depth feeling or alchemy pots. */
  children?: ReactNode
  /** Trailing control, such as the map button. */
  action?: ReactNode
}) {
  const region = regionOf(depth)
  const boss = bossOf(depth)
  const Heading = headingLevel === 4 ? 'h4' : 'h3'

  return (
    <div className="flex h-14 w-full items-stretch overflow-hidden border-y border-border bg-black">
      {showDepth ? (
        <div
          aria-hidden
          className={cn(
            'grid w-14 shrink-0 place-items-center text-region-foreground inset-shadow-raised',
            region.fill
          )}
        >
          <span className="font-pixel text-px-2">{depth}</span>
        </div>
      ) : null}
      <div
        className="pixelated relative flex min-w-0 flex-1 items-center gap-3 px-3"
        style={{
          backgroundImage: `url(${floorStripUrl(depth, tileset)})`,
          backgroundSize: '256px 64px',
        }}
      >
        <div
          aria-hidden
          className="pointer-events-none absolute inset-0 bg-linear-to-r from-black/75 via-black/50 to-black/30"
        />
        <div className="relative flex min-w-0 items-baseline gap-2.5">
          <Heading className="ink-outline font-pixel text-px-1 whitespace-nowrap text-white">
            {title ?? `Floor ${depth}`}
          </Heading>
          <span className="truncate text-sm font-semibold text-white/85">
            {meta ?? (boss ? `${region.name}, boss: ${boss}` : region.name)}
          </span>
        </div>
        {children ? (
          <div className="relative flex min-w-0 items-center gap-2">
            {children}
          </div>
        ) : null}
        {action ? <div className="relative ml-auto">{action}</div> : null}
      </div>
    </div>
  )
}
