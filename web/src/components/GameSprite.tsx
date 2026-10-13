import { useEffect, useRef, useState } from 'react'
import {
  itemDetailAvailable,
  scheduleHideItemDetail,
  showItemDetail,
  togglePinItemDetail,
} from '@/components/item-detail'
import { cn } from '@/lib/utils'

/** One cell on a sprite sheet, in source pixels. */
export type SpriteFrame = {
  x: number
  y: number
  w: number
  h: number
}

export type SpriteSheet = {
  url: string
  width: number
  height: number
}

/**
 * TextureFilm index on a uniform grid. `cols` is integer division, matching
 * `TextureFilm(width, height)`.
 */
export function filmFrames(
  sheetWidth: number,
  frameWidth: number,
  frameHeight: number,
  indices: readonly number[]
): SpriteFrame[] {
  const cols = Math.floor(sheetWidth / frameWidth)
  return indices.map((index) => {
    const col = index % cols
    const row = Math.floor(index / cols)
    return {
      x: col * frameWidth,
      y: row * frameHeight,
      w: frameWidth,
      h: frameHeight,
    }
  })
}

type Listener = (now: number) => void

const listeners = new Set<Listener>()
let clock = 0

function subscribe(listener: Listener) {
  listeners.add(listener)
  if (!clock) {
    const tick = (now: number) => {
      for (const current of listeners) current(now)
      clock = listeners.size > 0 ? requestAnimationFrame(tick) : 0
    }
    clock = requestAnimationFrame(tick)
  }
  return () => {
    listeners.delete(listener)
    if (listeners.size === 0 && clock) {
      cancelAnimationFrame(clock)
      clock = 0
    }
  }
}

/**
 * Hold the first frame when the user asks for reduced motion. Visual checks
 * set that media feature, so a live clip stays on its opening cell there.
 */
function useFrameIndex(count: number, fps: number, loop: boolean) {
  const [index, setIndex] = useState(0)

  useEffect(() => {
    if (count <= 1 || fps <= 0) return
    const media = window.matchMedia('(prefers-reduced-motion: reduce)')
    let stop = () => {}
    const start = () => {
      stop()
      if (media.matches) {
        setIndex(0)
        return
      }
      const startAt = performance.now()
      const delayMs = 1000 / fps
      stop = subscribe((now) => {
        const step = Math.floor((now - startAt) / delayMs)
        const next = loop ? step % count : Math.min(step, count - 1)
        setIndex((current) => (current === next ? current : next))
      })
    }
    start()
    media.addEventListener('change', start)
    return () => {
      stop()
      media.removeEventListener('change', start)
    }
  }, [count, fps, loop])

  return count <= 1 ? 0 : index
}

/**
 * One in-game object drawn from a sprite sheet. Pass several frames to play
 * them in order. `fps` is the SPD `Animation` rate: each frame holds for
 * `1 / fps` seconds. A single frame stays still.
 */
export function GameSprite({
  sheet,
  frames,
  fps = 2,
  loop = true,
  size,
  scale: scaleProp,
  label,
  title,
  className,
  textKey,
}: {
  sheet: SpriteSheet
  frames: readonly SpriteFrame[]
  /** Ignored when `frames` has one cell. Default 2, a common idle rate. */
  fps?: number
  loop?: boolean
  /** Fit the first frame's longer edge to this many CSS pixels. */
  size?: number
  /** Source pixels to CSS pixels. Overrides `size`. Default 1. */
  scale?: number
  label: string
  title?: string
  className?: string
  /**
   * Examine-window key (`gameObjectText` or an item class). Hover opens the
   * shared description panel; click pins it.
   */
  textKey?: string
}) {
  const index = useFrameIndex(frames.length, fps, loop)
  const anchorRef = useRef<HTMLButtonElement>(null)
  const cell = frames[Math.min(index, Math.max(frames.length - 1, 0))]
  const detail = { className: textKey }
  const interactive = itemDetailAvailable(detail)
  if (!cell) return null
  const basis = frames[0] ?? cell
  const scale =
    scaleProp ?? (size != null ? size / Math.max(basis.w, basis.h) : 1)
  const face = (
    <span
      role="img"
      aria-label={label}
      title={interactive ? undefined : title}
      className="block"
      style={{
        width: cell.w * scale,
        height: cell.h * scale,
        backgroundImage: `url(${sheet.url})`,
        backgroundRepeat: 'no-repeat',
        backgroundSize: `${sheet.width * scale}px ${sheet.height * scale}px`,
        backgroundPosition: `-${cell.x * scale}px -${cell.y * scale}px`,
        imageRendering: 'pixelated',
      }}
    />
  )
  if (!interactive) {
    return (
      <span className={cn('inline-block shrink-0 align-middle', className)}>
        {face}
      </span>
    )
  }

  return (
    <button
      ref={anchorRef}
      type="button"
      data-slot="game-sprite"
      className={cn(
        'inline-block shrink-0 cursor-pointer border-0 bg-transparent p-0 align-middle select-none focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-ring',
        className
      )}
      onPointerEnter={() => {
        const anchor = anchorRef.current
        if (anchor) showItemDetail(anchor, detail)
      }}
      onPointerLeave={() => {
        const anchor = anchorRef.current
        if (anchor) scheduleHideItemDetail(anchor)
      }}
      onClick={(event) => {
        event.preventDefault()
        const anchor = anchorRef.current
        if (anchor) togglePinItemDetail(anchor, detail)
      }}
    >
      {face}
    </button>
  )
}
