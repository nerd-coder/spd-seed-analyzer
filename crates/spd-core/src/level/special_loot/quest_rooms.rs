//! Quest-related room prizes (wandmaker, blacksmith).

use super::placement::burn_drop_pos;
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::terrain::{EMPTY, EMPTY_SP, EXIT, TRAP, WALL};
use crate::level::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

/// `MassGraveRoom.paint` terrain, skeleton occupancy, and loot.
pub(super) fn mass_grave_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Barricade entrance → PotionOfLiquidFlame into itemsToSpawn
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLiquidFlame",
        ItemCategory::Potion,
    ));

    // `MassGraveRoom.paint` replaces the whole interior before placing actors.
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] =
                    if x == room.left || x == room.right || y == room.top || y == room.bottom {
                        WALL
                    } else {
                        crate::level::CUSTOM_DECO_EMPTY
                    };
            }
        }
    }

    // 50% 1 skeleton, 50% 2. These room-painted mobs must occupy their cells
    // before the later ambient pass calls `findMob`.
    let n_skel = Random::int_max(2); // 0 or 1 → loop i=0..=n_skel → 1 or 2
    let mut mob_positions = Vec::new();
    for _ in 0..=n_skel {
        burn_drop_pos(room, &mut mob_positions);
        if let Some(&(x, y)) = mob_positions.last() {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.mob_occupied[cell] = true;
                map.known_mobs[cell] = Some("Skeleton");
            }
        }
    }

    let mut out = Vec::new();
    // 100% corpse dust, 2x gold(1), 2x30% gold, 1x60% random, 1x30% armor
    let mut items: Vec<GeneratedItem> = Vec::new();
    items.push(GeneratedItem::new("CorpseDust", ItemCategory::Other));
    {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        g.quantity = 1;
        items.push(g);
    }
    {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        g.quantity = 1;
        items.push(g);
    }
    if Random::float() <= 0.3 {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        items.push(g);
    }
    if Random::float() <= 0.3 {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        items.push(g);
    }
    if Random::float() <= 0.6 {
        items.push(dungeon.generator.random(dungeon.depth));
    }
    if Random::float() <= 0.3 {
        items.push(
            dungeon
                .generator
                .random_armor(dungeon.depth / 5, dungeon.depth),
        );
    }

    // Java only rejects existing heaps here; mobs do not exclude an item cell.
    let mut heap_positions = Vec::new();
    for mut item in items {
        burn_drop_pos(room, &mut heap_positions);
        // Haunted-if-cursed: no extra RNG for analysis
        item.source = Some("MassGraveRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "skeleton",
        });
    }
    out
}

/// `RitualSiteRoom.paint` — select its jittered ritual center and enqueue four candles.
pub(super) fn ritual_site_setup(
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let center = room.as_rect().center_room();
    for y in (center.y - 1)..=(center.y + 1) {
        for x in (center.x - 1)..=(center.x + 1) {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = crate::level::CUSTOM_DECO_EMPTY;
                map.item_allowed[cell] = false;
                map.character_allowed[cell] = false;
            }
        }
    }
    for _ in 0..4 {
        items_to_spawn.push(GeneratedItem::new("CeremonialCandle", ItemCategory::Other));
    }
    Vec::new()
}

/// `RotGardenRoom.paint` key. Geometry, heart, and lasher RNG are painted by
/// `special_loot::geometry` before this helper runs.
pub(super) fn rot_garden_setup(
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    let _ = room;
    Vec::new()
}

