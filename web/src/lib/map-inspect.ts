import {
  itemClassName,
  mobClassName,
  plantClassName,
  trapClassName,
} from '@/generated/game-ids'
import { Terrain } from '@/lib/dungeon-tile-visuals'
import { itemLabel, mobLabel } from '@/lib/item-title'
import catalog from '@/lib/map-object-catalog.json'
import type {
  FloorMap,
  MapHeapItem,
  MapMob,
  MapPlant,
  MapTransition,
  MapTrap,
} from '@/lib/spd-wasm'

type ObjectText = { name: string; desc: string }

/** English examine text for mobs, traps, and plants at the pinned commit. */
const OBJECT_TEXT: Record<string, ObjectText> = catalog

/** Subclasses without their own message keys share the parent's text. */
const TEXT_FALLBACK: Record<string, string> = {
  RedShaman: 'Shaman',
  BlueShaman: 'Shaman',
  PurpleShaman: 'Shaman',
  VaultGhoul: 'Ghoul',
  VaultGolem: 'Golem',
}

const HEAP_PLACES: Record<string, string> = {
  chest: 'In a chest',
  locked_chest: 'In a locked chest',
  crystal_chest: 'In a crystal chest',
  tomb: 'In a tomb',
  skeleton: 'On skeletal remains',
  remains: 'On remains',
  for_sale: 'For sale',
}

export type MapInspectEntry =
  | { kind: 'item'; item: MapHeapItem; place?: string }
  | { kind: 'mob'; mob: MapMob }
  | { kind: 'loot'; label: string }
  | { kind: 'trap'; trap: MapTrap }
  | { kind: 'plant'; plant: MapPlant }
  | { kind: 'transition'; transition: MapTransition }

export type MapInspection = {
  cell: number
  entries: MapInspectEntry[]
  /** Run history can change this cell's heap or mob. */
  uncertain: boolean
  /** Behind a secret door, or a hidden trap. */
  hidden: boolean
  secretDoor: boolean
}

export function inspectCell(map: FloorMap, cell: number): MapInspection {
  const entries: MapInspectEntry[] = []
  for (const transition of map.transitions) {
    if (transition.cell === cell)
      entries.push({ kind: 'transition', transition })
  }
  for (const mob of map.mobs) {
    if (mob.cell === cell) entries.push({ kind: 'mob', mob })
  }
  for (const heap of map.heaps) {
    if (heap.cell !== cell) continue
    for (const item of heap.items) {
      entries.push({ kind: 'item', item, place: HEAP_PLACES[heap.heap_type] })
    }
  }
  const covered = entries.length > 0
  for (const marker of map.markers) {
    if (marker.cell === cell && marker.kind === 'item' && !covered) {
      entries.push({ kind: 'loot', label: marker.label })
    }
  }
  for (const trap of map.traps) {
    if (trap.cell === cell) entries.push({ kind: 'trap', trap })
  }
  for (const plant of map.plants) {
    if (plant.cell === cell) entries.push({ kind: 'plant', plant })
  }
  const hiddenCell = map.hidden_cells?.includes(cell) ?? false
  const hiddenTrap = map.traps.some(
    (trap) => trap.cell === cell && !trap.visible
  )
  return {
    cell,
    entries,
    uncertain: map.uncertain_cells?.includes(cell) ?? false,
    hidden: hiddenCell || hiddenTrap,
    secretDoor: hiddenCell && map.tiles[cell] === Terrain.SECRET_DOOR,
  }
}

export function hasInspection(inspection: MapInspection) {
  return inspection.entries.length > 0 || inspection.secretDoor
}

export function objectText(className: string): ObjectText | null {
  return (
    OBJECT_TEXT[className] ??
    OBJECT_TEXT[TEXT_FALLBACK[className] ?? ''] ??
    null
  )
}

function capitalize(text: string) {
  return text.charAt(0).toLocaleUpperCase() + text.slice(1)
}

export function mobTitle(mob: MapMob): string {
  const text = objectText(mobClassName(mob.mob))
  return text ? capitalize(text.name) : mobLabel(mob.mob)
}

export function itemTitle(item: MapHeapItem): string {
  const title = itemLabel(item.item)
  const level =
    item.level > 0 ? ` +${item.level}` : item.level < 0 ? ` ${item.level}` : ''
  const quantity = item.quantity > 1 ? `${item.quantity}× ` : ''
  return `${quantity}${title}${level}`
}

export function itemClass(item: MapHeapItem): string {
  return itemClassName(item.item)
}

export function trapTitle(trap: MapTrap): string {
  const text = objectText(trapClassName(trap.trap))
  return text ? capitalize(text.name) : 'Trap'
}

export function plantTitle(plant: MapPlant): string {
  const text = objectText(plantClassName(plant.plant))
  return text ? capitalize(text.name) : 'Plant'
}

export function transitionTitle(transition: MapTransition): string {
  switch (transition.type) {
    case 'SURFACE':
      return 'Dungeon entrance'
    case 'REGULAR_ENTRANCE':
      return `Stairs up to floor ${transition.dest_depth}`
    case 'REGULAR_EXIT':
      return `Stairs down to floor ${transition.dest_depth}`
    case 'BRANCH_ENTRANCE':
      return 'Way back up'
    case 'BRANCH_EXIT':
      return 'Quest branch entrance'
    default:
      return 'Transition'
  }
}
