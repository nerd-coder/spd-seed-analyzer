//! Exotic Crystals conversion of generated potions and scrolls.

use crate::items::exotic::regular_to_exotic;
use crate::level::trinkets;
use crate::random::Random;

/// Java `ExoticPotion/Scroll.regToExo` check: roll `Random.Float()` only when the
/// class has a counterpart, then swap if the held Exotic Crystals chance hits.
pub(crate) fn maybe_convert_exotic_consumable(class_name: &'static str) -> &'static str {
    let Some(exotic) = regular_to_exotic(class_name) else {
        return class_name;
    };
    if Random::float() < trinkets::consumable_exotic_chance() {
        exotic
    } else {
        class_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::exotic::regular_to_exotic;
    use crate::level::trinkets::{self, set_held};
    use crate::trinkets::{MapProfile, TrinketEvent, TrinketEventAction, TrinketKind};

    struct ClearHeldOnDrop;

    impl Drop for ClearHeldOnDrop {
        fn drop(&mut self) {
            set_held(None);
        }
    }

    fn hold_exotic_crystals(level: u8) -> ClearHeldOnDrop {
        let profile = MapProfile {
            trinket_events: vec![TrinketEvent {
                before_depth: 1,
                action: TrinketEventAction::Acquired {
                    trinket: TrinketKind::ExoticCrystals,
                    min_upgrades: Some(level),
                },
            }],
            ..MapProfile::default()
        };
        trinkets::reset(1);
        set_held(profile.held_at(1));
        ClearHeldOnDrop
    }

    fn assert_converts_iff_roll_below_chance(level: u8, chance: f32) {
        let _guard = hold_exotic_crystals(level);
        for seed in 0..80 {
            Random::reset_generators();
            Random::push_generator_seeded(seed);
            let converted = maybe_convert_exotic_consumable("PotionOfHealing");
            Random::pop_generator();

            Random::reset_generators();
            Random::push_generator_seeded(seed);
            let roll = Random::float();
            Random::pop_generator();

            let expect_exotic = roll < chance;
            assert_eq!(
                converted == "PotionOfShielding",
                expect_exotic,
                "seed {seed} roll {roll} chance {chance}"
            );
            if !expect_exotic {
                assert_eq!(converted, "PotionOfHealing");
            }
        }
    }

    #[test]
    fn no_trinket_burns_float_without_converting() {
        trinkets::reset(1);
        set_held(None);
        let _guard = ClearHeldOnDrop;
        for seed in 0..40 {
            Random::reset_generators();
            Random::push_generator_seeded(seed);
            let converted = maybe_convert_exotic_consumable("ScrollOfTransmutation");
            let after = Random::int();
            Random::pop_generator();

            Random::reset_generators();
            Random::push_generator_seeded(seed);
            let _ = Random::float();
            assert_eq!(Random::int(), after);
            Random::pop_generator();

            assert_eq!(converted, "ScrollOfTransmutation");
        }
    }

    #[test]
    fn held_plus_zero_converts_at_point_two() {
        assert_converts_iff_roll_below_chance(0, 0.2);
    }

    #[test]
    fn held_plus_one_converts_at_point_four() {
        assert_converts_iff_roll_below_chance(1, 0.4);
    }

    #[test]
    fn held_plus_three_converts_at_point_eight() {
        assert_converts_iff_roll_below_chance(3, 0.8);
    }

    #[test]
    fn unmapped_classes_do_not_consume_a_float() {
        let _guard = hold_exotic_crystals(3);
        Random::reset_generators();
        Random::push_generator_seeded(99);
        let before = Random::peek_ints(1);
        assert_eq!(maybe_convert_exotic_consumable("Gold"), "Gold");
        assert_eq!(Random::int(), before[0]);
        Random::pop_generator();
    }

    #[test]
    fn every_regular_consumable_has_an_exotic_counterpart() {
        assert!(regular_to_exotic("PotionOfExperience").is_some());
        assert!(regular_to_exotic("ScrollOfTransmutation").is_some());
    }
}
