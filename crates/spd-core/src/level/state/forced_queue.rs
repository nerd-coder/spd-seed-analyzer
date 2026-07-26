//! Seed-only projection of the exact initial `itemsToSpawn` queue.

use crate::items::model::{ForcedDropRole, GeneratedItem, ItemProvenance};
use crate::report::{ItemEntry, ItemPredictionKind};

fn constrained_entry(name: &str, category: &str, condition: impl Into<String>) -> ItemEntry {
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
        source: Some("initial forced queue".into()),
    }
}

pub(super) fn public_entries(depth: i32, initial: &[GeneratedItem]) -> Vec<ItemEntry> {
    if !matches!(depth, 1..=4 | 6..=9 | 11..=14 | 16..=19 | 21..=24) {
        return Vec::new();
    }
    let mut entries = vec![constrained_entry(
        "guaranteed food-category item",
        "food",
        "One food-category item is generated on every regular floor; persistent Generator history can change its identity, and a room can change how it appears or where it is found.",
    )];
    if initial
        .iter()
        .any(|item| item.provenance == ItemProvenance::Forced(ForcedDropRole::HallsTorch))
    {
        entries.insert(
            0,
            constrained_entry(
                "two initially queued Torches",
                "other",
                "HallsLevel queues exactly two Torches before the base food source on regular depths 21–24; room consumption, survival, heap type, and final cells are not asserted.",
            ),
        );
    }
    if initial
        .iter()
        .any(|item| item.provenance == ItemProvenance::Forced(ForcedDropRole::LargeFeelingFood))
    {
        entries.push(constrained_entry(
            "second guaranteed food-category item",
            "food",
            "The seeded Large feeling generates a second food-category item; Generator history can change identity, appearance, and final placement.",
        ));
    }
    for item in initial {
        let (name, condition) = match item.provenance {
            ItemProvenance::Forced(ForcedDropRole::HallsTorch) => continue,
            ItemProvenance::Forced(ForcedDropRole::StrengthPotion) => (
                "initially queued Potion of Strength",
                "The seed-stable limited-drop schedule queues this identity; a room may consume it, so final heap and placement are not asserted.",
            ),
            ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
                forbidden_runes_sensitive: false,
            }) => (
                "initially queued Scroll of Upgrade",
                "The odd-count seed-stable schedule queues this identity even with Forbidden Runes; a room may consume it, so final heap and placement are not asserted.",
            ),
            ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
                forbidden_runes_sensitive: true,
            }) => (
                "Scroll of Upgrade",
                "Removed when Forbidden Runes is active; otherwise generated, with its final source and placement not asserted.",
            ),
            ItemProvenance::Forced(ForcedDropRole::ArcaneStylus) => (
                "initially queued Arcane Stylus",
                "The seed-stable limited-drop schedule queues this identity; a room may consume it, so final heap and placement are not asserted.",
            ),
            ItemProvenance::Forced(ForcedDropRole::EnchantmentStone) => (
                "initially queued Stone of Enchantment",
                "The seed-stable limited-drop schedule queues this identity; a room may consume it, so final heap and placement are not asserted.",
            ),
            ItemProvenance::Forced(ForcedDropRole::IntuitionStone) => (
                "initially queued Stone of Intuition",
                "The seed-stable limited-drop schedule queues this identity; a room may consume it, so final heap and placement are not asserted.",
            ),
            ItemProvenance::Forced(ForcedDropRole::TrinketCatalyst) => (
                "initially queued Trinket Catalyst",
                "The seed-stable limited-drop schedule queues this identity; a room may consume it, otherwise its locked chest and key placement are not asserted.",
            ),
            _ => continue,
        };
        entries.push(constrained_entry(
            name,
            &format!("{:?}", item.category).to_ascii_lowercase(),
            condition,
        ));
    }
    entries
}
