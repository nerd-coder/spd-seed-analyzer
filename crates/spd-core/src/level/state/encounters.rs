use std::collections::BTreeMap;

use crate::items::model::{GeneratedItem, ItemCategory};
use crate::report::{
    CombatReward, CombatRewardPrediction, FloorMap, InitialEncounter, RewardChance,
};

pub(super) fn initial_encounters(
    depth: i32,
    runtime_sensitive_layout: bool,
    map: Option<&FloorMap>,
    placed_items: &[GeneratedItem],
) -> Vec<InitialEncounter> {
    if depth != 1 || runtime_sensitive_layout {
        return Vec::new();
    }
    let Some(map) = map else {
        return Vec::new();
    };

    let mut counts = BTreeMap::<&str, u32>::new();
    for mob in &map.mobs {
        *counts.entry(&mob.class_name).or_default() += 1;
    }

    counts
        .into_iter()
        .map(|(class_name, quantity)| InitialEncounter {
            class_name: class_name.to_string(),
            name: encounter_name(class_name).to_string(),
            quantity,
            combat_rewards: combat_rewards(class_name, placed_items),
        })
        .collect()
}

fn combat_rewards(class_name: &str, placed_items: &[GeneratedItem]) -> Vec<CombatReward> {
    match class_name {
        "Albino" => vec![fixed_reward("Mystery Meat", "MysteryMeat", "food")],
        "Piranha" => vec![fixed_reward("Mystery Meat", "MysteryMeat", "food")],
        "PhantomPiranha" => vec![fixed_reward("Phantom Meat", "PhantomMeat", "food")],
        "Snake" => vec![CombatReward {
            name: "random seed".into(),
            class_name: None,
            category: "seed".into(),
            prediction: CombatRewardPrediction::RuntimeChance,
            chance: Some(RewardChance {
                numerator: 1,
                denominator: 4,
            }),
        }],
        "Statue" | "ArmoredStatue" => generated_rewards(placed_items, |source| {
            source.split(':').next() == Some("StatueRoom")
        }),
        "Mimic" => generated_rewards(placed_items, |source| {
            source.rsplit(':').next() == Some("mimic")
        }),
        "GoldenMimic" => generated_rewards(placed_items, |source| {
            source.rsplit(':').next() == Some("golden_mimic")
        }),
        "CrystalMimic" => generated_rewards(placed_items, |source| {
            source.rsplit(':').next() == Some("crystal_mimic")
        }),
        _ => Vec::new(),
    }
}

fn fixed_reward(name: &str, class_name: &str, category: &str) -> CombatReward {
    CombatReward {
        name: name.into(),
        class_name: Some(class_name.into()),
        category: category.into(),
        prediction: CombatRewardPrediction::Guaranteed,
        chance: None,
    }
}

fn generated_rewards(
    placed_items: &[GeneratedItem],
    source_matches: impl Fn(&str) -> bool,
) -> Vec<CombatReward> {
    placed_items
        .iter()
        .filter(|item| item.source.as_deref().is_some_and(&source_matches))
        .map(|item| CombatReward {
            name: item.title(),
            class_name: Some(item.class_name.clone()),
            category: category_name(item.category).into(),
            prediction: CombatRewardPrediction::GeneratedWithFloor,
            chance: None,
        })
        .collect()
}

fn category_name(category: ItemCategory) -> &'static str {
    match category {
        ItemCategory::Weapon => "weapon",
        ItemCategory::Armor => "armor",
        ItemCategory::Missile => "missile",
        ItemCategory::Wand => "wand",
        ItemCategory::Ring => "ring",
        ItemCategory::Artifact => "artifact",
        ItemCategory::Potion => "potion",
        ItemCategory::Scroll => "scroll",
        ItemCategory::Stone => "stone",
        ItemCategory::Seed => "seed",
        ItemCategory::Food => "food",
        ItemCategory::Gold => "gold",
        ItemCategory::Trinket => "trinket",
        ItemCategory::Other => "other",
    }
}

fn encounter_name(class_name: &str) -> &str {
    match class_name {
        "Albino" => "Albino Rat",
        "ArmoredStatue" => "Armored Statue",
        "CrystalMimic" => "Crystal Mimic",
        "GoldenMimic" => "Golden Mimic",
        "PhantomPiranha" => "Phantom Piranha",
        "DemonSpawner" => "Demon Spawner",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mimic_variants_only_report_their_own_carried_rewards() {
        let items = [
            carried("PotionOfHealing", "SuspiciousChestRoom:mimic"),
            carried("RingOfWealth", "golden_mimic"),
            carried("WandOfFireblast", "CrystalVaultRoom:crystal_mimic"),
        ];

        assert_eq!(reward_classes("Mimic", &items), ["PotionOfHealing"]);
        assert_eq!(reward_classes("GoldenMimic", &items), ["RingOfWealth"]);
        assert_eq!(reward_classes("CrystalMimic", &items), ["WandOfFireblast"]);
    }

    fn carried(class_name: &str, source: &str) -> GeneratedItem {
        let mut item = GeneratedItem::new(class_name, ItemCategory::Other);
        item.source = Some(source.into());
        item
    }

    fn reward_classes(class_name: &str, items: &[GeneratedItem]) -> Vec<String> {
        combat_rewards(class_name, items)
            .into_iter()
            .filter_map(|reward| reward.class_name)
            .collect()
    }
}
