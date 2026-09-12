//! Pinned `RitualSiteRoom.paint` cages, marker, and ceremonial candles.

use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, CUSTOM_DECO, CUSTOM_DECO_EMPTY, EMPTY, REGION_DECO, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

use super::map_simple_image;

const PRISON_QUEST: &str = "prison_quest";

pub fn paint(
    room: &Room,
    room_index: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let mut valid = [true; 10];
    valid[0] = false;
    valid[3] = false;
    valid[6] = false;
    valid[9] = false;
    for &other in &room.connected {
        let Some(door) = doors.get(room_index, other) else {
            continue;
        };
        if door.y == room.top {
            valid[(door.x - room.left) as usize] = false;
        } else if door.y <= room.top + 2 {
            if door.x == room.left {
                valid[1] = false;
            } else {
                valid[8] = false;
            }
        }
    }

    let mut top_row = Point::new(room.left, room.top);
    let mut cage_row = Random::int_max(2);
    for i in (room.left..room.right).step_by(3) {
        top_row.x = i;
        draw_inside(map, room, top_row, 2, WALL);

        let plus_one = (top_row.x + 1 - room.left) as usize;
        let plus_two = (top_row.x + 2 - room.left) as usize;
        if !valid[plus_one] && !valid[plus_two] {
            continue;
        }
        if !valid[plus_two] {
            top_row.x += 1;
        } else if !valid[plus_one] {
            top_row.x += 2;
        } else {
            top_row.x += Random::int_range_inclusive(1, 2);
        }

        valid[(top_row.x - room.left) as usize] = false;
        if cage_row == 0 {
            draw_inside(map, room, top_row, 2, REGION_DECO);
        } else {
            draw_inside(map, room, top_row, 2, CUSTOM_DECO);
            map.record_custom_terrain(
                "Table",
                PRISON_QUEST,
                (top_row.x, top_row.y + 1, 1, 2),
                map_simple_image(1, 2, 0, 0, 256),
            );
        }
        cage_row -= 1;
    }

    let mut tries = 100;
    loop {
        let i = Random::int_range_inclusive(1, 9);
        if valid[i as usize] {
            top_row.x = room.left + i;
            draw_inside(map, room, top_row, 1, REGION_DECO);
            tries = 0;
        }
        let keep_going = tries > 0;
        tries -= 1;
        if !keep_going {
            break;
        }
    }

    let mut ritual = room.as_rect().center_room();
    ritual.y += 1;
    map.record_custom_tile(
        "RitualMarker",
        PRISON_QUEST,
        (ritual.x - 2, ritual.y - 2, 5, 5),
        map_simple_image(5, 5, 0, 2, 256),
    );
    fill(map, ritual.x - 1, ritual.y - 1, 3, 3, CUSTOM_DECO_EMPTY);
    apply_place_masks(map, room, ritual);

    for _ in 0..4 {
        items_to_spawn.push(GeneratedItem::new("CeremonialCandle", ItemCategory::Other));
    }
    Vec::new()
}

fn apply_place_masks(map: &mut TerrainMap, room: &Room, ritual: Point) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let Some(cell) = map.point_to_cell(x, y) else {
                continue;
            };
            let inside = x > room.left && y > room.top && x < room.right && y < room.bottom;
            let far = (x - ritual.x).abs().max((y - ritual.y).abs()) >= 2;
            map.item_allowed[cell] = inside && far;
            map.character_allowed[cell] = inside && far;
        }
    }
}

fn draw_inside(map: &mut TerrainMap, room: &Room, from: Point, distance: i32, terrain: i32) {
    let step = if from.x == room.left {
        Point::new(1, 0)
    } else if from.x == room.right {
        Point::new(-1, 0)
    } else if from.y == room.top {
        Point::new(0, 1)
    } else {
        Point::new(0, -1)
    };
    let mut point = Point::new(from.x + step.x, from.y + step.y);
    for _ in 0..distance {
        set(map, point.x, point.y, terrain);
        point.x += step.x;
        point.y += step.y;
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
