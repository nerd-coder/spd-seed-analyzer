//! Secret room prize painters.

use super::placement::{burn_drop_pos, find_prize_item};
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, ALCHEMY, EMPTY, EMPTY_SP, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

pub(super) fn secret_library(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Approximate: 2-3 scrolls
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                dungeon
                    .generator
                    .random_category(Category::Scroll, dungeon.depth)
            });
        item.source = Some("SecretLibraryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn secret_runestone(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // SecretRunestoneRoom.java:64 — pushed before the stone drops (zero-RNG append)
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLiquidFlame",
        ItemCategory::Potion,
    ));
    let mut out = Vec::new();
    for index in 0..3 {
        let required = if index == 2 { EMPTY_SP } else { EMPTY };
        let cell = loop {
            let point = room.random();
            let cell = map
                .point_to_cell(point.x, point.y)
                .expect("room point is on map");
            if map.map[cell] == required && (index != 1 || !map.heap_occupied[cell]) {
                break cell;
            }
        };
        let mut item = if index == 2 {
            GeneratedItem::new("StoneOfEnchantment", ItemCategory::Stone)
        } else {
            dungeon
                .generator
                .random_using_defaults(Category::Stone, dungeon.depth)
        };
        item.source = Some("SecretRunestoneRoom".into());
        map.record_heap(cell, "heap", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn secret_artillery(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = dungeon
            .generator
            .random_missile(dungeon.depth / 5, false, dungeon.depth);
        item.source = Some("SecretArtilleryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn secret_laboratory(room: &Room, map: &mut TerrainMap) -> Vec<PlacedLoot> {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] =
                    if x == room.left || x == room.right || y == room.top || y == room.bottom {
                        WALL
                    } else {
                        EMPTY_SP
                    };
            }
        }
    }

    // Room.center() independently resolves each even inclusive dimension.
    let center_x = (room.left + room.right) / 2
        + if (room.right - room.left) % 2 == 1 {
            Random::int_max(2)
        } else {
            0
        };
    let center_y = (room.top + room.bottom) / 2
        + if (room.bottom - room.top) % 2 == 1 {
            Random::int_max(2)
        } else {
            0
        };
    if let Some(cell) = map.point_to_cell(center_x, center_y) {
        map.map[cell] = ALCHEMY;
    }

    for _ in 0..2 {
        let cell = empty_sp_heap_cell(room, map);
        let mut crystal = GeneratedItem::new("EnergyCrystal", ItemCategory::Other);
        crystal.quantity = Random::int_range_inclusive(3, 5);
        crystal.source = Some("SecretLaboratoryRoom".into());
        map.record_heap(cell, "heap", crystal);
    }

    // Iteration order is the pinned JVM HashMap key order used by
    // Random.chances(HashMap) in SecretLaboratoryRoom.
    let classes = [
        "PotionOfHealing",
        "PotionOfHaste",
        "PotionOfToxicGas",
        "PotionOfLiquidFlame",
        "PotionOfMindVision",
        "PotionOfFrost",
        "PotionOfInvisibility",
        "PotionOfLevitation",
        "PotionOfParalyticGas",
        "PotionOfPurity",
        "PotionOfExperience",
    ];
    let mut chances = vec![1.0, 4.0, 3.0, 3.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0, 6.0];
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    for _ in 0..n {
        let cell = empty_sp_heap_cell(room, map);
        let choice = Random::chances(&chances) as usize;
        chances[choice] = 0.0;
        // Every listed regular potion has an ExoticPotion mapping. With no
        // ExoticCrystals trinket the conversion cannot win, but still rolls.
        let _ = Random::float();
        let mut item = GeneratedItem::new(classes[choice], ItemCategory::Potion);
        item.source = Some("SecretLaboratoryRoom".into());
        map.record_heap(cell, "heap", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

fn empty_sp_heap_cell(room: &Room, map: &TerrainMap) -> usize {
    loop {
        let point = room.random();
        let Some(cell) = map.point_to_cell(point.x, point.y) else {
            continue;
        };
        if map.map[cell] == EMPTY_SP && !map.heap_occupied[cell] {
            return cell;
        }
    }
}

pub(super) fn secret_larder(depth: i32, room: &Room, map: &mut TerrainMap) -> Vec<PlacedLoot> {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] =
                    if x == room.left || x == room.right || y == room.top || y == room.bottom {
                        WALL
                    } else {
                        EMPTY_SP
                    };
            }
        }
    }

    let center_x = (room.left + room.right) / 2
        + if (room.right - room.left) % 2 == 1 {
            Random::int_max(2)
        } else {
            0
        };
    let center_y = (room.top + room.bottom) / 2
        + if (room.bottom - room.top) % 2 == 1 {
            Random::int_max(2)
        } else {
            0
        };
    for y in (center_y - 1)..=(center_y + 1) {
        for x in (center_x - 1)..=(center_x + 1) {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = crate::level::terrain::WATER;
            }
        }
    }
    if let Some(cell) = map.point_to_cell(center_x, center_y) {
        map.map[cell] = crate::level::terrain::GRASS;
        map.plant_occupied[cell] = true;
    }

    // Measure food in one ChargrilledMeat ration
    // (`STARVING - HUNGRY`). Pasty restores `STARVING`, exactly three units.
    let mut food_units = 1 + depth / 5;
    let mut out = Vec::new();
    while food_units > 0 {
        let class_name = if food_units >= 3 {
            food_units -= 3;
            "Pasty"
        } else {
            food_units -= 1;
            "ChargrilledMeat"
        };
        let cell = empty_sp_heap_cell(room, map);
        let mut item = GeneratedItem::new(class_name, ItemCategory::Food);
        item.source = Some("SecretLarderRoom".into());
        map.record_heap(cell, "heap", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn secret_hoard(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
) -> Vec<PlacedLoot> {
    // SecretHoardRoom paints half its interior as roughly eight piles' worth
    // of gold, then rolls a visible trap at every point in its inclusive rect.
    let trap_name = if Random::int_max(2) == 0 {
        "RockfallTrap"
    } else if dungeon.depth >= 10 {
        "DisintegrationTrap"
    } else {
        "PoisonDartTrap"
    };
    let total_gold = ((room.width() - 2) * (room.height() - 2)) / 2;
    let gold_ratio = 8.0 / total_gold as f32;
    let mut out = Vec::new();
    for _ in 0..total_gold {
        let cell = loop {
            let x = Random::int_range_inclusive(room.left + 1, room.right - 1);
            let y = Random::int_range_inclusive(room.top + 1, room.bottom - 1);
            let cell = map.point_to_cell(x, y).expect("hoard interior cell");
            if !map.heap_occupied[cell] {
                break cell;
            }
        };
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        g.quantity = (g.quantity as f32 * gold_ratio).round() as i32;
        g.source = Some("SecretHoardRoom".into());
        map.record_heap(cell, "heap", g.clone());
        out.push(PlacedLoot {
            item: g,
            heap_type: "heap",
        });
    }
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if Random::int_max(2) != 0 {
                continue;
            }
            let cell = map.point_to_cell(x, y).expect("hoard room cell");
            if map.map[cell] == crate::level::terrain::EMPTY {
                map.map[cell] = crate::level::terrain::TRAP;
                map.trap_names[cell] = Some(trap_name);
                map.trap_destroys_items[cell] = true;
            }
        }
    }
    out
}
