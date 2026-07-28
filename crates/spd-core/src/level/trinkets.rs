//! Map-affecting trinket state read by SPD's static generation callbacks.

use std::cell::RefCell;

use crate::random::Random;
use crate::report::MapTrinketProfile;

use super::Feeling;

#[derive(Default)]
struct FeelingDeck {
    values: Vec<bool>,
    shuffles: usize,
}

impl FeelingDeck {
    fn next(&mut self, seed: i64, initial: [bool; 6]) -> bool {
        if self.values.is_empty() {
            self.values.extend(initial);
            Random::push_generator_seeded(seed.wrapping_add(1));
            for _ in 0..=self.shuffles {
                Random::shuffle_list(&mut self.values);
            }
            self.shuffles += 1;
            Random::pop_generator();
        }
        self.values.remove(0)
    }
}

struct TrinketState {
    seed: i64,
    held: MapTrinketProfile,
    mossy: FeelingDeck,
    trap: FeelingDeck,
}

impl Default for TrinketState {
    fn default() -> Self {
        Self {
            seed: 0,
            held: MapTrinketProfile::NoMapAffectingTrinkets,
            mossy: FeelingDeck::default(),
            trap: FeelingDeck::default(),
        }
    }
}

thread_local! {
    static STATE: RefCell<TrinketState> = RefCell::new(TrinketState::default());
}

pub fn reset(seed: i64) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        *state = TrinketState {
            seed,
            ..TrinketState::default()
        };
    });
}

pub fn set_held(held: MapTrinketProfile) {
    STATE.with(|state| state.borrow_mut().held = held);
}

fn level(profile: MapTrinketProfile, mossy: bool) -> Option<u8> {
    use MapTrinketProfile::*;
    match (profile, mossy) {
        (MossyClump0, true) | (TrapMechanism0, false) => Some(0),
        (MossyClump1, true) | (TrapMechanism1, false) => Some(1),
        (MossyClump2, true) | (TrapMechanism2, false) => Some(2),
        (MossyClump3, true) | (TrapMechanism3, false) => Some(3),
        _ => None,
    }
}

fn mimic_tooth_level(profile: MapTrinketProfile) -> Option<u8> {
    use MapTrinketProfile::*;
    match profile {
        MimicTooth0 => Some(0),
        MimicTooth1 => Some(1),
        MimicTooth2 => Some(2),
        MimicTooth3 => Some(3),
        _ => None,
    }
}

fn override_chance(level: Option<u8>) -> f32 {
    level.map_or(0.0, |level| 0.25 + 0.25 * f32::from(level))
}

/// Pinned `Level.create` default-feeling branch, including short-circuit RNG.
pub fn override_default_feeling() -> Feeling {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let mossy_level = level(state.held, true);
        if Random::float() < override_chance(mossy_level) {
            let seed = state.seed;
            return if state
                .mossy
                .next(seed, [true, true, false, false, false, false])
            {
                Feeling::Grass
            } else {
                Feeling::Water
            };
        }

        let trap_level = level(state.held, false);
        if Random::float() < override_chance(trap_level) {
            let seed = state.seed;
            return if state
                .trap
                .next(seed, [true, true, true, false, false, false])
            {
                Feeling::Traps
            } else {
                Feeling::Chasm
            };
        }
        Feeling::None
    })
}

pub fn trap_reveal_chance() -> f32 {
    STATE.with(|state| {
        level(state.borrow().held, false).map_or(0.0, |level| 0.1 + 0.1 * f32::from(level))
    })
}

/// `MimicTooth.mimicChanceMultiplier()` for the currently held profile.
pub fn mimic_chance_multiplier() -> f32 {
    STATE.with(|state| {
        mimic_tooth_level(state.borrow().held).map_or(1.0, |level| 1.5 + 0.5 * f32::from(level))
    })
}

pub fn has_mimic_tooth() -> bool {
    STATE.with(|state| mimic_tooth_level(state.borrow().held).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_trinkets_preserve_both_default_branch_float_calls() {
        Random::reset_generators();
        Random::push_generator_seeded(77);
        reset(123);
        set_held(MapTrinketProfile::NoMapAffectingTrinkets);
        assert_eq!(override_default_feeling(), Feeling::None);
        let after = Random::int();
        Random::pop_generator();

        Random::push_generator_seeded(77);
        let _ = Random::float();
        let _ = Random::float();
        assert_eq!(after, Random::int());
        Random::pop_generator();
    }

    #[test]
    fn max_level_mossy_clump_uses_the_persistent_seeded_deck() {
        Random::reset_generators();
        Random::push_generator_seeded(91);
        reset(456);
        set_held(MapTrinketProfile::MossyClump3);
        let feelings: Vec<_> = (0..6).map(|_| override_default_feeling()).collect();
        assert_eq!(
            feelings
                .iter()
                .filter(|&&value| value == Feeling::Grass)
                .count(),
            2
        );
        assert_eq!(
            feelings
                .iter()
                .filter(|&&value| value == Feeling::Water)
                .count(),
            4
        );
        Random::pop_generator();
    }

    #[test]
    fn trap_mechanism_level_controls_reveal_fraction() {
        reset(1);
        set_held(MapTrinketProfile::TrapMechanism0);
        assert!((trap_reveal_chance() - 0.1).abs() < f32::EPSILON);
        set_held(MapTrinketProfile::TrapMechanism3);
        assert!((trap_reveal_chance() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn mimic_tooth_level_controls_its_multiplier() {
        reset(1);
        set_held(MapTrinketProfile::NoMapAffectingTrinkets);
        assert_eq!(mimic_chance_multiplier(), 1.0);
        set_held(MapTrinketProfile::MimicTooth0);
        assert_eq!(mimic_chance_multiplier(), 1.5);
        set_held(MapTrinketProfile::MimicTooth3);
        assert_eq!(mimic_chance_multiplier(), 3.0);
        assert!(has_mimic_tooth());
    }
}
