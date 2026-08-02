//! Quest rewards created during `initRooms`, before build and paint.

use crate::dungeon::DungeonState;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::TerrainMap;
use crate::quests;
use crate::report::{
    AmbitiousImpQuestBaseline, AmbitiousImpQuestContract, BlacksmithObjective, GhostTarget,
    GhostTargetRule, ImpTarget, ImpTargetRule, OldWandmakerQuestBaseline,
    OldWandmakerQuestContract, QuestDepthRange, QuestReport, QuestRewardSelection,
    SadGhostQuestBaseline, SadGhostQuestContract, TrollBlacksmithQuestBaseline,
    TrollBlacksmithQuestContract, WandmakerObjective,
};
use crate::rooms::room::Room;

pub(super) struct InitQuestRewards {
    pub items: Vec<GeneratedItem>,
    pub quests: Vec<QuestReport>,
}

pub(super) fn take_pending(dungeon: &mut DungeonState) -> InitQuestRewards {
    let mut result = InitQuestRewards {
        items: Vec::new(),
        quests: Vec::new(),
    };

    if let Some(bs) = quests::take_blacksmith_pending(&mut dungeon.blacksmith) {
        result.quests.push(blacksmith_report(bs.quest_type));
        for mut reward in bs.rewards {
            if matches!(
                reward.category,
                ItemCategory::Weapon | ItemCategory::Missile
            ) {
                reward.enchantment = bs.smith_enchant.clone();
            } else if reward.category == ItemCategory::Armor {
                reward.enchantment = bs.smith_glyph.clone();
            }
            result.items.push(reward);
        }
    }

    if let Some(imp) = quests::take_imp_pending(&mut dungeon.imp) {
        result
            .quests
            .push(imp_report(imp.target, imp.required_tokens));
        result.items.push(imp.reward);
    }
    result
}

pub(super) struct SpawnedQuestRewards {
    pub items: Vec<GeneratedItem>,
    pub quests: Vec<QuestReport>,
}

pub(super) fn spawn_npcs(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    map: &mut TerrainMap,
) -> SpawnedQuestRewards {
    let mut result = SpawnedQuestRewards {
        items: Vec::new(),
        quests: Vec::new(),
    };
    if let Some(exit) = rooms.iter().find(|room| room.is_exit() && !room.is_empty()) {
        if let Some(ghost) = quests::try_spawn_ghost(dungeon, exit, map) {
            map.mob_occupied[ghost.cell] = true;
            map.known_mobs[ghost.cell] = Some("Ghost");
            result.quests.push(ghost_report(ghost.quest_type));
            result.items.extend([ghost.weapon, ghost.armor]);
        }
    }
    if let Some(entrance) = rooms
        .iter()
        .find(|room| room.is_entrance() && !room.is_empty())
    {
        if let Some(wandmaker) = quests::try_spawn_wandmaker(dungeon, entrance, map) {
            map.mob_occupied[wandmaker.cell] = true;
            map.known_mobs[wandmaker.cell] = Some("Wandmaker");
            result.quests.push(wandmaker_report(wandmaker.quest_type));
            result.items.extend([wandmaker.wand1, wandmaker.wand2]);
        }
    }
    result
}

fn reward_selection(
    item_source: &str,
    option_count: u32,
    selected_count: u32,
    favor_requirement: Option<u32>,
) -> QuestRewardSelection {
    QuestRewardSelection {
        item_source: item_source.into(),
        option_count,
        selected_count,
        favor_requirement,
    }
}

fn ghost_report(quest_type: quests::GhostType) -> QuestReport {
    let target = match quest_type {
        quests::GhostType::FetidRat => GhostTarget::FetidRat,
        quests::GhostType::GnollTrickster => GhostTarget::GnollTrickster,
        quests::GhostType::GreatCrab => GhostTarget::GreatCrab,
    };
    QuestReport::SadGhost {
        contract: SadGhostQuestContract {
            spawn_depth_range: QuestDepthRange { min: 2, max: 4 },
            target_rules: vec![
                GhostTargetRule {
                    spawn_depth: 2,
                    target: GhostTarget::FetidRat,
                },
                GhostTargetRule {
                    spawn_depth: 3,
                    target: GhostTarget::GnollTrickster,
                },
                GhostTargetRule {
                    spawn_depth: 4,
                    target: GhostTarget::GreatCrab,
                },
            ],
            rewards: reward_selection("Ghost.Quest", 2, 1, None),
        },
        baseline: SadGhostQuestBaseline { target },
    }
}

fn wandmaker_report(quest_type: quests::WandmakerQuestType) -> QuestReport {
    let objective = match quest_type {
        quests::WandmakerQuestType::CorpseDust => WandmakerObjective::CorpseDust,
        quests::WandmakerQuestType::ElementalEmbers => WandmakerObjective::ElementalEmbers,
        quests::WandmakerQuestType::Rotberry => WandmakerObjective::Rotberry,
    };
    QuestReport::OldWandmaker {
        contract: OldWandmakerQuestContract {
            spawn_depth_range: QuestDepthRange { min: 7, max: 9 },
            objective_options: vec![
                WandmakerObjective::CorpseDust,
                WandmakerObjective::ElementalEmbers,
                WandmakerObjective::Rotberry,
            ],
            rewards: reward_selection("Wandmaker.Quest", 2, 1, None),
        },
        baseline: OldWandmakerQuestBaseline { objective },
    }
}

fn blacksmith_report(quest_type: quests::BlacksmithQuestType) -> QuestReport {
    let objective = match quest_type {
        quests::BlacksmithQuestType::Crystal => BlacksmithObjective::Crystal,
        quests::BlacksmithQuestType::Gnoll => BlacksmithObjective::Gnoll,
        quests::BlacksmithQuestType::Fungi => unreachable!("Fungi cannot spawn in pinned SPD"),
    };
    QuestReport::TrollBlacksmith {
        contract: TrollBlacksmithQuestContract {
            spawn_depth_range: QuestDepthRange { min: 12, max: 14 },
            objective_options: vec![BlacksmithObjective::Crystal, BlacksmithObjective::Gnoll],
            rewards: reward_selection("Blacksmith.Quest", 4, 1, Some(2_000)),
        },
        baseline: TrollBlacksmithQuestBaseline { objective },
    }
}

fn imp_report(target: quests::ImpQuestTarget, required_tokens: u8) -> QuestReport {
    let target = match target {
        quests::ImpQuestTarget::Monks => ImpTarget::Monk,
        quests::ImpQuestTarget::Golems => ImpTarget::Golem,
    };
    QuestReport::AmbitiousImp {
        contract: AmbitiousImpQuestContract {
            spawn_depth_range: QuestDepthRange { min: 17, max: 19 },
            target_rules: vec![
                ImpTargetRule {
                    spawn_depth: 17,
                    target: ImpTarget::Monk,
                    required_tokens: 5,
                },
                ImpTargetRule {
                    spawn_depth: 18,
                    target: ImpTarget::Monk,
                    required_tokens: 5,
                },
                ImpTargetRule {
                    spawn_depth: 18,
                    target: ImpTarget::Golem,
                    required_tokens: 4,
                },
                ImpTargetRule {
                    spawn_depth: 19,
                    target: ImpTarget::Golem,
                    required_tokens: 4,
                },
            ],
            rewards: reward_selection("Imp.Quest", 1, 1, None),
        },
        baseline: AmbitiousImpQuestBaseline {
            target,
            required_tokens: required_tokens.into(),
        },
    }
}
