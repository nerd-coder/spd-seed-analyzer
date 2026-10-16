import { Terrain, TILE_PX } from '@/lib/dungeon-tile-visuals'
import type { FloorMap } from '@/lib/spd-wasm'

/**
 * Areas a player must search for: rooms behind secret doors, the doors, and
 * hidden traps. Each region fades as one unit and is revealed as one unit.
 */
export type HiddenRegions = {
  /** Cell index → region index. */
  regionOf: Map<number, number>
  /** Cells to repaint per region, including neighbours whose art stitches to it. */
  regions: number[][]
  /**
   * Hidden room floor plus walls seen only from inside it; blacked out in
   * the concealed layer like unexplored space.
   */
  unexplored: Set<number>
}

function isWall(tile: number | undefined) {
  return tile === Terrain.WALL || tile === Terrain.WALL_DECO
}

function neighbours8(map: FloorMap, cell: number) {
  const x = cell % map.width
  const out: number[] = []
  for (const dy of [-1, 0, 1]) {
    for (const dx of [-1, 0, 1]) {
      if ((dx === 0 && dy === 0) || x + dx < 0 || x + dx >= map.width) continue
      const next = cell + dy * map.width + dx
      if (next >= 0 && next < map.tiles.length) out.push(next)
    }
  }
  return out
}

export function hiddenRegions(map: FloorMap): HiddenRegions {
  const hidden = new Set(map.hidden_cells ?? [])
  const regionOf = new Map<number, number>()
  const regions: number[][] = []
  const unexplored = new Set<number>()

  for (const start of hidden) {
    if (regionOf.has(start)) continue
    const region = regions.length
    const cells: number[] = []
    const queue = [start]
    regionOf.set(start, region)
    while (queue.length) {
      const cell = queue.pop() as number
      cells.push(cell)
      const x = cell % map.width
      const neighbours = [
        cell - map.width,
        cell + map.width,
        x > 0 ? cell - 1 : -1,
        x + 1 < map.width ? cell + 1 : -1,
      ]
      for (const next of neighbours) {
        if (hidden.has(next) && !regionOf.has(next)) {
          regionOf.set(next, region)
          queue.push(next)
        }
      }
    }
    regions.push(cells)
  }

  for (const cell of hidden) {
    if (map.tiles[cell] !== Terrain.SECRET_DOOR) unexplored.add(cell)
  }

  // Walls bordering a hidden room join its region. Walls also touching
  // explored floor stay visible but are repainted from the concealed layer,
  // which stitches them as if the room were solid rock.
  for (const cell of [...unexplored]) {
    const region = regionOf.get(cell)
    if (region == null) continue
    for (const wall of neighbours8(map, cell)) {
      if (regionOf.has(wall) || !isWall(map.tiles[wall])) continue
      regionOf.set(wall, region)
      regions[region].push(wall)
      const seenOutside = neighbours8(map, wall).some(
        (next) => !hidden.has(next) && !isWall(map.tiles[next])
      )
      if (!seenOutside) unexplored.add(wall)
    }
  }

  // Wall, overhang, and floor-edge art stitch against neighbours, so a cell
  // outside the room can still draw a notch for its door or floor.
  const repainted = new Set(regionOf.keys())
  for (const cell of hidden) {
    const region = regionOf.get(cell)
    if (region == null) continue
    for (const next of neighbours8(map, cell)) {
      if (repainted.has(next)) continue
      repainted.add(next)
      regions[region].push(next)
    }
  }

  for (const trap of map.traps) {
    if (trap.visible) continue
    const region = regionOf.get(trap.cell)
    if (region != null) continue
    regionOf.set(trap.cell, regions.length)
    regions.push([trap.cell])
  }

  return { regionOf, regions, unexplored }
}

/** The floor as if every secret door and hidden trap had been found. */
export function revealedMap(map: FloorMap): FloorMap {
  return {
    ...map,
    tiles: map.tiles.map((tile) =>
      tile === Terrain.SECRET_DOOR
        ? Terrain.DOOR
        : tile === Terrain.SECRET_TRAP
          ? Terrain.TRAP
          : tile
    ),
    traps: map.traps.map((trap) => ({ ...trap, visible: true })),
  }
}

/** The floor before searching: unexplored space reads as solid wall. */
export function concealedMap(map: FloorMap, unexplored: Set<number>): FloorMap {
  return {
    ...map,
    tiles: map.tiles.map((tile, cell) =>
      unexplored.has(cell) ? Terrain.WALL : tile
    ),
  }
}

/** Black out unexplored cells on the concealed layer, like the void. */
export function blackOutUnexplored(
  canvas: HTMLCanvasElement,
  map: FloorMap,
  unexplored: Set<number>,
  scale: number
) {
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  const size = TILE_PX * scale
  ctx.fillStyle = '#000'
  for (const cell of unexplored) {
    ctx.fillRect(
      (cell % map.width) * size,
      Math.floor(cell / map.width) * size,
      size,
      size
    )
  }
}

/** Clip paths in static-map pixels, one per region. */
export function regionPaths(
  map: FloorMap,
  regions: number[][],
  scale: number
): Path2D[] {
  const size = TILE_PX * scale
  return regions.map((cells) => {
    const path = new Path2D()
    for (const cell of cells) {
      path.rect(
        (cell % map.width) * size,
        Math.floor(cell / map.width) * size,
        size,
        size
      )
    }
    return path
  })
}

const PULSE_SECONDS = 2.4

/** Concealment alpha: 1 looks like the game before searching, 0 is revealed. */
export function concealAlpha(elapsedSeconds: number, animate: boolean) {
  if (!animate) return 0.5
  return 0.5 - 0.5 * Math.cos((2 * Math.PI * elapsedSeconds) / PULSE_SECONDS)
}
