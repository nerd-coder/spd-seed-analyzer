/** SPD ItemSpriteSheet indices for items.png (16x16 cells, 16 cols, 256x512). */
export const ITEM_SHEET = {
  url: "/assets/sprites/items.png",
  size: 16,
  cols: 16,
  width: 256,
  height: 512,
} as const;

/** Known class_name → sheet index. Potions/scrolls/rings use appearance maps. */
export const CLASS_ICON: Record<string, number> = {
  AlchemistsToolkit: 245,
  AssassinsBlade: 124,
  BattleAxe: 121,
  BlindweedSeed: 395,
  Bolas: 152,
  Bomb: 80,
  ChaliceOfBlood: 253,
  ChaoticCenser: 286,
  ClericArmor: 186,
  CloakOfShadows: 240,
  ClothArmor: 176,
  CrackedSpyglass: 288,
  Crossbow: 125,
  Cudgel: 97,
  Dagger: 100,
  Dart: 160,
  DimensionalSundial: 277,
  Dirk: 108,
  DriedRose: 260,
  DuelistArmor: 185,
  EarthrootSeed: 392,
  EtherealChains: 248,
  ExoticCrystals: 275,
  EyeOfNewt: 282,
  FadeleafSeed: 394,
  FerretTuft: 287,
  FirebloomSeed: 385,
  FishingSpear: 148,
  Flail: 122,
  Food: 437,
  ForceCube: 159,
  Gauntlet: 133,
  Glaive: 130,
  Gloves: 98,
  Gold: 18,
  Greataxe: 131,
  Greatshield: 132,
  Greatsword: 128,
  HandAxe: 105,
  HeavyBoomerang: 156,
  HolyTome: 263,
  Honeypot: 53,
  HornOfPlenty: 249,
  HuntressArmor: 184,
  IcecapSeed: 388,
  Javelin: 154,
  Katana: 126,
  Kunai: 153,
  LeatherArmor: 177,
  Longsword: 120,
  Mace: 113,
  MageArmor: 182,
  MageroyalSeed: 391,
  MagesStaff: 101,
  MailArmor: 178,
  MasterThievesArmband: 241,
  MimicTooth: 280,
  MossyClump: 276,
  MysteryMeat: 432,
  ParchmentScrap: 273,
  Pasty: 438,
  PetrifiedSeed: 274,
  Pickaxe: 468,
  PlateArmor: 180,
  Quarterstaff: 107,
  Rapier: 99,
  RatSkull: 272,
  RogueArmor: 183,
  RotberrySeed: 384,
  RoundShield: 115,
  RunicBlade: 123,
  Sai: 116,
  SaltCube: 283,
  SandalsOfNature: 256,
  ScaleArmor: 179,
  Scimitar: 114,
  ShardOfOblivion: 285,
  Shortsword: 104,
  Shuriken: 149,
  Sickle: 109,
  SkeletonKey: 264,
  SorrowmossSeed: 390,
  Spear: 106,
  StarflowerSeed: 393,
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
  StormvineSeed: 389,
  Stylus: 49,
  SungrassSeed: 387,
  SwiftthistleSeed: 386,
  Sword: 112,
  TalismanOfForesight: 243,
  ThirteenLeafClover: 278,
  ThrowingClub: 150,
  ThrowingHammer: 158,
  ThrowingKnife: 146,
  ThrowingSpear: 151,
  ThrowingSpike: 145,
  ThrowingStone: 147,
  TimekeepersHourglass: 244,
  Tomahawk: 155,
  TrapMechanism: 279,
  Trident: 157,
  TrinketCatalyst: 70,
  UnstableSpellbook: 246,
  VialOfBlood: 284,
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
  WarHammer: 129,
  WarScythe: 134,
  WarriorArmor: 181,
  Whip: 117,
  WondrousResin: 281,
  WornShortsword: 96,
};

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
};

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
};

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
};

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
};

/** Potion class order matches Generator / Potion.colors insertion. */
const POTION_CLASSES = [
  "PotionOfStrength",
  "PotionOfHealing",
  "PotionOfMindVision",
  "PotionOfFrost",
  "PotionOfLiquidFlame",
  "PotionOfToxicGas",
  "PotionOfHaste",
  "PotionOfInvisibility",
  "PotionOfLevitation",
  "PotionOfParalyticGas",
  "PotionOfPurity",
  "PotionOfExperience",
] as const;

