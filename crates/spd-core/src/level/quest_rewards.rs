//! Quest rewards created during `initRooms`, before build and paint.

use crate::dungeon::DungeonState;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::TerrainMap;
use crate::quests;
use crate::rooms::room::Room;

pub(super) struct InitQuestRewards {
    pub items: Vec<GeneratedItem>,
    pub summaries: Vec<String>,
    pub public_labels: Vec<Option<String>>,
}

pub(super) fn take_pending(dungeon: &mut DungeonState) -> InitQuestRewards {
    let mut result = InitQuestRewards {
        items: Vec::new(),
        summaries: Vec::new(),
        public_labels: Vec::new(),
    };

    if let Some(bs) = quests::take_blacksmith_pending(&mut dungeon.blacksmith) {
        let public_summary = safe_summary(&bs.summary, "reward options");
        result.summaries.push(bs.summary);
        result.public_labels.push(Some(public_summary));
        for mut reward in bs.rewards {
            if reward.category == ItemCategory::Weapon {
                reward.enchantment = bs.smith_enchant.clone();
            } else if reward.category == ItemCategory::Armor {
                reward.enchantment = bs.smith_glyph.clone();
            }
            result.items.push(reward);
        }
    }

    if let Some(imp) = quests::take_imp_pending(&mut dungeon.imp) {
        let reward_label = match imp.reward.provenance {
            crate::items::model::ItemProvenance::Quest(
                crate::items::model::QuestRewardRole::ImpRing {
                    identity_exact: true,
                    ..
                },
            ) => imp.reward.title(),
            _ => "ring reward".into(),
        };
        let public_summary = safe_summary(&imp.summary, &reward_label);
        result.summaries.push(imp.summary);
        result.public_labels.push(Some(public_summary));
        result.items.push(imp.reward);
    }
    result
}

pub(super) struct SpawnedQuestRewards {
    pub items: Vec<GeneratedItem>,
    pub summaries: Vec<String>,
    pub public_labels: Vec<Option<String>>,
    pub wand_rng_tail_sensitive: bool,
}

pub(super) fn spawn_npcs(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    map: &mut TerrainMap,
) -> SpawnedQuestRewards {
    let mut result = SpawnedQuestRewards {
        items: Vec::new(),
        summaries: Vec::new(),
        public_labels: Vec::new(),
        wand_rng_tail_sensitive: false,
    };
    if let Some(exit) = rooms.iter().find(|room| room.is_exit() && !room.is_empty()) {
        if let Some(ghost) = quests::try_spawn_ghost(dungeon, exit, map) {
            map.mob_occupied[ghost.cell] = true;
            map.known_mobs[ghost.cell] = Some("Ghost");
            let public_summary = safe_summary(&ghost.summary, "reward options");
            result.summaries.push(ghost.summary);
            result.public_labels.push(Some(public_summary));
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
            let public_summary = safe_summary(&wandmaker.summary, "one of two +1…+3 wands");
            result.summaries.push(wandmaker.summary);
            result.public_labels.push(Some(public_summary));
            result.items.extend([wandmaker.wand1, wandmaker.wand2]);
            result.wand_rng_tail_sensitive = true;
        }
    }
    result
}

fn safe_summary(summary: &str, reward_label: &str) -> String {
    let prefix = summary
        .split_once(" — ")
        .map_or(summary, |(prefix, _)| prefix);
    format!("{prefix} — {reward_label}")
}
