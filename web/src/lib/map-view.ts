import { TILE_PX } from '@/lib/dungeon-tile-visuals'

/** Largest on-screen tile. Enough to read pixel art, still bounded. */
export const MAX_TILE_CSS = 128

const FIT_PADDING = 16
const PAN_MARGIN = 48

export type MapView = {
  zoom: number
  x: number
  y: number
}

export type MapFrame = {
  view: MapView
  minZoom: number
  maxZoom: number
  width: number
  height: number
}

export function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value))
}

/** View zoom that fits the unscaled canvas inside the stage with padding. */
export function fitZoom(
  width: number,
  height: number,
  contentWidth: number,
  contentHeight: number
) {
  const availableWidth = Math.max(1, width - FIT_PADDING * 2)
  const availableHeight = Math.max(1, height - FIT_PADDING * 2)
  return Math.min(
    availableWidth / contentWidth,
    availableHeight / contentHeight
  )
}

export function zoomBounds(rasterScale: number, fittedZoom: number) {
  const maxZoom = MAX_TILE_CSS / (TILE_PX * rasterScale)
  const minZoom = Math.min(Math.max(fittedZoom, 0.0001), maxZoom)
  return { minZoom, maxZoom: Math.max(minZoom, maxZoom) }
}

export function centeredView(
  zoom: number,
  width: number,
  height: number,
  contentWidth: number,
  contentHeight: number
): MapView {
  return {
    zoom,
    x: (width - contentWidth * zoom) / 2,
    y: (height - contentHeight * zoom) / 2,
  }
}

/** Keep the point under the cursor fixed while the zoom changes. */
export function zoomAtPoint(
  view: MapView,
  nextZoom: number,
  anchorX: number,
  anchorY: number
): MapView {
  const ratio = nextZoom / view.zoom
  return {
    zoom: nextZoom,
    x: anchorX - (anchorX - view.x) * ratio,
    y: anchorY - (anchorY - view.y) * ratio,
  }
}

function clampAxis(
  offset: number,
  container: number,
  display: number,
  margin: number
) {
  if (display <= container) return (container - display) / 2
  return clamp(offset, container - display - margin, margin)
}

export function clampPan(
  view: MapView,
  width: number,
  height: number,
  contentWidth: number,
  contentHeight: number
): MapView {
  return {
    zoom: view.zoom,
    x: clampAxis(view.x, width, contentWidth * view.zoom, PAN_MARGIN),
    y: clampAxis(view.y, height, contentHeight * view.zoom, PAN_MARGIN),
  }
}

export function layoutMapView(input: {
  width: number
  height: number
  contentWidth: number
  contentHeight: number
  rasterScale: number
  previous: MapView | null
}): MapFrame {
  const { width, height, contentWidth, contentHeight, rasterScale, previous } =
    input
  if (width <= 0 || height <= 0 || contentWidth <= 0 || contentHeight <= 0) {
    return {
      view: { zoom: 1, x: 0, y: 0 },
      minZoom: 1,
      maxZoom: 1,
      width,
      height,
    }
  }

  const fitted = fitZoom(width, height, contentWidth, contentHeight)
  const { minZoom, maxZoom } = zoomBounds(rasterScale, fitted)
  const zoom = clamp(previous?.zoom ?? fitted, minZoom, maxZoom)
  const base = previous
    ? { ...previous, zoom }
    : centeredView(zoom, width, height, contentWidth, contentHeight)
  return {
    minZoom,
    maxZoom,
    width,
    height,
    view: clampPan(base, width, height, contentWidth, contentHeight),
  }
}

export function wheelZoomFactor(event: {
  deltaY: number
  deltaMode: number
  pageHeight: number
}) {
  let delta = event.deltaY
  if (event.deltaMode === 1) delta *= 16
  else if (event.deltaMode === 2) delta *= event.pageHeight
  return Math.exp(-delta * 0.0015)
}
