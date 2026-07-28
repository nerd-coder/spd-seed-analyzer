import type { MapTrinketProfile, TrinketSelectionReport } from '@/lib/spd-wasm'

export type TrinketOption = {
  value: MapTrinketProfile
  className: string | null
  label: string
  description: string
}

export const TRINKET_OPTIONS: TrinketOption[] = [
  {
    value: 'mossy_clump',
    className: 'MossyClump',
    label: 'Mossy Clump',
    description:
      'Can replace ordinary floor feelings with grass or water, changing layouts and later generation.',
  },
  {
    value: 'trap_mechanism',
    className: 'TrapMechanism',
    label: 'Trap Mechanism',
    description:
      'Can create trap or chasm floors and reveal more generated traps as it is upgraded.',
  },
  {
    value: 'mimic_tooth',
    className: 'MimicTooth',
    label: 'Mimic Tooth',
    description:
      'Raises generated Mimic chances and can change loot-deck history and Sad Ghost rewards.',
  },
  {
    value: 'no_map_affecting_trinkets',
    className: null,
    label: 'No tracked trinket effect',
    description:
      'Stops these three modeled effects after a transmutation. Other trinket effects remain outside this profile.',
  },
]

export function trinketOption(value: MapTrinketProfile): TrinketOption {
  return (
    TRINKET_OPTIONS.find((option) => option.value === value) ??
    TRINKET_OPTIONS[TRINKET_OPTIONS.length - 1]
  )
}

export function trinketAcquisitionLabel(
  option: TrinketOption,
  selection: TrinketSelectionReport
): string {
  if (!option.className) return 'Player-selected history'
  if (selection.catalyst_options.includes(option.className)) {
    return 'Catalyst offer'
  }
  const index = selection.transmutation_sequence.indexOf(option.className)
  return index >= 0 ? `Transmutation ${index + 1}` : 'Later transmutation'
}
