//! Static `Generator.Category` tables (class lists + default probs).

use super::Category;
use crate::items::model::ItemCategory;

#[derive(Debug, Clone)]
pub struct CategoryDef {
    pub category: Category,
    pub first_prob: f32,
    pub second_prob: f32,
    pub classes: &'static [&'static str],
    pub default_probs: Option<&'static [f32]>,
    pub default_probs2: Option<&'static [f32]>,
    pub item_category: ItemCategory,
}

impl Category {
    pub fn def(self) -> CategoryDef {
        match self {
            Category::Trinket => CategoryDef {
                category: self,
                first_prob: 0.0,
                second_prob: 0.0,
                classes: TRINKET,
                default_probs: Some(TRINKET_PROBS),
                default_probs2: None,
                item_category: ItemCategory::Trinket,
            },
            Category::Weapon => CategoryDef {
                category: self,
                first_prob: 2.0,
                second_prob: 2.0,
                classes: &[],
                default_probs: None,
                default_probs2: None,
                item_category: ItemCategory::Weapon,
            },
            Category::WepT1 => wep_tier(self, WEP_T1, WEP_T1_PROBS),
            Category::WepT2 => wep_tier(self, WEP_T2, WEP_T2_PROBS),
            Category::WepT3 => wep_tier(self, WEP_T3, WEP_T3_PROBS),
            Category::WepT4 => wep_tier(self, WEP_T4, WEP_T4_PROBS),
            Category::WepT5 => wep_tier(self, WEP_T5, WEP_T5_PROBS),
            Category::Armor => CategoryDef {
                category: self,
                first_prob: 2.0,
                second_prob: 1.0,
                classes: ARMOR,
                default_probs: None, // uses floorSetTierProbs, not deck
                default_probs2: None,
                item_category: ItemCategory::Armor,
            },
            Category::Missile => CategoryDef {
                category: self,
                first_prob: 1.0,
                second_prob: 2.0,
                classes: &[],
                default_probs: None,
                default_probs2: None,
                item_category: ItemCategory::Missile,
            },
            Category::MisT1 => mis_tier(self, MIS_T1, MIS_T1_PROBS),
            Category::MisT2 => mis_tier(self, MIS_T2, MIS_T2_PROBS),
            Category::MisT3 => mis_tier(self, MIS_T3, MIS_T3_PROBS),
            Category::MisT4 => mis_tier(self, MIS_T4, MIS_T4_PROBS),
            Category::MisT5 => mis_tier(self, MIS_T5, MIS_T5_PROBS),
            Category::Wand => CategoryDef {
                category: self,
                first_prob: 1.0,
                second_prob: 1.0,
                classes: WAND,
                default_probs: Some(WAND_PROBS),
                default_probs2: None,
                item_category: ItemCategory::Wand,
            },
            Category::Ring => CategoryDef {
                category: self,
                first_prob: 1.0,
                second_prob: 0.0,
                classes: RING,
                default_probs: Some(RING_PROBS),
                default_probs2: None,
                item_category: ItemCategory::Ring,
            },
            Category::Artifact => CategoryDef {
                category: self,
                first_prob: 0.0,
                second_prob: 1.0,
                classes: ARTIFACT,
                default_probs: Some(ARTIFACT_PROBS),
                default_probs2: None,
                item_category: ItemCategory::Artifact,
            },
            Category::Food => CategoryDef {
                category: self,
                first_prob: 0.0,
                second_prob: 0.0,
                classes: FOOD,
                default_probs: Some(FOOD_PROBS),
                default_probs2: None,
                item_category: ItemCategory::Food,
            },
            Category::Potion => CategoryDef {
                category: self,
                first_prob: 8.0,
                second_prob: 8.0,
                classes: POTION,
                default_probs: Some(POTION_PROBS),
                default_probs2: Some(POTION_PROBS2),
                item_category: ItemCategory::Potion,
            },
            Category::Seed => CategoryDef {
                category: self,
                first_prob: 1.0,
                second_prob: 1.0,
                classes: SEED,
                default_probs: Some(SEED_PROBS),
                default_probs2: None,
                item_category: ItemCategory::Seed,
            },
            Category::Scroll => CategoryDef {
                category: self,
                first_prob: 8.0,
                second_prob: 8.0,
                classes: SCROLL,
                default_probs: Some(SCROLL_PROBS),
                default_probs2: Some(SCROLL_PROBS2),
                item_category: ItemCategory::Scroll,
            },
            Category::Stone => CategoryDef {
                category: self,
                first_prob: 1.0,
                second_prob: 1.0,
                classes: STONE,
                default_probs: Some(STONE_PROBS),
                default_probs2: None,
                item_category: ItemCategory::Stone,
            },
            Category::Gold => CategoryDef {
                category: self,
                first_prob: 10.0,
                second_prob: 10.0,
                classes: GOLD,
                // Java keeps probs=[1] without a deck seed (defaultProbs == null).
                default_probs: None,
                default_probs2: None,
                item_category: ItemCategory::Gold,
            },
        }
    }
}

