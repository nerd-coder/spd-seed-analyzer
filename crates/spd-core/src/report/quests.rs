//! Machine-readable public quest report types.

use serde::{Deserialize, Serialize};
use spd_objects::{Enchantment, ItemId};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestDepthRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestRewardSelection {
    /// Joins canonical reward groups in `FloorReport.items` through `ItemGroup.source`.
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
        variable: SadGhostQuestVariable,
    },
    OldWandmaker {
        contract: OldWandmakerQuestContract,
        variable: OldWandmakerQuestVariable,
    },
    TrollBlacksmith {
        contract: TrollBlacksmithQuestContract,
        variable: TrollBlacksmithQuestVariable,
    },
    AmbitiousImp {
        contract: AmbitiousImpQuestContract,
        variable: AmbitiousImpQuestVariable,
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
pub struct SadGhostQuestVariable {
    pub target: GhostTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OldWandmakerQuestContract {
    pub spawn_depth_range: QuestDepthRange,
    pub objective_options: Vec<WandmakerObjective>,
    pub rewards: QuestRewardSelection,
    /// Wand 1 then wand 2. Empty when the reward draw did not run.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<WandmakerRewardSlot>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WandmakerObjective {
    CorpseDust,
    ElementalEmbers,
    Rotberry,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct OldWandmakerQuestVariable {
    pub objective: WandmakerObjective,
}

/// One ordered wand option.
///
/// `level` is the seed-drawn pair's +N, not a per-candidate roll.
/// Wand 1 is always `level_pinned`. Wand 2 is pinned only when every offset
/// in the window rejects the same number of duplicate wands as the seed draw.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WandmakerRewardSlot {
    /// 0 = wand 1, 1 = wand 2.
    pub slot: u8,
    pub item: ItemId,
    pub level: i32,
    pub level_pinned: bool,
    pub candidates: Vec<ImpSlotCandidate>,
    /// Number of deck-index offsets in the window. Denominator for `weight`.
    pub scenario_count: u32,
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
pub struct TrollBlacksmithQuestVariable {
    pub objective: BlacksmithObjective,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AmbitiousImpQuestContract {
    pub spawn_depth_range: QuestDepthRange,
    pub rewards: QuestRewardSelection,
    /// The six take-out slots, in `rewardOptions` order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<ImpRewardSlot>,
    /// True when no reachable deck drift changes any slot's upgrade or
    /// enchant, making every `level` a seed-only guarantee.
    ///
    /// False only when `UnstableSpellbook` is reachable in slot 0: its
    /// constructor burns a `SCROLL.defaultProbsTotal` loop on the floor's
    /// ambient stream (`UnstableSpellbook.java:84-104`), which shifts every
    /// later roll.
    #[serde(default)]
    pub upgrades_pinned: bool,
    /// Total upgrade value across the six options. The artifact slot counts
    /// its `transferUpgrade` amount, not its cap-scaled stored level.
    #[serde(default)]
    pub total_upgrade_value: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct AmbitiousImpQuestVariable {
    pub spawn_depth: u32,
}

/// Which `Imp.Quest.spawn` draw site fills a reward slot (`Imp.java:318-350`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImpSlotRole {
    /// `Generator.randomArtifact()`, or a RING-deck draw once artifacts run out.
    Artifact,
    /// `Generator.random(RING)`, re-drawn while it duplicates the artifact slot.
    Ring,
    Weapon,
    Missile,
    /// `new PlateArmor()` — no deck, no `Armor.random()`.
    Armor,
    Wand,
}

/// How a slot's upgrade level is fixed. Both variants are seed-independent.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImpLevelRule {
    /// `Random.IntRange(min, max)` — uniform over the inclusive range.
    Range { min: i32, max: i32 },
    /// `Artifact.transferUpgrade(transfer)` — scaled by each artifact's level
    /// cap, so the stored level depends on the drawn class.
    TransferUpgrade { transfer: i32 },
}

/// One class a slot can hold, with the deck-index drift that produces it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImpSlotCandidate {
    pub item: ItemId,
    /// Deck-index offsets from the seed's own draw that yield this class,
    /// nearest first. `0` is the seed's own draw.
    pub drifts: Vec<i32>,
    /// Number of enumerated offsets yielding this class. Divide by the slot's
    /// `scenario_count` for a share of the enumerated window.
    pub weight: u32,
}

impl ImpSlotCandidate {
    pub fn class_name(&self) -> &'static str {
        self.item.class_name()
    }
}

/// Per-slot projection for the six take-out options.
///
/// `candidates` is a share of an *enumerated deck-drift window*, not an
/// empirical player frequency. Only `level_rule`, `enchanted`, and
/// `fixed_item` are seed-independent guarantees.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImpRewardSlot {
    pub slot: u8,
    pub role: ImpSlotRole,
    /// The class this seed draws when gameplay has not shifted the deck.
    pub item: ItemId,
    pub level_rule: ImpLevelRule,
    /// Slot always carries an enchantment or glyph (50/40/10 common/uncommon/rare).
    pub enchanted: bool,
    /// The rolled upgrade for this slot. Pinned by the seed: every class in a
    /// category consumes the same ambient RNG, so deck drift moves the class
    /// without moving the level. See `AmbitiousImpQuestContract::upgrades_pinned`
    /// for the single exception.
    pub level: i32,
    /// The rolled enchantment or glyph, pinned on the same stream as the level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enchantment: Option<Enchantment>,
    /// Set when the class cannot vary at all (the plate armor slot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_item: Option<ItemId>,
    /// Empty when `fixed_item` is set.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<ImpSlotCandidate>,
    /// Denominator for `ImpSlotCandidate::weight`.
    pub scenario_count: u32,
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
                variable: SadGhostQuestVariable {
                    target: GhostTarget::FetidRat,
                },
            },
            QuestReport::OldWandmaker {
                contract: OldWandmakerQuestContract {
                    spawn_depth_range: depth_range,
                    objective_options: vec![],
                    rewards: rewards(),
                    slots: Vec::new(),
                },
                variable: OldWandmakerQuestVariable {
                    objective: WandmakerObjective::CorpseDust,
                },
            },
            QuestReport::TrollBlacksmith {
                contract: TrollBlacksmithQuestContract {
                    spawn_depth_range: depth_range,
                    objective_options: vec![],
                    rewards: rewards(),
                },
                variable: TrollBlacksmithQuestVariable {
                    objective: BlacksmithObjective::Crystal,
                },
            },
            QuestReport::AmbitiousImp {
                contract: AmbitiousImpQuestContract {
                    spawn_depth_range: depth_range,
                    rewards: rewards(),
                    slots: Vec::new(),
                    upgrades_pinned: true,
                    total_upgrade_value: 0,
                },
                variable: AmbitiousImpQuestVariable { spawn_depth: 17 },
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
