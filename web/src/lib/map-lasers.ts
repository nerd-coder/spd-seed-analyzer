import { TILE_PX } from '@/lib/dungeon-tile-visuals'
import type { FloorMap, MapLaserBeam } from '@/lib/spd-wasm'
import { laserFires, laserWarns, TURN_SECONDS } from '@/lib/vault-schedule'

/** `Effects.Type.DEATH_RAY` on `effects/effects.png`. */
const DEATH_RAY = { x: 16, y: 16, width: 16, height: 8 }
/** `Beam.DeathRay` duration. */
const RAY_SECONDS = 0.5
/** `Icons.TARGET` on `interfaces/icons.png`. */
const TARGET = { x: 0, y: 32, size: 16 }
/** `TargetedCell` holds at least this alpha until the shot. */
const TARGET_HOLD = 0.6

export function hasLaserBeams(map: FloorMap) {
  return (map.laser_beams?.length ?? 0) > 0
}

/** `SentrySprite.center()`: an 8×15 sprite raised 6px above its cell. */
function laserOrigin(map: FloorMap, cell: number) {
  return {
    x: ((cell % map.width) + 0.5) * TILE_PX,
    y: Math.floor(cell / map.width) * TILE_PX + 2.5,
  }
}

/** `DungeonTilemap.raisedTileCenterToWorld`. */
function raisedCenter(map: FloorMap, cell: number) {
  return {
    x: ((cell % map.width) + 0.5) * TILE_PX,
    y: (Math.floor(cell / map.width) + 0.1) * TILE_PX,
  }
}

function drawRay(
  ctx: CanvasRenderingContext2D,
  effects: HTMLImageElement,
  map: FloorMap,
  beam: MapLaserBeam,
  age: number
) {
  const p = 1 - age / RAY_SECONDS
  const end = beam.path.at(-1)
  if (p <= 0 || end == null) return
  const s = laserOrigin(map, beam.cell)
  const e = raisedCenter(map, end)
  const length = Math.hypot(e.x - s.x, e.y - s.y)
  ctx.save()
  ctx.translate(s.x, s.y)
  ctx.rotate(Math.atan2(e.y - s.y, e.x - s.x))
  ctx.scale(length / DEATH_RAY.width, p)
  ctx.globalAlpha = p
  ctx.drawImage(
    effects,
    DEATH_RAY.x,
    DEATH_RAY.y,
    DEATH_RAY.width,
    DEATH_RAY.height,
    0,
    -DEATH_RAY.height / 2,
    DEATH_RAY.width,
    DEATH_RAY.height
  )
  ctx.restore()
}

/** `Icons.TARGET` hardlit 0xFF0000, as `TargetedCell` draws it. */
function redTarget(icons: HTMLImageElement) {
  const canvas = document.createElement('canvas')
  canvas.width = TARGET.size
  canvas.height = TARGET.size
  const ctx = canvas.getContext('2d')
  if (!ctx) return canvas
  ctx.drawImage(
    icons,
    TARGET.x,
    TARGET.y,
    TARGET.size,
    TARGET.size,
    0,
    0,
    TARGET.size,
    TARGET.size
  )
  ctx.globalCompositeOperation = 'source-in'
  ctx.fillStyle = '#ff0000'
  ctx.fillRect(0, 0, TARGET.size, TARGET.size)
  return canvas
}

/**
 * `TargetedCell.update`: alpha falls 1/s, held at 0.6 until the shot turn,
 * then fades out; scale is `alpha^0.33`.
 */
function targetAlpha(age: number) {
  const held = Math.max(1 - Math.min(age, TURN_SECONDS), TARGET_HOLD)
  return age <= TURN_SECONDS ? held : held - (age - TURN_SECONDS)
}

function drawTargets(
  ctx: CanvasRenderingContext2D,
  target: HTMLCanvasElement,
  map: FloorMap,
  cells: number[],
  age: number
) {
  const alpha = targetAlpha(age)
  if (alpha <= 0) return
  const size = TILE_PX * alpha ** 0.33
  ctx.globalAlpha = alpha
  for (const cell of cells) {
    const cx = ((cell % map.width) + 0.5) * TILE_PX
    const cy = (Math.floor(cell / map.width) + 0.5) * TILE_PX
    ctx.drawImage(target, cx - size / 2, cy - size / 2, size, size)
  }
}

const targets = new WeakMap<HTMLImageElement, HTMLCanvasElement>()

/**
 * `VaultLaser.act` beams and warnings, on the same turn clock as sentry scans.
 * Draws in map-pixel space; the caller applies viewport translation.
 */
export function drawLaserBeams(
  ctx: CanvasRenderingContext2D,
  effects: HTMLImageElement,
  icons: HTMLImageElement,
  map: FloorMap,
  scale: number,
  seconds: number
) {
  const beams = map.laser_beams
  if (!beams?.length) return
  const now = seconds / TURN_SECONDS
  const turn = Math.floor(now)
  const age = (now - turn) * TURN_SECONDS
  let target = targets.get(icons)
  if (!target) {
    target = redTarget(icons)
    targets.set(icons, target)
  }
  ctx.save()
  ctx.scale(scale, scale)
  ctx.imageSmoothingEnabled = false
  for (const beam of beams) {
    for (const back of [2, 1, 0]) {
      const warned = turn - back
      if (laserWarns(beam, warned)) {
        drawTargets(ctx, target, map, beam.path, age + back * TURN_SECONDS)
      }
    }
  }
  ctx.globalCompositeOperation = 'lighter'
  for (const beam of beams) {
    if (laserFires(beam, turn)) drawRay(ctx, effects, map, beam, age)
  }
  ctx.restore()
}
