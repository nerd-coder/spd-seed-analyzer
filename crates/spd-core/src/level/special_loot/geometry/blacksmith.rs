//! Pinned `BlacksmithRoom.paint` terrain, NPC, pedestal heaps, and smithy tiles.

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{ItemCategory, ItemProvenance, QuestRewardRole};
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::terrain::{
    TerrainMap, CUSTOM_DECO, CUSTOM_DECO_WTR, EMPTY, EMPTY_SP, EXIT, PEDESTAL, REGION_DECO_ALT,
    WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;

const CAVES_QUEST: &str = "caves_quest";

pub fn paint(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    room_index: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
) -> Vec<PlacedLoot> {
    let room = &rooms[room_index];
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);
    fill_margin(map, room, 2, EMPTY_SP);
    fill(
        map,
        room.left + 2,
        room.top + 1,
        room.width() - 4,
        1,
        REGION_DECO_ALT,
    );

    for &other in &room.connected {
        if let Some(door) = doors.get(room_index, other) {
            draw_inside(map, room, door.x, door.y, 1, EMPTY);
        }
    }
    fill_margin(map, room, 2, EMPTY_SP);

    let npc_x = room.left + 3;
    let npc_y = room.top + 3;
    let npc_cell = map
        .point_to_cell(npc_x, npc_y)
        .expect("Blacksmith NPC is on map");
    map.mob_occupied[npc_cell] = true;
    map.known_mobs[npc_cell] = Some("Blacksmith");

    set(map, npc_x + 1, npc_y, CUSTOM_DECO_WTR);
    set(map, npc_x + 1, npc_y - 1, CUSTOM_DECO);
    set(map, npc_x, npc_y - 1, CUSTOM_DECO);
    set(map, npc_x - 1, npc_y - 1, CUSTOM_DECO);

    let mut equip_x = room.right - 3;
    let mut equip_y = room.bottom - 3;
    if room.height() == 8 {
        equip_y += 1;
    }
    let mut out = Vec::new();
    for _ in 0..2 {
        let cell = map
            .point_to_cell(equip_x, equip_y)
            .expect("BlacksmithRoom heap is on map");
        out.push(drop_equipment(dungeon, map, cell));
        set(map, equip_x, equip_y, PEDESTAL);
        equip_x -= 1;
    }

    let mut entrance_on_left = Random::int_max(2) == 0;
    for &other in &room.connected {
        let Some(door) = doors.get(room_index, other) else {
            continue;
        };
        if door.y <= room.top + 2 {
            if door.x <= room.left + 1 {
                entrance_on_left = false;
            } else if door.x >= room.right - 1 {
                entrance_on_left = true;
            }
        }
    }
    let entrance_x = if entrance_on_left {
        room.left + 1
    } else {
        room.right - 1
    };
    let entrance_y = room.top + 1;
    let entrance_cell = map
        .point_to_cell(entrance_x, entrance_y)
        .expect("Blacksmith branch exit is on map");
    map.map[entrance_cell] = EXIT;
    map.branch_exits.push(entrance_cell);
    map.record_custom_tile(
        "QuestEntrance",
        CAVES_QUEST,
        (entrance_x, entrance_y, 1, 1),
        vec![0],
    );

    let tile_x = room.left + 2;
    let tile_y = room.top + 2;
    let tile_w = room.width() - 4;
    let tile_h = room.height() - 4;
    map.record_custom_tile(
        "SmithyVisuals",
        CAVES_QUEST,
        (tile_x, tile_y, tile_w, tile_h),
        smithy_visuals(map, tile_x, tile_y, tile_w, tile_h),
    );
    map.record_custom_wall(
        "FurnaceOverhang",
        CAVES_QUEST,
        (npc_x - 1, npc_y - 2, 1, 1),
        vec![3],
    );

    apply_placement_masks(map, room);
    out
}

