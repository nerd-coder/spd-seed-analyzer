//! Static seed-only contracts emitted by room painters.

use crate::report::{ItemEntry, ItemPredictionKind, NumericRange};

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
                class_name: None,
                category: category.into(),
                tier: None,
                tier_range: None,
                level: None,
                level_range: None,
                cursed,
                prediction: ItemPredictionKind::Constrained,
                conditional_notes: (!note.is_empty())
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
                _ => None,
            };
            if let Some(class_name) = exact_class {
                entry.class_name = Some(class_name.into());
                entry.level = Some(0);
                entry.prediction = ItemPredictionKind::Exact;
            }
        }
        entries
    }
}

fn larder_entries(depth: i32) -> Vec<ItemEntry> {
    let units = 1 + depth / 5;
    [("Pasty", units / 3), ("ChargrilledMeat", units % 3)]
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .map(|(class_name, count)| ItemEntry {
            name: format!("{count} × {class_name}"),
            class_name: Some(class_name.into()),
            category: "food".into(),
            tier: None,
            tier_range: None,
            level: Some(0),
            level_range: None,
            cursed: Some(false),
            prediction: ItemPredictionKind::Exact,
            conditional_notes: vec![format!(
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
        "PoolRoom" | "SentryRoom" => vec![("single room reward source", "other", None, "May spawn an eligible guaranteed floor prize; fallback is weapon, missile weapon, or armor from the room's floor-set distribution, forced uncursed with curse-enchantment stripped and a possible +1 upgrade.")],
        "TrapsRoom" => vec![("single room reward source", "other", None, "May spawn an eligible guaranteed floor prize; fallback is weapon, missile weapon, or armor from the room's floor-set distribution, forced uncursed with curse-enchantment stripped.")],
        "StatueRoom" => vec![("Statue weapon", "weapon", Some(false), "A positively enchanted melee weapon is carried; Rat Skull may change the statue encounter variant.")],
        "SacrificeRoom" => vec![("weapon reward", "weapon", Some(true), "A cursed weapon from the one-higher floor-set tier range. Its final upgrade is +0..+3; prior weapon-deck history can alter identity, and Parchment Scrap may alter enchantment chance.")],
        "CrystalVaultRoom" => vec![("two crystal-vault reward sources", "other", None, "Categories rotate among wand, ring, and artifact; an exhausted artifact pool falls back to a ring, and the second chest may conditionally be a Crystal Mimic.")],
        "CrystalChoiceRoom" => vec![("3–4 potion/scroll sources", "other", None, ""), ("hidden crystal-choice reward", "other", None, "Wand, ring, or artifact; an exhausted artifact pool falls back to a ring.")],
        "CrystalPathRoom" => vec![("three potion sources", "potion", None, ""), ("three scroll sources", "scroll", None, "")],
        "PitRoom" => vec![("Pit main reward", "other", None, "One ring, artifact, weapon, missile weapon, or armor family reward; artifact exhaustion may alter the path."), ("1–2 Pit supplements", "other", None, "")],
        "SecretLibraryRoom" => vec![("2–3 distinct scroll-base rewards", "scroll", None, "Exotic Crystals may conditionally convert each base scroll.")],
        "SecretLaboratoryRoom" => vec![("two Energy Crystal stacks", "other", Some(false), ""), ("2–3 distinct potion-base rewards", "potion", None, "Exotic Crystals may conditionally convert each base potion.")],
        "SecretArtilleryRoom" => vec![("Double Bomb", "other", Some(false), ""), ("two default missile sources", "missile", None, "Parchment Scrap may change their curse state.")],
        "SecretRunestoneRoom" => vec![("two default stone sources", "stone", Some(false), ""), ("Stone of Enchantment", "stone", Some(false), "")],
        "SecretLarderRoom" => vec![("depth-derived food cache", "food", Some(false), "Food identities and count follow the pinned depth-derived larder recipe.")],
        "SecretGardenRoom" => vec![("four secret-garden planting attempts", "seed", None, "The room attempts Starflower, Seedpod, Dewcatcher, then Seedpod or Dewcatcher; Barren Land prevents the plants from existing.")],
        "SecretHoardRoom" => vec![("exactly 16 gold piles", "gold", Some(false), "")],
        "SecretMazeRoom" => vec![("Secret Maze equipment", "other", Some(false), "Weapon or armor, always uncursed.")],
        "SecretSummoningRoom" => vec![("conditional summoning-room reward", "other", None, "Its presence depends on runtime state.")],
        "SecretChestChasmRoom" => vec![("four locked default-stock sources", "other", None, "Matching key support is also generated.")],
        "SecretHoneypotRoom" => vec![("Shattered Pot", "other", Some(false), ""), ("Honeypot", "other", Some(false), ""), ("Bomb variant", "other", Some(false), "Bomb or Double Bomb.")],
        "LibraryRoom" => vec![
            ("1–3 Library scroll rewards", "scroll", Some(false), "The first is Scroll of Identify or Scroll of Remove Curse. Later rewards are an available guaranteed Trinket Catalyst, an available guaranteed scroll, or a generated scroll; a Catalyst changes that individual reward's type."),
        ],
        "TreasuryRoom" => vec![
            ("2–3 Treasury chest rewards", "gold / trinket", None, "Each reward is a guaranteed Trinket Catalyst when one is available, otherwise a generated gold pile. The seed chooses a common chest-or-open-pile presentation; chest rewards may be carried by Mimics."),
            ("six small gold piles", "gold", Some(false), "Conditional: these additional 5–12 gold piles spawn only when the seed chooses open piles instead of chests."),
        ],
        "StorageRoom" | "MagicalFireRoom" => vec![
            ("3–4 room rewards", "potion / scroll / food / gold / other", None, "The seed may include one Honeypot. Other rewards are either an eligible guaranteed floor prize or generated potion, scroll, food, or gold; concrete identities depend on prior limited-item and Generator state."),
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
        ],
        _ => Vec::new(),
    }
}

fn has_contract(room: &str) -> bool {
    !contracts(room).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::Category;
    use crate::random::Random;
    use crate::run::{dungeon_from_run, init_run};

    #[test]
    fn persistent_weapon_deck_changes_internal_class_not_statue_contract() {
        Random::reset_generators();
        let mut base = dungeon_from_run(init_run(42));
        base.depth = 8;
        let mut advanced = base.clone();
        Random::push_generator_seeded(331);
        for _ in 0..5 {
            let _ = advanced
                .generator
                .random_category(Category::Weapon, advanced.depth);
        }
        Random::pop_generator();

        Random::push_generator_seeded(991);
        let a = base
            .generator
            .random_category(Category::Weapon, base.depth)
            .class_name;
        Random::pop_generator();
        Random::push_generator_seeded(991);
        let b = advanced
            .generator
            .random_category(Category::Weapon, advanced.depth)
            .class_name;
        Random::pop_generator();
        assert_ne!(a, b, "altered persistent weapon deck changes exact class");

        let fact = RoomPublicFact::new("StatueRoom", 8).expect("contract");
        let json = serde_json::to_string(&fact.entries()).expect("serialize contract");
        assert!(!json.contains(&a));
        assert!(!json.contains(&b));
    }

    #[test]
    fn larder_contract_is_exact_and_depth_derived() {
        let entries = RoomPublicFact::new("SecretLarderRoom", 14)
            .expect("contract")
            .entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].class_name.as_deref(), Some("Pasty"));
        assert_eq!(entries[0].prediction, ItemPredictionKind::Exact);
    }

    #[test]
    fn sacrifice_contract_preserves_tier_and_upgrade_bounds() {
        for (depth, minimum_tier) in [
            (1, 2),
            (4, 2),
            (6, 3),
            (9, 3),
            (11, 3),
            (14, 3),
            (16, 4),
            (19, 4),
            (21, 4),
            (24, 4),
        ] {
            let entries = RoomPublicFact::new("SacrificeRoom", depth)
                .expect("Sacrifice contract")
                .entries();
            assert_eq!(entries.len(), 1);
            let entry = &entries[0];
            assert_eq!(entry.name, "weapon reward");
            assert_eq!(entry.category, "weapon");
            assert_eq!(entry.tier, None);
            assert_eq!(
                entry.tier_range,
                Some(NumericRange {
                    min: minimum_tier,
                    max: 5
                })
            );
            assert_eq!(entry.cursed, Some(true));
            assert_eq!(entry.class_name, None);
            assert_eq!(entry.level, None);
            assert_eq!(entry.level_range, Some(NumericRange { min: 0, max: 3 }));
            assert_eq!(entry.prediction, ItemPredictionKind::Constrained);
        }
    }

    #[test]
    fn armory_contract_preserves_count_category_and_equipment_bounds() {
        for (depth, minimum_tier) in [(1, 2), (8, 2), (12, 3), (18, 3), (22, 4)] {
            let entries = RoomPublicFact::new("ArmoryRoom", depth)
                .expect("Armory contract")
                .entries();
            assert_eq!(entries.len(), 2);
            let base = &entries[0];
            assert_eq!(base.name, "2–3 distinct Armory base rewards");
            assert_eq!(base.category, "bomb / weapon / armor / missile");
            assert_eq!(
                base.tier_range,
                Some(NumericRange {
                    min: minimum_tier,
                    max: 5,
                })
            );
            assert_eq!(base.level_range, Some(NumericRange { min: 0, max: 2 }));
            assert_eq!(base.cursed, None);

            let catalyst = &entries[1];
            assert_eq!(catalyst.class_name.as_deref(), None);
            assert_eq!(catalyst.category, "trinket");
            assert_eq!(catalyst.cursed, Some(false));
            assert!(catalyst.conditional_notes[0].starts_with("Conditional:"));
        }
    }

    #[test]
    fn formerly_generic_room_sets_expose_their_fixed_reward_structure() {
        let laboratory = RoomPublicFact::new("LaboratoryRoom", 12)
            .expect("Laboratory contract")
            .entries();
        assert_eq!(laboratory.len(), 3);
        assert_eq!(laboratory[0].class_name.as_deref(), Some("EnergyCrystal"));
        assert_eq!(laboratory[0].level, Some(0));

        let toxic = RoomPublicFact::new("ToxicGasRoom", 12)
            .expect("Toxic Gas contract")
            .entries();
        assert_eq!(toxic.len(), 2);
        assert_eq!(toxic[0].category, "gold");
        assert_eq!(toxic[1].name, "two Toxic Gas chest rewards");

        let ring = RoomPublicFact::new("RingRoom", 12)
            .expect("Ring contract")
            .entries();
        assert_eq!(ring[0].name, "conditional guaranteed item");
        assert!(ring[0].conditional_notes[0].starts_with("Conditional:"));
    }

    #[test]
    fn exact_fixed_room_items_expose_a_proven_default_level() {
        for room in [
            "SecretArtilleryRoom",
            "SecretLaboratoryRoom",
            "SecretRunestoneRoom",
            "SecretHoneypotRoom",
        ] {
            for entry in RoomPublicFact::new(room, 12)
                .expect("room contract")
                .entries()
            {
                if entry.prediction == ItemPredictionKind::Exact {
                    assert_eq!(entry.level, Some(0), "{} in {room}", entry.name);
                }
            }
        }
    }

    #[test]
    fn public_item_notes_only_describe_spawn_facts_and_conditions() {
        for room in [
            "StudyRoom",
            "RitualRoom",
            "RingRoom",
            "SuspiciousChestRoom",
            "GrassyGraveRoom",
            "CrystalChoiceRoom",
            "CrystalPathRoom",
            "PitRoom",
            "SecretLaboratoryRoom",
            "SecretArtilleryRoom",
            "SecretRunestoneRoom",
            "SecretLarderRoom",
            "SecretHoardRoom",
            "SecretMazeRoom",
            "SecretSummoningRoom",
            "SecretChestChasmRoom",
            "SecretHoneypotRoom",
        ] {
            let entries = RoomPublicFact::new(room, 12)
                .expect("room contract")
                .entries();
            for note in entries.iter().flat_map(|entry| &entry.conditional_notes) {
                let note = note.to_ascii_lowercase();
                for lifecycle_term in [
                    concat!("not ", "asserted"),
                    "placement",
                    "heap",
                    "cell",
                    "queue",
                    "consume",
                ] {
                    assert!(
                        !note.contains(lifecycle_term),
                        "{room} exposes lifecycle term {lifecycle_term}: {note}"
                    );
                }
            }
        }
    }

    #[test]
    fn artifact_exhaustion_changes_vault_branch_not_public_contract() {
        Random::reset_generators();
        let mut fresh = dungeon_from_run(init_run(91));
        fresh.depth = 12;
        let mut exhausted = fresh.clone();
        Random::push_generator_seeded(712);
        let fresh_artifact = fresh
            .generator
            .random_artifact(fresh.depth)
            .expect("fresh artifact pool")
            .class_name;
        Random::pop_generator();
        Random::push_generator_seeded(712);
        while exhausted
            .generator
            .random_artifact(exhausted.depth)
            .is_some()
        {}
        assert!(exhausted
            .generator
            .random_artifact(exhausted.depth)
            .is_none());
        Random::pop_generator();

        for room in ["CrystalVaultRoom", "CrystalChoiceRoom", "PitRoom"] {
            let json =
                serde_json::to_string(&RoomPublicFact::new(room, 12).expect("contract").entries())
                    .expect("serialize contract");
            assert!(!json.contains(&fresh_artifact));
            assert!(json.contains("artifact"));
        }
    }
}
