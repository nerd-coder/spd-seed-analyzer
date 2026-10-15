import { type ItemId, itemClassName } from '@/generated/game-ids'
import { ItemClass, parseItemClass } from '@/lib/item-class'
import { ItemCategory, parseItemCategory } from '@/lib/item-kind'

const NOTABLE_CATEGORIES = new Set<ItemCategory>([
  ItemCategory.Weapon,
  ItemCategory.Armor,
  ItemCategory.Missile,
  ItemCategory.Wand,
  ItemCategory.Ring,
  ItemCategory.Artifact,
  ItemCategory.Trinket,
])

/** Gear whose report category is `other` still belongs with equipment. */
const NOTABLE_CLASSES = new Set<ItemClass>([
  ItemClass.AlchemistsToolkit,
  ItemClass.CapeOfThorns,
  ItemClass.ChaliceOfBlood,
  ItemClass.CloakOfShadows,
  ItemClass.DriedRose,
  ItemClass.EtherealChains,
  ItemClass.HolyTome,
  ItemClass.HornOfPlenty,
  ItemClass.LloydsBeacon,
  ItemClass.MasterThievesArmband,
  ItemClass.SandalsOfNature,
  ItemClass.TalismanOfForesight,
  ItemClass.TimekeepersHourglass,
  ItemClass.UnstableSpellbook,
  ItemClass.AssassinsBlade,
  ItemClass.BattleAxe,
  ItemClass.Crossbow,
  ItemClass.Cudgel,
  ItemClass.Dagger,
  ItemClass.Dirk,
  ItemClass.Flail,
  ItemClass.Gauntlet,
  ItemClass.Glaive,
  ItemClass.Gloves,
  ItemClass.Greataxe,
  ItemClass.Greatshield,
  ItemClass.Greatsword,
  ItemClass.HandAxe,
  ItemClass.Katana,
  ItemClass.Longsword,
  ItemClass.Mace,
  ItemClass.MagesStaff,
  ItemClass.Quarterstaff,
  ItemClass.Rapier,
  ItemClass.RoundShield,
  ItemClass.RunicBlade,
  ItemClass.Sai,
  ItemClass.Scimitar,
  ItemClass.Shortsword,
  ItemClass.Sickle,
  ItemClass.Spear,
  ItemClass.Sword,
  ItemClass.WarHammer,
  ItemClass.WarScythe,
  ItemClass.Whip,
  ItemClass.WornShortsword,
  ItemClass.Pickaxe,
  ItemClass.Bolas,
  ItemClass.FishingSpear,
  ItemClass.ForceCube,
  ItemClass.HeavyBoomerang,
  ItemClass.Javelin,
  ItemClass.Kunai,
  ItemClass.Shuriken,
  ItemClass.ThrowingClub,
  ItemClass.ThrowingHammer,
  ItemClass.ThrowingKnife,
  ItemClass.ThrowingSpear,
  ItemClass.ThrowingSpike,
  ItemClass.ThrowingStone,
  ItemClass.Tomahawk,
  ItemClass.Trident,
  ItemClass.Dart,
  ItemClass.RotDart,
  ItemClass.IncendiaryDart,
  ItemClass.AdrenalineDart,
  ItemClass.HealingDart,
  ItemClass.ChillingDart,
  ItemClass.ShockingDart,
  ItemClass.PoisonDart,
  ItemClass.CleansingDart,
  ItemClass.ParalyticDart,
  ItemClass.HolyDart,
  ItemClass.DisplacingDart,
  ItemClass.BlindingDart,
  ItemClass.ShatteredPot,
  ItemClass.ClothArmor,
  ItemClass.ClericArmor,
  ItemClass.DuelistArmor,
  ItemClass.HuntressArmor,
  ItemClass.LeatherArmor,
  ItemClass.MageArmor,
  ItemClass.MailArmor,
  ItemClass.PlateArmor,
  ItemClass.RogueArmor,
  ItemClass.ScaleArmor,
  ItemClass.WarriorArmor,
  ItemClass.WandOfBlastWave,
  ItemClass.WandOfCorrosion,
  ItemClass.WandOfCorruption,
  ItemClass.WandOfDisintegration,
  ItemClass.WandOfFireblast,
  ItemClass.WandOfFrost,
  ItemClass.WandOfLightning,
  ItemClass.WandOfLivingEarth,
  ItemClass.WandOfMagicMissile,
  ItemClass.WandOfPrismaticLight,
  ItemClass.WandOfRegrowth,
  ItemClass.WandOfTransfusion,
  ItemClass.WandOfWarding,
  ItemClass.RingOfAccuracy,
  ItemClass.RingOfArcana,
  ItemClass.RingOfElements,
  ItemClass.RingOfEnergy,
  ItemClass.RingOfEvasion,
  ItemClass.RingOfForce,
  ItemClass.RingOfFuror,
  ItemClass.RingOfHaste,
  ItemClass.RingOfMight,
  ItemClass.RingOfSharpshooting,
  ItemClass.RingOfTenacity,
  ItemClass.RingOfWealth,
  ItemClass.TrinketCatalyst,
  ItemClass.ChaoticCenser,
  ItemClass.CrackedSpyglass,
  ItemClass.DimensionalSundial,
  ItemClass.ExoticCrystals,
  ItemClass.EyeOfNewt,
  ItemClass.FerretTuft,
  ItemClass.MimicTooth,
  ItemClass.MossyClump,
  ItemClass.ParchmentScrap,
  ItemClass.PetrifiedSeed,
  ItemClass.RatSkull,
  ItemClass.SaltCube,
  ItemClass.ShardOfOblivion,
  ItemClass.ThirteenLeafClover,
  ItemClass.TrapMechanism,
  ItemClass.VialOfBlood,
  ItemClass.WondrousResin,
])

function notableCategory(category: string) {
  const recognized = category
    .split('/')
    .map((part) => parseItemCategory(part.trim()))
    .filter((part): part is ItemCategory => part != null)
  return (
    recognized.length > 0 &&
    recognized.every((part) => NOTABLE_CATEGORIES.has(part))
  )
}

function notableClass(className: string | null | undefined) {
  const parsed = parseItemClass(className)
  return parsed != null && NOTABLE_CLASSES.has(parsed)
}

/** Rings, wands, equipment, artifacts, and trinkets. */
export function isNotableSpawn(item: {
  label?: string
  category: string
  item?: ItemId
  candidates?: ItemId[]
}) {
  if (notableCategory(item.category)) return true
  if (item.label != null && /\bequipment\b/i.test(item.label)) return true
  if (item.item != null && notableClass(itemClassName(item.item))) return true
  return (
    item.candidates?.some((candidate) =>
      notableClass(itemClassName(candidate))
    ) ?? false
  )
}
