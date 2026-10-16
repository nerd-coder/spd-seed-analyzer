import { useStore } from '@tanstack/react-store'
import { useEffect, useMemo, useRef } from 'react'
import { Terrain } from '@/lib/dungeon-tile-visuals'
import { drawMobEmotes, hasSleepingMobs } from '@/lib/map-emotes'
import { drawMobs, hasAnimatedMobs, mapMobSheets } from '@/lib/map-entities'
import {
  blackOutUnexplored,
  concealAlpha,
  concealedMap,
  hiddenRegions,
  regionPaths,
  revealedMap,
} from '@/lib/map-hidden'
import { hasBlobEffects, ParticleField } from '@/lib/map-particles'
import type { FloorMap, IdentityMaps } from '@/lib/spd-wasm'
import {
  type ConcealOverlay,
  drawFloorMap,
  loadMapAssets,
  mapViewport,
  renderMapLayers,
  renderStaticMap,
  TILE_PX,
} from '@/lib/tiles'
import { cn } from '@/lib/utils'
import { createMapCanvasStore } from '@/stores/ui'

const REDUCED_MOTION_QUERY = '(prefers-reduced-motion: reduce)'
/** Mobs are drawn per frame between the layers; heaps stay static. */
const STATIC_ENTITIES = { item: true, mob: false }
/** The concealed overlay holds mobs on their first idle frame. */
const ALL_ENTITIES = { item: true, mob: true }

function prefersReducedMotion() {
  return (
    typeof window !== 'undefined' &&
    window.matchMedia(REDUCED_MOTION_QUERY).matches
  )
}

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
  /**
   * Extra CSS scale on the bitmap. Keeps `image-rendering: pixelated`
   * instead of scaling the canvas through a CSS transform.
   */
  displayScale?: number
  /**
   * Cover the parent with CSS. Does not change `canvas.width` / `canvas.height`.
   */
  fill?: boolean
  /**
   * Animate water, mob idles, blob particles, and the hidden-area pulse;
   * disabled for thumbnails to avoid many RAF loops.
   */
  animateWater?: boolean
  /** Hovered or pinned cell; its hidden region is shown revealed and still. */
  focusCell?: number | null
}