/// `BlacksmithRoom.paint` — two random armor/weapon/missile drops + NPC/exit placement RNG.
pub(super) fn blacksmith_room_prizes(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    room_index: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
) -> Vec<PlacedLoot> {
    let room = &rooms[room_index];
    fill_room(map, room, 0, WALL);
    fill_room(map, room, 1, TRAP);
    for &other in &room.connected {
        if let Some(door) = doors.get(room_index, other) {
            draw_inside(map, room, door.x, door.y, 2, EMPTY);
        }
    }
    // Java fills this after `drawInside`, so the corridor's second tile is
    // restored to EMPTY_SP and remains eligible for the two reward heaps.
    fill_room(map, room, 2, EMPTY_SP);

    let mut out = Vec::new();
    let mut heap_positions = Vec::new();
    for _ in 0..2 {
        // The terrain predicate is checked before generating the item. It does
        // not reject an existing heap, so the two rewards may merge.
        let point = loop {
            let point = room.random();
            let cell = map
                .point_to_cell(point.x, point.y)
                .expect("BlacksmithRoom point is on map");
            if map.map[cell] == EMPTY_SP {
                break point;
            }
        };
        let cat = *Random::one_of(&[Category::Armor, Category::Weapon, Category::Missile]);
        let mut item = dungeon.generator.random_category(cat, dungeon.depth);
        item.source = Some("BlacksmithRoom".into());
        let cell = map
            .point_to_cell(point.x, point.y)
            .expect("BlacksmithRoom heap is on map");
        map.record_heap(cell, "heap", item.clone());
        heap_positions.push((point.x, point.y));
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }

    let npc = loop {
        let point = room.random_margin(2);
        if !heap_positions.contains(&(point.x, point.y)) {
            break point;
        }
    };
    let npc_cell = map
        .point_to_cell(npc.x, npc.y)
        .expect("Blacksmith NPC is on map");
    map.mob_occupied[npc_cell] = true;
    map.known_mobs[npc_cell] = Some("Blacksmith");

    // The pinned do/while accidentally re-checks the heap at `npc.pos`, which
    // is already known empty. Consequently the branch exit only avoids the NPC.
    let entrance = loop {
        let point = room.random_margin(2);
        if point != npc {
            break point;
        }
    };
    let entrance_cell = map
        .point_to_cell(entrance.x, entrance.y)
        .expect("Blacksmith branch exit is on map");
    map.map[entrance_cell] = EXIT;
    map.character_allowed[entrance_cell] = false;

    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let Some(cell) = map.point_to_cell(x, y) else {
                continue;
            };
            if map.map[cell] == TRAP {
                map.trap_names[cell] = Some("BurningTrap");
                map.trap_destroys_items[cell] = true;
            }
        }
    }
    out
}

fn fill_room(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    for y in (room.top + margin)..=(room.bottom - margin) {
        for x in (room.left + margin)..=(room.right - margin) {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = terrain;
            }
        }
    }
}

fn draw_inside(
    map: &mut TerrainMap,
    room: &Room,
    from_x: i32,
    from_y: i32,
    distance: i32,
    terrain: i32,
) {
    let (dx, dy) = if from_x == room.left {
        (1, 0)
    } else if from_x == room.right {
        (-1, 0)
    } else if from_y == room.top {
        (0, 1)
    } else {
        (0, -1)
    };
    for step in 1..=distance {
        if let Some(cell) = map.point_to_cell(from_x + dx * step, from_y + dy * step) {
            map.map[cell] = terrain;
        }
    }
}

/// `AmbitiousImpRoom.paint` NPC offset relative to its single entrance.
pub(super) fn ambitious_imp_room_npc(
    rooms: &[Room],
    room_index: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
) {
    let room = &rooms[room_index];
    let center = room.as_rect().center_room();
    let Some(door) = room
        .connected
        .iter()
        .find_map(|&other| doors.get(room_index, other))
    else {
        // A valid AmbitiousImpRoom has exactly one entrance. Retain the pinned
        // draw shape if an incomplete partial layout failed to connect it.
        let _ = Random::int_range_inclusive(-1, 1);
        return;
    };

    let (x, y) = if door.x == room.left || door.x == room.right {
        (
            center.x + if door.x == room.left { -2 } else { 2 },
            center.y + Random::int_range_inclusive(-1, 1),
        )
    } else {
        (
            center.x + Random::int_range_inclusive(-1, 1),
            center.y + if door.y == room.top { -2 } else { 2 },
        )
    };
    if let Some(cell) = map.point_to_cell(x, y) {
        map.mob_occupied[cell] = true;
        map.known_mobs[cell] = Some("Imp");
    }
}
