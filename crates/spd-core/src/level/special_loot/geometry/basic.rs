//! Weak-floor and demon-spawner room painters.

use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, CHASM, EMPTY, EMPTY_SP, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

pub(super) fn paint_weak_floor(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, CHASM);

    let Some(door) = entrance(room, room_index, doors) else {
        return;
    };
    let well = if door.x == room.left {
        for y in (room.top + 1)..room.bottom {
            draw_inside(
                map,
                room,
                Point::new(room.left, y),
                Random::int_range_inclusive(1, room.width() - 4),
                EMPTY_SP,
            );
        }
        Point::new(
            room.right - 1,
            if Random::int_max(2) == 0 {
                room.top + 2
            } else {
                room.bottom - 1
            },
        )
    } else if door.x == room.right {
        for y in (room.top + 1)..room.bottom {
            draw_inside(
                map,
                room,
                Point::new(room.right, y),
                Random::int_range_inclusive(1, room.width() - 4),
                EMPTY_SP,
            );
        }
        Point::new(
            room.left + 1,
            if Random::int_max(2) == 0 {
                room.top + 2
            } else {
                room.bottom - 1
            },
        )
    } else if door.y == room.top {
        for x in (room.left + 1)..room.right {
            draw_inside(
                map,
                room,
                Point::new(x, room.top),
                Random::int_range_inclusive(1, room.height() - 4),
                EMPTY_SP,
            );
        }
        Point::new(
            if Random::int_max(2) == 0 {
                room.left + 1
            } else {
                room.right - 1
            },
            room.bottom - 1,
        )
    } else {
        for x in (room.left + 1)..room.right {
            draw_inside(
                map,
                room,
                Point::new(x, room.bottom),
                Random::int_range_inclusive(1, room.height() - 4),
                EMPTY_SP,
            );
        }
        Point::new(
            if Random::int_max(2) == 0 {
                room.left + 1
            } else {
                room.right - 1
            },
            room.top + 2,
        )
    };
    set(map, well, CHASM);
}

pub(super) fn paint_demon_spawner(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    // The point is used for the spawned mob. `Room.center()` consumes one
    // Int(2) for each even-sized axis even though the mob is not exported yet.
    let center = room.as_rect().center_room();
    if let Some(cell) = map.point_to_cell(center.x, center.y) {
        map.mob_occupied[cell] = true;
    }

    // DemonSpawnerRoom refuses all three ambient painter types everywhere.
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.water_allowed[cell] = false;
                map.grass_allowed[cell] = false;
                map.trap_allowed[cell] = false;
            }
        }
    }
}

fn entrance(room: &Room, room_index: usize, doors: &DoorMap) -> Option<Point> {
    room.connected.first().and_then(|&other| {
        doors
            .get(room_index, other)
            .map(|door| Point::new(door.x, door.y))
    })
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

fn draw_inside(map: &mut TerrainMap, room: &Room, from: Point, distance: i32, terrain: i32) {
    let (dx, dy) = if from.x == room.left {
        (1, 0)
    } else if from.x == room.right {
        (-1, 0)
    } else if from.y == room.top {
        (0, 1)
    } else {
        (0, -1)
    };
    for step in 1..=distance {
        set(
            map,
            Point::new(from.x + dx * step, from.y + dy * step),
            terrain,
        );
    }
}
