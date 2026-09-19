use super::*;
use crate::quests::imp::ImpQuestState;
use crate::run::{dungeon_from_run, init_run};

fn spawned_imp(seed: i64) -> Option<(crate::dungeon::DungeonState, ImpQuestState)> {
    let mut dungeon = dungeon_from_run(init_run(seed));
    for depth in 1..=19 {
        dungeon.depth = depth;
        let _ = crate::level::create_level_partial(&mut dungeon);
    }
    if !dungeon.imp.spawned {
        return None;
    }
    let imp = dungeon.imp.clone();
    Some((dungeon, imp))
}

/// Every seed that spawns the Imp yields six slots, and the baseline pool is
/// always present in its own distribution at drift 0.
#[test]
fn baseline_sits_at_drift_zero() {
    let mut checked = 0;
    for seed in 0..25_i64 {
        let Some((dungeon, imp)) = spawned_imp(seed) else {
            continue;
        };
        let slots = build_reward_slots(
            &dungeon.generator,
            &imp.slot_draws,
            &imp.reward_options,
            imp.depth,
            DEFAULT_DRIFT,
        );
        assert_eq!(slots.len(), 6, "seed {seed}");
        for (slot, item) in slots.iter().zip(&imp.reward_options) {
            assert_eq!(slot.baseline_class, item.class_name, "seed {seed}");
            if slot.fixed_class.is_some() {
                assert!(slot.candidates.is_empty());
                continue;
            }
            // The fresh-run draw must lead the list: it is the concrete answer
            // for a player who has not shifted the deck.
            let first = slot
                .candidates
                .first()
                .unwrap_or_else(|| panic!("seed {seed} slot {} has no candidates", slot.slot));
            assert!(
                first.drifts.contains(&0),
                "seed {seed} slot {}: first candidate must be the fresh-run draw, got {:?}",
                slot.slot,
                first
            );
            assert_eq!(
                first.class_name, item.class_name,
                "seed {seed} slot {} drift 0 must be the baseline draw",
                slot.slot
            );
            assert_eq!(
                slot.candidates
                    .iter()
                    .filter(|candidate| candidate.drifts.contains(&0))
                    .count(),
                1,
                "seed {seed} slot {}: exactly one row holds the fresh-run draw",
                slot.slot
            );
        }
        checked += 1;
    }
    assert!(checked > 0, "no seed in range spawned the Imp");
}

/// The armor slot has no deck at all: `new PlateArmor()` (`Imp.java:345`).
#[test]
fn armor_slot_is_always_plate() {
    for seed in 0..25_i64 {
        let Some((dungeon, imp)) = spawned_imp(seed) else {
            continue;
        };
        let slots = build_reward_slots(
            &dungeon.generator,
            &imp.slot_draws,
            &imp.reward_options,
            imp.depth,
            DEFAULT_DRIFT,
        );
        let armor = &slots[4];
        assert_eq!(armor.role, ImpSlotRole::Armor);
        assert_eq!(armor.fixed_class.as_deref(), Some("PlateArmor"));
        assert!(armor.enchanted, "plate is always inscribed");
        assert_eq!(armor.scenario_count, 1);
    }
}

/// Guaranteed level ranges come straight from `Imp.java:318-350` and must hold
/// for the concrete baseline pool on every seed.
#[test]
fn level_rules_bound_the_baseline_pool() {
    for seed in 0..25_i64 {
        let Some((dungeon, imp)) = spawned_imp(seed) else {
            continue;
        };
        let slots = build_reward_slots(
            &dungeon.generator,
            &imp.slot_draws,
            &imp.reward_options,
            imp.depth,
            DEFAULT_DRIFT,
        );
        for (slot, item) in slots.iter().zip(&imp.reward_options) {
            match slot.level_rule {
                ImpLevelRule::Range { min, max } => assert!(
                    (min..=max).contains(&item.level),
                    "seed {seed} slot {} level {} outside {min}..={max}",
                    slot.slot,
                    item.level
                ),
                ImpLevelRule::TransferUpgrade { transfer } => {
                    assert_eq!(transfer, 5);
                    assert!(item.level > 0, "seed {seed} artifact level");
                }
            }
            assert!(!item.cursed, "seed {seed}: Imp options are forced uncursed");
        }
    }
}

/// A wider window may only add candidates; it can never change drift 0 or drop
/// a class the narrow window already found.
#[test]
fn deep_window_is_a_superset() {
    for seed in 0..25_i64 {
        let Some((dungeon, imp)) = spawned_imp(seed) else {
            continue;
        };
        let narrow = build_reward_slots(
            &dungeon.generator,
            &imp.slot_draws,
            &imp.reward_options,
            imp.depth,
            DEFAULT_DRIFT,
        );
        let deep = build_reward_slots(
            &dungeon.generator,
            &imp.slot_draws,
            &imp.reward_options,
            imp.depth,
            DEEP_DRIFT,
        );
        for (narrow_slot, deep_slot) in narrow.iter().zip(&deep) {
            assert_eq!(narrow_slot.baseline_class, deep_slot.baseline_class);
            assert!(deep_slot.scenario_count >= narrow_slot.scenario_count);
            for candidate in &narrow_slot.candidates {
                assert!(
                    deep_slot
                        .candidates
                        .iter()
                        .any(|other| other.class_name == candidate.class_name),
                    "seed {seed}: deep window dropped {}",
                    candidate.class_name
                );
            }
        }
    }
}

/// Slot 2 never duplicates slot 1's class at the baseline draw — the `do/while`
/// in `Imp.java:330-335` re-draws on a collision.
#[test]
fn ring_slot_never_duplicates_the_artifact_slot() {
    for seed in 0..25_i64 {
        let Some((_, imp)) = spawned_imp(seed) else {
            continue;
        };
        assert_ne!(
            imp.reward_options[0].class_name, imp.reward_options[1].class_name,
            "seed {seed}"
        );
    }
}
