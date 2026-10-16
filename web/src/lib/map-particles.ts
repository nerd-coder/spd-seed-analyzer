import { blobClassName } from '@/generated/game-ids'
import { TILE_PX } from '@/lib/dungeon-tile-visuals'
import type { FloorMap } from '@/lib/spd-wasm'

/**
 * Blob emitters from pinned v4.0.2 `Blob.use(BlobEmitter)`: each tick emits
 * one particle per blob cell, at a random point inside `bound` of the cell.
 */
type Kind =
  | 'bubble'
  | 'healing'
  | 'question'
  | 'toxic'
  | 'shaft'
  | 'elmo'
  | 'sacrificial'

type EmitterDef = {
  kind: Kind
  interval: number
  bound?: readonly [number, number, number, number]
}

const BLOB_EMITTERS: Record<string, EmitterDef> = {
  Alchemy: { kind: 'bubble', interval: 0.33 },
  WaterOfHealth: { kind: 'healing', interval: 0.5 },
  WaterOfAwareness: { kind: 'question', interval: 0.3 },
  Foliage: { kind: 'shaft', interval: 0.9 },
  // Painted at full volume: 0.01 + (volume / max) * 0.09.
  SacrificialFire: { kind: 'sacrificial', interval: 0.1 },
  EternalFire: { kind: 'elmo', interval: 0.02 },
  VaultFlameTraps: { kind: 'elmo', interval: 0.3, bound: [0.4, 0.4, 0.6, 0.6] },
  ToxicGas: { kind: 'toxic', interval: 0.4 },
}

/** `Speck` film frames (7×7 on `effects/specks.png`). */
const SPECK_SIZE = 7
const SPECK_FRAME: Partial<Record<Kind, number>> = {
  healing: 0,
  question: 3,
  bubble: 12,
  toxic: 13,
}
const TOXIC_HARDLIGHT = '#50ff60'

type Particle = {
  kind: Kind
  x: number
  y: number
  vy: number
  ay: number
  angle: number
  spin: number
  size: number
  lifespan: number
  left: number
}

type Emitter = { def: EmitterDef; cells: number[]; time: number }

function rand(min: number, max: number) {
  return min + Math.random() * (max - min)
}

function spawn(kind: Kind, x: number, y: number): Particle {
  const p: Particle = {
    kind,
    x,
    y,
    vy: 0,
    ay: 0,
    angle: 0,
    spin: 0,
    size: 1,
    lifespan: 1,
    left: 1,
  }
  switch (kind) {
    case 'bubble':
      p.vy = -15
      p.size = rand(0.8, 1)
      p.lifespan = rand(0.8, 1.5)
      break
    case 'healing':
      p.vy = -20
      break
    case 'question':
      p.lifespan = 0.8
      break
    case 'toxic':
      p.spin = 30
      p.angle = rand(0, 360)
      p.lifespan = rand(1, 3)
      break
    case 'shaft':
      p.lifespan = 1.2
      p.vy = -6
      p.left = p.lifespan + rand(0, p.lifespan)
      return p
    case 'elmo':
      p.lifespan = 0.6
      p.ay = -80
      p.size = 4
      break
    case 'sacrificial':
      p.lifespan = 0.6
      p.ay = -100
      p.size = 4
      p.y -= 4
      break
  }
  p.left = p.lifespan
  return p
}

/** Alpha, scale, and colour for one frame, per the pinned `update()` bodies. */
function look(p: Particle) {
  const t = 1 - p.left / p.lifespan
  const q = p.left / p.lifespan
  switch (p.kind) {
    case 'bubble':
      return { alpha: t < 0.2 ? t * 5 : 1, scale: p.size }
    case 'healing':
      return { alpha: t < 0.5 ? 1 : 2 - t * 2, scale: 1 }
    case 'question':
      return { alpha: 1, scale: Math.sqrt(t < 0.5 ? t : 1 - t) * 3 }
    case 'toxic':
      return { alpha: Math.sqrt((t < 0.5 ? t : 1 - t) * 0.5), scale: 1 + t }
    case 'shaft':
      return { alpha: q < 0.5 ? q : 1 - q, scale: 1 }
    case 'elmo':
      return { alpha: q > 0.8 ? (1 - q) * 5 : 1, scale: p.size * q }
    case 'sacrificial':
      return { alpha: q > 0.75 ? (1 - q) * 4 : 1, scale: p.size * q }
  }
}

const PIXEL_COLOR: Partial<Record<Kind, string>> = {
  shaft: '#ffffff',
  elmo: '#22ee66',
  sacrificial: '#4488ee',
}

function tintedSpeck(specks: HTMLImageElement, frame: number, color: string) {
  const canvas = document.createElement('canvas')
  canvas.width = SPECK_SIZE
  canvas.height = SPECK_SIZE
  const ctx = canvas.getContext('2d')
  if (!ctx) return canvas
  const sx = frame * SPECK_SIZE
  ctx.drawImage(specks, sx, 0, 7, 7, 0, 0, 7, 7)
  ctx.globalCompositeOperation = 'multiply'
  ctx.fillStyle = color
  ctx.fillRect(0, 0, SPECK_SIZE, SPECK_SIZE)
  ctx.globalCompositeOperation = 'destination-in'
  ctx.drawImage(specks, sx, 0, 7, 7, 0, 0, 7, 7)
  return canvas
}