fn wep_tier(cat: Category, classes: &'static [&'static str], probs: &'static [f32]) -> CategoryDef {
    CategoryDef {
        category: cat,
        first_prob: 0.0,
        second_prob: 0.0,
        classes,
        default_probs: Some(probs),
        default_probs2: None,
        item_category: ItemCategory::Weapon,
    }
}

fn mis_tier(cat: Category, classes: &'static [&'static str], probs: &'static [f32]) -> CategoryDef {
    CategoryDef {
        category: cat,
        first_prob: 0.0,
        second_prob: 0.0,
        classes,
        default_probs: Some(probs),
        default_probs2: None,
        item_category: ItemCategory::Missile,
    }
}

// --- tables ---

const POTION: &[&str] = &[
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
];
const POTION_PROBS: &[f32] = &[0., 3., 2., 1., 2., 1., 1., 1., 1., 1., 1., 1.];
const POTION_PROBS2: &[f32] = &[0., 3., 2., 2., 1., 2., 1., 1., 1., 1., 1., 0.];

const SCROLL: &[&str] = &[
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
];
const SCROLL_PROBS: &[f32] = &[0., 3., 2., 1., 2., 1., 1., 1., 1., 1., 1., 1.];
const SCROLL_PROBS2: &[f32] = &[0., 3., 2., 2., 1., 2., 1., 1., 1., 1., 1., 0.];

const SEED: &[&str] = &[
    "RotberrySeed",
    "SungrassSeed",
    "FadeleafSeed",
    "IcecapSeed",
    "FirebloomSeed",
    "SorrowmossSeed",
    "SwiftthistleSeed",
    "BlindweedSeed",
    "StormvineSeed",
    "EarthrootSeed",
    "MageroyalSeed",
    "StarflowerSeed",
];
const SEED_PROBS: &[f32] = &[0., 2., 2., 2., 2., 2., 2., 2., 2., 2., 2., 1.];

const STONE: &[&str] = &[
    "StoneOfEnchantment",
    "StoneOfIntuition",
    "StoneOfDetectMagic",
    "StoneOfFlock",
    "StoneOfShock",
    "StoneOfBlink",
    "StoneOfDeepSleep",
    "StoneOfClairvoyance",
    "StoneOfAggression",
    "StoneOfBlast",
    "StoneOfFear",
    "StoneOfAugmentation",
];
const STONE_PROBS: &[f32] = &[0., 2., 2., 2., 2., 2., 2., 2., 2., 2., 2., 0.];

const WAND: &[&str] = &[
    "WandOfMagicMissile",
    "WandOfLightning",
    "WandOfDisintegration",
    "WandOfFireblast",
    "WandOfCorrosion",
    "WandOfBlastWave",
    "WandOfLivingEarth",
    "WandOfFrost",
    "WandOfPrismaticLight",
    "WandOfWarding",
    "WandOfTransfusion",
    "WandOfCorruption",
    "WandOfRegrowth",
];
const WAND_PROBS: &[f32] = &[3.; 13];

const WEP_T1: &[&str] = &[
    "WornShortsword",
    "MagesStaff",
    "Dagger",
    "Gloves",
    "Rapier",
    "Cudgel",
];
const WEP_T1_PROBS: &[f32] = &[2., 0., 2., 2., 2., 2.];

const WEP_T2: &[&str] = &[
    "Shortsword",
    "HandAxe",
    "Spear",
    "Quarterstaff",
    "Dirk",
    "Sickle",
    "Pickaxe",
];
const WEP_T2_PROBS: &[f32] = &[2., 2., 2., 2., 2., 2., 0.];

