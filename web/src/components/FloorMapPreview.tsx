import { Maximize2 } from 'lucide-react'
import { useMemo } from 'react'

import { FloorMapCanvas } from '@/components/FloorMapCanvas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import type { FloorMap } from '@/lib/spd-wasm'
import { TILE_PX } from '@/lib/tiles'
import { cn } from '@/lib/utils'

const PREVIEW_BOX = 128

type Props = {
  map: FloorMap
  depth: number
  className?: string
}

/** Pick a render scale so the expanded map fills most of the viewport. */
function fullscreenScale(map: FloorMap): number {
  if (typeof window === 'undefined') return 2
  const budget = Math.min(window.innerWidth - 48, window.innerHeight - 140)
  const tileEdge = Math.max(map.width, map.height) * TILE_PX
  return Math.max(1, Math.min(4, Math.floor(budget / tileEdge) || 1))
}

/**
 * Fixed 128×128 clickable map thumbnail; opens a dialog for a larger view.
 */
export function FloorMapPreview({ map, depth, className }: Props) {
  const expandScale = useMemo(() => fullscreenScale(map), [map])

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
              scale={1}
              maxDisplay={PREVIEW_BOX}
              canvasClassName="border-0"
            />
          </span>
          <span
            className="pointer-events-none absolute right-1 bottom-1 flex size-5 items-center justify-center bg-black/55 text-white opacity-70 transition-opacity group-hover:opacity-100"
            aria-hidden
          >
            <Maximize2 className="size-3" />
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
            {map.width}×{map.height} · {map.tileset}
          </DialogDescription>
        </DialogHeader>
        <div className="flex min-h-0 flex-1 items-center justify-center overflow-auto bg-black/80 p-2">
          <FloorMapCanvas
            map={map}
            scale={expandScale}
            maxDisplay={
              typeof window !== 'undefined'
                ? Math.min(window.innerWidth - 64, window.innerHeight - 160)
                : 640
            }
          />
        </div>
      </DialogContent>
    </Dialog>
  )
}
