//! Exact `CryptRoom.paint` geometry and tomb cell selection.

use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, EMPTY, STATUE, WALL};
use crate::rooms::room::Room;

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) -> usize {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    // Java resolves both coordinates before inspecting the entrance, so an
    // even inclusive room dimension still consumes its center jitter even if
    // that coordinate is replaced by the directional tomb offset below.
    let mut tomb = room.as_rect().center_room();
    let entrance = room
        .connected
        .iter()
        .find_map(|&other| doors.get(room_index, other))
        .expect("placed CryptRoom has an entrance");

    if entrance.x == room.left {
        set(map, Point::new(room.right - 1, room.top + 1), STATUE);
        set(map, Point::new(room.right - 1, room.bottom - 1), STATUE);
        tomb.x = room.right - 2;
    } else if entrance.x == room.right {
        set(map, Point::new(room.left + 1, room.top + 1), STATUE);
        set(map, Point::new(room.left + 1, room.bottom - 1), STATUE);
        tomb.x = room.left + 2;
    } else if entrance.y == room.top {
        set(map, Point::new(room.left + 1, room.bottom - 1), STATUE);
        set(map, Point::new(room.right - 1, room.bottom - 1), STATUE);
        tomb.y = room.bottom - 2;
    } else {
        set(map, Point::new(room.left + 1, room.top + 1), STATUE);
        set(map, Point::new(room.right - 1, room.top + 1), STATUE);
        tomb.y = room.top + 2;
    }

    map.point_to_cell(tomb.x, tomb.y)
        .expect("CryptRoom tomb is inside the map")
}

fn set(map: &mut TerrainMap, point: Point, terrain: i32) {
    if let Some(cell) = map.point_to_cell(point.x, point.y) {
        map.map[cell] = terrain;
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            set(map, Point::new(x, y), terrain);
        }
    }
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    for y in (room.top + margin)..=(room.bottom - margin) {
        for x in (room.left + margin)..=(room.right - margin) {
            set(map, Point::new(x, y), terrain);
        }
    }
}
