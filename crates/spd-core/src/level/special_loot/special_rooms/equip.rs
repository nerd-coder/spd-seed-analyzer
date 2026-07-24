//! Equip-focused special rooms: Crypt, Armory, Pool, Statue.

#[cfg(test)]
use super::super::placement::burn_terrain_pos;
use super::super::placement::find_prize_item;
use super::{is_curse_enchant, is_good_glyph};
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::Point;
use crate::items::enchants;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, EMPTY, STATUE, WALL, WATER};
use crate::random::Random;
use crate::rooms::room::Room;

pub fn crypt_prize(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    // CryptRoom.java:51 — IronKey pushed before prize generation (zero-RNG append)
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
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
pub fn armory_prizes_on_map(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    entrance: Point,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);
    let side = Random::int_max(2) == 0;
    let statue = if entrance.x == room.left {
        Point::new(
            room.right - 1,
            if side { room.top + 1 } else { room.bottom - 1 },
        )
    } else if entrance.x == room.right {
        Point::new(
            room.left + 1,
            if side { room.top + 1 } else { room.bottom - 1 },
        )
    } else if entrance.y == room.top {
        Point::new(
            if side { room.left + 1 } else { room.right - 1 },
            room.bottom - 1,
        )
    } else {
        Point::new(
            if side { room.left + 1 } else { room.right - 1 },
            room.top + 1,
        )
    };
    set_terrain(map, statue, STATUE);

    let n = Random::int_range_inclusive(2, 3);
    let mut prize_cats = [1.0f32, 1.0, 1.0, 1.0];
    for _ in 0..n {
        let cell = armory_drop_cell(room, map);
        map.heap_occupied[cell] = true;
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
        map.record_heap(cell, "heap", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }

    if let Some(mut cata) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
        let cell = armory_drop_cell(room, map);
        cata.source = Some("ArmoryRoom".into());
        map.record_heap(cell, "heap", cata.clone());
        out.push(PlacedLoot {
            item: cata,
            heap_type: "heap",
        });
    }
    // ArmoryRoom.java:78 — IronKey is the last statement of paint()
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    out
}

fn armory_drop_cell(room: &Room, map: &TerrainMap) -> usize {
    loop {
        let point = room.random();
        let cell = map
            .point_to_cell(point.x, point.y)
            .expect("placed ArmoryRoom point is inside map");
        if map.map[cell] == EMPTY && !map.heap_occupied[cell] {
            return cell;
        }
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            set_terrain(map, Point::new(x, y), terrain);
        }
    }
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    for y in (room.top + margin)..=(room.bottom - margin) {
        for x in (room.left + margin)..=(room.right - margin) {
            set_terrain(map, Point::new(x, y), terrain);
        }
    }
}

fn set_terrain(map: &mut TerrainMap, point: Point, terrain: i32) {
    if let Some(cell) = map.point_to_cell(point.x, point.y) {
        map.map[cell] = terrain;
    }
}

pub fn bomb_random() -> GeneratedItem {
    // Bomb.random: 1/4 DoubleBomb else Bomb
    if Random::int_max(4) == 0 {
        let mut b = GeneratedItem::new("DoubleBomb", ItemCategory::Other);
        b.quantity = 2;
        b
    } else {
        GeneratedItem::new("Bomb", ItemCategory::Other)
    }
}
#[cfg(test)]
pub fn pool_prize(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    let prize = pool_prize_without_piranhas(dungeon, items_to_spawn);
    for _ in 0..3 {
        let _ = Random::float();
        burn_terrain_pos(room, /*water-like*/ true);
    }
    prize
}

pub fn pool_prize_on_map(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    let prize = pool_prize_without_piranhas(dungeon, items_to_spawn);
    for _ in 0..3 {
        // Piranha.random always rolls the rare PhantomPiranha replacement.
        let phantom = Random::float() < 1.0 / 50.0;
        loop {
            let point = room.random();
            let Some(cell) = map.point_to_cell(point.x, point.y) else {
                continue;
            };
            if map.map[cell] == WATER && !map.mob_occupied[cell] {
                map.mob_occupied[cell] = true;
                map.known_mobs[cell] = Some(if phantom { "PhantomPiranha" } else { "Piranha" });
                break;
            }
        }
    }
    prize
}

fn pool_prize_without_piranhas(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    // Java returns immediately when `findPrizeItem` succeeds. In particular,
    // that path does not consume the equipment-category or upgrade rolls.
    if Random::int_max(3) == 0 {
        if let Some(mut prize) = find_prize_item(items_to_spawn, None) {
            prize.source = Some("PoolRoom".into());
            items_to_spawn.push(GeneratedItem::new(
                "PotionOfInvisibility",
                ItemCategory::Potion,
            ));
            return PlacedLoot {
                item: prize,
                heap_type: "chest",
            };
        }
    }
    let mut prize = pool_equip(dungeon);
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("PoolRoom".into());
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfInvisibility",
        ItemCategory::Potion,
    ));
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
pub fn statue_weapon(
    dungeon: &mut DungeonState,
    _room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    // StatueRoom.java:46 — IronKey pushed before Statue.random() (zero-RNG append)
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
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