export function hasBlobEffects(map: FloorMap) {
  return map.blobs.some(
    (blob) =>
      BLOB_EMITTERS[blobClassName(blob.blob)] != null &&
      blob.cells.some((cell) => cell.value > 0)
  )
}

/** Live blob particles for one floor, simulated in map pixels. */
export class ParticleField {
  private emitters: Emitter[] = []
  private particles: Particle[] = []
  private toxic: HTMLCanvasElement | null = null
  private width: number
  private specks: HTMLImageElement

  private constructor(width: number, specks: HTMLImageElement) {
    this.width = width
    this.specks = specks
  }

  private add(def: EmitterDef, cells: number[]) {
    if (cells.length === 0) return
    this.emitters.push({ def, cells, time: rand(0, def.interval) })
  }

  static forMap(map: FloorMap, specks: HTMLImageElement) {
    const field = new ParticleField(map.width, specks)
    const discoverable = map.discoverable.length === map.tiles.length
    for (const blob of map.blobs) {
      const def = BLOB_EMITTERS[blobClassName(blob.blob)]
      if (!def) continue
      field.add(
        def,
        blob.cells
          .filter((cell) => cell.value > 0)
          .map((cell) => cell.cell)
          .filter((cell) => !discoverable || map.discoverable[cell])
      )
    }
    return field
  }

  /** One blob's emitter on a lone tile at the origin, for UI marks. */
  static forTile(blobName: string, specks: HTMLImageElement) {
    const field = new ParticleField(1, specks)
    const def = BLOB_EMITTERS[blobName]
    if (def) field.add(def, [0])
    return field
  }

  get active() {
    return this.emitters.length > 0
  }

  step(dt: number) {
    const elapsed = Math.min(dt, 0.1)
    for (const emitter of this.emitters) {
      emitter.time += elapsed
      while (emitter.time > emitter.def.interval) {
        emitter.time -= emitter.def.interval
        this.emit(emitter)
      }
    }
    for (const p of this.particles) {
      p.vy += p.ay * elapsed
      p.y += p.vy * elapsed
      p.angle += p.spin * elapsed
      p.left -= elapsed
    }
    this.particles = this.particles.filter((p) => p.left > 0)
  }

  private emit({ def, cells }: Emitter) {
    const [left, top, right, bottom] = def.bound ?? [0, 0, 1, 1]
    for (const cell of cells) {
      const x = (cell % this.width) + rand(left, right)
      const y = Math.floor(cell / this.width) + rand(top, bottom)
      this.particles.push(spawn(def.kind, x * TILE_PX, y * TILE_PX))
    }
  }

  /** Draw in map-pixel space; the caller applies viewport translation. */
  draw(ctx: CanvasRenderingContext2D, scale: number) {
    if (this.particles.length === 0) return
    ctx.save()
    ctx.scale(scale, scale)
    for (const p of this.particles) {
      const { alpha, scale: size } = look(p)
      if (alpha <= 0 || size <= 0) continue
      ctx.globalAlpha = Math.min(1, alpha)
      const pixel = PIXEL_COLOR[p.kind]
      if (pixel) {
        ctx.globalCompositeOperation = 'lighter'
        ctx.fillStyle = pixel
        if (p.kind === 'shaft') {
          const q = p.left / p.lifespan
          const w = (1 - q) * 4
          const h = 16 + (1 - q) * 16
          ctx.fillRect(p.x + 0.5 - w / 2, p.y + 0.5 - h / 2, w, h)
        } else {
          ctx.fillRect(p.x + 0.5 - size / 2, p.y + 0.5 - size / 2, size, size)
        }
        continue
      }
      ctx.globalCompositeOperation = 'source-over'
      this.drawSpeck(ctx, p, size)
    }
    ctx.restore()
  }

  private drawSpeck(ctx: CanvasRenderingContext2D, p: Particle, size: number) {
    const frame = SPECK_FRAME[p.kind] ?? 0
    const half = SPECK_SIZE / 2
    ctx.save()
    ctx.translate(p.x, p.y)
    if (p.angle) ctx.rotate((p.angle * Math.PI) / 180)
    ctx.scale(size, size)
    if (p.kind === 'toxic') {
      this.toxic ??= tintedSpeck(this.specks, frame, TOXIC_HARDLIGHT)
      ctx.drawImage(this.toxic, -half, -half)
    } else {
      ctx.drawImage(
        this.specks,
        frame * SPECK_SIZE,
        0,
        SPECK_SIZE,
        SPECK_SIZE,
        -half,
        -half,
        SPECK_SIZE,
        SPECK_SIZE
      )
    }
    ctx.restore()
  }
}