fn drop_equipment(dungeon: &mut DungeonState, map: &mut TerrainMap, cell: usize) -> PlacedLoot {
    let cat = *Random::one_of(&[Category::Armor, Category::Weapon, Category::Missile]);
    let mut item = dungeon.generator.random_category(cat, dungeon.depth);
    item.source = Some("BlacksmithRoom".into());
    let tier = crate::generator::equipment_tier_for_class(&item.class_name)
        .expect("BlacksmithRoom equipment has a tier");
    item.provenance = ItemProvenance::Quest(match item.category {
        ItemCategory::Weapon => QuestRewardRole::BlacksmithRoomWeapon { tier },
        ItemCategory::Missile => QuestRewardRole::BlacksmithRoomMissile { tier },
        ItemCategory::Armor => QuestRewardRole::BlacksmithRoomArmor { tier },
        _ => unreachable!("BlacksmithRoom equipment category"),
    });
    map.record_heap(cell, "heap", item.clone());
    PlacedLoot {
        item,
        heap_type: "heap",
    }
}

fn smithy_visuals(
    map: &TerrainMap,
    tile_x: i32,
    tile_y: i32,
    tile_w: i32,
    tile_h: i32,
) -> Vec<i16> {
    let len = (tile_w * tile_h) as usize;
    let mut data = vec![-1; len];
    for (i, slot) in data.iter_mut().enumerate() {
        *slot = if i == 0 {
            7
        } else if i == 1 {
            16
        } else if i == 2 {
            17
        } else if i as i32 / tile_w == 1 && i as i32 % tile_w == 2 {
            18
        } else {
            floor_or_pedestal(map, tile_x, tile_y, tile_w, i, len)
        };
    }
    data
}

fn floor_or_pedestal(
    map: &TerrainMap,
    tile_x: i32,
    tile_y: i32,
    tile_w: i32,
    i: usize,
    len: usize,
) -> i16 {
    let x = tile_x + i as i32 % tile_w;
    let y = tile_y + i as i32 / tile_w;
    match map.point_to_cell(x, y).map(|cell| map.map[cell]) {
        Some(EMPTY_SP) => {
            let last_row = i >= len - tile_w as usize;
            let col = i as i32 % tile_w;
            if last_row {
                if col == 0 {
                    12
                } else if col == tile_w - 1 {
                    14
                } else {
                    13
                }
            } else if col == 0 {
                8
            } else if col == tile_w - 1 {
                10
            } else {
                -1
            }
        }
        Some(PEDESTAL) => {
            if i >= len - tile_w as usize {
                20
            } else {
                21
            }
        }
        _ => -1,
    }
}

fn apply_placement_masks(map: &mut TerrainMap, room: &Room) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let Some(cell) = map.point_to_cell(x, y) else {
                continue;
            };
            map.character_allowed[cell] = false;
            map.item_allowed[cell] = map.map[cell] == EMPTY;
        }
    }
}

fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}

fn fill(map: &mut TerrainMap, x: i32, y: i32, w: i32, h: i32, terrain: i32) {
    for dy in 0..h {
        for dx in 0..w {
            set(map, x + dx, y + dy, terrain);
        }
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    fill(
        map,
        room.left,
        room.top,
        room.width(),
        room.height(),
        terrain,
    );
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    fill(
        map,
        room.left + margin,
        room.top + margin,
        room.width() - margin * 2,
        room.height() - margin * 2,
        terrain,
    );
}

fn draw_inside(map: &mut TerrainMap, room: &Room, from_x: i32, from_y: i32, n: i32, terrain: i32) {
    let (dx, dy) = if from_x == room.left {
        (1, 0)
    } else if from_x == room.right {
        (-1, 0)
    } else if from_y == room.top {
        (0, 1)
    } else {
        (0, -1)
    };
    for step in 1..=n {
        set(map, from_x + dx * step, from_y + dy * step, terrain);
    }
}
