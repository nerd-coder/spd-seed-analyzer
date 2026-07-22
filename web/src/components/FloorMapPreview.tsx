import { ArrowsOut } from '@phosphor-icons/react'
import { useMemo, useState } from 'react'

import { FloorMapCanvas } from '@/components/FloorMapCanvas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import type { FloorMap, IdentityMaps } from '@/lib/spd-wasm'
import { mapViewport, TILE_PX } from '@/lib/tiles'
import { cn } from '@/lib/utils'

const PREVIEW_BOX = 128

type Props = {
  map: FloorMap
  identities: IdentityMaps
  depth: number
  className?: string
}

/** Pick a render scale so the expanded map fills most of the viewport. */
function fullscreenScale(map: FloorMap): number {
  if (typeof window === 'undefined') return 2
  const budget = Math.min(window.innerWidth - 48, window.innerHeight - 140)
  const viewport = mapViewport(map)
  const tileEdge = Math.max(viewport.width, viewport.height) * TILE_PX
  // The dialog scrolls; allow one 2× tile of overflow so HKT keeps the pinned
  // 2× composition after retaining its raised-overhang row.
  const fit = Math.floor((budget + 2 * TILE_PX) / tileEdge)
  return Math.max(1, Math.min(4, fit || 1))
}

/**
 * Fixed 128×128 clickable map thumbnail; opens a dialog for a larger view.
 */
export function FloorMapPreview({ map, identities, depth, className }: Props) {
  const expandScale = useMemo(() => fullscreenScale(map), [map])
  const viewport = useMemo(() => mapViewport(map), [map])
  const [showItems, setShowItems] = useState(false)
  const [showMobs, setShowMobs] = useState(false)
  const itemMarkers = map.markers.filter(
    (marker) => marker.kind === 'item'
  ).length
  const mobMarkers = map.markers.filter(
    (marker) => marker.kind === 'mob'
  ).length

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
        </button>
      </DialogTrigger>
      <DialogContent
        className="flex max-h-[min(92vh,56rem)] w-auto max-w-[min(96vw,56rem)] flex-col gap-3 sm:max-w-[min(96vw,56rem)]"
        showCloseButton
      >
        <DialogHeader>
          <DialogTitle className="font-mono">Floor {depth}</DialogTitle>
          <DialogDescription>
            {map.width}×{map.height} · {map.tileset} · discoverable crop{' '}
            {viewport.width}×{viewport.height}
          </DialogDescription>
        </DialogHeader>
        {map.markers.length > 0 && (
          <div className="flex flex-wrap items-center gap-x-4 gap-y-2">
            {itemMarkers > 0 && (
              <Label htmlFor={`floor-${depth}-item-markers`}>
                <Switch
                  id={`floor-${depth}-item-markers`}
                  size="sm"
                  checked={showItems}
                  onCheckedChange={setShowItems}
                />
                Items ({itemMarkers})
              </Label>
            )}
            {mobMarkers > 0 && (
              <Label htmlFor={`floor-${depth}-mob-markers`}>
                <Switch
                  id={`floor-${depth}-mob-markers`}
                  size="sm"
                  checked={showMobs}
                  onCheckedChange={setShowMobs}
                />
                Known mobs ({mobMarkers})
              </Label>
            )}
            <p className="w-full text-muted-foreground text-xs">
              Markers cover engine-confirmed cells only. Mob generation is exact
              on depth 1, partial and source-aligned on depths 2–4 and 6, and
              not yet ported on depths 7–24.
            </p>
          </div>
        )}
        <div className="flex min-h-0 flex-1 items-start justify-start overflow-auto bg-black/80 p-2">
          <FloorMapCanvas
            map={map}
            identities={identities}
            className="m-auto"
            scale={expandScale}
            animateWater
            showItems={showItems}
            showMobs={showMobs}
          />
        </div>
      </DialogContent>
    </Dialog>
  )
}
