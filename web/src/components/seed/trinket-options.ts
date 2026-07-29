import type { TrinketKind, TrinketSelectionReport } from '@/lib/spd-wasm'

export type TrinketOption = {
  value: TrinketKind
  className: string
  label: string
  description: string
}

export const TRINKET_OPTIONS: TrinketOption[] = [
  {
    value: 'rat_skull',
    className: 'RatSkull',
    label: 'Rat Skull',
    description: 'Changes statue and Crystal Vault generation while held.',
  },
  {
    value: 'parchment_scrap',
    className: 'ParchmentScrap',
    label: 'Parchment Scrap',
    description: 'Can preserve effects on selected quest rewards.',
  },
  {
    value: 'petrified_seed',
    className: 'PetrifiedSeed',
    label: 'Petrified Seed',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'exotic_crystals',
    className: 'ExoticCrystals',
    label: 'Exotic Crystals',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'mossy_clump',
    className: 'MossyClump',
    label: 'Mossy Clump',
    description: 'Can replace ordinary floor feelings with grass or water.',
  },
  {
    value: 'dimensional_sundial',
    className: 'DimensionalSundial',
    label: 'Dimensional Sundial',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'thirteen_leaf_clover',
    className: 'ThirteenLeafClover',
    label: 'Thirteen-leaf Clover',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'trap_mechanism',
    className: 'TrapMechanism',
    label: 'Trap Mechanism',
    description: 'Can create trap or chasm floors and reveal generated traps.',
  },
  {
    value: 'mimic_tooth',
    className: 'MimicTooth',
    label: 'Mimic Tooth',
    description: 'Raises generated Mimic chances and can shift item decks.',
  },
  {
    value: 'wondrous_resin',
    className: 'WondrousResin',
    label: 'Wondrous Resin',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'eye_of_newt',
    className: 'EyeOfNewt',
    label: 'Eye of Newt',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'salt_cube',
    className: 'SaltCube',
    label: 'Salt Cube',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'vial_of_blood',
    className: 'VialOfBlood',
    label: 'Vial of Blood',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'shard_of_oblivion',
    className: 'ShardOfOblivion',
    label: 'Shard of Oblivion',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'chaotic_censer',
    className: 'ChaoticCenser',
    label: 'Chaotic Censer',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'ferret_tuft',
    className: 'FerretTuft',
    label: 'Ferret Tuft',
    description: 'Recorded as part of the held-trinket history.',
  },
  {
    value: 'cracked_spyglass',
    className: 'CrackedSpyglass',
    label: 'Cracked Spyglass',
    description: 'Adds hidden level loot and can alter later artifact state.',
  },
]

export function trinketOption(value: TrinketKind): TrinketOption {
  return (
    TRINKET_OPTIONS.find((option) => option.value === value) ??
    TRINKET_OPTIONS[0]
  )
}

export function trinketKindFromClassName(
  className: string
): TrinketKind | null {
  return (
    TRINKET_OPTIONS.find((option) => option.className === className)?.value ??
    null
  )
}

export function trinketAcquisitionLabel(
  option: TrinketOption,
  selection: TrinketSelectionReport
): string {
  if (selection.catalyst_options.includes(option.className)) {
    return 'Catalyst offer'
  }
  const index = selection.transmutation_sequence.indexOf(option.className)
  return index >= 0 ? `Transmutation ${index + 1}` : 'Later transmutation'
}
