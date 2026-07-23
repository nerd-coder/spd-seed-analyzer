//! Consumable/stock special rooms: Library, Treasury, Storage, Runestone, Laboratory.

use super::super::placement::{burn_drop_pos, find_prize_item, find_prize_item_category};
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, EMPTY, STATUE, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

pub fn library_prizes(
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
    // LibraryRoom.java:65 — IronKey is the last statement of paint()
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
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

pub fn treasury_prizes_on_map(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);
    let statue = room.as_rect().center_room();
    set_terrain(map, statue.x, statue.y, STATUE);

    let heap_chest = Random::int_max(2) == 0;
    let n = Random::int_range_inclusive(2, 3);
    let mimic_chance = 0.2f32; // 1/5 without MimicTooth
    for _ in 0..n {
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
                randomize_item(&mut g, dungeon.depth);
                g
            });
        let cell = treasury_drop_cell(room, map, true);
        let heap_type = if heap_chest && dungeon.depth > 1 && Random::float() < mimic_chance {
            item.source = Some("TreasuryRoom:mimic".into());
            map.mob_occupied[cell] = true;
            map.known_mobs[cell] = Some("Mimic");
            "mimic"
        } else {
            item.source = Some("TreasuryRoom".into());
            if heap_chest {
                "chest"
            } else {
                "heap"
            }
        };
        if !matches!(heap_type, "mimic" | "golden_mimic") {
            map.record_heap(cell, heap_type, item.clone());
        }
        out.push(PlacedLoot { item, heap_type });
    }
    if !heap_chest {
        for _ in 0..6 {
            let cell = treasury_drop_cell(room, map, false);
            let mut gold = GeneratedItem::new("Gold", ItemCategory::Gold);
            gold.quantity = Random::int_range_inclusive(5, 12);
            gold.source = Some("TreasuryRoom".into());
            // Small piles may merge with an existing heap, exactly like
            // Level.drop; they reject only non-EMPTY terrain.
            map.record_heap(cell, "heap", gold);
        }
    }
    // TreasuryRoom.java:76 — IronKey pushed after the small gold piles
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    out
}

fn treasury_drop_cell(room: &Room, map: &TerrainMap, reject_occupied: bool) -> usize {
    for _ in 0..10_000 {
        let point = room.random();
        let Some(cell) = map.point_to_cell(point.x, point.y) else {
            continue;
        };
        if map.map[cell] == EMPTY
            && (!reject_occupied || (!map.heap_occupied[cell] && !map.mob_occupied[cell]))
        {
            return cell;
        }
    }
    panic!("placed TreasuryRoom must have a valid drop cell");
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            set_terrain(map, x, y, terrain);
        }
    }
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    for y in (room.top + margin)..=(room.bottom - margin) {
        for x in (room.left + margin)..=(room.right - margin) {
            set_terrain(map, x, y, terrain);
        }
    }
}

fn set_terrain(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}
pub fn storage_prizes(
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

pub fn storage_prize(
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

#[cfg(test)]
pub fn runestone_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    runestone_prizes_shared(dungeon, room, None, items_to_spawn)
}

pub fn runestone_prizes_on_map(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    runestone_prizes_shared(dungeon, room, Some(map), items_to_spawn)
}

fn runestone_prizes_shared(
    dungeon: &mut DungeonState,
    room: &Room,
    mut map: Option<&mut TerrainMap>,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let n = Random::normal_int_range(2, 3);
    let mut occupied = Vec::new();
    for _ in 0..n {
        if let Some(map) = map.as_deref_mut() {
            loop {
                let point = room.random();
                let Some(cell) = map.point_to_cell(point.x, point.y) else {
                    continue;
                };
                if map.map[cell] == EMPTY && !map.heap_occupied[cell] {
                    map.heap_occupied[cell] = true;
                    break;
                }
            }
        } else {
            burn_drop_pos(room, &mut occupied);
        }
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
    // RunestoneRoom.java:64 — IronKey is the last statement of paint()
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    out
}
