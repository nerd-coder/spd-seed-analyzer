use super::conditions::{item_conditions, item_conditions_typed};
use super::{is_blacklisted, is_unpublished_main_loot};
use crate::items::model::{GeneratedItem, ItemProvenance, QuestRewardRole, RoomLootRole};
use crate::report::{ItemEnchantment, ItemEntry, ItemPredictionKind};

/// Project one concrete result from the fresh/no-history replay. The result is
/// marked as baseline analysis inside the public item list: it is useful for a
/// compact seed-finder-style report, but it is not a universal seed claim.
pub(super) fn item_entry(item: &GeneratedItem) -> Option<ItemEntry> {
    let room_baseline = matches!(
        item.provenance,
        ItemProvenance::Room(
            RoomLootRole::PoolEquipment
                | RoomLootRole::SuspiciousChestGold
                | RoomLootRole::SuspiciousChestMimicReward
        )
    );
    if item.source.is_none()
        || (is_blacklisted(item) && !room_baseline)
        || (is_unpublished_main_loot(item) && !room_baseline)
        || item.provenance == ItemProvenance::Quest(QuestRewardRole::WandmakerPersisted)
    {
        return None;
    }
    let quest_role = match item.provenance {
        ItemProvenance::Quest(role) => Some(role),
        _ => None,
    };
    // Wandmaker and Imp keep their full categories internally for parity and
    // conservative reasoning. Publishing those categories on a concrete
    // baseline sample makes shared renderers present every class at the
    // sample's exact level, which is not a sound cross-profile claim.
    let candidate_classes = if matches!(
        quest_role,
        Some(QuestRewardRole::WandmakerWand | QuestRewardRole::ImpRing)
    ) {
        Vec::new()
    } else {
        item.candidate_classes.clone()
    };
    let artifact_conditional = item.artifact_conditional
        && item
            .source
            .as_deref()
            .is_some_and(|source| source.rsplit(':').next() == Some("heap"));
    let notes = vec![
        "Fresh/no-history baseline replay; player-controlled generation history can change this result."
            .into(),
    ];
    let title = item.title();
    let source = match item.provenance {
        ItemProvenance::Room(RoomLootRole::PoolEquipment) => Some("PoolRoom:equipment".into()),
        ItemProvenance::Room(RoomLootRole::SuspiciousChestGold) => {
            Some("SuspiciousChestRoom:gold".into())
        }
        ItemProvenance::Room(RoomLootRole::SuspiciousChestMimicReward) => {
            Some("SuspiciousChestRoom:mimic_reward".into())
        }
        _ => item.source.clone(),
    };
    let mut public_name: String = if item.category == crate::items::model::ItemCategory::Gold {
        "gold".into()
    } else {
        title.strip_prefix("cursed ").unwrap_or(&title).into()
    };
    if room_baseline
        && item.level == 0
        && matches!(
            item.category,
            crate::items::model::ItemCategory::Weapon
                | crate::items::model::ItemCategory::Armor
                | crate::items::model::ItemCategory::Missile
                | crate::items::model::ItemCategory::Ring
        )
    {
        public_name.push_str(" +0");
    }
    Some(ItemEntry {
        name: public_name,
        quantity: item.quantity.max(1),
        class_name: Some(item.class_name.clone()),
        candidate_classes,
        category: format!("{:?}", item.category).to_ascii_lowercase(),
        tier: crate::generator::weapon_tier_for_class(&item.class_name),
        tier_range: None,
        level: Some(item.level),
        level_range: None,
        cursed: Some(item.cursed),
        enchantment: item
            .enchantment
            .clone()
            .map(|enchantment_type| ItemEnchantment {
                enchantment_type,
                conditions: Vec::new(),
            }),
        prediction: ItemPredictionKind::Baseline,
        spawn_conditions: item_conditions(artifact_conditional),
        conditions: item_conditions_typed(quest_role, false),
        notes,
        source,
    })
}
