//! Machine-readable public quest report types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestDepthRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestRewardSelection {
    /// Joins canonical reward entries in `FloorReport.items` through `ItemEntry.source`.
    pub item_source: String,
    pub option_count: u32,
    pub selected_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favor_requirement: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QuestReport {
    SadGhost {
        contract: SadGhostQuestContract,
        baseline: SadGhostQuestBaseline,
    },
    OldWandmaker {
        contract: OldWandmakerQuestContract,
        baseline: OldWandmakerQuestBaseline,
    },
    TrollBlacksmith {
        contract: TrollBlacksmithQuestContract,
        baseline: TrollBlacksmithQuestBaseline,
    },
    AmbitiousImp {
        contract: AmbitiousImpQuestContract,
        baseline: AmbitiousImpQuestBaseline,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SadGhostQuestContract {
    pub spawn_depth_range: QuestDepthRange,
    pub target_rules: Vec<GhostTargetRule>,
    pub rewards: QuestRewardSelection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhostTargetRule {
    pub spawn_depth: u32,
    pub target: GhostTarget,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GhostTarget {
    FetidRat,
    GnollTrickster,
    GreatCrab,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SadGhostQuestBaseline {
    pub target: GhostTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OldWandmakerQuestContract {
    pub spawn_depth_range: QuestDepthRange,
    pub objective_options: Vec<WandmakerObjective>,
    pub rewards: QuestRewardSelection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WandmakerObjective {
    CorpseDust,
    ElementalEmbers,
    Rotberry,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct OldWandmakerQuestBaseline {
    pub objective: WandmakerObjective,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrollBlacksmithQuestContract {
    pub spawn_depth_range: QuestDepthRange,
    pub objective_options: Vec<BlacksmithObjective>,
    pub rewards: QuestRewardSelection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BlacksmithObjective {
    Crystal,
    Gnoll,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrollBlacksmithQuestBaseline {
    pub objective: BlacksmithObjective,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AmbitiousImpQuestContract {
    pub spawn_depth_range: QuestDepthRange,
    pub target_rules: Vec<ImpTargetRule>,
    pub rewards: QuestRewardSelection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImpTargetRule {
    pub spawn_depth: u32,
    pub target: ImpTarget,
    pub required_tokens: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImpTarget {
    Monk,
    Golem,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AmbitiousImpQuestBaseline {
    pub target: ImpTarget,
    pub required_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rewards() -> QuestRewardSelection {
        QuestRewardSelection {
            item_source: "Quest.Source".into(),
            option_count: 1,
            selected_count: 1,
            favor_requirement: None,
        }
    }

    #[test]
    fn serializes_every_quest_as_a_discriminated_record_without_a_summary() {
        let depth_range = QuestDepthRange { min: 1, max: 2 };
        let reports = vec![
            QuestReport::SadGhost {
                contract: SadGhostQuestContract {
                    spawn_depth_range: depth_range,
                    target_rules: vec![],
                    rewards: rewards(),
                },
                baseline: SadGhostQuestBaseline {
                    target: GhostTarget::FetidRat,
                },
            },
            QuestReport::OldWandmaker {
                contract: OldWandmakerQuestContract {
                    spawn_depth_range: depth_range,
                    objective_options: vec![],
                    rewards: rewards(),
                },
                baseline: OldWandmakerQuestBaseline {
                    objective: WandmakerObjective::CorpseDust,
                },
            },
            QuestReport::TrollBlacksmith {
                contract: TrollBlacksmithQuestContract {
                    spawn_depth_range: depth_range,
                    objective_options: vec![],
                    rewards: rewards(),
                },
                baseline: TrollBlacksmithQuestBaseline {
                    objective: BlacksmithObjective::Crystal,
                },
            },
            QuestReport::AmbitiousImp {
                contract: AmbitiousImpQuestContract {
                    spawn_depth_range: depth_range,
                    target_rules: vec![],
                    rewards: rewards(),
                },
                baseline: AmbitiousImpQuestBaseline {
                    target: ImpTarget::Monk,
                    required_tokens: 5,
                },
            },
        ];

        let value = serde_json::to_value(reports).expect("serialize quest reports");
        let types: Vec<_> = value
            .as_array()
            .expect("quest array")
            .iter()
            .map(|quest| quest["type"].as_str().expect("type tag"))
            .collect();
        assert_eq!(
            types,
            [
                "sad_ghost",
                "old_wandmaker",
                "troll_blacksmith",
                "ambitious_imp"
            ]
        );
        assert!(!value.to_string().contains("summary"));
    }
}
