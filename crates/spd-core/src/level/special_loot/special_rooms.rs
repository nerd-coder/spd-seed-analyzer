//! Standard special-room prize painters (Crypt, Armory, Library, …).

use super::placement::{
    burn_drop_pos, burn_terrain_pos, find_prize_item, find_prize_item_category,
};
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::enchants;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::random::Random;
use crate::rooms::room::Room;

pub(super) fn crypt_prize(dungeon: &mut DungeonState) -> PlacedLoot {
    // Generator.randomArmor((depth/5)+1)
    let mut prize = dungeon
        .generator
        .random_armor((dungeon.depth / 5) + 1, dungeon.depth);
    // always roll a curse glyph (parchment scrap isolation)
    let curse = enchants::random_armor_curse(None).to_string();
    if !prize.cursed {
        prize.level += 1;
        if !is_good_glyph(&prize) {
            prize.enchantment = Some(curse);
        }
    }
    prize.cursed = true;
    prize.source = Some("CryptRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "tomb",
    }
}

fn is_good_glyph(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => !matches!(
            e,
            "AntiEntropy"
                | "Corrosion"
                | "Displacement"
                | "Metabolism"
                | "Multiplicity"
                | "Stench"
                | "Overgrowth"
                | "Bulk"
        ),
        None => false,
    }
}

pub(super) fn is_curse_enchant(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => matches!(
            e,
            "Annoying"
                | "Displacing"
                | "Dazzling"
                | "Explosive"
                | "Sacrificial"
                | "Wayward"
                | "Polarized"
                | "Friendly"
                | "AntiEntropy"
                | "Corrosion"
                | "Displacement"
                | "Metabolism"
                | "Multiplicity"
                | "Stench"
                | "Overgrowth"
                | "Bulk"
        ),
        None => false,
    }
}

pub(super) fn armory_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    // statue position Random.Int(2)
    let _ = Random::int_max(2);

    let n = Random::int_range_inclusive(2, 3);
    let mut prize_cats = [1.0f32, 1.0, 1.0, 1.0];
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let index = Random::chances(&prize_cats).max(0) as usize;
        prize_cats[index] = 0.0;
        let mut item = match index {
            0 => bomb_random(),
            1 => dungeon
                .generator
                .random_weapon(dungeon.depth / 5, false, dungeon.depth),
            2 => dungeon
                .generator
                .random_armor(dungeon.depth / 5, dungeon.depth),
            _ => dungeon
                .generator
                .random_missile(dungeon.depth / 5, false, dungeon.depth),
        };
        item.source = Some("ArmoryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }

    if let Some(mut cata) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
        burn_drop_pos(room, &mut occupied);
        cata.source = Some("ArmoryRoom".into());
        out.push(PlacedLoot {
            item: cata,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn bomb_random() -> GeneratedItem {
    // Bomb.random: 1/4 DoubleBomb else Bomb
    if Random::int_max(4) == 0 {
        let mut b = GeneratedItem::new("DoubleBomb", ItemCategory::Other);
        b.quantity = 2;
        b
    } else {
        GeneratedItem::new("Bomb", ItemCategory::Other)
    }
}

pub(super) fn library_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let n = Random::normal_int_range(1, 3);
    let mut occupied = Vec::new();
    for i in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = if i == 0 {
            if Random::int_max(2) == 0 {
                GeneratedItem::new("ScrollOfIdentify", ItemCategory::Scroll)
            } else {
                GeneratedItem::new("ScrollOfRemoveCurse", ItemCategory::Scroll)
            }
        } else {
            library_prize(dungeon, items_to_spawn)
        };
        item.source = Some("LibraryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

fn library_prize(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> GeneratedItem {
    if let Some(cata) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
        return cata;
    }
    if let Some(scroll) = find_prize_item_category(items_to_spawn, ItemCategory::Scroll) {
        return scroll;
    }
    dungeon
        .generator
        .random_category(Category::Scroll, dungeon.depth)
}

pub(super) fn treasury_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    // center statue — no RNG
    let heap_chest = Random::int_max(2) == 0;
    let n = Random::int_range_inclusive(2, 3);
    let mimic_chance = 0.2f32; // 1/5 without MimicTooth
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
                randomize_item(&mut g, dungeon.depth);
                g
            });
        let heap_type = if heap_chest && dungeon.depth > 1 && Random::float() < mimic_chance {
            item.source = Some("TreasuryRoom:mimic".into());
            "mimic"
        } else {
            item.source = Some("TreasuryRoom".into());
            if heap_chest {
                "chest"
            } else {
                "heap"
            }
        };
        out.push(PlacedLoot { item, heap_type });
    }
    if !heap_chest {
        for _ in 0..6 {
            let _ = Random::int_range_inclusive(room.left + 1, room.right - 1);
            let _ = Random::int_range_inclusive(room.top + 1, room.bottom - 1);
            // small gold piles blacklisted from report — still burn quantity RNG
            let _qty = Random::int_range_inclusive(5, 12);
        }
    }
    out
}

