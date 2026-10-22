import { describe, expect, test } from 'bun:test'
import type { MapLaserBeam } from '@/lib/spd-wasm'
import {
  decodeFlameTrap,
  type FlameTrap,
  flameFires,
  flameWarns,
  laserFires,
  laserWarns,
} from '@/lib/vault-schedule'

const TURNS = 60

/** Literal `Blob.act` + `VaultFlameTraps.act` for one cell. */
function simulateFlame({ initial, after, triggers }: FlameTrap) {
  let cooldown = initial
  let cur = 0
  const fires: boolean[] = [false]
  const warns: boolean[] = [false]
  for (let turn = 1; turn <= TURNS; turn++) {
    fires.push(cur > 0)
    if (cur > 0) cur--
    if (cooldown <= 0) cooldown = after
    cooldown--
    if (cooldown <= 0) cur += triggers
    warns.push(cur > 0)
  }
  return { fires, warns }
}

/** Literal `VaultLaser.act`. */
function simulateLaser(beam: MapLaserBeam) {
  let cooldown = beam.cooldown
  let fired = 0
  const fires: boolean[] = [false]
  const warns: boolean[] = [false]
  for (let turn = 1; turn <= TURNS; turn++) {
    cooldown--
    fires.push(cooldown <= 0)
    if (cooldown <= 0) {
      fired++
      if (fired < beam.shots) {
        cooldown = 1
      } else {
        fired = 0
        cooldown = beam.interval
      }
    }
    warns.push(cooldown === 1 && beam.warning)
  }
  return { fires, warns }
}

describe('vault flame vents', () => {
  const traps: FlameTrap[] = [
    { initial: 0, after: 2, triggers: 1 },
    { initial: 1, after: 2, triggers: 1 },
    { initial: 1, after: 1, triggers: 1 },
    ...Array.from({ length: 13 }, (_, initial) => ({
      initial,
      after: 5,
      triggers: 2,
    })),
  ]

  test.each(traps)('matches VaultFlameTraps.act for %o', (trap) => {
    const { fires, warns } = simulateFlame(trap)
    for (let turn = 1; turn <= TURNS; turn++) {
      expect([turn, flameFires(trap, turn)]).toEqual([turn, fires[turn]])
      expect([turn, flameWarns(trap, turn)]).toEqual([turn, warns[turn]])
    }
  })

  test('decodes the packed blob value', () => {
    expect(decodeFlameTrap(3 + (5 << 8) + (2 << 16))).toEqual({
      initial: 3,
      after: 5,
      triggers: 2,
    })
    expect(decodeFlameTrap(0)).toBeNull()
  })
})

describe('vault lasers', () => {
  const beams: MapLaserBeam[] = [
    ...[7, 8, 9].flatMap((interval) =>
      Array.from({ length: interval }, (_, i) => ({
        cell: 0,
        cooldown: i + 1,
        interval,
        shots: 1,
        warning: true,
        path: [1],
      }))
    ),
    ...Array.from({ length: 8 }, (_, i) => ({
      cell: 0,
      cooldown: i + 1,
      interval: 3,
      shots: 1,
      warning: false,
      path: [1],
    })),
    ...Array.from({ length: 8 }, (_, i) => ({
      cell: 0,
      cooldown: i + 1,
      interval: 2,
      shots: 6,
      warning: false,
      path: [1],
    })),
  ]

  test.each(beams)('matches VaultLaser.act for %o', (beam) => {
    const { fires, warns } = simulateLaser(beam)
    for (let turn = 1; turn <= TURNS; turn++) {
      expect([turn, laserFires(beam, turn)]).toEqual([turn, fires[turn]])
      expect([turn, laserWarns(beam, turn)]).toEqual([turn, warns[turn]])
    }
  })
})
