use super::conditions::{item_conditions, item_conditions_typed};
use super::{is_blacklisted, is_unpublished_main_loot, sources::reported_source};
use crate::items::model::{
    GeneratedItem, ItemProvenance, QuestRewardRole, RoomLootRole, ShopStockRole,
};
use crate::report::{ItemEnchantment, ItemEntry, ItemPredictionKind};

/// Project one concrete result that varies with gameplay. It is the item this
/// seed rolls on the default profile; player choices can shift it, so it is
/// never a universal seed claim.
pub(super) fn item_entry(item: &GeneratedItem) -> Option<ItemEntry> {
    let room_variable = matches!(
        item.provenance,
        ItemProvenance::Room(
            RoomLootRole::PoolEquipment
                | RoomLootRole::SuspiciousChestGold
                | RoomLootRole::SuspiciousChestMimicReward
                | RoomLootRole::CrystalChoiceHidden
                | RoomLootRole::CrystalVaultPrize
                | RoomLootRole::SecretHoneypotBomb
                | RoomLootRole::GrassyGravePrize
                | RoomLootRole::GrassyGraveGold
                | RoomLootRole::RuntimeSensitive
                | RoomLootRole::CryptArmor
                | RoomLootRole::StatueWeapon
        )
    );
    let quest_variable = matches!(
        item.provenance,
        ItemProvenance::Quest(
            QuestRewardRole::GhostWeapon { .. }
                | QuestRewardRole::WandmakerWand
                | QuestRewardRole::ImpVaultOption { .. }
                | QuestRewardRole::BlacksmithWeapon { .. }
                | QuestRewardRole::BlacksmithMissile { .. }
                | QuestRewardRole::BlacksmithArmor { .. }
                | QuestRewardRole::BlacksmithRoomWeapon { .. }
                | QuestRewardRole::BlacksmithRoomMissile { .. }
        )
    );
    let shop_variable = matches!(
        item.provenance,
        ItemProvenance::Shop(
            ShopStockRole::DeckWeapon { .. }
                | ShopStockRole::DeckMissile { .. }
                | ShopStockRole::ChooseBag
                | ShopStockRole::DeckRareWand
                | ShopStockRole::DeckRareRing
                | ShopStockRole::DeckRareArtifactOrRing
        )
    );
    // Main-loop and treasury mimics carry seed-generated prizes. Room prizes
    // that already have their own provenance stay on that path.
    let mimic_drop = !room_variable && is_mimic_carried_source(item.source.as_deref());
    if item.source.is_none()
        || !(room_variable || quest_variable || shop_variable || mimic_drop)
        || (is_blacklisted(item) && !room_variable)
        || (is_unpublished_main_loot(item) && !room_variable && !mimic_drop)
        || item.provenance == ItemProvenance::Quest(QuestRewardRole::WandmakerPersisted)
    {
        return None;
    }
    let quest_role = match item.provenance {
        ItemProvenance::Quest(role) => Some(role),
        _ => None,
    };
    // Publishing an internal category list on a concrete variable item makes
    // shared renderers present every class at the sample's exact level, which
    // is not a sound cross-profile claim. Imp still carries that list internally.
    let candidate_classes = if matches!(
        quest_role,
        Some(QuestRewardRole::WandmakerWand | QuestRewardRole::ImpVaultOption { .. })
    ) {
        Vec::new()
    } else {
        item.candidate_classes.clone()
    };
    let artifact_conditional = (item.artifact_conditional
        && item
            .source
            .as_deref()
            .is_some_and(|source| source.rsplit(':').next() == Some("heap")))
        || matches!(
            quest_role,
            Some(QuestRewardRole::ImpVaultOption { slot: 0 })
        );
    let notes = vec![
        "Varies with gameplay; player-controlled generation history can change this result.".into(),
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
        _ => reported_source(item, false),
    };
    let mut public_name: String = if item.category == crate::items::model::ItemCategory::Gold {
        "gold".into()
    } else {
        title.strip_prefix("cursed ").unwrap_or(&title).into()
    };
    if room_variable
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
        quantity_range: None,
        label: None,
        item: Some(crate::report::item_id(&item.class_name)),
        candidates: crate::report::item_ids(&candidate_classes),
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
                enchantment_type: spd_objects::Enchantment::from_class_name_lossy(
                    &enchantment_type,
                ),
                conditions: Vec::new(),
            }),
        prediction: ItemPredictionKind::Variable,
        spawn_conditions: item_conditions(artifact_conditional),
        conditions: item_conditions_typed(quest_role, false),
        notes,
        source,
    })
}

/// `mimic`, `golden_mimic`, and `ebony_mimic` sources, including `mimic:mimic`.
pub(super) fn is_mimic_carried_source(source: Option<&str>) -> bool {
    source.is_some_and(|source| {
        matches!(
            source.rsplit(':').next(),
            Some("mimic" | "golden_mimic" | "ebony_mimic")
        )
    })
}
