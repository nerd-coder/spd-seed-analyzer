use super::*;
use crate::generator::Category;
use crate::items::model::{ItemCategory, ItemProvenance, QuestRewardRole};
use crate::run::init_run;
use std::collections::BTreeSet;

fn generate_with_seed(seed: i64, use_defaults: bool) -> BlacksmithQuestState {
    let mut generator = init_run(42).generator;
    Random::reset_generators();
    Random::push_generator_seeded(seed);
    let mut state = BlacksmithQuestState::default();
    generate_rewards(&mut state, &mut generator, 13, use_defaults);
    Random::pop_generator();
    state
}

#[test]
fn rewards_are_deterministic_and_share_the_smith_contract() {
    let a = generate_with_seed(999, true);
    let b = generate_with_seed(999, true);

    assert_eq!(a.smith_rewards.len(), 4);
    assert_eq!(
        a.smith_rewards
            .iter()
            .map(|item| item.category)
            .collect::<Vec<_>>(),
        [
            ItemCategory::Weapon,
            ItemCategory::Weapon,
            ItemCategory::Missile,
            ItemCategory::Armor,
        ]
    );
    assert_eq!(a.smith_rewards.len(), b.smith_rewards.len());
    for (x, y) in a.smith_rewards.iter().zip(b.smith_rewards.iter()) {
        assert_eq!(x.class_name, y.class_name);
        assert_eq!(x.level, y.level);
        assert!(!x.cursed);
        assert!(x.enchantment.is_none());
        assert!(x.potential_enchantment.is_some());
    }
    assert_eq!(a.smith_enchant, b.smith_enchant);
    assert_eq!(a.smith_glyph, b.smith_glyph);
    assert_ne!(a.smith_rewards[0].class_name, a.smith_rewards[1].class_name);
    assert!(a
        .smith_rewards
        .iter()
        .all(|item| item.level == a.smith_rewards[0].level));
}

#[test]
fn parchment_threshold_is_shared_and_never_exceeds_plus_one() {
    let mut seen = BTreeSet::new();
    for seed in 0..256 {
        let state = generate_with_seed(seed, true);
        let thresholds: Vec<_> = state
            .smith_rewards
            .iter()
            .map(|item| match item.provenance {
                ItemProvenance::Quest(
                    QuestRewardRole::BlacksmithWeapon {
                        minimum_parchment_level,
                        ..
                    }
                    | QuestRewardRole::BlacksmithMissile {
                        minimum_parchment_level,
                        ..
                    }
                    | QuestRewardRole::BlacksmithArmor {
                        minimum_parchment_level,
                        ..
                    },
                ) => minimum_parchment_level,
                _ => panic!("unexpected Blacksmith reward provenance"),
            })
            .collect();
        assert!(thresholds
            .iter()
            .all(|&threshold| threshold == thresholds[0]));
        assert_eq!(state.smith_enchant.is_some(), thresholds[0].is_none());
        assert_eq!(state.smith_glyph.is_some(), thresholds[0].is_none());
        seen.insert(thresholds[0]);
    }
    assert_eq!(seen, BTreeSet::from([None, Some(0), Some(1)]));
}

#[test]
fn missile_is_a_weapon_for_smith_enchantment_and_provenance() {
    let state = generate_with_seed(999, true);
    let missile = &state.smith_rewards[2];

    assert_eq!(missile.category, ItemCategory::Missile);
    assert_eq!(
        missile.potential_enchantment,
        state.smith_rewards[0].potential_enchantment
    );
    assert!(matches!(
        missile.provenance,
        ItemProvenance::Quest(QuestRewardRole::BlacksmithMissile { .. })
    ));
}

#[test]
fn spawn_pool_uses_defaults_without_advancing_equipment_decks() {
    let mut generator = init_run(42).generator;
    let categories = [
        Category::WepT3,
        Category::WepT4,
        Category::WepT5,
        Category::MisT3,
        Category::MisT4,
        Category::MisT5,
    ];
    let before: Vec<_> = categories
        .iter()
        .map(|&category| generator.deck_dropped(category))
        .collect();

    Random::reset_generators();
    Random::push_generator_seeded(999);
    generate_rewards(
        &mut BlacksmithQuestState::default(),
        &mut generator,
        13,
        true,
    );
    Random::pop_generator();

    let after: Vec<_> = categories
        .iter()
        .map(|&category| generator.deck_dropped(category))
        .collect();
    assert_eq!(after, before);
}

#[test]
fn depth14_always_spawns_when_not_spawned() {
    Random::reset_generators();
    let mut bs = BlacksmithQuestState::default();
    let mut generator = init_run(1).generator;
    Random::push_generator_seeded(1);
    let mut specs = Vec::new();
    assert!(try_spawn(&mut bs, &mut generator, 14, &mut specs));
    assert_eq!(specs[0].name, "BlacksmithRoom");
    assert!(matches!(bs.quest_type, 1 | 2));
    assert_eq!(bs.smith_rewards.len(), 4);
    assert!(!bs.smith_rewards.is_empty());
    Random::pop_generator();
}

#[test]
fn depth11_never_spawns() {
    Random::reset_generators();
    let mut bs = BlacksmithQuestState::default();
    let mut generator = init_run(1).generator;
    let mut specs = Vec::new();
    assert!(!try_spawn(&mut bs, &mut generator, 11, &mut specs));
    assert!(specs.is_empty());
}
