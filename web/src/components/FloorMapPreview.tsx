import { ArrowsOut, SpinnerGapIcon } from '@phosphor-icons/react'
import { useMemo, useRef, useState } from 'react'

import { FloorMapCanvas } from '@/components/FloorMapCanvas'
import { MapSettingsPanel } from '@/components/MapSettingsPanel'
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

const PREVIEW_BOX = 128

type Props = {
  map: FloorMap
  identities: IdentityMaps
  depth: number
  className?: string
  loading?: boolean
}

/** Preserve the previous fitted scale, now bounded to the available choices. */
function initialZoom(map: FloorMap): string {
  if (typeof window === 'undefined') return '1'
  const budget = Math.min(window.innerWidth - 48, window.innerHeight - 140)
  const viewport = mapViewport(map)
  const tileEdge = Math.max(viewport.width, viewport.height) * TILE_PX
  const fit = Math.floor((budget + 2 * TILE_PX) / tileEdge)
  return String(Math.max(1, Math.min(2, fit || 1)))
}

/**
 * Fixed 128×128 clickable map thumbnail; opens a dialog for a larger view.
 */
export function FloorMapPreview({
  map,
  identities,
  depth,
  className,
  loading = false,
}: Props) {
  const viewport = useMemo(() => mapViewport(map), [map])
  const dialogContentRef = useRef<HTMLDivElement>(null)
  const [zoom, setZoom] = useState(() => initialZoom(map))

  return (
    <Dialog>
      <DialogTrigger asChild>
        <button
          type="button"
          className={cn(
            'group relative size-32 shrink-0 cursor-zoom-in overflow-hidden border bg-black/80 outline-none',
            'hover:ring-1 hover:ring-ring focus-visible:ring-2 focus-visible:ring-ring',
            className
          )}
          title={`Floor ${depth} map — click to expand`}
          aria-label={`Expand floor ${depth} map`}
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
          {loading ? (
            <span className="absolute inset-0 flex items-center justify-center bg-black/65 text-white">
              <SpinnerGapIcon className="animate-spin" aria-hidden />
              <span className="sr-only">Regenerating floor {depth} map</span>
            </span>
          ) : null}
        </button>
      </DialogTrigger>
      <DialogContent
        ref={dialogContentRef}
        className="inset-0 flex h-dvh max-h-none w-screen max-w-none translate-x-0 translate-y-0 flex-col gap-3 sm:top-1/2 sm:left-1/2 sm:h-[min(94vh,72rem)] sm:w-[min(96vw,80rem)] sm:max-w-[min(96vw,80rem)] sm:-translate-x-1/2 sm:-translate-y-1/2"
        onOpenAutoFocus={(event) => {
          event.preventDefault()
          dialogContentRef.current?.focus()
        }}
        showCloseButton
      >
        <DialogHeader>
          <DialogTitle className="font-mono">Floor {depth}</DialogTitle>
          <DialogDescription>
            {map.width}×{map.height} · {map.tileset} · discoverable crop{' '}
            {viewport.width}×{viewport.height} · layout only, before mobs and
            items
          </DialogDescription>
        </DialogHeader>
        <div className="relative min-h-0 flex-1 overflow-hidden bg-black/80">
          <MapSettingsPanel zoom={zoom} onZoomChange={setZoom} />
          <div
            className="flex size-full items-start justify-start overflow-auto p-2"
            data-testid="map-scroll-container"
          >
            <FloorMapCanvas
              map={map}
              identities={identities}
              className="m-auto"
              scale={Number(zoom)}
              animateWater
            />
          </div>
          {loading ? (
            <div className="absolute inset-0 flex items-center justify-center gap-2 bg-black/65 text-sm text-white">
              <SpinnerGapIcon className="animate-spin" aria-hidden />
              Regenerating maps…
            </div>
          ) : null}
        </div>
      </DialogContent>
    </Dialog>
  )
}
