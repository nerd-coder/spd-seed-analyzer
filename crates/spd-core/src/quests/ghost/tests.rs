use super::*;
use crate::run::{dungeon_from_run, init_run};
use std::collections::BTreeSet;

const V4_WEAPON_ENCHANTS: [&str; 4] = ["Venomous", "Eldritch", "Vorpal", "Crystal"];
const ARMOR_GLYPHS: [&str; 13] = [
    "Obfuscation",
    "Swiftness",
    "Viscosity",
    "Potential",
    "Brimstone",
    "Stone",
    "Entanglement",
    "Repulsion",
    "Camouflage",
    "Flow",
    "Affection",
    "AntiMagic",
    "Thorns",
];

fn generate_rewards_at(seed: i64) -> (GeneratedItem, GeneratedItem) {
    Random::reset_generators();
    let run = init_run(seed);
    Random::push_generator_seeded(seed);
    let rewards = {
        let mut dungeon = dungeon_from_run(run);
        dungeon.depth = 4;
        generate_rewards(&mut dungeon)
    };
    Random::pop_generator();
    rewards
}

#[test]
fn ghost_rewards_deterministic() {
    Random::reset_generators();
    let run = init_run(42);
    Random::push_generator_seeded(999);
    let (w1, a1) = {
        let mut d = dungeon_from_run(run.clone());
        d.depth = 4;
        generate_rewards(&mut d)
    };
    Random::pop_generator();

    Random::reset_generators();
    Random::push_generator_seeded(999);
    let (w2, a2) = {
        let mut d = dungeon_from_run(run);
        d.depth = 4;
        generate_rewards(&mut d)
    };
    Random::pop_generator();

    assert_eq!(w1.class_name, w2.class_name);
    assert_eq!(w1.level, w2.level);
    assert_eq!(a1.class_name, a2.class_name);
    assert_eq!(a1.level, a2.level);
    assert!(w1.potential_enchantment.is_some());
    assert!(a1.potential_enchantment.is_some());
    let ItemProvenance::Quest(QuestRewardRole::GhostWeapon { .. }) = w1.provenance else {
        panic!("Ghost weapon provenance");
    };
    assert_eq!(
        w1.candidate_classes.last().map(String::as_str),
        Some(w1.class_name.as_str())
    );
}

#[test]
fn depth4_always_attempts_when_not_spawned() {
    // int_max(1) is always 0
    Random::reset_generators();
    for _ in 0..20 {
        assert_eq!(Random::int_max(1), 0);
    }
}

#[test]
fn ghost_weapon_potential_intersects_v4_enchant_names() {
    let mut weapons = BTreeSet::new();
    let mut glyphs = BTreeSet::new();
    for seed in 0..200 {
        let (weapon, armor) = generate_rewards_at(seed);
        weapons.insert(
            weapon
                .potential_enchantment
                .expect("Ghost always rolls potential_enchantment"),
        );
        glyphs.insert(
            armor
                .potential_enchantment
                .expect("Ghost always rolls a potential glyph"),
        );
    }
    assert!(
        V4_WEAPON_ENCHANTS
            .iter()
            .copied()
            .any(|name| weapons.contains(name)),
        "expected a v4 weapon enchant in 200 Ghost rolls, got {weapons:?}"
    );
    assert!(
        glyphs
            .iter()
            .all(|name| ARMOR_GLYPHS.contains(&name.as_str())),
        "Ghost armor glyphs should stay on the v3 set, got {glyphs:?}"
    );
}

#[test]
fn ghost_reward_enchant_pin() {
    let (weapon, armor) = generate_rewards_at(0);
    assert_eq!(weapon.potential_enchantment.as_deref(), Some("Venomous"));
    assert_eq!(armor.potential_enchantment.as_deref(), Some("Flow"));
}
