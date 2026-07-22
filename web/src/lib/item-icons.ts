/** SPD ItemSpriteSheet indices for items.png (16x16 cells, 16 cols, 256x512). */
export const ITEM_SHEET = {
  url: '/assets/sprites/items.png',
  size: 16,
  cols: 16,
  width: 256,
  height: 512,
} as const

/** Known class_name → sheet index. Potions/scrolls/rings use appearance maps. */
export const CLASS_ICON: Record<string, number> = {
  // Artifacts (crystal vault / random)
  AlchemistsToolkit: 245,
  CapeOfThorns: 242,
  ChaliceOfBlood: 253,
  CloakOfShadows: 240,
  DriedRose: 260,
  EtherealChains: 248,
  HolyTome: 263,
  HornOfPlenty: 249,
  LloydsBeacon: 247,
  MasterThievesArmband: 241,
  SandalsOfNature: 256,
  TalismanOfForesight: 243,
  TimekeepersHourglass: 244,
  UnstableSpellbook: 246,

  // Weapons / armor / missiles (blacksmith, statue, shops)
  AssassinsBlade: 124,
  BattleAxe: 121,
  Bolas: 152,
  Crossbow: 125,
  Cudgel: 97,
  Dagger: 100,
  Dirk: 108,
  FishingSpear: 148,
  Flail: 122,
  ForceCube: 159,
  Gauntlet: 133,
  Glaive: 130,
  Gloves: 98,
  Greataxe: 131,
  Greatshield: 132,
  Greatsword: 128,
  HandAxe: 105,
  HeavyBoomerang: 156,
  Javelin: 154,
  Katana: 126,
  Kunai: 153,
  Longsword: 120,
  Mace: 113,
  MagesStaff: 101,
  Quarterstaff: 107,
  Rapier: 99,
  RoundShield: 115,
  RunicBlade: 123,
  Sai: 116,
  Scimitar: 114,
  Shortsword: 104,
  Shuriken: 149,
  Sickle: 109,
  Spear: 106,
  Sword: 112,
  ThrowingClub: 150,
  ThrowingHammer: 158,
  ThrowingKnife: 146,
  ThrowingSpear: 151,
  ThrowingSpike: 145,
  ThrowingStone: 147,
  Tomahawk: 155,
  Trident: 157,
  WarHammer: 129,
  WarScythe: 134,
  Whip: 117,
  WornShortsword: 96,

  ClothArmor: 176,
  ClericArmor: 186,
  DuelistArmor: 185,
  HuntressArmor: 184,
  LeatherArmor: 177,
  MageArmor: 182,
  MailArmor: 178,
  PlateArmor: 180,
  RogueArmor: 183,
  ScaleArmor: 179,
  WarriorArmor: 181,

  // Darts (shop tipped)
  Dart: 160,
  RotDart: 161,
  IncendiaryDart: 162,
  AdrenalineDart: 163,
  HealingDart: 164,
  ChillingDart: 165,
  ShockingDart: 166,
  PoisonDart: 167,
  CleansingDart: 168,
  ParalyticDart: 169,
  HolyDart: 170,
  DisplacingDart: 171,
  BlindingDart: 172,

  // Wands
  WandOfBlastWave: 216,
  WandOfCorrosion: 214,
  WandOfCorruption: 217,
  WandOfDisintegration: 212,
  WandOfFireblast: 209,
  WandOfFrost: 210,
  WandOfLightning: 211,
  WandOfLivingEarth: 215,
  WandOfMagicMissile: 208,
  WandOfPrismaticLight: 213,
  WandOfRegrowth: 219,
  WandOfTransfusion: 220,
  WandOfWarding: 218,

  // Consumables / misc (shop + rooms)
  Alchemize: 423,
  Ankh: 48,
  Bomb: 80,
  CrystalKey: 57,
  DoubleBomb: 81,
  EnergyCrystal: 19,
  Food: 437,
  Gold: 18,
  GuidePage: 496,
  Honeypot: 53,
  IronKey: 55,
  MagicalHolster: 485,
  MysteryMeat: 432,
  Pasty: 438,
  PotionBandolier: 484,
  ScrollHolder: 483,
  SkeletonKey: 264,
  SmallRation: 435,
  Stylus: 49,
  TrinketCatalyst: 70,
  VelvetPouch: 482,
  Waterskin: 480,

  // Seeds
  BlindweedSeed: 395,
  EarthrootSeed: 392,
  FadeleafSeed: 394,
  FirebloomSeed: 385,
  IcecapSeed: 388,
  MageroyalSeed: 391,
  RotberrySeed: 384,
  SorrowmossSeed: 390,
  StarflowerSeed: 393,
  StormvineSeed: 389,
  SungrassSeed: 387,
  SwiftthistleSeed: 386,
  // Garden / secret-garden specials (seedpod/dewcatcher have plant tiles only)
  BlandfruitBushSeed: 440, // BLANDFRUIT food icon as stand-in
  SeedpodSeed: 393, // starflower stand-in
  DewcatcherSeed: 387, // sungrass stand-in

  // Stones
  StoneOfAggression: 336,
  StoneOfAugmentation: 337,
  StoneOfBlast: 339,
  StoneOfBlink: 340,
  StoneOfClairvoyance: 341,
  StoneOfDeepSleep: 342,
  StoneOfDetectMagic: 343,
  StoneOfEnchantment: 344,
  StoneOfFear: 338,
  StoneOfFlock: 345,
  StoneOfIntuition: 346,
  StoneOfShock: 347,

  // Trinkets
  ChaoticCenser: 286,
  CrackedSpyglass: 288,
  DimensionalSundial: 277,
  ExoticCrystals: 275,
  EyeOfNewt: 282,
  FerretTuft: 287,
  MimicTooth: 280,
  MossyClump: 276,
  ParchmentScrap: 273,
  PetrifiedSeed: 274,
  Pickaxe: 468,
  RatSkull: 272,
  SaltCube: 283,
  ShardOfOblivion: 285,
  ThirteenLeafClover: 278,
  TrapMechanism: 279,
  VialOfBlood: 284,
  WondrousResin: 281,
}

