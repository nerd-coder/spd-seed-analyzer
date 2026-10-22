import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'

/** `Chrome.Type.WINDOW`: `NinePatch(CHROME, 0, 0, 20, 20, 6)` (`Chrome.java:50-51`). */
const CHROME = { url: '/assets/interfaces/chrome.png', width: 128, height: 64 }
const WINDOW = { x: 0, y: 0, size: 20, margin: 6 }

type Axis = 'start' | 'middle' | 'end'
const AXES: readonly Axis[] = ['start', 'middle', 'end']

/**
 * Background for one nine-patch cell. Corners and edge thickness scale by
 * `scale`; a middle slice stretches to the cell, so its size and position are
 * percentages: `sheet / slice` wide, offset `origin / (sheet - slice)`.
 */
function axisBackground(
  axis: Axis,
  origin: number,
  sheet: number,
  scale: number
) {
  const { size, margin } = WINDOW
  if (axis === 'middle') {
    const slice = size - 2 * margin
    const start = origin + margin
    return {
      size: `${(sheet / slice) * 100}%`,
      position: `${(start / (sheet - slice)) * 100}%`,
    }
  }
  const start = axis === 'start' ? origin : origin + size - margin
  return { size: `${sheet * scale}px`, position: `-${start * scale}px` }
}

type Inset = number | 'auto'

/** `[start, end, length]` of a cell along one axis. */
function axisBox(axis: Axis, edge: number): [Inset, Inset, Inset] {
  if (axis === 'start') return [0, 'auto', edge]
  if (axis === 'end') return ['auto', 0, edge]
  return [edge, edge, 'auto']
}

/**
 * Content in the game's window frame, drawn from `chrome.png` at a
 * whole-number scale. The frame is game art, so contents use Torchlit tokens.
 */
export function GameWindow({
  scale = 2,
  className,
  children,
}: {
  scale?: number
  className?: string
  children: ReactNode
}) {
  const edge = WINDOW.margin * scale
  return (
    <div
      data-game-window
      className={cn('dark text-foreground relative', className)}
      style={{ padding: edge }}
    >
      <div aria-hidden className="pointer-events-none absolute inset-0">
        {AXES.flatMap((row) =>
          AXES.map((col) => {
            const x = axisBackground(col, WINDOW.x, CHROME.width, scale)
            const y = axisBackground(row, WINDOW.y, CHROME.height, scale)
            const [left, right, width] = axisBox(col, edge)
            const [top, bottom, height] = axisBox(row, edge)
            return (
              <span
                key={`${row}-${col}`}
                className="pixelated absolute"
                style={{
                  left,
                  right,
                  width,
                  top,
                  bottom,
                  height,
                  backgroundImage: `url(${CHROME.url})`,
                  backgroundRepeat: 'no-repeat',
                  backgroundSize: `${x.size} ${y.size}`,
                  backgroundPosition: `${x.position} ${y.position}`,
                }}
              />
            )
          })
        )}
      </div>
      <div className="relative">{children}</div>
    </div>
  )
}
