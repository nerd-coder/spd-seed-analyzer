//! Player-supplied trinket history and seed-determined catalyst availability.

use serde::{Deserialize, Serialize};

use crate::generator::Category;
use crate::run::{dungeon_from_run, RunState};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MapTrinketProfile {
    NoMapAffectingTrinkets,
    MossyClump,
    TrapMechanism,
    MimicTooth,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct HeldTrinketProfile {
    pub trinket: MapTrinketProfile,
    pub level: u8,
    /// First main-path depth generated with this held state.
    pub start_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapProfile {
    /// Chronological held states, including upgrades and transmutations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub held_trinkets: Vec<HeldTrinketProfile>,
    pub meta: MapMetaProfile,
    /// Whether the run enables SPD's `NO_SCROLLS` challenge (Forbidden Runes).
    #[serde(default)]
    pub forbidden_runes: bool,
}

impl Default for MapProfile {
    fn default() -> Self {
        Self {
            held_trinkets: Vec::new(),
            meta: MapMetaProfile::Fresh,
            forbidden_runes: false,
        }
    }
}

impl MapProfile {
    pub(crate) fn held_at(&self, depth: u32) -> Option<HeldTrinketProfile> {
        self.held_trinkets
            .iter()
            .rev()
            .find(|state| state.start_depth <= depth)
            .copied()
    }

    pub(crate) fn validate(&self, first_effective_depth: u32) -> Result<(), ProfileError> {
        let mut previous_depth = None;
        let mut previous_level = None;
        for state in &self.held_trinkets {
            if state.level > 3 {
                return Err(ProfileError::LevelOutOfRange(state.level));
            }
            if !(1..=26).contains(&state.start_depth) {
                return Err(ProfileError::DepthOutOfRange(state.start_depth));
            }
            if previous_depth.is_none() && state.start_depth < first_effective_depth {
                return Err(ProfileError::BeforeTrinketAvailable {
                    requested: state.start_depth,
                    earliest: first_effective_depth,
                });
            }
            if previous_depth.is_some_and(|depth| state.start_depth <= depth) {
                return Err(ProfileError::DepthsNotIncreasing);
            }
            if previous_level.is_some_and(|level| state.level < level) {
                return Err(ProfileError::LevelReduced);
            }
            previous_depth = Some(state.start_depth);
            previous_level = Some(state.level);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MapMetaProfile {
    Fresh,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrinketSelectionReport {
    pub catalyst_depth: u32,
    pub first_alchemy_pot_depth: u32,
    pub first_alchemy_pot_is_secret: bool,
    /// Floor where both prerequisites can first be in the hero's possession.
    pub selection_depth: u32,
    /// First not-yet-generated floor the selected trinket can influence.
    pub first_effective_depth: u32,
    /// The Catalyst's four seed-determined choices (TRINKET draws 0–3).
    pub catalyst_options: Vec<String>,
    /// Successive first-deck transmutation results (TRINKET draws 4–16).
    pub transmutation_sequence: Vec<String>,
}

impl TrinketSelectionReport {
    pub(crate) fn for_run(run: &RunState) -> Self {
        let deck = run
            .generator
            .preview_category_classes(Category::Trinket, 17, 1);
        let mut dungeon = dungeon_from_run(run.clone());
        let (catalyst_depth, first_alchemy_pot_depth, first_alchemy_pot_is_secret) =
            crate::level::first_trinket_availability(&mut dungeon);
        let selection_depth = catalyst_depth.max(first_alchemy_pot_depth);

        Self {
            catalyst_depth,
            first_alchemy_pot_depth,
            first_alchemy_pot_is_secret,
            selection_depth,
            first_effective_depth: selection_depth + 1,
            catalyst_options: deck[..4].to_vec(),
            transmutation_sequence: deck[4..].to_vec(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    #[error("held trinket level +{0} is outside the supported +0 to +3 range")]
    LevelOutOfRange(u8),
    #[error("held trinket start floor {0} is outside the main-path floor range")]
    DepthOutOfRange(u32),
    #[error(
        "held trinket cannot affect floor {requested}; this seed's earliest possible floor is {earliest}"
    )]
    BeforeTrinketAvailable { requested: u32, earliest: u32 },
    #[error("held trinket start floors must be strictly increasing")]
    DepthsNotIncreasing,
    #[error("a trinket upgrade level cannot be reduced later in the run")]
    LevelReduced,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn selection_report_combines_catalyst_and_first_pot_timing() {
        for seed in 0..50 {
            let report = TrinketSelectionReport::for_run(&crate::init_run(seed));
            assert!((1..=4).contains(&report.catalyst_depth));
            assert!((1..=4).contains(&report.first_alchemy_pot_depth));
            assert_eq!(
                report.selection_depth,
                report.catalyst_depth.max(report.first_alchemy_pot_depth)
            );
            assert_eq!(report.first_effective_depth, report.selection_depth + 1);
            assert_eq!(report.catalyst_options.len(), 4);
            assert_eq!(report.transmutation_sequence.len(), 13);
            assert_eq!(
                report
                    .catalyst_options
                    .iter()
                    .chain(&report.transmutation_sequence)
                    .collect::<HashSet<_>>()
                    .len(),
                17
            );
        }
    }

    #[test]
    fn held_history_preserves_levels_across_upgrades_and_transmutation() {
        let profile = MapProfile {
            held_trinkets: vec![
                HeldTrinketProfile {
                    trinket: MapTrinketProfile::MossyClump,
                    level: 0,
                    start_depth: 4,
                },
                HeldTrinketProfile {
                    trinket: MapTrinketProfile::MossyClump,
                    level: 2,
                    start_depth: 7,
                },
                HeldTrinketProfile {
                    trinket: MapTrinketProfile::MimicTooth,
                    level: 2,
                    start_depth: 9,
                },
            ],
            ..MapProfile::default()
        };

        profile.validate(4).expect("valid progression");
        assert_eq!(profile.held_at(3), None);
        assert_eq!(profile.held_at(6).map(|state| state.level), Some(0));
        assert_eq!(profile.held_at(8).map(|state| state.level), Some(2));
        assert_eq!(
            profile.held_at(9).map(|state| state.trinket),
            Some(MapTrinketProfile::MimicTooth)
        );

        let mut reduced = profile;
        reduced.held_trinkets[2].level = 1;
        assert!(matches!(
            reduced.validate(4),
            Err(ProfileError::LevelReduced)
        ));
    }
}
