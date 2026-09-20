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

/// Fingerprint of everything the player actually banks: the six upgrades and
/// the two enchant/glyph rolls.
fn upgrade_fingerprint(options: &[crate::items::model::GeneratedItem]) -> String {
    options
        .iter()
        .map(|item| {
            format!(
                "{}{}",
                item.level,
                item.enchantment
                    .as_deref()
                    .map(|e| format!("/{e}"))
                    .unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Upgrades and enchants do not move with deck drift.
///
/// Every class inside a category costs the same ambient RNG in `item.random()`,
/// so shifting a deck index changes *which* item a slot holds without changing
/// its `+N` or its enchant. The lone exception is `UnstableSpellbook`, whose
/// constructor burns a `SCROLL.defaultProbsTotal` loop on the ambient stream.
#[test]
fn deck_drift_moves_classes_but_not_upgrades() {
    use crate::generator::Category;
    use crate::random::Random;

    for seed in [0_i64, 7, 42, 99, 1234] {
        let base = crate::run::init_run(seed).generator;

        let reference = {
            let mut generator = base.clone();
            Random::reset_generators();
            Random::push_generator_seeded(777);
            let (options, _) = super::super::rewards::generate_reward_options(&mut generator, 18);
            Random::pop_generator();
            upgrade_fingerprint(&options)
        };

        for category in [
            Category::Ring,
            Category::Wand,
            Category::WepT4,
            Category::WepT5,
            Category::MisT4,
            Category::MisT5,
        ] {
            for bump in 1..=3 {
                let mut generator = base.clone();
                Random::reset_generators();
                Random::push_generator_seeded(4242);
                for _ in 0..bump {
                    let _ = generator.random_category(category, 18);
                }
                Random::pop_generator();

                Random::reset_generators();
                Random::push_generator_seeded(777);
                let (options, _) =
                    super::super::rewards::generate_reward_options(&mut generator, 18);
                Random::pop_generator();
                assert_eq!(
                    upgrade_fingerprint(&options),
                    reference,
                    "seed {seed}: {category:?}+{bump} must not move upgrades or enchants"
                );
            }
        }
    }
}

/// The artifact deck is the one that can move later upgrades, and only when it
/// lands on `UnstableSpellbook`.
#[test]
fn only_unstable_spellbook_shifts_later_upgrades() {
    use crate::random::Random;

    let mut saw_spellbook = false;
    for seed in [0_i64, 7, 42] {
        let base = crate::run::init_run(seed).generator;
        let mut reference: Option<String> = None;

        for skip in 0..11 {
            let mut generator = base.clone();
            Random::reset_generators();
            Random::push_generator_seeded(4242);
            for _ in 0..skip {
                let _ = generator.random_artifact(18);
            }
            Random::pop_generator();

            Random::reset_generators();
            Random::push_generator_seeded(777);
            let (options, _) = super::super::rewards::generate_reward_options(&mut generator, 18);
            Random::pop_generator();

            // Slot 0's own stored level is cap-scaled from its class, so compare
            // only the five slots drawn after it.
            let later = upgrade_fingerprint(&options[1..]);
            if options[0].class_name == "UnstableSpellbook" {
                saw_spellbook = true;
                continue;
            }
            match &reference {
                None => reference = Some(later),
                Some(expected) => assert_eq!(
                    &later, expected,
                    "seed {seed}: artifact {} must not move later upgrades",
                    options[0].class_name
                ),
            }
        }
    }
    assert!(
        saw_spellbook,
        "sweep must cover the UnstableSpellbook case it exempts"
    );
}

/// `upgrades_pinned` is exactly "UnstableSpellbook is not reachable in slot 0".
#[test]
fn upgrades_pinned_tracks_the_spellbook_window() {
    let mut seen_pinned = false;
    let mut seen_unpinned = false;
    for seed in 0..40_i64 {
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
        let reachable = slots[0]
            .candidates
            .iter()
            .any(|candidate| candidate.class_name == "UnstableSpellbook");
        assert_eq!(upgrades_pinned(&slots), !reachable, "seed {seed}");
        if reachable {
            seen_unpinned = true;
        } else {
            seen_pinned = true;
        }

        // The pool's upgrade worth counts the artifact as its transfer amount.
        let expected: i32 = slots
            .iter()
            .map(|slot| match slot.level_rule {
                ImpLevelRule::TransferUpgrade { transfer } => transfer,
                ImpLevelRule::Range { .. } => slot.baseline_level,
            })
            .sum();
        assert_eq!(total_upgrade_value(&slots), expected, "seed {seed}");
        assert!(total_upgrade_value(&slots) >= 5 + 2 * 4 + 3, "seed {seed}");
    }
    assert!(seen_pinned && seen_unpinned, "cover both pinned states");
}
