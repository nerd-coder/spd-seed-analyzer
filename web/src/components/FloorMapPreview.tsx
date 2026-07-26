import { ArrowsOut } from '@phosphor-icons/react'
import { useMemo, useRef, useState } from 'react'

import { FloorMapCanvas } from '@/components/FloorMapCanvas'
import { MapSettingsPanel } from '@/components/MapSettingsPanel'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Field, FieldDescription, FieldLabel } from '@/components/ui/field'
import { NativeSelect, NativeSelectOption } from '@/components/ui/native-select'
import type { FloorMap, IdentityMaps, MapTrinketProfile } from '@/lib/spd-wasm'
import { mapViewport, TILE_PX } from '@/lib/tiles'
import { cn } from '@/lib/utils'

const PREVIEW_BOX = 128

type Props = {
  map: FloorMap | null
  identities: IdentityMaps
  depth: number
  className?: string
  trinket: MapTrinketProfile
  onConfigure: (trinket: MapTrinketProfile) => Promise<void>
}

/** Preserve the previous fitted scale, now bounded to the available choices. */
function initialZoom(map: FloorMap | null): string {
  if (!map) return '1'
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
  trinket,
  onConfigure,
}: Props) {
  const viewport = useMemo(() => (map ? mapViewport(map) : null), [map])
  const dialogContentRef = useRef<HTMLDivElement>(null)
  const [zoom, setZoom] = useState(() => initialZoom(map))
  const [selectedTrinket, setSelectedTrinket] = useState(trinket)

  return (
    <Dialog>
      <DialogTrigger asChild>
        <button
          type="button"
          className={cn(
            'group relative flex size-32 shrink-0 cursor-zoom-in items-center justify-center overflow-hidden border bg-black/80 p-2 text-center text-xs text-white outline-none',
            'hover:ring-1 hover:ring-ring focus-visible:ring-2 focus-visible:ring-ring',
            className
          )}
          title={`Configure floor ${depth} map`}
          aria-label={`Configure floor ${depth} map`}
        >
          {map ? (
            <span className="absolute inset-0 flex items-center justify-center">
              <FloorMapCanvas
                map={map}
                identities={identities}
                scale={1}
                maxDisplay={PREVIEW_BOX}
                canvasClassName="border-0"
              />
            </span>
          ) : (
            <span>Configure map</span>
          )}
          {map ? (
            <span
              className="pointer-events-none absolute right-1 bottom-1 flex size-5 items-center justify-center bg-black/55 text-white opacity-70 transition-opacity group-hover:opacity-100"
              aria-hidden
            >
              <ArrowsOut size={12} />
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
            {map && viewport
              ? `${map.width}×${map.height} · ${map.tileset} · discoverable crop ${viewport.width}×${viewport.height} · layout only, before mobs and items`
              : 'Choose the trinket held while this floor is generated.'}
          </DialogDescription>
        </DialogHeader>
        <Field orientation="responsive">
          <div>
            <FieldLabel htmlFor={`floor-${depth}-trinket`}>
              Map-affecting trinket
            </FieldLabel>
            <FieldDescription>
              Replays the seed through this floor and preserves prior trinket
              deck history.
            </FieldDescription>
          </div>
          <NativeSelect
            id={`floor-${depth}-trinket`}
            value={selectedTrinket}
            onChange={(event) =>
              setSelectedTrinket(event.target.value as MapTrinketProfile)
            }
          >
            <NativeSelectOption value="no_map_affecting_trinkets">
              None
            </NativeSelectOption>
            {[0, 1, 2, 3].map((level) => (
              <NativeSelectOption
                key={`mossy-${level}`}
                value={`mossy_clump${level}`}
              >
                Mossy Clump +{level}
              </NativeSelectOption>
            ))}
            {[0, 1, 2, 3].map((level) => (
              <NativeSelectOption
                key={`trap-${level}`}
                value={`trap_mechanism${level}`}
              >
                Trap Mechanism +{level}
              </NativeSelectOption>
            ))}
          </NativeSelect>
          <Button
            type="button"
            size="sm"
            disabled={selectedTrinket === trinket}
            onClick={() => void onConfigure(selectedTrinket)}
          >
            Generate
          </Button>
        </Field>
        {map ? (
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
          </div>
        ) : null}
      </DialogContent>
    </Dialog>
  )
}