export const POTION_COLOR_ICON: Record<string, number> = {
  crimson: 352,
  amber: 353,
  golden: 354,
  jade: 355,
  turquoise: 356,
  azure: 357,
  indigo: 358,
  magenta: 359,
  bistre: 360,
  charcoal: 361,
  silver: 362,
  ivory: 363,
}

export const SCROLL_RUNE_ICON: Record<string, number> = {
  KAUNAN: 304,
  kaunan: 304,
  SOWILO: 305,
  sowilo: 305,
  LAGUZ: 306,
  laguz: 306,
  YNGVI: 307,
  yngvi: 307,
  GYFU: 308,
  gyfu: 308,
  RAIDO: 309,
  raido: 309,
  ISAZ: 310,
  isaz: 310,
  MANNAZ: 311,
  mannaz: 311,
  NAUDIZ: 312,
  naudiz: 312,
  BERKANAN: 313,
  berkanan: 313,
  ODAL: 314,
  odal: 314,
  TIWAZ: 315,
  tiwaz: 315,
}

export const RING_GEM_ICON: Record<string, number> = {
  garnet: 224,
  ruby: 225,
  topaz: 226,
  emerald: 227,
  onyx: 228,
  opal: 229,
  tourmaline: 230,
  sapphire: 231,
  amethyst: 232,
  quartz: 233,
  agate: 234,
  diamond: 235,
}

export const CATEGORY_HOLDER: Record<string, number> = {
  weapon: 1,
  armor: 2,
  missile: 3,
  wand: 4,
  ring: 5,
  artifact: 6,
  trinket: 7,
  food: 8,
  potion: 10,
  seed: 11,
  scroll: 12,
  stone: 13,
  gold: 18,
  other: 0,
}

