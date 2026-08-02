/**
 * Human-readable labels for item sources (seed-finder UX).
 * Backend strings stay machine-ish; UI maps them here.
 */

/** Room / quest / spawn tags that appear in `item.source`. */
const SOURCE_LABELS: Record<string, string> = {
  // Guaranteed / main drops
  forced: 'Guaranteed',
  heap: 'Floor drop',
  hidden: 'Hidden',

  // Heap types (createItems + special rooms)
  chest: 'Chest',
  locked_chest: 'Locked chest',
  crystal_chest: 'Crystal chest',
  ebony_chest: 'Ebony chest',
  mimic: 'Mimic',
  golden_mimic: 'Golden mimic',
  crystal_mimic: 'Crystal mimic',
  skeleton: 'Skeleton',
  tomb: 'Tomb',
  statue: 'Statue',
  remains: 'Remains',

  // Shops & special rooms
  ShopRoom: 'Shop',
  CryptRoom: 'Crypt',
  ArmoryRoom: 'Armory',
  LibraryRoom: 'Library',
  TreasuryRoom: 'Treasury',
  PoolRoom: 'Pool',
  SuspiciousChestRoom: 'Suspicious Chest',
  equipment: 'Equipment fallback',
  gold: 'Gold fallback',
  mimic_reward: 'Mimic bonus',
  hidden_reward: 'Hidden reward',
  bomb: 'Bomb reward',
  prize: 'Generator reward',
  gold_tombs: 'Gold tombs',
  StorageRoom: 'Storage',
  RunestoneRoom: 'Runestone',
  LaboratoryRoom: 'Laboratory',
  StatueRoom: 'Statue room',
  MassGraveRoom: 'Mass grave',
  GrassyGraveRoom: 'Grassy grave',
  CrystalVaultRoom: 'Crystal vault',
  CrystalChoiceRoom: 'Crystal choice',
  CrystalPathRoom: 'Crystal path',
  BlacksmithRoom: 'Blacksmith forge',
  GardenRoom: 'Garden',
  MagicWellRoom: 'Magic well',
  PitRoom: 'Pit room',
  SentryRoom: 'Sentry room',
  TrapsRoom: 'Traps room',
  MagicalFireRoom: 'Magical fire',
  SacrificeRoom: 'Sacrifice room',
  ToxicGasRoom: 'Toxic gas room',

  // Secret rooms
  SecretLibraryRoom: 'Secret library',
  SecretRunestoneRoom: 'Secret runestone',
  SecretArtilleryRoom: 'Secret artillery',
  SecretLaboratoryRoom: 'Secret lab',
  SecretLarderRoom: 'Secret larder',
  SecretHoardRoom: 'Secret hoard',
  SecretHoneypotRoom: 'Secret honeypot',
  SecretMazeRoom: 'Secret maze',
  SecretSummoningRoom: 'Secret summoning',
  SecretChestChasmRoom: 'Secret chest chasm',
  SecretGardenRoom: 'Secret garden',
  SecretWellRoom: 'Secret well',

  // Heap-like tags from specials
  plant: 'Plant',
  well: 'Well',
  sacrificial: 'Sacrificial fire',

  // Quest rewards
  'Ghost.Quest': 'Ghost quest',
  'Wandmaker.Quest': 'Wandmaker quest',
  'Blacksmith.Quest': 'Blacksmith rewards',
  'Imp.Quest': 'Imp quest',
}

/** Title-case snake/camel fragments when no explicit map entry. */
function fallbackLabel(raw: string): string {
  if (!raw) return raw
  // already spaced human text
  if (raw.includes(' ')) return raw
  // snake_case
  if (raw.includes('_')) {
    return raw
      .split('_')
      .filter(Boolean)
      .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
      .join(' ')
  }
  // PascalCase / dotted Class.Name → strip Room / Quest tails lightly
  const base = raw
    .replace(/\.Quest$/, ' quest')
    .replace(/Room$/, '')
    .replace(/([a-z])([A-Z])/g, '$1 $2')
  return base.charAt(0).toUpperCase() + base.slice(1)
}

/**
 * Format a backend `item.source` string for display.
 * Handles `heap_type:RoomName` and `RoomName:mimic` styles.
 */
export function formatItemSource(
  source: string | null | undefined
): string | null {
  if (!source) return null
  if (source === 'guaranteed floor spawn') return null
  const parts = source.split(':').filter(Boolean)
  if (parts.length === 0) return null

  const labels = parts.map((p) => SOURCE_LABELS[p] ?? fallbackLabel(p))

  // Drop redundant second label when identical (e.g. mimic:mimic)
  const unique: string[] = []
  for (const l of labels) {
    if (unique[unique.length - 1] !== l) unique.push(l)
  }

  // "Locked chest · Floor drop" is noisy — prefer the more specific first part
  // when second is generic floor drop / same room context.
  if (unique.length === 2 && unique[1] === 'Floor drop') {
    return unique[0]
  }

  return unique.join(' · ')
}

/** Whether a source is high-value for seed finding (quests / crystal / shop / etc.). */
export function isHighlightSource(source: string | null | undefined): boolean {
  if (!source) return false
  return /Quest|Crystal|Shop|Blacksmith|Crypt|Armory|Pool|Suspicious|Treasury|Library|Statue|Secret|MassGrave|GrassyGrave|Laboratory|Storage|Runestone|Pit|Garden|Sentry|Traps|MagicalFire|Sacrifice|ToxicGas|MagicWell/i.test(
    source
  )
}
