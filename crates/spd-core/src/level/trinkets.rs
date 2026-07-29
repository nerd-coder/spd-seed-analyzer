//! Map-affecting trinket state read by SPD's static generation callbacks.

use std::cell::RefCell;

use crate::random::Random;
use crate::trinkets::ActiveTrinket;
use crate::TrinketKind;

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

#[derive(Default)]
struct TrinketState {
    seed: i64,
    held: Option<ActiveTrinket>,
    mossy: FeelingDeck,
    mossy_instance: Option<u32>,
    trap: FeelingDeck,
    trap_instance: Option<u32>,
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

pub fn set_held(held: Option<ActiveTrinket>) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(active) = held {
            match active.trinket {
                TrinketKind::MossyClump if state.mossy_instance != Some(active.instance) => {
                    state.mossy = FeelingDeck::default();
                    state.mossy_instance = Some(active.instance);
                }
                TrinketKind::TrapMechanism if state.trap_instance != Some(active.instance) => {
                    state.trap = FeelingDeck::default();
                    state.trap_instance = Some(active.instance);
                }
                _ => {}
            }
        }
        state.held = held;
    });
}

fn level(held: Option<ActiveTrinket>, trinket: TrinketKind) -> Option<u8> {
    held.filter(|held| held.trinket == trinket)
        .map(|held| held.level)
}

fn override_chance(level: Option<u8>) -> f32 {
    level.map_or(0.0, |level| 0.25 + 0.25 * f32::from(level))
}

/// Pinned `Level.create` default-feeling branch, including short-circuit RNG.
pub fn override_default_feeling() -> Feeling {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let mossy_level = level(state.held, TrinketKind::MossyClump);
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

        let trap_level = level(state.held, TrinketKind::TrapMechanism);
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
        level(state.borrow().held, TrinketKind::TrapMechanism)
            .map_or(0.0, |level| 0.1 + 0.1 * f32::from(level))
    })
}

/// `MimicTooth.mimicChanceMultiplier()` for the currently held profile.
pub fn mimic_chance_multiplier() -> f32 {
    STATE.with(|state| {
        level(state.borrow().held, TrinketKind::MimicTooth)
            .map_or(1.0, |level| 1.5 + 0.5 * f32::from(level))
    })
}

pub fn has_mimic_tooth() -> bool {
    STATE.with(|state| level(state.borrow().held, TrinketKind::MimicTooth).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_trinkets_preserve_both_default_branch_float_calls() {
        Random::reset_generators();
        Random::push_generator_seeded(77);
        reset(123);
        set_held(None);
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
        set_held(Some(ActiveTrinket {
            trinket: TrinketKind::MossyClump,
            level: 3,
            instance: 1,
        }));
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
    fn a_new_mossy_clump_instance_resets_its_feeling_deck() {
        Random::reset_generators();
        Random::push_generator_seeded(91);
        reset(456);
        set_held(Some(ActiveTrinket {
            trinket: TrinketKind::MossyClump,
            level: 3,
            instance: 1,
        }));
        let _ = override_default_feeling();
        STATE.with(|state| assert!(!state.borrow().mossy.values.is_empty()));

        set_held(Some(ActiveTrinket {
            trinket: TrinketKind::MossyClump,
            level: 3,
            instance: 2,
        }));
        STATE.with(|state| {
            let state = state.borrow();
            assert!(state.mossy.values.is_empty());
            assert_eq!(state.mossy_instance, Some(2));
        });
        Random::pop_generator();
    }

    #[test]
    fn trap_mechanism_level_controls_reveal_fraction() {
        reset(1);
        set_held(Some(ActiveTrinket {
            trinket: TrinketKind::TrapMechanism,
            level: 0,
            instance: 1,
        }));
        assert!((trap_reveal_chance() - 0.1).abs() < f32::EPSILON);
        set_held(Some(ActiveTrinket {
            trinket: TrinketKind::TrapMechanism,
            level: 3,
            instance: 1,
        }));
        assert!((trap_reveal_chance() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn mimic_tooth_level_controls_its_multiplier() {
        reset(1);
        set_held(None);
        assert_eq!(mimic_chance_multiplier(), 1.0);
        set_held(Some(ActiveTrinket {
            trinket: TrinketKind::MimicTooth,
            level: 0,
            instance: 1,
        }));
        assert_eq!(mimic_chance_multiplier(), 1.5);
        set_held(Some(ActiveTrinket {
            trinket: TrinketKind::MimicTooth,
            level: 3,
            instance: 1,
        }));
        assert_eq!(mimic_chance_multiplier(), 3.0);
        assert!(has_mimic_tooth());
    }
}
