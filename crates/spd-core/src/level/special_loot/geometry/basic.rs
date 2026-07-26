//! Basic special-room terrain painters.

use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{
    TerrainMap, BOOKSHELF, CHASM, EMPTY, EMPTY_SP, EMPTY_WELL, GRASS, HIGH_GRASS, PEDESTAL, WALL,
    WATER,
};
use crate::random::Random;
use crate::rooms::room::Room;

/// Pinned `PitRoom.paint` canvas and well placement.
pub(super) fn paint_pit(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let door = entrance(room, room_index, doors).expect("placed PitRoom has an entrance");
    let well = if door.x == room.left {
        Point::new(
            room.right - 1,
            if Random::int_max(2) == 0 {
                room.top + 1
            } else {
                room.bottom - 1
            },
        )
    } else if door.x == room.right {
        Point::new(
            room.left + 1,
            if Random::int_max(2) == 0 {
                room.top + 1
            } else {
                room.bottom - 1
            },
        )
    } else if door.y == room.top {
        Point::new(
            if Random::int_max(2) == 0 {
                room.left + 1
            } else {
                room.right - 1
            },
            room.bottom - 1,
        )
    } else {
        Point::new(
            if Random::int_max(2) == 0 {
                room.left + 1
            } else {
                room.right - 1
            },
            room.top + 1,
        )
    };
    set(map, well, EMPTY_WELL);
}

pub(super) fn paint_pool(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) -> Option<usize> {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, WATER);

    let door = entrance(room, room_index, doors)?;
    let pedestal = if door.x == room.left {
        for y in (room.top + 1)..room.bottom {
            set(map, Point::new(room.left + 1, y), EMPTY_SP);
        }
        Point::new(room.right - 1, room.top + room.height() / 2)
    } else if door.x == room.right {
        for y in (room.top + 1)..room.bottom {
            set(map, Point::new(room.right - 1, y), EMPTY_SP);
        }
        Point::new(room.left + 1, room.top + room.height() / 2)
    } else if door.y == room.top {
        for x in (room.left + 1)..room.right {
            set(map, Point::new(x, room.top + 1), EMPTY_SP);
        }
        Point::new(room.left + room.width() / 2, room.bottom - 1)
    } else {
        for x in (room.left + 1)..room.right {
            set(map, Point::new(x, room.bottom - 1), EMPTY_SP);
        }
        Point::new(room.left + room.width() / 2, room.top + 1)
    };
    set(map, pedestal, PEDESTAL);
    let cell = map.point_to_cell(pedestal.x, pedestal.y);
    if let Some(cell) = cell {
        map.heap_occupied[cell] = true;
        map.item_allowed[cell] = false;
    }
    cell
}

/// Pinned `GardenRoom.paint` terrain layers. The loot helper performs the
/// subsequent bush and `plantPos` rolls in Java order.
pub(super) fn paint_garden(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, HIGH_GRASS);
    fill_margin(map, room, 2, GRASS);
}

/// Pinned `LibraryRoom.paint` canvas, including the bookshelf interrupted by
/// the one-cell entrance passage.
pub(super) fn paint_library(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY_SP);
    for x in (room.left + 1)..room.right {
        set(map, Point::new(x, room.top + 1), BOOKSHELF);
    }
    let door = entrance(room, room_index, doors).expect("placed LibraryRoom has an entrance");
    draw_inside(map, room, door, 1, EMPTY_SP);
}

/// Pinned `SecretLibraryRoom.paint` canvas before its reward loop.
pub(super) fn paint_secret_library(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, BOOKSHELF);
    fill_ellipse(map, room, 2, EMPTY_SP);
    let door = entrance(room, room_index, doors).expect("placed SecretLibraryRoom has an entrance");
    let distance = if door.x == room.left || door.x == room.right {
        (room.width() - 3) / 2
    } else {
        (room.height() - 3) / 2
    };
    draw_inside(map, room, door, distance, EMPTY_SP);
}

pub(super) fn paint_runestone(
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
    draw_inside(map, room, door, 2, EMPTY_SP);
    fill_margin(map, room, 2, EMPTY);
}

pub(super) fn paint_secret_runestone(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);
    let door =
        entrance(room, room_index, doors).expect("placed SecretRunestoneRoom has an entrance");
    let center = room.as_rect().center_room();
    if door.x == room.left || door.x == room.right {
        for y in (room.top + 1)..room.bottom {
            set(map, Point::new(center.x, y), BOOKSHELF);
        }
        let (left, right) = if door.x == room.left {
            (center.x + 1, room.right - 1)
        } else {
            (room.left + 1, center.x - 1)
        };
        for y in (room.top + 1)..room.bottom {
            for x in left..=right {
                set(map, Point::new(x, y), EMPTY_SP);
            }
        }
    } else {
        for x in (room.left + 1)..room.right {
            set(map, Point::new(x, center.y), BOOKSHELF);
        }
        let (top, bottom) = if door.y == room.top {
            (center.y + 1, room.bottom - 1)
        } else {
            (room.top + 1, center.y - 1)
        };
        for y in top..=bottom {
            for x in (room.left + 1)..room.right {
                set(map, Point::new(x, y), EMPTY_SP);
            }
        }
    }
}

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
        map.known_mobs[cell] = Some("DemonSpawner");
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

/// Pinned `AmbitiousImpRoom.paint` canvas and placement permissions.
pub(super) fn paint_ambitious_imp(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) {
    fill_room(map, room, crate::level::terrain::WALL_DECO);
    fill_margin(map, room, 1, crate::level::terrain::EMPTY);
    let center = room.as_rect().center_room();
    for (dx, dy) in [(-2, -2), (2, -2), (-2, 2), (2, 2)] {
        set(
            map,
            Point::new(center.x + dx, center.y + dy),
            crate::level::terrain::REGION_DECO,
        );
    }
    for (dx, dy) in [(-3, -3), (3, -3), (-3, 3), (3, 3)] {
        set(
            map,
            Point::new(center.x + dx, center.y + dy),
            crate::level::terrain::WALL_DECO,
        );
    }
    if let Some(door) = entrance(room, room_index, doors) {
        draw_inside(map, room, door, 1, crate::level::terrain::EMPTY);
    }
    set(map, center, crate::level::terrain::EXIT);

    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.item_allowed[cell] = false;
                map.character_allowed[cell] = false;
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

fn fill_ellipse(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    let x = room.left + margin;
    let y = room.top + margin;
    let width = room.width() - margin * 2;
    let height = room.height() - margin * 2;
    let radius_h = height as f64 / 2.0;
    let radius_w = width as f64 / 2.0;
    for row in 0..height {
        let row_y = -radius_h + 0.5 + row as f64;
        let mut row_width = 2.0
            * ((radius_w * radius_w) * (1.0 - row_y * row_y / (radius_h * radius_h)))
                .max(0.0)
                .sqrt();
        row_width = if width % 2 == 0 {
            (row_width / 2.0).round() * 2.0
        } else {
            (row_width / 2.0).floor() * 2.0 + 1.0
        };
        let row_width = row_width as i32;
        let start = x + (width - row_width) / 2;
        for column in start..(start + row_width) {
            set(map, Point::new(column, y + row), terrain);
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
