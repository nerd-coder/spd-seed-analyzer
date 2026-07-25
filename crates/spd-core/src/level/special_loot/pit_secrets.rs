//! Pit, secret maze/summoning/chest-chasm prizes.

use super::special_rooms::is_curse_enchant;
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

/// `PitRoom.paint` — skeleton main loot (ring/artifact/equip) + 1–2 consumables + CrystalKey.
pub(super) fn pit_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
) -> Vec<PlacedLoot> {
    // `level.pointToCell(center())` chooses the shared skeleton heap cell.
    // `Room.center()` burns one `Random.Int(2)` per even-sized dimension.
    let remains = room.as_rect().center_room();
    let remains = map
        .point_to_cell(remains.x, remains.y)
        .expect("placed PitRoom center is inside the map");

    // Main loot: ring / artifact / equip (weapon×2, missile, armor×2).
    // Challenges.isItemBlocked is always false without challenges — single draw.
    let mut main = match Random::int_max(3) {
        0 => dungeon
            .generator
            .random_category(Category::Ring, dungeon.depth),
        1 => dungeon
            .generator
            .random_category(Category::Artifact, dungeon.depth),
        _ => {
            let cats = [
                Category::Weapon,
                Category::Weapon,
                Category::Missile,
                Category::Armor,
                Category::Armor,
            ];
            let cat = *Random::one_of(&cats);
            dungeon.generator.random_category(cat, dungeon.depth)
        }
    };
    main.source = Some("PitRoom".into());
    let mut out = vec![PlacedLoot {
        item: main,
        heap_type: "skeleton",
    }];

    let n = Random::int_range_inclusive(1, 2);
    for _ in 0..n {
        let cats = [
            Category::Potion,
            Category::Scroll,
            Category::Food,
            Category::Gold,
        ];
        let cat = *Random::one_of(&cats);
        let mut prize = dungeon.generator.random_category(cat, dungeon.depth);
        prize.source = Some("PitRoom".into());
        out.push(PlacedLoot {
            item: prize,
            heap_type: "skeleton",
        });
    }

    let mut key = GeneratedItem::new("CrystalKey", ItemCategory::Other);
    key.source = Some("PitRoom".into());
    out.push(PlacedLoot {
        item: key,
        heap_type: "skeleton",
    });

    // `Heap.items.add(item)` inserts at the front, so the final observable
    // stack is the reverse of PitRoom's drop order.
    for prize in out.iter().rev() {
        map.record_heap(remains, "skeleton", prize.item.clone());
    }
    out
}

/// `SecretMazeRoom.paint` prize — +1 floor-set weapon/armor, never cursed, 33% upgrade.
/// Full maze geometry and its RNG run immediately before this helper.
pub(super) fn secret_maze_prize(dungeon: &mut DungeonState) -> PlacedLoot {
    let floor = (dungeon.depth / 5) + 1;
    let mut prize = if Random::int_max(2) == 0 {
        let mut w = dungeon.generator.random_weapon(floor, true, dungeon.depth);
        if is_curse_enchant(&w) {
            w.enchantment = None;
        }
        w
    } else {
        let mut a = dungeon.generator.random_armor(floor, dungeon.depth);
        if is_curse_enchant(&a) {
            a.enchantment = None;
        }
        a
    };
    prize.cursed = false;
    // cursedKnown = true is UI-only in full game
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("SecretMazeRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

/// `SecretSummoningRoom.paint` — center skeleton with `Generator.random()`.
pub(super) fn secret_summoning_prize(dungeon: &mut DungeonState) -> PlacedLoot {
    // Trap reveal chance is 0 without TrapMechanism trinket — no extra RNG in trap loop.
    let mut item = dungeon.generator.random(dungeon.depth);
    item.source = Some("SecretSummoningRoom".into());
    PlacedLoot {
        item,
        heap_type: "skeleton",
    }
}

/// `SecretChestChasmRoom.paint` — 4 locked chests (`randomUsingDefaults`) + golden keys + levitation.
pub(super) fn secret_chest_chasm(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    fill_room(map, room, crate::level::terrain::WALL);
    fill_margin(map, room, 1, crate::level::terrain::CHASM);
    let chest_points = [
        Point::new(room.left + 3, room.top + 3),
        Point::new(room.right - 3, room.top + 3),
        Point::new(room.right - 3, room.bottom - 3),
        Point::new(room.left + 3, room.bottom - 3),
    ];
    for point in chest_points {
        let mut item = dungeon.generator.random_using_defaults_any(dungeon.depth);
        item.source = Some("SecretChestChasmRoom".into());
        let cell = map
            .point_to_cell(point.x, point.y)
            .expect("secret chest cell");
        map.map[cell] = crate::level::terrain::EMPTY_SP;
        map.record_heap(cell, "locked_chest", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "locked_chest",
        });
    }
    let key_points = [
        Point::new(room.left + 1, room.top + 1),
        Point::new(room.right - 1, room.top + 1),
        Point::new(room.right - 1, room.bottom - 1),
        Point::new(room.left + 1, room.bottom - 1),
    ];
    for point in key_points {
        let mut key = GeneratedItem::new("GoldenKey", ItemCategory::Other);
        key.source = Some("SecretChestChasmRoom".into());
        let cell = map
            .point_to_cell(point.x, point.y)
            .expect("secret key cell");
        map.map[cell] = crate::level::terrain::EMPTY_SP;
        map.record_heap(cell, "heap", key.clone());
        out.push(PlacedLoot {
            item: key,
            heap_type: "heap",
        });
    }
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLevitation",
        ItemCategory::Potion,
    ));
    out
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = terrain;
            }
        }
    }
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    for y in (room.top + margin)..=(room.bottom - margin) {
        for x in (room.left + margin)..=(room.right - margin) {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = terrain;
            }
        }
    }
}
