use super::conditions::{item_conditions, item_conditions_typed};
use super::{is_blacklisted, is_unpublished_main_loot};
use crate::items::model::{GeneratedItem, ItemProvenance, QuestRewardRole};
use crate::report::{ItemEnchantment, ItemEntry, ItemPredictionKind};

/// Project one concrete result from the fresh/no-history replay. This is kept
/// separate from the conservative public item list: a baseline is useful for
/// a compact seed-finder-style report, but it is not a universal seed claim.
pub(super) fn item_entry(item: &GeneratedItem) -> Option<ItemEntry> {
    if item.source.is_none()
        || is_blacklisted(item)
        || is_unpublished_main_loot(item)
        || item.provenance == ItemProvenance::Quest(QuestRewardRole::WandmakerPersisted)
    {
        return None;
    }
    let quest_role = match item.provenance {
        ItemProvenance::Quest(role) => Some(role),
        _ => None,
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
    Some(ItemEntry {
        name: title.strip_prefix("cursed ").unwrap_or(&title).into(),
        quantity: item.quantity.max(1),
        class_name: Some(item.class_name.clone()),
        candidate_classes: item.candidate_classes.clone(),
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
        source: item.source.clone(),
    })
}
