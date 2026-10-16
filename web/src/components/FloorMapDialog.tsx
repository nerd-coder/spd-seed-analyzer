import { MapTrifoldIcon } from '@phosphor-icons/react'
import { useMemo, useRef } from 'react'

import { InteractiveFloorMap } from '@/components/InteractiveFloorMap'
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

type Props = {
  map: FloorMap
  identities: IdentityMaps
  depth: number
  mapLabel?: string
  dialogTitle?: string
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

/** Map button for a floor banner; opens the full map in a dialog. */
export function FloorMapDialog({
  map,
  identities,
  depth,
  mapLabel = `floor ${depth} map`,
  dialogTitle = `Floor ${depth}`,
}: Props) {
  const viewport = useMemo(() => mapViewport(map), [map])
  const dialogContentRef = useRef<HTMLDivElement>(null)
  const rasterScale = useMemo(() => rasterScaleFor(map), [map])
  const contentWidth = viewport.width * TILE_PX * rasterScale
  const contentHeight = viewport.height * TILE_PX * rasterScale

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button
          type="button"
          variant="secondary"
          size="icon"
          // Dialog triggers set aria-haspopup, which skips the shared press offset.
          className="active:translate-y-px"
          title={`Open the ${mapLabel}`}
          aria-label={`Expand ${mapLabel}`}
        >
          <MapTrifoldIcon className="size-5" />
        </Button>
      </DialogTrigger>
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
          <DialogTitle>{dialogTitle}</DialogTitle>
          <DialogDescription>
            Items, mobs, and traps as generated for this seed. Hover or tap a
            tile to inspect it; pulsing areas are hidden until searched.
          </DialogDescription>
        </DialogHeader>
        <div className="relative z-0 min-h-0 min-w-0 flex-1 overflow-hidden bg-black/80">
          <InteractiveFloorMap
            map={map}
            identities={identities}
            viewport={viewport}
            rasterScale={rasterScale}
            contentWidth={contentWidth}
            contentHeight={contentHeight}
          />
        </div>
      </DialogContent>
    </Dialog>
  )
}
