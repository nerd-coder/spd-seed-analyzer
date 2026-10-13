import {
  ArrowsOut,
  MapTrifoldIcon,
  SpinnerGapIcon,
} from '@phosphor-icons/react'
import { useMemo, useRef } from 'react'

import { FloorMapCanvas } from '@/components/FloorMapCanvas'
import { MapStage } from '@/components/MapStage'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import type { FloorMap, IdentityMaps } from '@/lib/spd-wasm'
import { mapViewport, TILE_PX } from '@/lib/tiles'
import { cn } from '@/lib/utils'

const PREVIEW_BOX = 64

type Props = {
  map: FloorMap
  identities: IdentityMaps
  depth: number
  mapLabel?: string
  dialogTitle?: string
  className?: string
  loading?: boolean
  /** `thumbnail` is a 64×64 box. `banner` covers its positioned parent. */
  variant?: 'thumbnail' | 'banner'
}

/**
 * Canvas backing-store scale. On-screen zoom is independent, so this stays on
 * the historical 1×/2× choice and map snapshots keep the same bitmap.
 */
function rasterScaleFor(map: FloorMap): number {
  if (typeof window === 'undefined') return 1
  const budget = Math.min(window.innerWidth - 48, window.innerHeight - 140)
  const viewport = mapViewport(map)
  const tileEdge = Math.max(viewport.width, viewport.height) * TILE_PX
  const fit = Math.floor((budget + 2 * TILE_PX) / tileEdge)
  return Math.max(1, Math.min(2, fit || 1))
}

/**
 * Map that opens a dialog for a larger view.
 * `thumbnail` is a 64×64 button. `banner` paints across its parent; only the
 * map icon opens the dialog.
 */
export function FloorMapPreview({
  map,
  identities,
  depth,
  mapLabel = `floor ${depth} map`,
  dialogTitle = `Floor ${depth}`,
  className,
  loading = false,
  variant = 'thumbnail',
}: Props) {
  const viewport = useMemo(() => mapViewport(map), [map])
  const dialogContentRef = useRef<HTMLDivElement>(null)
  const rasterScale = useMemo(() => rasterScaleFor(map), [map])
  const contentWidth = viewport.width * TILE_PX * rasterScale
  const contentHeight = viewport.height * TILE_PX * rasterScale
  const banner = variant === 'banner'

  const loadingOverlay = loading ? (
    <span className="pointer-events-none absolute inset-0 z-20 flex items-center justify-center bg-black/65 text-white">
      <SpinnerGapIcon className="animate-spin" aria-hidden />
      <span className="sr-only">Regenerating {mapLabel}</span>
    </span>
  ) : null

  return (
    <Dialog>
      {banner ? (
        <>
          <span
            aria-hidden
            className="pointer-events-none absolute inset-0 z-0"
          >
            <FloorMapCanvas
              map={map}
              identities={identities}
              scale={1}
              fill
              canvasClassName="border-0"
            />
          </span>
          {loadingOverlay}
          <DialogTrigger asChild>
            <Button
              type="button"
              variant="ghost"
              size="icon-lg"
              // Dialog triggers set aria-haspopup, which skips the shared press offset.
              className="absolute right-1.5 bottom-1.5 z-30 active:translate-y-px"
              title={`${mapLabel} - click to expand`}
              aria-label={`Expand ${mapLabel}`}
            >
              <MapTrifoldIcon className="size-6" />
            </Button>
          </DialogTrigger>
        </>
      ) : (
        <DialogTrigger asChild>
          <button
            type="button"
            className={cn(
              'group relative size-16 shrink-0 cursor-zoom-in overflow-hidden border bg-black/80 outline-none',
              'hover:ring-1 hover:ring-ring focus-visible:ring-2 focus-visible:ring-ring',
              className
            )}
            title={`${mapLabel} - click to expand`}
            aria-label={`Expand ${mapLabel}`}
          >
            <span className="absolute inset-0 flex items-center justify-center">
              <FloorMapCanvas
                map={map}
                identities={identities}
                scale={1}
                maxDisplay={PREVIEW_BOX}
                canvasClassName="border-0"
              />
            </span>
            <span
              className="pointer-events-none absolute right-1 bottom-1 flex size-5 items-center justify-center bg-black/55 text-white opacity-70 transition-opacity group-hover:opacity-100"
              aria-hidden
            >
              <ArrowsOut size={12} />
            </span>
            {loadingOverlay}
          </button>
        </DialogTrigger>
      )}
      <DialogContent
        ref={dialogContentRef}
        className="inset-0 flex h-dvh max-h-none w-full max-w-none translate-x-0 translate-y-0 flex-col gap-3 overflow-hidden sm:top-1/2 sm:left-1/2 sm:h-[min(94vh,72rem)] sm:w-[min(96vw,80rem)] sm:max-w-[min(96vw,80rem)] sm:-translate-x-1/2 sm:-translate-y-1/2"
        onOpenAutoFocus={(event) => {
          event.preventDefault()
          dialogContentRef.current?.focus()
        }}
        showCloseButton
      >
        <DialogHeader className="shrink-0 pr-14">
          <DialogTitle className="font-mono">{dialogTitle}</DialogTitle>
          <DialogDescription>
            {map.width}×{map.height} · {map.tileset} · discoverable crop{' '}
            {viewport.width}×{viewport.height} · layout only, before mobs and
            items
          </DialogDescription>
        </DialogHeader>
        <div className="relative z-0 min-h-0 min-w-0 flex-1 overflow-hidden bg-black/80">
          <MapStage
            contentWidth={contentWidth}
            contentHeight={contentHeight}
            rasterScale={rasterScale}
          >
            {(zoom) => (
              <FloorMapCanvas
                map={map}
                identities={identities}
                scale={rasterScale}
                displayScale={zoom}
                canvasClassName="border-0"
                animateWater
              />
            )}
          </MapStage>
          {loading ? (
            <div className="absolute inset-0 z-20 flex items-center justify-center gap-2 bg-black/65 text-sm text-white">
              <SpinnerGapIcon className="animate-spin" aria-hidden />
              Regenerating maps…
            </div>
          ) : null}
        </div>
      </DialogContent>
    </Dialog>
  )
}
