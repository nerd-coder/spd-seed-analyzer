import {
  type ReactNode,
  type PointerEvent as ReactPointerEvent,
  useLayoutEffect,
  useRef,
  useState,
} from 'react'

import { MapSettingsPanel } from '@/components/MapSettingsPanel'
import {
  clamp,
  clampPan,
  layoutMapView,
  type MapFrame,
  type MapView,
  wheelZoomFactor,
  zoomAtPoint,
} from '@/lib/map-view'
import { cn } from '@/lib/utils'

const ZOOM_STEP = 1.5

type Metrics = {
  width: number
  height: number
  contentWidth: number
  contentHeight: number
  minZoom: number
  maxZoom: number
}

type Props = {
  contentWidth: number
  contentHeight: number
  /** Canvas backing-store scale. View zoom is applied as CSS size on top. */
  rasterScale: number
  children: (zoom: number) => ReactNode
}

function formatZoom(zoom: number) {
  return zoom.toFixed(4)
}

export function MapStage({
  contentWidth,
  contentHeight,
  rasterScale,
  children,
}: Props) {
  const containerRef = useRef<HTMLDivElement>(null)
  const metricsRef = useRef<Metrics>({
    width: 0,
    height: 0,
    contentWidth,
    contentHeight,
    minZoom: 1,
    maxZoom: 1,
  })
  const pointers = useRef(new Map<number, { x: number; y: number }>())
  const pinch = useRef<{ distance: number; midX: number; midY: number } | null>(
    null
  )
  const [frame, setFrame] = useState<MapFrame | null>(null)

  useLayoutEffect(() => {
    const node = containerRef.current
    if (!node) return

    const measure = (refit: boolean) => {
      const width = node.clientWidth
      const height = node.clientHeight
      setFrame((current) => {
        const next = layoutMapView({
          width,
          height,
          contentWidth,
          contentHeight,
          rasterScale,
          previous: refit ? null : (current?.view ?? null),
        })
        metricsRef.current = {
          width,
          height,
          contentWidth,
          contentHeight,
          minZoom: next.minZoom,
          maxZoom: next.maxZoom,
        }
        return next
      })
    }

    measure(true)
    const observer = new ResizeObserver(() => measure(false))
    observer.observe(node)
    return () => observer.disconnect()
  }, [contentWidth, contentHeight, rasterScale])

  useLayoutEffect(() => {
    const node = containerRef.current
    if (!node) return

    const onWheel = (event: WheelEvent) => {
      event.preventDefault()
      const metrics = metricsRef.current
      if (metrics.width <= 0 || metrics.height <= 0) return
      const rect = node.getBoundingClientRect()
      const factor = wheelZoomFactor({
        deltaY: event.deltaY,
        deltaMode: event.deltaMode,
        pageHeight: metrics.height,
      })
      setFrame((current) =>
        applyZoom(
          current,
          metrics,
          factor,
          event.clientX - rect.left,
          event.clientY - rect.top
        )
      )
    }

    node.addEventListener('wheel', onWheel, { passive: false })
    return () => node.removeEventListener('wheel', onWheel)
  }, [])

  function zoomBy(factor: number) {
    const metrics = metricsRef.current
    setFrame((current) =>
      applyZoom(current, metrics, factor, metrics.width / 2, metrics.height / 2)
    )
  }

  function pointFrom(event: ReactPointerEvent) {
    const rect = containerRef.current?.getBoundingClientRect()
    if (!rect) return { x: 0, y: 0 }
    return { x: event.clientX - rect.left, y: event.clientY - rect.top }
  }

  function rememberPinch() {
    const points = [...pointers.current.values()]
    if (points.length < 2) {
      pinch.current = null
      return
    }
    const [first, second] = points
    pinch.current = {
      distance: Math.hypot(first.x - second.x, first.y - second.y),
      midX: (first.x + second.x) / 2,
      midY: (first.y + second.y) / 2,
    }
  }

  function onPointerDown(event: ReactPointerEvent<HTMLDivElement>) {
    if (event.pointerType === 'mouse' && event.button !== 0) return
    try {
      event.currentTarget.setPointerCapture(event.pointerId)
    } catch {
      // Synthetic or already-released pointers cannot be captured.
    }
    pointers.current.set(event.pointerId, pointFrom(event))
    rememberPinch()
  }

  function onPointerMove(event: ReactPointerEvent<HTMLDivElement>) {
    const previous = pointers.current.get(event.pointerId)
    if (!previous) return
    const next = pointFrom(event)
    pointers.current.set(event.pointerId, next)
    const metrics = metricsRef.current

    if (pointers.current.size >= 2 && pinch.current) {
      const last = pinch.current
      rememberPinch()
      const currentPinch = pinch.current
      if (!currentPinch || last.distance < 1 || currentPinch.distance < 1) {
        return
      }
      const factor = currentPinch.distance / last.distance
      setFrame((current) => {
        if (!current) return current
        const shifted: MapView = {
          ...current.view,
          x: current.view.x + (currentPinch.midX - last.midX),
          y: current.view.y + (currentPinch.midY - last.midY),
        }
        const zoom = clamp(
          shifted.zoom * factor,
          metrics.minZoom,
          metrics.maxZoom
        )
        return {
          ...current,
          view: clampPan(
            zoomAtPoint(shifted, zoom, currentPinch.midX, currentPinch.midY),
            metrics.width,
            metrics.height,
            metrics.contentWidth,
            metrics.contentHeight
          ),
        }
      })
      return
    }

    const dx = next.x - previous.x
    const dy = next.y - previous.y
    if (dx === 0 && dy === 0) return
    setFrame((current) => {
      if (!current) return current
      return {
        ...current,
        view: clampPan(
          { ...current.view, x: current.view.x + dx, y: current.view.y + dy },
          metrics.width,
          metrics.height,
          metrics.contentWidth,
          metrics.contentHeight
        ),
      }
    })
  }

  function endPointer(event: ReactPointerEvent<HTMLDivElement>) {
    pointers.current.delete(event.pointerId)
    rememberPinch()
  }

  const zoom = frame?.view.zoom ?? 1
  const pannable =
    frame != null &&
    (contentWidth * zoom > frame.width + 1 ||
      contentHeight * zoom > frame.height + 1)
  const atMin = frame != null && zoom <= frame.minZoom * 1.001
  const atMax = frame != null && zoom >= frame.maxZoom * 0.999

  return (
    <div className="relative size-full">
      <MapSettingsPanel
        canZoomIn={!atMax}
        canZoomOut={!atMin}
        onZoomIn={() => zoomBy(ZOOM_STEP)}
        onZoomOut={() => zoomBy(1 / ZOOM_STEP)}
      />
      <div
        ref={containerRef}
        role="application"
        aria-label="Floor map"
        className={cn(
          'relative size-full touch-none overflow-hidden select-none',
          pannable ? 'cursor-grab active:cursor-grabbing' : 'cursor-zoom-in'
        )}
        data-testid="map-scroll-container"
        data-zoom={frame ? formatZoom(zoom) : undefined}
        data-min-zoom={frame ? formatZoom(frame.minZoom) : undefined}
        data-max-zoom={frame ? formatZoom(frame.maxZoom) : undefined}
        data-pan-x={frame ? frame.view.x.toFixed(2) : undefined}
        data-pan-y={frame ? frame.view.y.toFixed(2) : undefined}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={endPointer}
        onPointerCancel={endPointer}
        onLostPointerCapture={endPointer}
        onDragStart={(event) => event.preventDefault()}
      >
        <div
          className="absolute top-0 left-0"
          style={{
            visibility: frame ? 'visible' : 'hidden',
            transform: `translate(${frame?.view.x ?? 0}px, ${frame?.view.y ?? 0}px)`,
          }}
        >
          {children(zoom)}
        </div>
      </div>
    </div>
  )
}

function applyZoom(
  current: MapFrame | null,
  metrics: Metrics,
  factor: number,
  anchorX: number,
  anchorY: number
) {
  if (!current || metrics.width <= 0 || metrics.height <= 0) return current
  const nextZoom = clamp(
    current.view.zoom * factor,
    metrics.minZoom,
    metrics.maxZoom
  )
  return {
    ...current,
    view: clampPan(
      zoomAtPoint(current.view, nextZoom, anchorX, anchorY),
      metrics.width,
      metrics.height,
      metrics.contentWidth,
      metrics.contentHeight
    ),
  }
}
