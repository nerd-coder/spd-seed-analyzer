//! Pinned `ShopRoom.paint`: shopkeeper plus clockwise FOR_SALE stock.

use crate::geom::Point;
use crate::items::model::GeneratedItem;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, EMPTY_SP, GRASS, HIGH_GRASS, WALL};
use crate::rooms::room::Room;

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    items: &[GeneratedItem],
) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY_SP);

    // Room.center() intentionally consumes optional axis-jitter draws.
    let center = room.as_rect().center_room();
    if let Some(cell) = map.point_to_cell(center.x, center.y) {
        map.mob_occupied[cell] = true;
        map.known_mobs[cell] = Some("Shopkeeper");
    }

    let Some(entrance) = room
        .connected
        .iter()
        .find_map(|&other| doors.get(room_index, other))
        .map(|door| Point::new(door.x, door.y))
    else {
        return;
    };
    place_items(map, room, entrance, items);
}

fn place_items(map: &mut TerrainMap, room: &Room, entrance: Point, items: &[GeneratedItem]) {
    let mut entry_inset = entrance;
    if entry_inset.y == room.top {
        entry_inset.y += 1;
    } else if entry_inset.y == room.bottom {
        entry_inset.y -= 1;
    } else if entry_inset.x == room.left {
        entry_inset.x += 1;
    } else {
        entry_inset.x -= 1;
    }

    let mut current = entry_inset;
    let mut inset = 1;
    for item in items {
        step_clockwise(&mut current, room, inset);
        if current == entry_inset {
            if entry_inset.y == room.top + inset {
                entry_inset.y += 1;
            } else if entry_inset.y == room.bottom - inset {
                entry_inset.y -= 1;
            }
            if entry_inset.x == room.left + inset {
                entry_inset.x += 1;
            } else if entry_inset.x == room.right - inset {
                entry_inset.x -= 1;
            }
            inset += 1;
            if inset > (room.width().min(room.height()) - 3) / 2 {
                break;
            }
            current = entry_inset;
            step_clockwise(&mut current, room, inset);
        }

        if let Some(cell) = map.point_to_cell(current.x, current.y) {
            if map.map[cell] == HIGH_GRASS {
                map.map[cell] = GRASS;
            }
            map.record_heap(cell, "for_sale", item.clone());
        }
    }
}

fn step_clockwise(point: &mut Point, room: &Room, inset: i32) {
    if point.x == room.left + inset && point.y != room.top + inset {
        point.y -= 1;
    } else if point.y == room.top + inset && point.x != room.right - inset {
        point.x += 1;
    } else if point.x == room.right - inset && point.y != room.bottom - inset {
        point.y += 1;
    } else {
        point.x -= 1;
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    fill_rect(map, room.left, room.top, room.right, room.bottom, terrain);
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    fill_rect(
        map,
        room.left + margin,
        room.top + margin,
        room.right - margin,
        room.bottom - margin,
        terrain,
    );
}

fn fill_rect(map: &mut TerrainMap, left: i32, top: i32, right: i32, bottom: i32, terrain: i32) {
    for y in top..=bottom {
        for x in left..=right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = terrain;
            }
        }
    }
}