export function FloorMapCanvas({
  map,
  identities,
  scale = 2,
  maxDisplay,
  displayScale = 1,
  fill = false,
  className,
  canvasClassName,
  animateWater = false,
  focusCell = null,
}: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const focusRef = useRef<number | null>(focusCell)
  const redrawRef = useRef<(() => void) | null>(null)
  const canvasStore = useMemo(
    () => createMapCanvasStore(prefersReducedMotion()),
    []
  )
  const { error, reducedMotion } = useStore(canvasStore)
  const viewport = useMemo(() => mapViewport(map), [map])
  const hidden = useMemo(() => hiddenRegions(map), [map])

  const naturalW = viewport.width * TILE_PX * scale
  const naturalH = viewport.height * TILE_PX * scale
  let displayW = naturalW
  let displayH = naturalH
  if (maxDisplay != null && maxDisplay > 0) {
    const fit = maxDisplay / Math.max(naturalW, naturalH)
    displayW = Math.max(1, Math.round(naturalW * fit))
    displayH = Math.max(1, Math.round(naturalH * fit))
  }
  const shownW = Math.max(1, displayW * displayScale)
  const shownH = Math.max(1, displayH * displayScale)
  const hasWater = animateWater && map.tiles.includes(Terrain.WATER)
  const hasHidden = hidden.regions.length > 0
  const hasMotion = useMemo(
    () => hasAnimatedMobs(map) || hasBlobEffects(map) || hasSleepingMobs(map),
    [map]
  )
  const animating =
    animateWater && (hasWater || hasHidden || hasMotion) && !reducedMotion

  useEffect(() => {
    focusRef.current = focusCell
    redrawRef.current?.()
  }, [focusCell])

  useEffect(() => {
    if (!animateWater) return
    const media = window.matchMedia(REDUCED_MOTION_QUERY)
    const update = () =>
      canvasStore.set({ ...canvasStore.get(), reducedMotion: media.matches })
    update()
    media.addEventListener?.('change', update)
    return () => media.removeEventListener?.('change', update)
  }, [animateWater, canvasStore])

  useEffect(() => {
    let cancelled = false
    let requestId = 0
    canvasStore.set({ ...canvasStore.get(), error: null })
    const canvas = canvasRef.current
    if (!canvas) return

    canvas.width = naturalW
    canvas.height = naturalH

    loadMapAssets(map.tileset, mapMobSheets(map))
      .then((assets) => {
        if (cancelled) return
        const ctx = canvas.getContext('2d')
        if (!ctx) return
        const layers = renderMapLayers(
          assets,
          hasHidden ? revealedMap(map) : map,
          identities,
          scale,
          STATIC_ENTITIES
        )
        const particles = animating
          ? ParticleField.forMap(map, assets.specks)
          : null
        let overlay: ConcealOverlay | undefined
        if (hasHidden) {
          const concealed = renderStaticMap(
            assets,
            concealedMap(map, hidden.unexplored),
            identities,
            scale,
            ALL_ENTITIES
          )
          blackOutUnexplored(concealed, map, hidden.unexplored, scale)
          overlay = {
            concealed,
            paths: regionPaths(map, hidden.regions, scale),
            alphas: hidden.regions.map(() => 0),
          }
        }
        const started = performance.now()
        let last = started
        const paint = (now: number) => {
          const seconds = (now - started) / 1000
          particles?.step((now - last) / 1000)
          last = now
          if (overlay) {
            const focused =
              focusRef.current == null
                ? undefined
                : hidden.regionOf.get(focusRef.current)
            const alpha = concealAlpha(seconds, animating)
            overlay.alphas = hidden.regions.map((_, region) =>
              region === focused ? 0 : alpha
            )
          }
          drawFloorMap(ctx, assets, layers, scale, seconds, viewport, overlay, {
            mobs: (layer) =>
              drawMobs(layer, assets, map, scale, animating ? seconds : null),
            effects: (layer) => {
              particles?.draw(layer, scale)
              drawMobEmotes(
                layer,
                assets.icons,
                map,
                scale,
                animating ? seconds : null
              )
            },
          })
        }
        const frame = (now: number) => {
          if (cancelled) return
          paint(now)
          if (animating) requestId = requestAnimationFrame(frame)
        }
        redrawRef.current = animating ? null : () => paint(performance.now())
        frame(started)
      })
      .catch((e: unknown) => {
        if (!cancelled) {
          canvasStore.set({
            ...canvasStore.get(),
            error: e instanceof Error ? e.message : String(e),
          })
        }
      })

    return () => {
      cancelled = true
      redrawRef.current = null
      cancelAnimationFrame(requestId)
    }
  }, [
    map,
    identities,
    scale,
    naturalW,
    naturalH,
    hasHidden,
    hidden,
    animating,
    viewport,
    canvasStore,
  ])

  return (
    <div
      className={cn(
        fill ? 'absolute inset-0' : 'inline-flex items-center justify-center',
        className
      )}
    >
      {error ? (
        <p className="text-destructive text-xs">
          {error} (place tilesheets under{' '}
          <code className="font-mono">public/assets/environment/</code>)
        </p>
      ) : (
        <canvas
          ref={canvasRef}
          className={cn(
            'rounded-none border bg-black/80',
            fill && 'block h-full w-full',
            canvasClassName
          )}
          data-map-canvas=""
          data-water-animation={
            hasWater && !reducedMotion ? 'running' : 'paused'
          }
          data-hidden-regions={hidden.regions.length}
          role="img"
          aria-label={`Shattered Pixel Dungeon floor map. ${map.heaps.length} heaps, ${map.mobs.length} mobs, ${map.traps.length} traps, ${map.transitions.length} transitions, and ${hidden.regions.length} hidden areas.`}
          style={
            fill
              ? {
                  width: '100%',
                  height: '100%',
                  objectFit: 'cover',
                  imageRendering: 'pixelated',
                }
              : {
                  width: shownW,
                  height: shownH,
                  imageRendering: 'pixelated',
                }
          }
        />
      )}
    </div>
  )
}