/** Potion class order matches Generator / Potion.colors insertion. */
const POTION_CLASSES = [
  'PotionOfStrength',
  'PotionOfHealing',
  'PotionOfMindVision',
  'PotionOfFrost',
  'PotionOfLiquidFlame',
  'PotionOfToxicGas',
  'PotionOfHaste',
  'PotionOfInvisibility',
  'PotionOfLevitation',
  'PotionOfParalyticGas',
  'PotionOfPurity',
  'PotionOfExperience',
] as const

const SCROLL_CLASSES = [
  'ScrollOfUpgrade',
  'ScrollOfIdentify',
  'ScrollOfRemoveCurse',
  'ScrollOfMirrorImage',
  'ScrollOfRecharging',
  'ScrollOfTeleportation',
  'ScrollOfLullaby',
  'ScrollOfMagicMapping',
  'ScrollOfRage',
  'ScrollOfRetribution',
  'ScrollOfTerror',
  'ScrollOfTransmutation',
] as const

const RING_CLASSES = [
  'RingOfAccuracy',
  'RingOfArcana',
  'RingOfElements',
  'RingOfEnergy',
  'RingOfEvasion',
  'RingOfForce',
  'RingOfFuror',
  'RingOfHaste',
  'RingOfMight',
  'RingOfSharpshooting',
  'RingOfTenacity',
  'RingOfWealth',
] as const

export type IconResolveOpts = {
  /** Unidentified potion color / scroll rune / ring gem. */
  appearance?: string | null
  category?: string | null
}

/**
 * Resolve an `items.png` sheet index for a class name (and optional appearance).
 * Potions/scrolls/rings prefer seed identity appearance when provided.
 */
export function resolveItemIconIndex(
  className: string | null | undefined,
  opts: IconResolveOpts = {}
): number {
  const cat = (opts.category ?? '').toLowerCase()
  const appearance = opts.appearance?.toLowerCase() ?? null

  if (className) {
    if (CLASS_ICON[className] != null) {
      return CLASS_ICON[className]
    }
    if (className.startsWith('PotionOf') || cat === 'potion') {
      if (appearance && POTION_COLOR_ICON[appearance] != null) {
        return POTION_COLOR_ICON[appearance]
      }
      return CATEGORY_HOLDER.potion
    }
    if (className.startsWith('ScrollOf') || cat === 'scroll') {
      if (appearance && SCROLL_RUNE_ICON[appearance] != null) {
        return SCROLL_RUNE_ICON[appearance]
      }
      // uppercase rune keys too
      if (opts.appearance && SCROLL_RUNE_ICON[opts.appearance] != null) {
        return SCROLL_RUNE_ICON[opts.appearance]
      }
      return CATEGORY_HOLDER.scroll
    }
    if (className.startsWith('RingOf') || cat === 'ring') {
      if (appearance && RING_GEM_ICON[appearance] != null) {
        return RING_GEM_ICON[appearance]
      }
      return CATEGORY_HOLDER.ring
    }
  }

  if (cat && CATEGORY_HOLDER[cat] != null) {
    return CATEGORY_HOLDER[cat]
  }
  return CATEGORY_HOLDER.other
}

/** Background style for a 16×16 cell on items.png (pixel-art, no smoothing). */
export function itemIconStyle(
  index: number,
  displayPx = 16
): {
  width: number
  height: number
  backgroundImage: string
  backgroundRepeat: 'no-repeat'
  backgroundSize: string
  backgroundPosition: string
  imageRendering: 'pixelated'
  flexShrink: number
} {
  const col = index % ITEM_SHEET.cols
  const row = Math.floor(index / ITEM_SHEET.cols)
  const scale = displayPx / ITEM_SHEET.size
  return {
    width: displayPx,
    height: displayPx,
    backgroundImage: `url(${ITEM_SHEET.url})`,
    backgroundRepeat: 'no-repeat',
    backgroundSize: `${ITEM_SHEET.width * scale}px ${ITEM_SHEET.height * scale}px`,
    backgroundPosition: `-${col * displayPx}px -${row * displayPx}px`,
    imageRendering: 'pixelated',
    flexShrink: 0,
  }
}

export { POTION_CLASSES, RING_CLASSES, SCROLL_CLASSES }
