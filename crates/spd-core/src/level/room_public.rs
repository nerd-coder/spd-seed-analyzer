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
                conditional_notes: vec![note.into()],
                source: Some(self.room.clone()),
            })
            .collect();
        for entry in &mut entries {
            if self.room == "SacrificeRoom" {
                entry.tier_range = Some(sacrifice_tier_range(self.depth));
                entry.level_range = Some(NumericRange { min: 0, max: 3 });
            }
            let exact_class = match entry.name.as_str() {
                "Double Bomb" => Some("DoubleBomb"),
                "Stone of Enchantment" => Some("StoneOfEnchantment"),
                "two Energy Crystal stacks" => Some("EnergyCrystal"),
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
                "Exact depth-derived larder count ({units} food units); cells are not asserted."
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

type Contract = (&'static str, &'static str, Option<bool>, &'static str);

fn contracts(room: &str) -> Vec<Contract> {
    match room {
        "CryptRoom" => vec![("Crypt armor reward", "armor", Some(true), "Armor from this depth's Crypt floor-set distribution; Parchment Scrap may alter its glyph and conditional upgrade.")],
        "StudyRoom" | "RitualRoom" => vec![("single center-room reward source", "other", None, "May consume an eligible queued prize; otherwise it is a potion or scroll source. Concrete identity and placement are not asserted.")],
        "RingRoom" => vec![("conditional queued-prize source", "other", None, "A reward exists only when an eligible queued prize is available; concrete identity and placement are not asserted.")],
        "SuspiciousChestRoom" => vec![("suspicious chest reward source", "other", None, "An eligible queued prize or gold fallback; the chest may become a Mimic with an additional general reward. Concrete identities and placement are not asserted.")],
        "GrassyGraveRoom" => vec![("geometry-derived tomb set", "other", None, "Exactly one tomb holds a general Generator prize and all remaining tombs hold gold; identities, quantities, and cells are not asserted.")],
        "ArmoryRoom" => vec![("2–3 distinct Armory rewards", "other", None, "Distinct base categories are bomb, weapon, armor, and missile weapon; an eligible queued Trinket Catalyst may add one reward.")],
        "PoolRoom" | "SentryRoom" => vec![("single room reward source", "other", None, "May consume an eligible queued prize; fallback is weapon, missile weapon, or armor from the room's floor-set distribution, forced uncursed with curse-enchantment stripped and a possible +1 upgrade.")],
        "TrapsRoom" => vec![("single room reward source", "other", None, "May consume an eligible queued prize; fallback is weapon, missile weapon, or armor from the room's floor-set distribution, forced uncursed with curse-enchantment stripped.")],
        "StatueRoom" => vec![("Statue weapon", "weapon", Some(false), "A positively enchanted melee weapon is carried; Rat Skull may change the statue encounter variant.")],
        "SacrificeRoom" => vec![("weapon reward", "weapon", Some(true), "A cursed weapon from the one-higher floor-set tier range. Its final upgrade is +0..+3; prior weapon-deck history can alter identity, and Parchment Scrap may alter enchantment chance.")],
        "CrystalVaultRoom" => vec![("two crystal-vault reward sources", "other", None, "Categories rotate among wand, ring, and artifact; an exhausted artifact pool falls back to a ring, and the second chest may conditionally be a Crystal Mimic.")],
        "CrystalChoiceRoom" => vec![("3–4 potion/scroll sources", "other", None, "Base potion/scroll identities are not asserted."), ("hidden crystal-choice reward", "other", None, "Wand, ring, or artifact; an exhausted artifact pool falls back to a ring.")],
        "CrystalPathRoom" => vec![("three potion sources", "potion", None, "Concrete regular/exotic identities and cells are not asserted."), ("three scroll sources", "scroll", None, "Concrete regular/exotic identities and cells are not asserted.")],
        "PitRoom" => vec![("Pit main reward", "other", None, "One ring, artifact, weapon, missile weapon, or armor family reward; artifact exhaustion may alter the path."), ("1–2 Pit supplements", "other", None, "Supplement identities and properties are not asserted.")],
        "SecretLibraryRoom" => vec![("2–3 distinct scroll-base rewards", "scroll", None, "Exotic Crystals may conditionally convert each base scroll.")],
        "SecretLaboratoryRoom" => vec![("two Energy Crystal stacks", "other", Some(false), "Stack quantities and cells are not asserted."), ("2–3 distinct potion-base rewards", "potion", None, "Exotic Crystals may conditionally convert each base potion.")],
        "SecretArtilleryRoom" => vec![("Double Bomb", "other", Some(false), "Fixed artillery reward identity; placement is not asserted."), ("two default missile sources", "missile", None, "Concrete default-pool missile identities, Parchment-sensitive curse state, and cells are not asserted.")],
        "SecretRunestoneRoom" => vec![("two default stone sources", "stone", Some(false), "Concrete default-pool identities and cells are not asserted."), ("Stone of Enchantment", "stone", Some(false), "Fixed runestone reward identity; placement is not asserted.")],
        "SecretLarderRoom" => vec![("depth-derived food cache", "food", Some(false), "Food identities and count follow the pinned depth-derived larder recipe; cells are not asserted here.")],
        "SecretGardenRoom" => vec![("four secret-garden planting attempts", "seed", None, "The room attempts Starflower, Seedpod, Dewcatcher, then Seedpod or Dewcatcher; Barren Land prevents the plants from existing.")],
        "SecretHoardRoom" => vec![("exactly 16 gold piles", "gold", Some(false), "Pile quantities and cells are not asserted.")],
        "SecretMazeRoom" => vec![("Secret Maze equipment", "other", Some(false), "Weapon-or-armor distribution; forced uncursed, while concrete identity, level, and enchantment are not asserted.")],
        "SecretSummoningRoom" => vec![("summoning-room reward", "other", None, "Concrete reward and haunted/trap visibility depend on runtime state and are not asserted.")],
        "SecretChestChasmRoom" => vec![("four locked default-stock sources", "other", None, "Concrete default-stock identities and chest cells are not asserted; matching key support is generated.")],
        "SecretHoneypotRoom" => vec![("Shattered Pot", "other", Some(false), "Fixed room reward identity; placement is not asserted."), ("Honeypot", "other", Some(false), "Fixed room reward identity; placement is not asserted."), ("Bomb variant", "other", Some(false), "Bomb or Double Bomb; concrete variant and placement are not asserted.")],
        "MagicalFireRoom" | "ToxicGasRoom" | "LibraryRoom" | "TreasuryRoom" | "StorageRoom" | "RunestoneRoom" | "LaboratoryRoom" => vec![("conditional room reward set", "other", None, "Queued prizes, generator state, or trinkets can alter concrete reward facts and the later floor tail.")],
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
