//! Pinned `SacrificeRoom.paint` terrain and sacrificial-fire blob.

use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, CHASM, EMBERS, EMPTY_SP, PEDESTAL, STATUE, WALL};
use crate::rooms::room::Room;

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    depth: i32,
) {
    fill(map, room, WALL);
    fill_inset(map, room, CHASM);

    // `Room.center()` must run before `entrance()` and its alignment nudge.
    let mut center = room.as_rect().center_room();
    let door = room
        .connected
        .first()
        .and_then(|&other| doors.get(room_index, other))
        .map(|door| Point::new(door.x, door.y))
        .expect("placed SacrificeRoom has an entrance");

    if door.x == room.left || door.x == room.right {
        if door.y == center.y {
            center.y += if crate::random::Random::int_max(2) == 0 {
                -1
            } else {
                1
            };
        }
        let mut point = draw_inside(map, room, door, (door.x - center.x).abs() - 2, EMPTY_SP);
        while point.y != center.y {
            set(map, point, EMPTY_SP);
            point.y += if point.y < center.y { 1 } else { -1 };
        }
    } else {
        if door.x == center.x {
            center.x += if crate::random::Random::int_max(2) == 0 {
                -1
            } else {
                1
            };
        }
        let mut point = draw_inside(map, room, door, (door.y - center.y).abs() - 2, EMPTY_SP);
        while point.x != center.x {
            set(map, point, EMPTY_SP);
            point.x += if point.x < center.x { 1 } else { -1 };
        }
    }

    for offset in [(-2, 0), (0, -2), (2, 0), (0, 2)] {
        let statue = Point::new(center.x + offset.0, center.y + offset.1);
        if statue.x > room.left
            && statue.x < room.right
            && statue.y > room.top
            && statue.y < room.bottom
        {
            set(map, statue, STATUE);
        }
    }
    for y in (center.y - 1)..=(center.y + 1) {
        for x in (center.x - 1)..=(center.x + 1) {
            set(map, Point::new(x, y), EMBERS);
        }
    }
    set(map, center, PEDESTAL);
    let cell = map
        .point_to_cell(center.x, center.y)
        .expect("SacrificeRoom center is inside the map");
    map.record_blob_cell("SacrificialFire", false, cell, (6 + depth * 4) as u32);
}

fn fill(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            set(map, Point::new(x, y), terrain);
        }
    }
}

fn fill_inset(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            set(map, Point::new(x, y), terrain);
        }
    }
}

fn draw_inside(
    map: &mut TerrainMap,
    room: &Room,
    from: Point,
    distance: i32,
    terrain: i32,
) -> Point {
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
        set(map, point, terrain);
        point.x += step.x;
        point.y += step.y;
    }
    point
}

fn set(map: &mut TerrainMap, point: Point, terrain: i32) {
    if let Some(cell) = map.point_to_cell(point.x, point.y) {
        map.map[cell] = terrain;
    }
}
