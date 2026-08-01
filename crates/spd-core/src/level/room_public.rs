//! Static seed-only contracts emitted by room painters.

use crate::report::{ItemCondition, ItemEntry, ItemPredictionKind, NumericRange};
use crate::trinkets::Challenge;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomPublicFact {
    pub room: String,
    depth: i32,
}

impl RoomPublicFact {
    pub fn new(room: &str, depth: i32) -> Option<Self> {
        has_contract(room).then(|| Self {
            room: room.into(),
            depth,
        })
    }

    pub fn entries(&self) -> Vec<ItemEntry> {
        if self.room == "SecretLarderRoom" {
            return larder_entries(self.depth);
        }
        let mut entries: Vec<_> = contracts(&self.room)
            .into_iter()
            .map(|(name, category, cursed, note)| ItemEntry {
                name: name.into(),
                quantity: 1,
                class_name: None,
                candidate_classes: Vec::new(),
                category: category.into(),
                tier: None,
                tier_range: None,
                level: None,
                level_range: None,
                cursed,
                enchantment: None,
                prediction: ItemPredictionKind::Constrained,
                spawn_conditions: Vec::new(),
                conditions: contract_conditions(note),
                notes: (!note.is_empty())
                    .then(|| note.into())
                    .into_iter()
                    .collect(),
                source: Some(self.room.clone()),
            })
            .collect();
        for entry in &mut entries {
            if self.room == "SacrificeRoom" {
                entry.tier_range = Some(sacrifice_tier_range(self.depth));
                entry.level_range = Some(NumericRange { min: 0, max: 3 });
            }
            if entry.name == "2–3 distinct Armory base rewards" {
                entry.tier_range = Some(regular_equipment_tier_range(self.depth));
                entry.level_range = Some(NumericRange { min: 0, max: 2 });
            }
            let exact_class = match entry.name.as_str() {
                "Double Bomb" => Some("DoubleBomb"),
                "Stone of Enchantment" => Some("StoneOfEnchantment"),
                "two Energy Crystal stacks" => Some("EnergyCrystal"),
                "5 Energy Crystals" => Some("EnergyCrystal"),
                "Shattered Pot" => Some("ShatteredPot"),
                "Honeypot" => Some("Honeypot"),
                "Potion of Frost" => Some("PotionOfFrost"),
                "Potion of Haste" => Some("PotionOfHaste"),
                "Potion of Invisibility" => Some("PotionOfInvisibility"),
                "Potion of Levitation" => Some("PotionOfLevitation"),
                "Potion of Liquid Flame" => Some("PotionOfLiquidFlame"),
                "Potion of Purity" => Some("PotionOfPurity"),
                _ => None,
            };
            if let Some(class_name) = exact_class {
                entry.class_name = Some(class_name.into());
                entry.level = Some(0);
                entry.prediction = ItemPredictionKind::Exact;
            }
            match entry.name.as_str() {
                "two Energy Crystal stacks" => {
                    entry.name = "Energy Crystal".into();
                    entry.quantity = 10;
                }
                "5 Energy Crystals" => {
                    entry.name = "Energy Crystal".into();
                    entry.quantity = 5;
                }
                _ => {}
            }
        }
        entries
    }
}

fn contract_conditions(note: &str) -> Vec<ItemCondition> {
    let mut conditions = Vec::new();
    if note.contains("artifact") {
        conditions.push(ItemCondition::Artifact { events: Vec::new() });
    }
    if note.contains("Barren Land") {
        conditions.push(ItemCondition::Challenge {
            challenge: Challenge::BarrenLand,
            enabled: false,
        });
    }
    conditions
}

fn larder_entries(depth: i32) -> Vec<ItemEntry> {
    let units = 1 + depth / 5;
    [("Pasty", units / 3), ("ChargrilledMeat", units % 3)]
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .map(|(class_name, count)| ItemEntry {
            name: class_name.into(),
            quantity: count,
            class_name: Some(class_name.into()),
            candidate_classes: Vec::new(),
            category: "food".into(),
            tier: None,
            tier_range: None,
            level: Some(0),
            level_range: None,
            cursed: Some(false),
            enchantment: None,
            prediction: ItemPredictionKind::Exact,
            spawn_conditions: Vec::new(),
            conditions: Vec::new(),
            notes: vec![format!(
                "Exact depth-derived larder count ({units} food units)."
            )],
            source: Some("SecretLarderRoom".into()),
        })
        .collect()
}

fn sacrifice_tier_range(depth: i32) -> NumericRange {
    // SacrificeRoom requests one floor set above the normal depth set. These
    // are the non-zero bounds of pinned Generator.floorSetTierProbs.
    match (depth / 5 + 1).clamp(0, 4) {
        0 => NumericRange { min: 1, max: 5 },
        1 => NumericRange { min: 2, max: 5 },
        2 | 3 => NumericRange { min: 3, max: 5 },
        4 => NumericRange { min: 4, max: 5 },
        _ => unreachable!(),
    }
}

