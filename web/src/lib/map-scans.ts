import { TILE_PX } from '@/lib/dungeon-tile-visuals'
import type { FloorMap, MapSentryScan } from '@/lib/spd-wasm'

/** `CheckedCell`: a solid 0xFF55AAFF square, as in the game. */
const CHECKED_COLOR = '#55aaff'
const START_ALPHA = 0.8
/** Displayed game turn length; the game advances turns as the hero acts. */
const TURN_SECONDS = 0.5
/** Longest CheckedCell life: the 7-tile scan delay plus the fade. */
const MAX_LIFE = START_ALPHA + 0.4

export function hasSentryScans(map: FloorMap) {
  return (map.sentry_scans?.length ?? 0) > 0
}

/** `CheckedCell.reset` delay: grows with distance from the sentry. */
function checkDelay(map: FloorMap, cell: number, source: number) {
  const dx = (cell % map.width) - (source % map.width)
  const dy = Math.floor(cell / map.width) - Math.floor(source / map.width)
  const delay = Math.sqrt(dx * dx + dy * dy) - 1
  return delay > 0 ? delay ** 0.67 / 10 : 0
}

function drawScan(
  ctx: CanvasRenderingContext2D,
  map: FloorMap,
  sentry: MapSentryScan,
  cells: number[],
  age: number,
  scale: number
) {
  for (const cell of cells) {
    const alpha = START_ALPHA - (age - checkDelay(map, cell, sentry.cell))
    if (alpha <= 0 || alpha > START_ALPHA) continue
    const size = TILE_PX * alpha * scale
    const cx = ((cell % map.width) + 0.5) * TILE_PX * scale
    const cy = (Math.floor(cell / map.width) + 0.5) * TILE_PX * scale
    ctx.globalAlpha = alpha
    ctx.fillRect(cx - size / 2, cy - size / 2, size, size)
  }
}

/**
 * `VaultSentry.act` scan flashes. A sentry scans on turns 1, 1 + cooldown, …
 * cycling through its steps; each scan fades out like `GameScene.checkedCell`.
 */
export function drawSentryScans(
  ctx: CanvasRenderingContext2D,
  map: FloorMap,
  scale: number,
  seconds: number
) {
  const scans = map.sentry_scans
  if (!scans?.length) return
  const turn = seconds / TURN_SECONDS
  ctx.save()
  ctx.fillStyle = CHECKED_COLOR
  for (const sentry of scans) {
    if (!sentry.steps.length || turn < 1) continue
    const cooldown = Math.max(1, sentry.cooldown)
    for (let k = Math.floor((turn - 1) / cooldown); k >= 0; k--) {
      const age = (turn - 1 - k * cooldown) * TURN_SECONDS
      if (age > MAX_LIFE) break
      const step = sentry.steps[k % sentry.steps.length]
      drawScan(ctx, map, sentry, step, age, scale)
    }
  }
  ctx.restore()
}
