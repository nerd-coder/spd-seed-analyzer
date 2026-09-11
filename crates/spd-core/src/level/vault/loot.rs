//! `VaultLevel.build` pre-queues T0 equipment and consumables before `initRooms`.
//!
//! `setupEquipment` fills every tier on the first call, so T1–T3 rolls happen
//! even though only T0 items are taken here. Those draws must land before
//! room chances or GridBuilder sees the wrong stream.

#![allow(dead_code)] // generate is test-only until PR 4

use std::collections::HashSet;

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::enchants::{random_armor_glyph, random_weapon_enchant};
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::random::Random;

const BANNED_WANDS: &[&str] = &["WandOfRegrowth", "WandOfTransfusion", "WandOfCorruption"];
const BANNED_RINGS: &[&str] = &["RingOfWealth", "RingOfMight", "RingOfForce"];

struct EquipmentPool {
    by_tier: [Vec<Option<GeneratedItem>>; 4],
    generated: HashSet<String>,
    higher_idx: usize,
    lower_idx: usize,
}

pub(super) fn queue_floor_loot(dungeon: &mut DungeonState) {
    dungeon.items_to_spawn.clear();
    let mut pool = EquipmentPool {
        by_tier: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        generated: HashSet::new(),
        higher_idx: 0,
        lower_idx: 0,
    };
    for _ in 0..4 {
        let _ = create_equipment(&mut pool, dungeon, 0);
    }
    dungeon
        .items_to_spawn
        .push(GeneratedItem::new("Dart", ItemCategory::Missile));
    setup_consumables();
    for _ in 0..3 {
        let _ = dungeon
            .generator
            .random_using_defaults(Category::Food, dungeon.depth);
    }
}

fn create_equipment(
    pool: &mut EquipmentPool,
    dungeon: &mut DungeonState,
    loot_tier: usize,
) -> GeneratedItem {
    setup_equipment(pool, dungeon);
    let list = &mut pool.by_tier[loot_tier];
    let idx = if loot_tier >= 2 {
        let idx = pool.higher_idx;
        if idx >= list.len() {
            pool.higher_idx = 0;
            0
        } else {
            pool.higher_idx += 1;
            idx
        }
    } else {
        let idx = pool.lower_idx;
        if idx >= list.len() {
            pool.lower_idx = 0;
            0
        } else {
            pool.lower_idx += 1;
            idx
        }
    };
    let mut slot = idx;
    while list[slot].is_none() {
        slot += 1;
        if slot >= list.len() {
            slot = 0;
        }
    }
    let mut loot = list[slot].take().expect("vault equipment slot");
    loot.cursed = false;
    loot
}

fn setup_equipment(pool: &mut EquipmentPool, dungeon: &mut DungeonState) {
    for tier in 0..4 {
        if pool.by_tier[tier].iter().all(Option::is_none) {
            pool.by_tier[tier] = setup_equipment_at_tier(pool, dungeon, tier)
                .into_iter()
                .map(Some)
                .collect();
        }
    }
}

fn setup_equipment_at_tier(
    pool: &mut EquipmentPool,
    dungeon: &mut DungeonState,
    loot_tier: usize,
) -> Vec<GeneratedItem> {
    let mut loot_list = Vec::new();
    let first_wep = match loot_tier {
        2 => Category::WepT3,
        3 => Category::WepT4,
        _ => Category::WepT2,
    };
    let loot = draw_unique(
        dungeon,
        first_wep,
        loot_tier > 1,
        &mut pool.generated,
        |_| false,
    );
    loot_list.push(finish_weapon(
        loot,
        if loot_tier == 0 {
            loot_tier as i32
        } else {
            loot_tier as i32 + 1
        },
        loot_tier,
    ));

    let second_wep = match loot_tier {
        0 => Category::WepT2,
        1 => Category::WepT3,
        2 => Category::WepT4,
        _ => Category::WepT5,
    };
    let loot = draw_unique(
        dungeon,
        second_wep,
        loot_tier > 0,
        &mut pool.generated,
        |_| false,
    );
    loot_list.push(finish_weapon(loot, loot_tier as i32, loot_tier));

    let missile = match loot_tier {
        0 => Category::MisT2,
        1 => Category::MisT3,
        2 => Category::MisT4,
        _ => Category::MisT5,
    };
    let loot = draw_unique(dungeon, missile, true, &mut pool.generated, |_| false);
    loot_list.push(finish_weapon(loot, loot_tier as i32, loot_tier));

    let armor_name = match loot_tier {
        0 => "LeatherArmor",
        1 => "MailArmor",
        2 => "ScaleArmor",
        _ => "PlateArmor",
    };
    pool.generated.insert(armor_name.to_string());
    let mut armor = GeneratedItem::new(armor_name, ItemCategory::Armor);
    armor.level = loot_tier as i32;
    if Random::int_max(3) >= loot_tier as i32 {
        armor.enchantment = None;
    } else {
        armor.enchantment = Some(random_armor_glyph(armor.enchantment.as_deref()).to_string());
    }
    loot_list.push(armor);

    let wand = draw_unique(dungeon, Category::Wand, true, &mut pool.generated, |name| {
        BANNED_WANDS.contains(&name)
    });
    let mut wand = wand;
    wand.level = loot_tier as i32;
    loot_list.push(wand);

    let ring = draw_unique(dungeon, Category::Ring, true, &mut pool.generated, |name| {
        BANNED_RINGS.contains(&name)
    });
    let mut ring = ring;
    ring.level = loot_tier as i32;
    loot_list.push(ring);

    loot_list
}

fn draw_unique(
    dungeon: &mut DungeonState,
    cat: Category,
    reject_duplicate: bool,
    generated: &mut HashSet<String>,
    banned: impl Fn(&str) -> bool,
) -> GeneratedItem {
    loop {
        let item = dungeon.generator.random_using_defaults(cat, dungeon.depth);
        if (!reject_duplicate || !generated.contains(&item.class_name)) && !banned(&item.class_name)
        {
            generated.insert(item.class_name.clone());
            return item;
        }
    }
}

fn finish_weapon(mut loot: GeneratedItem, level: i32, loot_tier: usize) -> GeneratedItem {
    loot.level = level;
    if Random::int_max(3) >= loot_tier as i32 {
        loot.enchantment = None;
    } else {
        loot.enchantment = Some(random_weapon_enchant(loot.enchantment.as_deref()).to_string());
    }
    loot
}

fn setup_consumables() {
    // `Random.oneOf` is watabou; `Collections.shuffle` in setupConsumables is
    // the JDK's unseeded Random and must not touch this stream.
    let _ = Random::int_max(2);
    let _ = Random::int_max(3);
    let _ = Random::int_max(2);
    let _ = Random::int_max(3);

    let _ = Random::int_max(2);
    let _ = Random::int_max(3);
    let _ = Random::int_max(2);
    let _ = Random::int_max(3);

    let _ = Random::int_max(2);
    let _ = Random::int_max(2);
    let _ = Random::int_max(2);
    let _ = Random::int_max(2);

    let _ = Random::int_max(2);
    let _ = Random::int_max(2);
    let _ = Random::int_max(2);
    let _ = Random::int_max(2);
}
