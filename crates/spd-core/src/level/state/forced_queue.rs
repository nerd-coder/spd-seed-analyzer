//! Seed-only projection of the exact initial `itemsToSpawn` queue.

use crate::items::model::{ForcedDropRole, GeneratedItem, ItemProvenance};
use crate::report::{ItemEntry, ItemPredictionKind};

fn constrained_spawn_entry(name: &str, category: &str, condition: impl Into<String>) -> ItemEntry {
    ItemEntry {
        name: name.into(),
        class_name: None,
        category: category.into(),
        tier: None,
        tier_range: None,
        level: None,
        level_range: None,
        cursed: None,
        prediction: ItemPredictionKind::Constrained,
        conditional_notes: vec![condition.into()],
        source: Some("guaranteed floor spawn".into()),
    }
}

fn guaranteed_spawn_entry(name: &str, class_name: &str, category: &str) -> ItemEntry {
    ItemEntry {
        name: name.into(),
        class_name: Some(class_name.into()),
        category: category.into(),
        tier: None,
        tier_range: None,
        level: Some(0),
        level_range: None,
        cursed: None,
        prediction: ItemPredictionKind::Exact,
        conditional_notes: Vec::new(),
        source: Some("guaranteed floor spawn".into()),
    }
}

pub(super) fn public_entries(depth: i32, initial: &[GeneratedItem]) -> Vec<ItemEntry> {
    if !matches!(depth, 1..=4 | 6..=9 | 11..=14 | 16..=19 | 21..=24) {
        return Vec::new();
    }
    let mut entries = vec![constrained_spawn_entry(
        "guaranteed food-category item",
        "food",
        "Its identity depends on persistent Generator history.",
    )];
    if initial
        .iter()
        .any(|item| item.provenance == ItemProvenance::Forced(ForcedDropRole::HallsTorch))
    {
        entries.insert(0, guaranteed_spawn_entry("two Torches", "Torch", "other"));
    }
    if initial
        .iter()
        .any(|item| item.provenance == ItemProvenance::Forced(ForcedDropRole::LargeFeelingFood))
    {
        entries.push(constrained_spawn_entry(
            "second guaranteed food-category item",
            "food",
            "Its identity depends on persistent Generator history.",
        ));
    }
    for item in initial {
        let (name, class_name) = match item.provenance {
            ItemProvenance::Forced(ForcedDropRole::HallsTorch) => continue,
            ItemProvenance::Forced(ForcedDropRole::StrengthPotion) => {
                ("Potion of Strength", "PotionOfStrength")
            }
            ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
                forbidden_runes_sensitive: false,
            }) => ("Scroll of Upgrade", "ScrollOfUpgrade"),
            ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
                forbidden_runes_sensitive: true,
            }) => {
                entries.push(constrained_spawn_entry(
                    "Scroll of Upgrade",
                    "scroll",
                    "Does not spawn when Forbidden Runes is active.",
                ));
                continue;
            }
            ItemProvenance::Forced(ForcedDropRole::ArcaneStylus) => ("Arcane Stylus", "Stylus"),
            ItemProvenance::Forced(ForcedDropRole::EnchantmentStone) => {
                ("Stone of Enchantment", "StoneOfEnchantment")
            }
            ItemProvenance::Forced(ForcedDropRole::IntuitionStone) => {
                ("Stone of Intuition", "StoneOfIntuition")
            }
            ItemProvenance::Forced(ForcedDropRole::TrinketCatalyst) => {
                ("Trinket Catalyst", "TrinketCatalyst")
            }
            _ => continue,
        };
        entries.push(guaranteed_spawn_entry(
            name,
            class_name,
            &format!("{:?}", item.category).to_ascii_lowercase(),
        ));
    }
    entries
}