pub(super) fn pool_prize(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    // pedestal position is geometric; prize first
    let mut prize = if Random::int_max(3) == 0 {
        find_prize_item(items_to_spawn, None).unwrap_or_else(|| pool_equip(dungeon))
    } else {
        pool_equip(dungeon)
    };
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("PoolRoom".into());

    // piranha placement burns RNG (3 piranhas)
    for _ in 0..3 {
        burn_terrain_pos(room, /*water-like*/ true);
    }

    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

fn pool_equip(dungeon: &mut DungeonState) -> GeneratedItem {
    let floor = (dungeon.depth / 5) + 1;
    match Random::int_max(5) {
        0 | 1 => dungeon.generator.random_weapon(floor, false, dungeon.depth),
        2 => dungeon
            .generator
            .random_missile(floor, false, dungeon.depth),
        _ => dungeon.generator.random_armor(floor, dungeon.depth),
    }
}

pub(super) fn storage_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let mut honey = Random::int_max(2) == 0;
    let n = Random::int_range_inclusive(3, 4);
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = if honey {
            honey = false;
            GeneratedItem::new("Honeypot", ItemCategory::Other)
        } else {
            storage_prize(dungeon, items_to_spawn)
        };
        item.source = Some("StorageRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    // addItemToSpawn PotionOfLiquidFlame — deferred to createItems path in full game;
    // we push into items_to_spawn so it can land as forced.
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLiquidFlame",
        ItemCategory::Potion,
    ));
    out
}

pub(super) fn storage_prize(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> GeneratedItem {
    if Random::int_max(3) != 0 {
        if let Some(p) = find_prize_item(items_to_spawn, None) {
            return p;
        }
    }
    let cat = *Random::one_of(&[
        Category::Potion,
        Category::Scroll,
        Category::Food,
        Category::Gold,
    ]);
    dungeon.generator.random_category(cat, dungeon.depth)
}

pub(super) fn runestone_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let n = Random::normal_int_range(2, 3);
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = if let Some(c) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
            c
        } else if let Some(s) = find_prize_item_category(items_to_spawn, ItemCategory::Stone) {
            s
        } else {
            dungeon
                .generator
                .random_category(Category::Stone, dungeon.depth)
        };
        item.source = Some("RunestoneRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn laboratory_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    // pot position Random.Int(2)
    let _ = Random::int_max(2);

    let mut occupied = Vec::new();
    burn_drop_pos(room, &mut occupied);
    // EnergyCrystal x5 — blacklisted elsewhere? report as lab energy
    let mut crystal = GeneratedItem::new("EnergyCrystal", ItemCategory::Other);
    crystal.quantity = 5;
    crystal.source = Some("LaboratoryRoom".into());
    // blacklisted in is_blacklisted — skip reporting energy
    let _ = crystal;

    let n = Random::normal_int_range(1, 2);
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = if let Some(c) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
            c
        } else if let Some(p) = find_prize_item(items_to_spawn, Some("PotionOfStrength")) {
            p
        } else {
            let cat = *Random::one_of(&[Category::Potion, Category::Stone]);
            dungeon.generator.random_category(cat, dungeon.depth)
        };
        item.source = Some("LaboratoryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    // alchemy guide pages — skip (document state not tracked)
    out
}

pub(super) fn statue_weapon(dungeon: &mut DungeonState, _room: &Room) -> PlacedLoot {
    // Statue.random: 10% armored (rat skull default)
    let _armored = Random::float() < 0.1;
    let mut weapon = dungeon
        .generator
        .random_category(Category::Weapon, dungeon.depth);
    weapon.cursed = false;
    weapon.enchantment = Some(enchants::random_weapon_enchant(None).to_string());
    weapon.source = Some("StatueRoom".into());
    PlacedLoot {
        item: weapon,
        heap_type: "statue",
    }
}
