import { useEffect, useState } from 'react'

/**
 * Viewport y where a sticky element's bottom sits once stuck. Its live
 * position is lower until the page scrolls far enough to stick it.
 */
export function stuckBottom(el: HTMLElement | null): number {
  if (!el) return 0
  return (Number.parseFloat(getComputedStyle(el).top) || 0) + el.offsetHeight
}

/**
 * Depth of the floor being read: the last `[data-floor-depth]` section in
 * `container` whose top has reached the stuck bottom of `anchor` (the sticky
 * strip). Null while the reader is above the first floor.
 */
export function useCurrentFloor(
  container: { current: HTMLElement | null },
  anchor: { current: HTMLElement | null }
): number | null {
  const [depth, setDepth] = useState<number | null>(null)

  useEffect(() => {
    let frame = 0
    const measure = () => {
      frame = 0
      const root = container.current
      if (!root) return
      const line = stuckBottom(anchor.current) + 1
      let current: number | null = null
      for (const section of root.querySelectorAll<HTMLElement>(
        '[data-floor-depth]'
      )) {
        if (section.getBoundingClientRect().top > line) break
        current = Number(section.dataset.floorDepth)
      }
      setDepth(current)
    }
    const schedule = () => {
      if (!frame) frame = requestAnimationFrame(measure)
    }
    measure()
    window.addEventListener('scroll', schedule, { passive: true })
    window.addEventListener('resize', schedule)
    return () => {
      if (frame) cancelAnimationFrame(frame)
      window.removeEventListener('scroll', schedule)
      window.removeEventListener('resize', schedule)
    }
  }, [container, anchor])

  return depth
}
