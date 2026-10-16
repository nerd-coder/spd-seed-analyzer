import { TILE_PX } from '@/lib/dungeon-tile-visuals'
import { mobSprite } from '@/lib/map-mob-sprites'
import type { FloorMap, MapMob } from '@/lib/spd-wasm'

/** `Icons.SLEEP` on `interfaces/icons.png`. */
const SLEEP = { x: 7, y: 88, width: 9, height: 8 }
/** `EmoIcon.Sleep`: scale ping-pongs between 1 and 1.2 at 0.5 per second. */
const SLEEP_MAX = 1.2
const SLEEP_RATE = 0.5
const SLEEP_PERIOD = (2 * (SLEEP_MAX - 1)) / SLEEP_RATE

function sleepScale(seconds: number, cell: number) {
  const phase = ((cell * 7919) % 97) / 97
  const t = (seconds / SLEEP_PERIOD + phase) % 1
  const span = SLEEP_MAX - 1
  const grown = t * 2 * span
  return 1 + (grown < span ? grown : 2 * span - grown)
}

function sleeping(map: FloorMap, mob: MapMob) {
  return (
    mob.state === 'sleeping' &&
    mob.cell >= 0 &&
    mob.cell < map.tiles.length &&
    (map.discoverable.length !== map.tiles.length || map.discoverable[mob.cell])
  )
}

export function hasSleepingMobs(map: FloorMap) {
  return map.mobs.some(
    (mob) => sleeping(map, mob) && mobSprite(mob.mob) != null
  )
}

/**
 * Sleep emotes over sleeping mobs, drawn above walls like GameScene's
 * `emoicons` group. `seconds` null draws the resting size.
 */
export function drawMobEmotes(
  ctx: CanvasRenderingContext2D,
  icons: HTMLImageElement,
  map: FloorMap,
  scale: number,
  seconds: number | null
) {
  for (const mob of map.mobs) {
    if (!sleeping(map, mob)) continue
    const spriteDef = mobSprite(mob.mob)
    if (!spriteDef) continue
    const cellX = mob.cell % map.width
    const cellY = Math.floor(mob.cell / map.width)
    const ownerX = cellX * TILE_PX + (TILE_PX - spriteDef.width) / 2
    const ownerY = (cellY + 1) * TILE_PX - spriteDef.height - 6
    // EmoIcon.Sleep centers 4px above its own middle at the owner's top-right.
    const x = Math.floor(ownerX + spriteDef.width - SLEEP.width / 2)
    const y = Math.floor(ownerY - 4 - SLEEP.height / 2)
    const size = seconds == null ? 1 : sleepScale(seconds, mob.cell)
    ctx.drawImage(
      icons,
      SLEEP.x,
      SLEEP.y,
      SLEEP.width,
      SLEEP.height,
      x * scale,
      y * scale,
      SLEEP.width * size * scale,
      SLEEP.height * size * scale
    )
  }
}
