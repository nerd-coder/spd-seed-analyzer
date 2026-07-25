//! Per-class `Item.random()` ports.

use super::enchants;
use super::model::{GeneratedItem, ItemCategory};
use crate::random::Random;

/// Apply type-specific randomization after the item class is chosen.
pub fn randomize_item(item: &mut GeneratedItem, dungeon_depth: i32) {
    randomize_item_with_parchment(item, dungeon_depth, None);
}

/// Apply item randomization with an explicit Parchment Scrap level.
pub fn randomize_item_with_parchment(
    item: &mut GeneratedItem,
    dungeon_depth: i32,
    parchment_scrap_level: Option<i32>,
) {
    match item.category {
        ItemCategory::Weapon | ItemCategory::Missile => {
            randomize_weapon(item, parchment_scrap_level)
        }
        ItemCategory::Armor => randomize_armor(item),
        ItemCategory::Wand => randomize_wand(item),
        ItemCategory::Ring => randomize_ring(item),
        ItemCategory::Artifact => randomize_artifact(item),
        ItemCategory::Gold => randomize_gold(item, dungeon_depth),
        // Potion, Scroll, Stone, Seed, Food, Trinket, Other: base Item.random() is no-op
        _ => {}
    }
}

fn randomize_weapon(item: &mut GeneratedItem, parchment_scrap_level: Option<i32>) {
    // +0: 75%, +1: 20%, +2: 5%
    let mut n = 0;
    if Random::int_max(4) == 0 {
        n += 1;
        if Random::int_max(5) == 0 {
            n += 1;
        }
    }
    item.level = n;

    // Separate RNG so parchment scrap variance doesn't affect levelgen stream
    Random::push_generator_seeded(Random::long());
    let effect_roll = Random::float();
    let (curse_multiplier, enchant_multiplier) = match parchment_scrap_level {
        Some(0) => (1.5, 2.0),
        Some(1) => (2.0, 4.0),
        Some(2) => (1.0, 7.0),
        Some(3) => (0.0, 10.0),
        _ => (1.0, 1.0),
    };
    if effect_roll < 0.3 * curse_multiplier {
        item.enchantment = Some(enchants::random_weapon_curse(None).to_string());
        item.cursed = true;
    } else if effect_roll >= 1.0 - 0.1 * enchant_multiplier {
        item.enchantment = Some(enchants::random_weapon_enchant(None).to_string());
    }
    Random::pop_generator();

    if item.category == ItemCategory::Missile && item.quantity == 1 {
        // Most missiles default quantity 3 in constructors
        item.quantity = 3;
    }
}

fn randomize_armor(item: &mut GeneratedItem) {
    let mut n = 0;
    if Random::int_max(4) == 0 {
        n += 1;
        if Random::int_max(5) == 0 {
            n += 1;
        }
    }
    item.level = n;

    Random::push_generator_seeded(Random::long());
    let effect_roll = Random::float();
    if effect_roll < 0.3 {
        item.enchantment = Some(enchants::random_armor_curse(None).to_string());
        item.cursed = true;
    } else if effect_roll >= 1.0 - 0.15 {
        item.enchantment = Some(enchants::random_armor_glyph(None).to_string());
    }
    Random::pop_generator();
}

fn randomize_wand(item: &mut GeneratedItem) {
    let mut n = 0;
    if Random::int_max(3) == 0 {
        n += 1;
        if Random::int_max(5) == 0 {
            n += 1;
        }
    }
    item.level = n;
    if Random::float() < 0.3 {
        item.cursed = true;
    }
}

fn randomize_ring(item: &mut GeneratedItem) {
    let mut n = 0;
    if Random::int_max(3) == 0 {
        n += 1;
        if Random::int_max(5) == 0 {
            n += 1;
        }
    }
    item.level = n;
    if Random::float() < 0.3 {
        item.cursed = true;
    }
}

fn randomize_artifact(item: &mut GeneratedItem) {
    if Random::float() < 0.3 {
        item.cursed = true;
    }
}

fn randomize_gold(item: &mut GeneratedItem, depth: i32) {
    // Random.IntRange(30 + depth*10, 60 + depth*20)
    let min = 30 + depth * 10;
    let max = 60 + depth * 20;
    item.quantity = Random::int_range_inclusive(min, max);
}
