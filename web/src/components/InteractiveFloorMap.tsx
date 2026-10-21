import {
  type CSSProperties,
  type PointerEvent as ReactPointerEvent,
  useMemo,
  useRef,
  useState,
} from 'react'

import { FloorMapCanvas } from '@/components/FloorMapCanvas'
import { MapInspectPanel } from '@/components/MapInspectPanel'
import { MapStage } from '@/components/MapStage'
import { hasInspection, inspectCell } from '@/lib/map-inspect'
import type { FloorMap, IdentityMaps } from '@/lib/spd-wasm'
import type { MapViewport } from '@/lib/tiles'
import { cn } from '@/lib/utils'

/** Pointer travel (CSS px) above which a press is a pan, not a tap. */
const TAP_SLOP = 5
const PANEL_GAP = 14
const PANEL_MAX = 320
const EDGE = 8
/** Below this container width the panel docks to the bottom edge. */
const DOCK_BELOW = 640

type Focus = { cell: number; x: number; y: number }

type Props = {
  map: FloorMap
  identities: IdentityMaps
  viewport: MapViewport
  rasterScale: number
  contentWidth: number
  contentHeight: number
}

function panelStyle(focus: Focus, width: number, height: number) {
  if (width < DOCK_BELOW) {
    return {
      left: EDGE,
      right: EDGE,
      bottom: EDGE,
      width: 'auto',
      maxWidth: 'none',
      maxHeight: Math.min(PANEL_MAX, height / 2),
    } satisfies CSSProperties
  }
  const style: CSSProperties = {}
  if (focus.x > width / 2) {
    style.right = width - focus.x + PANEL_GAP
  } else {
    style.left = focus.x + PANEL_GAP
  }
  if (focus.y > height / 2) {
    style.bottom = height - focus.y + PANEL_GAP
    style.maxHeight = Math.min(PANEL_MAX, focus.y - PANEL_GAP - EDGE)
  } else {
    style.top = focus.y + PANEL_GAP
    style.maxHeight = Math.min(PANEL_MAX, height - focus.y - PANEL_GAP - EDGE)
  }
  return style
}

/**
 * Pannable floor map with tile inspection: hover shows what the tile holds,
 * a tap or click pins it. Hidden regions under the focus stop pulsing.
 */
export function InteractiveFloorMap({
  map,
  identities,
  viewport,
  rasterScale,
  contentWidth,
  contentHeight,
}: Props) {
  const rootRef = useRef<HTMLElement>(null)
  const pressRef = useRef<{ x: number; y: number } | null>(null)
  const [hover, setHover] = useState<Focus | null>(null)
  const [pinned, setPinned] = useState<Focus | null>(null)
  const hiddenCells = useMemo(
    () =>
      new Set([...(map.hidden_cells ?? []), ...(map.concealed_cells ?? [])]),
    [map]
  )

  function focusAt(clientX: number, clientY: number): Focus | null {
    const root = rootRef.current
    const canvas = root?.querySelector('canvas[data-map-canvas]')
    if (!root || !canvas) return null
    const rect = canvas.getBoundingClientRect()
    const fx = (clientX - rect.left) / rect.width
    const fy = (clientY - rect.top) / rect.height
    if (!(fx >= 0 && fx < 1 && fy >= 0 && fy < 1)) return null
    const x = viewport.x + Math.floor(fx * viewport.width)
    const y = viewport.y + Math.floor(fy * viewport.height)
    const rootRect = root.getBoundingClientRect()
    return {
      cell: y * map.width + x,
      x: clientX - rootRect.left,
      y: clientY - rootRect.top,
    }
  }

  function onPointerMove(event: ReactPointerEvent<HTMLElement>) {
    if (event.buttons !== 0) {
      if (hover) setHover(null)
      return
    }
    const next = focusAt(event.clientX, event.clientY)
    if (next?.cell !== hover?.cell) setHover(next)
  }

  function onClick(event: React.MouseEvent<HTMLElement>) {
    const press = pressRef.current
    pressRef.current = null
    if (
      event.target instanceof Element &&
      event.target.closest('[data-slot="map-inspect"]')
    ) {
      return
    }
    if (
      press &&
      Math.hypot(event.clientX - press.x, event.clientY - press.y) > TAP_SLOP
    ) {
      return
    }
    const next = focusAt(event.clientX, event.clientY)
    if (!next || !hasInspection(inspectCell(map, next.cell))) {
      setPinned(null)
      return
    }
    setPinned((current) => (current?.cell === next.cell ? null : next))
  }

  const panelFocus = pinned ?? hover
  const inspection = panelFocus ? inspectCell(map, panelFocus.cell) : null
  const showPanel = inspection != null && hasInspection(inspection)
  const hoverInspectable =
    hover != null && hasInspection(inspectCell(map, hover.cell))
  const root = rootRef.current
  const highlightCells = [pinned?.cell, showPanel ? panelFocus?.cell : null]
  const focusCell = hover?.cell ?? pinned?.cell ?? null

  return (
    <section
      ref={rootRef}
      aria-label="Floor map with tile details"
      className="relative size-full"
      data-testid="interactive-floor-map"
      onKeyDown={(event) => {
        if (event.key === 'Escape' && pinned) {
          event.stopPropagation()
          setPinned(null)
        }
      }}
      onPointerDownCapture={(event) => {
        pressRef.current = { x: event.clientX, y: event.clientY }
      }}
      onPointerMove={onPointerMove}
      onPointerLeave={() => setHover(null)}
      onClick={onClick}
    >
      <MapStage
        contentWidth={contentWidth}
        contentHeight={contentHeight}
        rasterScale={rasterScale}
      >
        {(zoom) => (
          <div
            className={cn(
              'relative leading-none',
              hoverInspectable && 'cursor-pointer'
            )}
          >
            <FloorMapCanvas
              map={map}
              identities={identities}
              scale={rasterScale}
              displayScale={zoom}
              canvasClassName="border-0"
              animateWater
              focusCell={focusCell}
            />
            {[...new Set(highlightCells)].map((cell) =>
              cell == null ? null : (
                <CellHighlight
                  key={cell}
                  cell={cell}
                  map={map}
                  viewport={viewport}
                  hidden={hiddenCells.has(cell)}
                />
              )
            )}
          </div>
        )}
      </MapStage>
      {showPanel && panelFocus && root ? (
        <MapInspectPanel
          inspection={inspection}
          identities={identities}
          tileset={map.tileset}
          pinned={pinned != null}
          style={panelStyle(panelFocus, root.clientWidth, root.clientHeight)}
        />
      ) : null}
    </section>
  )
}

function CellHighlight({
  cell,
  map,
  viewport,
  hidden,
}: {
  cell: number
  map: FloorMap
  viewport: MapViewport
  hidden: boolean
}) {
  const x = (cell % map.width) - viewport.x
  const y = Math.floor(cell / map.width) - viewport.y
  return (
    <div
      aria-hidden
      className={
        hidden
          ? 'pointer-events-none absolute outline-2 outline-primary outline-dashed'
          : 'pointer-events-none absolute outline-2 outline-primary'
      }
      style={{
        left: `${(x / viewport.width) * 100}%`,
        top: `${(y / viewport.height) * 100}%`,
        width: `${100 / viewport.width}%`,
        height: `${100 / viewport.height}%`,
      }}
    />
  )
}
