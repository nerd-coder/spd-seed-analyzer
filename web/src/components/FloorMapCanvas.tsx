import { useEffect, useRef, useState } from 'react'
import type { FloorMap } from '@/lib/spd-wasm'
import { drawFloorMap, loadTileset, TILE_PX } from '@/lib/tiles'
import { cn } from '@/lib/utils'

type Props = {
  map: FloorMap
  /** Pixel scale per game tile for canvas backing store (1 = 16px, 2 = 32px, …). */
  scale?: number
  /**
   * Cap the longer displayed edge (CSS px). Canvas is letterboxed to fit while
   * keeping aspect ratio (nearest-neighbor via pixelated rendering).
   */
  maxDisplay?: number
  className?: string
  canvasClassName?: string
}

export function FloorMapCanvas({
  map,
  scale = 2,
  maxDisplay,
  className,
  canvasClassName,
}: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [error, setError] = useState<string | null>(null)

  const naturalW = map.width * TILE_PX * scale
  const naturalH = map.height * TILE_PX * scale
  let displayW = naturalW
  let displayH = naturalH
  if (maxDisplay != null && maxDisplay > 0) {
    const fit = maxDisplay / Math.max(naturalW, naturalH)
    displayW = Math.max(1, Math.round(naturalW * fit))
    displayH = Math.max(1, Math.round(naturalH * fit))
  }

  useEffect(() => {
    let cancelled = false
    setError(null)
    const canvas = canvasRef.current
    if (!canvas) return

    canvas.width = naturalW
    canvas.height = naturalH

    loadTileset(map.tileset)
      .then((img) => {
        if (cancelled) return
        const ctx = canvas.getContext('2d')
        if (!ctx) return
        drawFloorMap(ctx, img, map.width, map.height, map.tiles, scale)
      })
      .catch((e: unknown) => {
        if (!cancelled) {
          setError(e instanceof Error ? e.message : String(e))
        }
      })

    return () => {
      cancelled = true
    }
  }, [map, scale, naturalW, naturalH])

  return (
    <div className={cn('inline-flex items-center justify-center', className)}>
      {error ? (
        <p className="text-destructive text-xs">
          {error} (place tilesheets under{' '}
          <code className="font-mono">public/assets/environment/</code>)
        </p>
      ) : (
        <canvas
          ref={canvasRef}
          className={cn('rounded-none border bg-black/80', canvasClassName)}
          style={{
            width: displayW,
            height: displayH,
            imageRendering: 'pixelated',
          }}
        />
      )}
    </div>
  )
}