const WEP_T3: &[&str] = &["Sword", "Mace", "Scimitar", "RoundShield", "Sai", "Whip"];
const WEP_T3_PROBS: &[f32] = &[2., 2., 2., 2., 2., 2.];

const WEP_T4: &[&str] = &[
    "Longsword",
    "BattleAxe",
    "Flail",
    "RunicBlade",
    "AssassinsBlade",
    "Crossbow",
    "Katana",
];
const WEP_T4_PROBS: &[f32] = &[2., 2., 2., 2., 2., 2., 2.];

const WEP_T5: &[&str] = &[
    "Greatsword",
    "WarHammer",
    "Glaive",
    "Greataxe",
    "Greatshield",
    "Gauntlet",
    "WarScythe",
];
const WEP_T5_PROBS: &[f32] = &[2., 2., 2., 2., 2., 2., 2.];

const ARMOR: &[&str] = &[
    "ClothArmor",
    "LeatherArmor",
    "MailArmor",
    "ScaleArmor",
    "PlateArmor",
    "WarriorArmor",
    "MageArmor",
    "RogueArmor",
    "HuntressArmor",
    "DuelistArmor",
    "ClericArmor",
];

const MIS_T1: &[&str] = &["ThrowingStone", "ThrowingKnife", "ThrowingSpike", "Dart"];
const MIS_T1_PROBS: &[f32] = &[3., 3., 3., 0.];
const MIS_T2: &[&str] = &["FishingSpear", "ThrowingClub", "Shuriken"];
const MIS_T2_PROBS: &[f32] = &[3., 3., 3.];
const MIS_T3: &[&str] = &["ThrowingSpear", "Kunai", "Bolas"];
const MIS_T3_PROBS: &[f32] = &[3., 3., 3.];
const MIS_T4: &[&str] = &["Javelin", "Tomahawk", "HeavyBoomerang"];
const MIS_T4_PROBS: &[f32] = &[3., 3., 3.];
const MIS_T5: &[&str] = &["Trident", "ThrowingHammer", "ForceCube"];
const MIS_T5_PROBS: &[f32] = &[3., 3., 3.];

const FOOD: &[&str] = &["Food", "Pasty", "MysteryMeat"];
const FOOD_PROBS: &[f32] = &[4., 1., 0.];

const RING: &[&str] = &[
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
];
const RING_PROBS: &[f32] = &[3.; 12];

const ARTIFACT: &[&str] = &[
    "AlchemistsToolkit",
    "ChaliceOfBlood",
    "CloakOfShadows",
    "DriedRose",
    "EtherealChains",
    "HolyTome",
    "HornOfPlenty",
    "MasterThievesArmband",
    "SandalsOfNature",
    "SkeletonKey",
    "TalismanOfForesight",
    "TimekeepersHourglass",
    "UnstableSpellbook",
];
// CloakOfShadows and HolyTome start at 0 (hero-class exclusive)
const ARTIFACT_PROBS: &[f32] = &[1., 1., 0., 1., 1., 0., 1., 1., 1., 1., 1., 1., 1.];

const TRINKET: &[&str] = &[
    "RatSkull",
    "ParchmentScrap",
    "PetrifiedSeed",
    "ExoticCrystals",
    "MossyClump",
    "DimensionalSundial",
    "ThirteenLeafClover",
    "TrapMechanism",
    "MimicTooth",
    "WondrousResin",
    "EyeOfNewt",
    "SaltCube",
    "VialOfBlood",
    "ShardOfOblivion",
    "ChaoticCenser",
    "FerretTuft",
    "CrackedSpyglass",
];
const TRINKET_PROBS: &[f32] = &[1.; 17];

const GOLD: &[&str] = &["Gold"];

/// Floor-set tier probabilities for weapons/armor/missiles.
pub const FLOOR_SET_TIER_PROBS: [[f32; 5]; 5] = [
    [0., 75., 20., 4., 1.],
    [0., 25., 50., 20., 5.],
    [0., 0., 40., 50., 10.],
    [0., 0., 20., 40., 40.],
    [0., 0., 0., 20., 80.],
];

pub const WEP_TIERS: [Category; 5] = [
    Category::WepT1,
    Category::WepT2,
    Category::WepT3,
    Category::WepT4,
    Category::WepT5,
];

pub const MIS_TIERS: [Category; 5] = [
    Category::MisT1,
    Category::MisT2,
    Category::MisT3,
    Category::MisT4,
    Category::MisT5,
];
