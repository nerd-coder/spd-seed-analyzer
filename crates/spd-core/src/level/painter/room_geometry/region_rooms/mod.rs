//! Region-specific structural standard rooms.

mod bridge;
mod caves_fissure;
mod circles;
mod pathing;

#[cfg(test)]
mod tests;

use crate::geom::Point;
use crate::level::terrain::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

use super::super::DoorMap;

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) -> bool {
    match room.name.as_str() {
        "RegionDecoBridgeRoom" | "RegionDecoBridgeEntranceRoom" | "RegionDecoBridgeExitRoom" => {
            bridge::paint(map, room, room_index, doors)
        }
        "CavesFissureRoom" | "CavesFissureEntranceRoom" | "CavesFissureExitRoom" => {
            caves_fissure::paint(map, room, room_index, doors)
        }
        "CirclePitRoom" | "CircleWallRoom" | "CircleWallEntranceRoom" | "CircleWallExitRoom" => {
            circles::paint(map, room, room_index, doors)
        }
        _ => return false,
    }
    true
}

fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}

fn terrain_at(map: &TerrainMap, x: i32, y: i32) -> Option<i32> {
    map.point_to_cell(x, y).map(|cell| map.map[cell])
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
    if left > right || top > bottom {
        return;
    }
    for y in top..=bottom {
        for x in left..=right {
            set(map, x, y, terrain);
        }
    }
}

fn draw_inside(map: &mut TerrainMap, room: &Room, from: Point, n: i32, terrain: i32) {
    let (dx, dy) = if from.x == room.left {
        (1, 0)
    } else if from.x == room.right {
        (-1, 0)
    } else if from.y == room.top {
        (0, 1)
    } else if from.y == room.bottom {
        (0, -1)
    } else {
        return;
    };
    for step in 1..=n {
        set(map, from.x + dx * step, from.y + dy * step, terrain);
    }
}

/// `Rect.center()` as used by pinned SPD rooms.
fn center(room: &Room) -> Point {
    Point::new(
        (room.left + room.right) / 2
            + if (room.right - room.left) % 2 == 1 {
                Random::int_max(2)
            } else {
                0
            },
        (room.top + room.bottom) / 2
            + if (room.bottom - room.top) % 2 == 1 {
                Random::int_max(2)
            } else {
                0
            },
    )
}

fn door_points(room: &Room, room_index: usize, doors: &DoorMap) -> Vec<Point> {
    room.connected
        .iter()
        .filter_map(|&other| doors.get(room_index, other))
        .map(|door| Point::new(door.x, door.y))
        .collect()
}
