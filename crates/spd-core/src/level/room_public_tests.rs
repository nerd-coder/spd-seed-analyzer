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
        assert!(catalyst.notes[0].starts_with("Conditional:"));
    }
}

#[test]
fn suspicious_chest_and_pool_contracts_expose_reward_bounds() {
    let suspicious = RoomPublicFact::new("SuspiciousChestRoom", 2)
        .expect("Suspicious Chest contract")
        .entries();
    assert_eq!(suspicious.len(), 2);
    assert_eq!(suspicious[0].name, "possible gold fallback");
    assert_eq!(suspicious[0].level, Some(0));
    assert_eq!(suspicious[0].cursed, Some(false));
    assert_eq!(suspicious[1].name, "conditional Mimic bonus reward");
    assert_eq!(
        suspicious[1].tier_range,
        Some(NumericRange { min: 2, max: 5 })
    );
    assert_eq!(
        suspicious[1].level_range,
        Some(NumericRange { min: 0, max: 2 })
    );

    for (depth, minimum_tier) in [(2, 2), (7, 3), (12, 3), (17, 4), (22, 4)] {
        let pool = RoomPublicFact::new("PoolRoom", depth)
            .expect("Pool contract")
            .entries();
        assert_eq!(pool.len(), 2);
        assert_eq!(pool[0].name, "possible Pool equipment reward");
        assert_eq!(pool[0].category, "weapon / armor / missile");
        assert_eq!(
            pool[0].tier_range,
            Some(NumericRange {
                min: minimum_tier,
                max: 5,
            })
        );
        assert_eq!(pool[0].level_range, Some(NumericRange { min: 0, max: 3 }));
        assert_eq!(pool[0].cursed, Some(false));
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
    assert_eq!(toxic.len(), 3);
    assert_eq!(toxic[0].category, "gold");
    assert_eq!(toxic[1].name, "two Toxic Gas chest rewards");

    let ring = RoomPublicFact::new("RingRoom", 12)
        .expect("Ring contract")
        .entries();
    assert_eq!(ring[0].name, "conditional guaranteed item");
    assert!(ring[0].notes[0].starts_with("Conditional:"));
}

#[test]
fn puzzle_room_contracts_expose_guaranteed_solution_potions() {
    for (room, class_name, name) in [
        ("PoolRoom", "PotionOfInvisibility", "Potion of Invisibility"),
        (
            "StorageRoom",
            "PotionOfLiquidFlame",
            "Potion of Liquid Flame",
        ),
        ("SentryRoom", "PotionOfHaste", "Potion of Haste"),
        ("TrapsRoom", "PotionOfLevitation", "Potion of Levitation"),
        ("MagicalFireRoom", "PotionOfFrost", "Potion of Frost"),
        ("ToxicGasRoom", "PotionOfPurity", "Potion of Purity"),
        (
            "SecretRunestoneRoom",
            "PotionOfLiquidFlame",
            "Potion of Liquid Flame",
        ),
        (
            "SecretChestChasmRoom",
            "PotionOfLevitation",
            "Potion of Levitation",
        ),
        (
            "MassGraveRoom",
            "PotionOfLiquidFlame",
            "Potion of Liquid Flame",
        ),
    ] {
        let entries = RoomPublicFact::new(room, 12)
            .expect("room contract")
            .entries();
        let potion = entries
            .iter()
            .find(|entry| entry.class_name.as_deref() == Some(class_name))
            .unwrap_or_else(|| panic!("missing {class_name} for {room}"));
        assert_eq!(potion.name, name);
        assert_eq!(potion.category, "potion");
        assert_eq!(potion.level, Some(0));
        assert_eq!(potion.cursed, Some(false));
        assert_eq!(potion.prediction, ItemPredictionKind::Exact);
        assert_eq!(potion.source.as_deref(), Some(room));
        assert!(potion.notes.is_empty());
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
fn secret_honeypot_contract_exposes_its_two_fixed_items() {
    let entries = RoomPublicFact::new("SecretHoneypotRoom", 12)
        .expect("Secret Honeypot contract")
        .entries();

    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].class_name.as_deref(), Some("ShatteredPot"));
    assert_eq!(entries[0].prediction, ItemPredictionKind::Exact);
    assert_eq!(entries[1].class_name.as_deref(), Some("Honeypot"));
    assert_eq!(entries[1].prediction, ItemPredictionKind::Exact);
    assert_eq!(entries[2].name, "Bomb variant");
    assert_eq!(entries[2].class_name, None);
    assert_eq!(entries[2].prediction, ItemPredictionKind::Constrained);
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
        for note in entries.iter().flat_map(|entry| &entry.notes) {
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
