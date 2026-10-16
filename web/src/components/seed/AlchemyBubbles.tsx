import { useEffect, useRef } from 'react'
import { loadImage, SPECKS_URL } from '@/lib/map-assets'
import { ParticleField } from '@/lib/map-particles'
import { TILE_PX } from '@/lib/tiles'

/** Source pixels around the pot tile: specks overhang sideways, rise above. */
const PAD_X = 4
const RISE = 24

/**
 * The map's `Alchemy` blob emitter over a pot icon. Sits on the icon's
 * bottom edge; `scale` matches the icon's source-to-CSS scale. Paused
 * off-screen and skipped under reduced motion.
 */
export function AlchemyBubbles({ scale }: { scale: number }) {
  const canvasRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return
    let cancelled = false
    let visible = false
    let requestId = 0
    let field: ParticleField | null = null
    let last = 0

    const frame = (now: number) => {
      const ctx = canvas.getContext('2d')
      if (cancelled || !visible || !field || !ctx) return
      field.step((now - last) / 1000)
      last = now
      ctx.clearRect(0, 0, canvas.width, canvas.height)
      ctx.imageSmoothingEnabled = false
      ctx.save()
      ctx.translate(PAD_X * scale, RISE * scale)
      field.draw(ctx, scale)
      ctx.restore()
      requestId = requestAnimationFrame(frame)
    }
    const start = () => {
      cancelAnimationFrame(requestId)
      last = performance.now()
      requestId = requestAnimationFrame(frame)
    }

    loadImage(SPECKS_URL).then((specks) => {
      if (cancelled) return
      field = ParticleField.forTile('Alchemy', specks)
      if (visible) start()
    })
    const observer = new IntersectionObserver(([entry]) => {
      visible = entry?.isIntersecting ?? false
      if (visible && field) start()
    })
    observer.observe(canvas)
    return () => {
      cancelled = true
      observer.disconnect()
      cancelAnimationFrame(requestId)
    }
  }, [scale])

  const width = (TILE_PX + PAD_X * 2) * scale
  const height = (TILE_PX + RISE) * scale
  return (
    <canvas
      ref={canvasRef}
      aria-hidden
      width={width}
      height={height}
      className="pointer-events-none absolute bottom-0"
      style={{
        left: -PAD_X * scale,
        width,
        height,
        imageRendering: 'pixelated',
      }}
    />
  )
}
