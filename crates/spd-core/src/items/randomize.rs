//! Per-class `Item.random()` ports.

use super::enchants;
use super::model::{GeneratedItem, ItemCategory};
use crate::level::trinkets;
use crate::random::Random;
use crate::TrinketKind;

/// Apply type-specific randomization after the item class is chosen.
pub fn randomize_item(item: &mut GeneratedItem, dungeon_depth: i32) {
    randomize_item_with_parchment(item, dungeon_depth, None);
}

/// Apply item randomization with an explicit Parchment Scrap level. `None`
/// reads the held trinket.
pub fn randomize_item_with_parchment(
    item: &mut GeneratedItem,
    dungeon_depth: i32,
    parchment_scrap_level: Option<i32>,
) {
    let parchment = || {
        parchment_scrap_level
            .unwrap_or_else(|| trinkets::trinket_level(TrinketKind::ParchmentScrap))
    };
    match item.category {
        ItemCategory::Weapon | ItemCategory::Missile => randomize_weapon(item, parchment()),
        ItemCategory::Armor => randomize_armor(item, parchment()),
        ItemCategory::Wand => randomize_wand(item),
        ItemCategory::Ring => randomize_ring(item),
        ItemCategory::Artifact => randomize_artifact(item),
        ItemCategory::Gold => randomize_gold(item, dungeon_depth),
        // Potion, Scroll, Stone, Seed, Food, Trinket, Other: base Item.random() is no-op
        _ => {}
    }
}

/// `Weapon/Armor.random()` upgrade roll: +0 75%, +1 20%, +2 5%.
fn roll_equipment_level() -> i32 {
    let mut n = 0;
    if Random::int_max(4) == 0 {
        n += 1;
        if Random::int_max(5) == 0 {
            n += 1;
        }
    }
    n
}

fn randomize_weapon(item: &mut GeneratedItem, parchment_level: i32) {
    item.level = roll_equipment_level();

    // Separate RNG so parchment scrap variance doesn't affect levelgen stream
    Random::push_generator_seeded(Random::long());
    let effect_roll = Random::float();
    if effect_roll < 0.3 * trinkets::parchment_curse_multiplier(parchment_level) {
        item.enchantment = Some(enchants::random_weapon_curse(None).to_string());
        item.cursed = true;
    } else if effect_roll >= 1.0 - 0.1 * trinkets::parchment_enchant_multiplier(parchment_level) {
        item.enchantment = Some(enchants::random_weapon_enchant(None).to_string());
    }
    Random::pop_generator();

    if item.category == ItemCategory::Missile && item.quantity == 1 {
        // Most missiles default quantity 3 in constructors
        item.quantity = 3;
    }
}

fn randomize_armor(item: &mut GeneratedItem, parchment_level: i32) {
    item.level = roll_equipment_level();

    Random::push_generator_seeded(Random::long());
    let effect_roll = Random::float();
    if effect_roll < 0.3 * trinkets::parchment_curse_multiplier(parchment_level) {
        item.enchantment = Some(enchants::random_armor_curse(None).to_string());
        item.cursed = true;
    } else if effect_roll >= 1.0 - 0.15 * trinkets::parchment_enchant_multiplier(parchment_level) {
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