fn regular_equipment_tier_range(depth: i32) -> NumericRange {
    // ArmoryRoom calls the no-argument Generator equipment helpers, which use
    // floor set `depth / 5`. Bounds are the non-zero entries in the pinned
    // Generator.floorSetTierProbs table. Bomb is the only non-equipment base
    // category and has no tier; the entry note makes that qualification clear.
    match (depth / 5).clamp(0, 4) {
        0 | 1 => NumericRange { min: 2, max: 5 },
        2 | 3 => NumericRange { min: 3, max: 5 },
        4 => NumericRange { min: 4, max: 5 },
        _ => unreachable!(),
    }
}

type Contract = (&'static str, &'static str, Option<bool>, &'static str);

fn contracts(room: &str) -> Vec<Contract> {
    match room {
        "CryptRoom" => vec![("Crypt armor reward", "armor", Some(true), "Armor from this depth's Crypt floor-set distribution; Parchment Scrap may alter its glyph and conditional upgrade.")],
        "StudyRoom" | "RitualRoom" => vec![("single center-room reward source", "other", None, "An eligible guaranteed item, otherwise a potion or scroll.")],
        "RingRoom" => vec![("conditional guaranteed item", "other", None, "Conditional: this standard room and its unchanged reward exist only when an eligible guaranteed floor item is available.")],
        "SuspiciousChestRoom" => vec![("suspicious chest reward source", "other", None, "An eligible guaranteed item or gold; a Mimic may add a general reward.")],
        "GrassyGraveRoom" => vec![("geometry-derived tomb set", "other", None, "One general Generator prize plus gold in every remaining tomb.")],
        "ArmoryRoom" => vec![
            ("2–3 distinct Armory base rewards", "bomb / weapon / armor / missile", None, "The seed chooses 2–3 categories without replacement. Equipment uses the displayed depth floor-set tier range and has a +0..+2 upgrade; Bomb/Double Bomb is tierless at +0 and uncursed. Equipment curse and enchantment depend on its isolated roll and Parchment Scrap state."),
            ("Trinket Catalyst", "trinket", Some(false), "Conditional: added as a separate reward only when a guaranteed Trinket Catalyst is available on this floor."),
        ],
        "PoolRoom" => vec![
            ("single room reward source", "other", None, "May spawn an eligible guaranteed floor prize; fallback is weapon, missile weapon, or armor from the room's floor-set distribution, forced uncursed with curse-enchantment stripped and a possible +1 upgrade."),
            ("Potion of Invisibility", "potion", Some(false), ""),
        ],
        "SentryRoom" => vec![
            ("single room reward source", "other", None, "May spawn an eligible guaranteed floor prize; fallback is weapon, missile weapon, or armor from the room's floor-set distribution, forced uncursed with curse-enchantment stripped and a possible +1 upgrade."),
            ("Potion of Haste", "potion", Some(false), ""),
        ],
        "TrapsRoom" => vec![
            ("single room reward source", "other", None, "May spawn an eligible guaranteed floor prize; fallback is weapon, missile weapon, or armor from the room's floor-set distribution, forced uncursed with curse-enchantment stripped."),
            ("Potion of Levitation", "potion", Some(false), ""),
        ],
        "StatueRoom" => vec![("Statue weapon", "weapon", Some(false), "A positively enchanted melee weapon is carried; Rat Skull may change the statue encounter variant.")],
        "SacrificeRoom" => vec![("weapon reward", "weapon", Some(true), "A cursed weapon from the one-higher floor-set tier range. Its final upgrade is +0..+3; prior weapon-deck history can alter identity, and Parchment Scrap may alter enchantment chance.")],
        "CrystalVaultRoom" => vec![("two crystal-vault reward sources", "other", None, "Categories rotate among wand, ring, and artifact; an exhausted artifact pool falls back to a ring, and the second chest may conditionally be a Crystal Mimic.")],
        "CrystalChoiceRoom" => vec![("3–4 potion/scroll sources", "other", None, ""), ("hidden crystal-choice reward", "other", None, "Wand, ring, or artifact; an exhausted artifact pool falls back to a ring.")],
        "CrystalPathRoom" => vec![("three potion sources", "potion", None, ""), ("three scroll sources", "scroll", None, "")],
        "PitRoom" => vec![("Pit main reward", "other", None, "One ring, artifact, weapon, missile weapon, or armor family reward; artifact exhaustion may alter the path."), ("1–2 Pit supplements", "other", None, "")],
        "SecretLibraryRoom" => vec![("2–3 distinct scroll-base rewards", "scroll", None, "Exotic Crystals may conditionally convert each base scroll.")],
        "SecretLaboratoryRoom" => vec![("two Energy Crystal stacks", "other", Some(false), ""), ("2–3 distinct potion-base rewards", "potion", None, "Exotic Crystals may conditionally convert each base potion.")],
        "SecretArtilleryRoom" => vec![("Double Bomb", "other", Some(false), ""), ("two default missile sources", "missile", None, "Parchment Scrap may change their curse state.")],
        "SecretRunestoneRoom" => vec![("two default stone sources", "stone", Some(false), ""), ("Stone of Enchantment", "stone", Some(false), ""), ("Potion of Liquid Flame", "potion", Some(false), "")],
        "SecretLarderRoom" => vec![("depth-derived food cache", "food", Some(false), "Food identities and count follow the pinned depth-derived larder recipe.")],
        "SecretGardenRoom" => vec![("four secret-garden planting attempts", "seed", None, "The room attempts Starflower, Seedpod, Dewcatcher, then Seedpod or Dewcatcher; Barren Land prevents the plants from existing.")],
        "SecretHoardRoom" => vec![("exactly 16 gold piles", "gold", Some(false), "")],
        "SecretMazeRoom" => vec![("Secret Maze equipment", "other", Some(false), "Weapon or armor, always uncursed.")],
        "SecretSummoningRoom" => vec![("conditional summoning-room reward", "other", None, "Its presence depends on runtime state.")],
        "SecretChestChasmRoom" => vec![("four locked default-stock sources", "other", None, "Matching key support is also generated."), ("Potion of Levitation", "potion", Some(false), "")],
        "SecretHoneypotRoom" => vec![("Shattered Pot", "other", Some(false), ""), ("Honeypot", "other", Some(false), ""), ("Bomb variant", "other", Some(false), "Bomb or Double Bomb.")],
        "LibraryRoom" => vec![
            ("1–3 Library scroll rewards", "scroll", Some(false), "The first is Scroll of Identify or Scroll of Remove Curse. Later rewards are an available guaranteed Trinket Catalyst, an available guaranteed scroll, or a generated scroll; a Catalyst changes that individual reward's type."),
        ],
        "TreasuryRoom" => vec![
            ("2–3 Treasury chest rewards", "gold / trinket", None, "Each reward is a guaranteed Trinket Catalyst when one is available, otherwise a generated gold pile. The seed chooses a common chest-or-open-pile presentation; chest rewards may be carried by Mimics."),
            ("six small gold piles", "gold", Some(false), "Conditional: these additional 5–12 gold piles spawn only when the seed chooses open piles instead of chests."),
        ],
        "StorageRoom" => vec![
            ("3–4 room rewards", "potion / scroll / food / gold / other", None, "The seed may include one Honeypot. Other rewards are either an eligible guaranteed floor prize or generated potion, scroll, food, or gold; concrete identities depend on prior limited-item and Generator state."),
            ("Potion of Liquid Flame", "potion", Some(false), ""),
        ],
        "MagicalFireRoom" => vec![
            ("3–4 room rewards", "potion / scroll / food / gold / other", None, "The seed may include one Honeypot. Other rewards are either an eligible guaranteed floor prize or generated potion, scroll, food, or gold; concrete identities depend on prior limited-item and Generator state."),
            ("Potion of Frost", "potion", Some(false), ""),
        ],
        "RunestoneRoom" => vec![
            ("2–3 runestone-room rewards", "stone / trinket", None, "Each reward is an available guaranteed Trinket Catalyst, an available guaranteed runestone, or otherwise a generated runestone."),
        ],
        "LaboratoryRoom" => vec![
            ("5 Energy Crystals", "other", Some(false), "A guaranteed stack of five."),
            ("1–2 laboratory rewards", "potion / stone / trinket", None, "Each reward is an available guaranteed Trinket Catalyst, an available Potion of Strength, or otherwise a generated potion or runestone."),
            ("Alchemy Guide pages", "other", Some(false), "Conditional count depends on previously found guide pages and chapter depth."),
        ],
        "ToxicGasRoom" => vec![
            ("double-sized gold pile", "gold", Some(false), "A guaranteed generated gold reward with doubled quantity, found with the skeleton."),
            ("two Toxic Gas chest rewards", "gold / trinket", None, "Each chest contains a guaranteed Trinket Catalyst when one is available, otherwise a generated gold pile."),
            ("Potion of Purity", "potion", Some(false), ""),
        ],
        "MassGraveRoom" => vec![("Potion of Liquid Flame", "potion", Some(false), "")],
        _ => Vec::new(),
    }
}

fn has_contract(room: &str) -> bool {
    !contracts(room).is_empty()
}

#[cfg(test)]
#[path = "room_public_tests.rs"]
mod tests;