const SCROLL_CLASSES = [
  "ScrollOfUpgrade",
  "ScrollOfIdentify",
  "ScrollOfRemoveCurse",
  "ScrollOfMirrorImage",
  "ScrollOfRecharging",
  "ScrollOfTeleportation",
  "ScrollOfLullaby",
  "ScrollOfMagicMapping",
  "ScrollOfRage",
  "ScrollOfRetribution",
  "ScrollOfTerror",
  "ScrollOfTransmutation",
] as const;

const RING_CLASSES = [
  "RingOfAccuracy",
  "RingOfArcana",
  "RingOfElements",
  "RingOfEnergy",
  "RingOfEvasion",
  "RingOfForce",
  "RingOfFuror",
  "RingOfHaste",
  "RingOfMight",
  "RingOfSharpshooting",
  "RingOfTenacity",
  "RingOfWealth",
] as const;

export type IconResolveOpts = {
  /** Unidentified potion color / scroll rune / ring gem. */
  appearance?: string | null;
  category?: string | null;
};

/**
 * Resolve an `items.png` sheet index for a class name (and optional appearance).
 * Potions/scrolls/rings prefer seed identity appearance when provided.
 */
export function resolveItemIconIndex(
  className: string | null | undefined,
  opts: IconResolveOpts = {},
): number {
  const cat = (opts.category ?? "").toLowerCase();
  const appearance = opts.appearance?.toLowerCase() ?? null;

  if (className) {
    if (CLASS_ICON[className] != null) {
      return CLASS_ICON[className];
    }
    if (className.startsWith("PotionOf") || cat === "potion") {
      if (appearance && POTION_COLOR_ICON[appearance] != null) {
        return POTION_COLOR_ICON[appearance];
      }
      return CATEGORY_HOLDER.potion;
    }
    if (className.startsWith("ScrollOf") || cat === "scroll") {
      if (appearance && SCROLL_RUNE_ICON[appearance] != null) {
        return SCROLL_RUNE_ICON[appearance];
      }
      // uppercase rune keys too
      if (opts.appearance && SCROLL_RUNE_ICON[opts.appearance] != null) {
        return SCROLL_RUNE_ICON[opts.appearance];
      }
      return CATEGORY_HOLDER.scroll;
    }
    if (className.startsWith("RingOf") || cat === "ring") {
      if (appearance && RING_GEM_ICON[appearance] != null) {
        return RING_GEM_ICON[appearance];
      }
      return CATEGORY_HOLDER.ring;
    }
    // DoubleBomb shares bomb art
    if (className === "DoubleBomb") return CLASS_ICON.Bomb ?? CATEGORY_HOLDER.other;
  }

  if (cat && CATEGORY_HOLDER[cat] != null) {
    return CATEGORY_HOLDER[cat];
  }
  return CATEGORY_HOLDER.other;
}

/** Background style for a 16×16 cell on items.png (pixel-art, no smoothing). */
export function itemIconStyle(
  index: number,
  displayPx = 16,
): {
  width: number;
  height: number;
  backgroundImage: string;
  backgroundRepeat: "no-repeat";
  backgroundSize: string;
  backgroundPosition: string;
  imageRendering: "pixelated";
  flexShrink: number;
} {
  const col = index % ITEM_SHEET.cols;
  const row = Math.floor(index / ITEM_SHEET.cols);
  const scale = displayPx / ITEM_SHEET.size;
  return {
    width: displayPx,
    height: displayPx,
    backgroundImage: `url(${ITEM_SHEET.url})`,
    backgroundRepeat: "no-repeat",
    backgroundSize: `${ITEM_SHEET.width * scale}px ${ITEM_SHEET.height * scale}px`,
    backgroundPosition: `-${col * displayPx}px -${row * displayPx}px`,
    imageRendering: "pixelated",
    flexShrink: 0,
  };
}

export { POTION_CLASSES, SCROLL_CLASSES, RING_CLASSES };
