//! Player-supplied first-generation main-path profile and Catalyst facts.

mod model;
mod validation;

use serde::{Deserialize, Serialize};

use crate::generator::Category;
use crate::run::{dungeon_from_run, RunState};

pub(crate) use model::ActiveTrinket;
pub use model::{
    ArtifactEvent, ArtifactEventAction, ArtifactKind, Challenge, ClaimState, MapProfile,
    TrinketEvent, TrinketEventAction, TrinketKind,
};
pub use validation::ProfileError;

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
    fn trinket_events_preserve_upgrade_state_and_reset_new_instances() {
        let profile = MapProfile {
            trinket_events: vec![
                TrinketEvent {
                    before_depth: 4,
                    action: TrinketEventAction::Acquired {
                        trinket: TrinketKind::MossyClump,
                    },
                },
                TrinketEvent {
                    before_depth: 7,
                    action: TrinketEventAction::Upgraded,
                },
                TrinketEvent {
                    before_depth: 9,
                    action: TrinketEventAction::Transmuted {
                        trinket: TrinketKind::MimicTooth,
                    },
                },
                TrinketEvent {
                    before_depth: 10,
                    action: TrinketEventAction::Transmuted {
                        trinket: TrinketKind::MossyClump,
                    },
                },
            ],
            ..MapProfile::default()
        };

        profile.validate(4).expect("valid progression");
        assert_eq!(profile.held_at(3), None);
        assert_eq!(profile.held_at(6).map(|state| state.level), Some(0));
        assert_eq!(profile.held_at(8).map(|state| state.level), Some(1));
        let reacquired = profile.held_at(10).expect("reacquired trinket");
        assert_eq!(reacquired.trinket, TrinketKind::MossyClump);
        assert_eq!(reacquired.level, 1);
        assert_eq!(reacquired.instance, 3);
    }

    #[test]
    fn validation_rejects_invalid_player_event_sequences() {
        let no_held_upgrade = MapProfile {
            trinket_events: vec![TrinketEvent {
                before_depth: 4,
                action: TrinketEventAction::Upgraded,
            }],
            ..MapProfile::default()
        };
        assert!(matches!(
            no_held_upgrade.validate(4),
            Err(ProfileError::TrinketUpgradeWithoutHeld)
        ));

        let duplicate_challenge = MapProfile {
            challenges: vec![Challenge::ForbiddenRunes, Challenge::ForbiddenRunes],
            ..MapProfile::default()
        };
        assert!(matches!(
            duplicate_challenge.validate(4),
            Err(ProfileError::ChallengeRepeated(Challenge::ForbiddenRunes))
        ));
    }

    #[test]
    fn profile_serializes_player_events_without_deck_counters() {
        let profile = MapProfile {
            challenges: vec![Challenge::BarrenLand, Challenge::ForbiddenRunes],
            trinket_events: vec![TrinketEvent {
                before_depth: 4,
                action: TrinketEventAction::Acquired {
                    trinket: TrinketKind::RatSkull,
                },
            }],
            artifact_events: vec![ArtifactEvent {
                before_depth: 2,
                action: ArtifactEventAction::Obtained {
                    artifact: ArtifactKind::ChaliceOfBlood,
                },
            }],
            claim_state: ClaimState {
                parchment_scrap_level: Some(1),
            },
        };

        let value = serde_json::to_value(profile).expect("profile JSON");
        assert_eq!(
            value["challenges"],
            serde_json::json!(["barren_land", "forbidden_runes"])
        );
        assert_eq!(value["trinket_events"][0]["kind"], "acquired");
        assert_eq!(value["trinket_events"][0]["trinket"], "rat_skull");
        assert_eq!(value["artifact_events"][0]["kind"], "obtained");
        assert_eq!(
            value["claim_state"]["parchment_scrap_level"],
            serde_json::Value::from(1)
        );
        assert!(value.get("deck_counters").is_none());
    }
}
