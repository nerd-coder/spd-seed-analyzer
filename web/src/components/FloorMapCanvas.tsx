import { useEffect, useMemo, useRef, useState } from 'react'
import { Terrain } from '@/lib/dungeon-tile-visuals'
import type { FloorMap, IdentityMaps } from '@/lib/spd-wasm'
import {
  drawFloorMap,
  loadMapAssets,
  mapViewport,
  renderStaticMap,
  TILE_PX,
} from '@/lib/tiles'
import { cn } from '@/lib/utils'

type Props = {
  map: FloorMap
  identities: IdentityMaps
  /** Pixel scale per game tile for canvas backing store (1 = 16px, 2 = 32px, …). */
  scale?: number
  /**
   * Cap the longer displayed edge (CSS px). Canvas is letterboxed to fit while
   * keeping aspect ratio (nearest-neighbor via pixelated rendering).
   */
  maxDisplay?: number
  className?: string
  canvasClassName?: string
  /** Animate the region water texture; disabled for thumbnails to avoid many RAF loops. */
  animateWater?: boolean
  showItems?: boolean
  showMobs?: boolean
}

export function FloorMapCanvas({
  map,
  identities,
  scale = 2,
  maxDisplay,
  className,
  canvasClassName,
  animateWater = false,
  showItems = false,
  showMobs = false,
}: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [error, setError] = useState<string | null>(null)
  const [reducedMotion, setReducedMotion] = useState(false)
  const viewport = useMemo(() => mapViewport(map), [map])

  const naturalW = viewport.width * TILE_PX * scale
  const naturalH = viewport.height * TILE_PX * scale
  let displayW = naturalW
  let displayH = naturalH
  if (maxDisplay != null && maxDisplay > 0) {
    const fit = maxDisplay / Math.max(naturalW, naturalH)
    displayW = Math.max(1, Math.round(naturalW * fit))
    displayH = Math.max(1, Math.round(naturalH * fit))
  }
  const hasWater = animateWater && map.tiles.includes(Terrain.WATER)
  const visibleMarkerLabels = map.markers
    .filter(
      (marker) =>
        (marker.kind === 'item' && showItems) ||
        (marker.kind === 'mob' && showMobs)
    )
    .map((marker) => marker.label)
  const markerDescription = visibleMarkerLabels.length
    ? ` Visible markers: ${visibleMarkerLabels.join(', ')}.`
    : ''

  useEffect(() => {
    if (!animateWater) return
    const media = window.matchMedia('(prefers-reduced-motion: reduce)')
    const update = () => setReducedMotion(media.matches)
    update()
    media.addEventListener?.('change', update)
    return () => media.removeEventListener?.('change', update)
  }, [animateWater])

  useEffect(() => {
    let cancelled = false
    let requestId = 0
    setError(null)
    const canvas = canvasRef.current
    if (!canvas) return

    canvas.width = naturalW
    canvas.height = naturalH

    loadMapAssets(map.tileset)
      .then((assets) => {
        if (cancelled) return
        const ctx = canvas.getContext('2d')
        if (!ctx) return
        const staticMap = renderStaticMap(assets, map, identities, scale, {
          item: showItems,
          mob: showMobs,
        })
        const started = performance.now()
        const frame = (now: number) => {
          if (cancelled) return
          drawFloorMap(
            ctx,
            assets,
            staticMap,
            scale,
            (now - started) / 1000,
            viewport
          )
          if (hasWater && !reducedMotion) {
            requestId = requestAnimationFrame(frame)
          }
        }
        frame(started)
      })
      .catch((e: unknown) => {
        if (!cancelled) {
          setError(e instanceof Error ? e.message : String(e))
        }
      })

    return () => {
      cancelled = true
      cancelAnimationFrame(requestId)
    }
  }, [
    map,
    identities,
    scale,
    naturalW,
    naturalH,
    hasWater,
    reducedMotion,
    showItems,
    showMobs,
    viewport,
  ])

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
          role="img"
          aria-label={`Shattered Pixel Dungeon floor map. ${map.heaps.length} exact heaps, ${map.mobs.length} exact mobs, ${map.traps.length} traps, and ${map.transitions.length} transitions.${markerDescription}`}
          title={markerDescription.trim() || undefined}
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
