import type { MapLaserBeam } from '@/lib/spd-wasm'

/**
 * Displayed game turn length; the game advances turns as the hero acts.
 * Turn `t` (1-based) acts at `t * TURN_SECONDS`.
 */
export const TURN_SECONDS = 0.5

/** `VaultLevel.VaultFlameTrap.setupTrap(cell, initialCD, afterTriggerCD, triggers)`. */
export type FlameTrap = { initial: number; after: number; triggers: number }

/** Unpacks the `initial | after << 8 | triggers << 16` blob cell value. */
export function decodeFlameTrap(value: number): FlameTrap | null {
  const after = (value >> 8) & 0xff
  if (after < 1) return null
  return { initial: value & 0xff, after, triggers: (value >> 16) & 0xff }
}

/**
 * First seeding turn of `VaultFlameTraps.act`: a cooldown at or below zero is
 * reset to `after` before the decrement. Later seeds follow every `after`
 * turns, each adding `triggers` to the blob. Every pinned room has
 * `triggers <= after`, so at most one seed is live at a time.
 */
function firstSeed({ initial, after }: FlameTrap) {
  return initial >= 1 ? initial : after
}

/** `evolve()` burns the cell on the `triggers` turns after each seed. */
export function flameFires(trap: FlameTrap, turn: number) {
  const first = firstSeed(trap)
  if (turn <= first) return false
  const since = (turn - 1 - first) % trap.after
  return since < trap.triggers
}

/** The blob holds volume (the warning pour) from a seed until its last burn. */
export function flameWarns(trap: FlameTrap, turn: number) {
  const first = firstSeed(trap)
  if (turn < first) return false
  return (turn - first) % trap.after < trap.triggers
}

/**
 * `VaultLaser.act`: fires when `curCooldown` reaches zero, `shots` turns in a
 * row, then waits `interval` turns from the last shot to the next.
 */
export function laserFires(beam: MapLaserBeam, turn: number) {
  const first = Math.max(1, beam.cooldown)
  if (turn < first) return false
  const period = Math.max(1, beam.shots - 1 + beam.interval)
  return (turn - first) % period < beam.shots
}

/** `curCooldown == 1 && giveWarning` after turn `turn` acts. */
export function laserWarns(beam: MapLaserBeam, turn: number) {
  return beam.warning && turn >= 1 && laserFires(beam, turn + 1)
}
