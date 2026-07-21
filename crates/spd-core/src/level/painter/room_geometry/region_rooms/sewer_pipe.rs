//! Sewer `SewerPipeRoom` geometry.

use crate::geom::Point;
use crate::level::terrain::{TerrainMap, EMPTY, WALL, WATER};
use crate::random::Random;
use crate::rooms::room::Room;

use super::super::super::DoorMap;
use super::circles::draw_line;
use super::{center, door_points, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(
    map: &mut TerrainMap,
    rooms: &[Room],
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) {
    fill_room(map, room, WALL);
    let door_points = door_points(room, room_index, doors);
    let connection = connection_center(room, &door_points);

    if door_points.len() == 1 || (door_points.len() == 2 && room.size_factor == 1) {
        paint_center_connections(map, room, &door_points, connection);
    } else if !door_points.is_empty() {
        paint_perimeter_connections(map, room, &door_points);
    }

    open_pipe_edges(map, room);
    paint_pipe_neighbours(map, rooms, room, room_index, doors);
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.water_allowed[cell] = false;
            }
        }
    }
}

pub(super) fn connection_center(room: &Room, doors: &[Point]) -> Point {
    if doors.len() <= 1 {
        return center(room);
    }

    let sum_x: i32 = doors.iter().map(|door| door.x).sum();
    let sum_y: i32 = doors.iter().map(|door| door.y).sum();
    let count = doors.len() as i32;
    let mut result = Point::new(sum_x / count, sum_y / count);
    // Pinned getDoorCenter stores integer sums in PointF. Both remainder tests
    // therefore fail, but their Random.Float calls are still visible.
    let _ = Random::float();
    let _ = Random::float();
    result.x = result.x.clamp(room.left + 2, room.right - 2);
    result.y = result.y.clamp(room.top + 2, room.bottom - 2);
    result
}

fn paint_center_connections(map: &mut TerrainMap, room: &Room, doors: &[Point], connection: Point) {
    for &door in doors {
        let mut start = door;
        if start.x == room.left {
            start.x += 2;
        } else if start.y == room.top {
            start.y += 2;
        } else if start.x == room.right {
            start.x -= 2;
        } else if start.y == room.bottom {
            start.y -= 2;
        }

        let right_shift = connection.x - start.x;
        let down_shift = connection.y - start.y;
        let (mid, end) = if door.x == room.left || door.x == room.right {
            let mid = Point::new(start.x + right_shift, start.y);
            (mid, Point::new(mid.x, mid.y + down_shift))
        } else {
            let mid = Point::new(start.x, start.y + down_shift);
            (mid, Point::new(mid.x + right_shift, mid.y))
        };
        draw_line(map, start, mid, WATER);
        draw_line(map, mid, end, WATER);
    }
}

fn paint_perimeter_connections(map: &mut TerrainMap, room: &Room, doors: &[Point]) {
    let mut points = doors.to_vec();
    if points.len() == 2 {
        if let Some(phantom) = phantom_door(room, doors) {
            points.push(phantom);
        }
    }
    for point in &mut points {
        if point.y == room.top {
            point.y += 2;
        } else if point.y == room.bottom {
            point.y -= 2;
        } else if point.x == room.left {
            point.x += 2;
        } else {
            point.x -= 2;
        }
    }

    let mut filled = vec![points.remove(0)];
    while !points.is_empty() {
        let mut best = (0, 0, i32::MAX);
        for (fi, from) in filled.iter().enumerate() {
            for (ti, to) in points.iter().enumerate() {
                let distance = distance_between(room, *from, *to);
                if distance < best.2 {
                    best = (fi, ti, distance);
                }
            }
        }
        let to = points[best.1];
        fill_between(map, room, filled[best.0], to);
        filled.push(to);
        points.remove(best.1);
    }
}

fn phantom_door(room: &Room, doors: &[Point]) -> Option<Point> {
    for _ in 0..10_000 {
        let point = if Random::int_max(2) == 0 {
            Point::new(
                if Random::int_max(2) == 0 {
                    room.left
                } else {
                    room.right
                },
                Random::int_range_inclusive(room.top + 2, room.bottom - 2),
            )
        } else {
            Point::new(
                Random::int_range_inclusive(room.left + 2, room.right - 2),
                if Random::int_max(2) == 0 {
                    room.top
                } else {
                    room.bottom
                },
            )
        };
        if doors
            .iter()
            .all(|door| door.x != point.x && door.y != point.y)
        {
            return Some(point);
        }
    }
    None
}

fn distance_between(room: &Room, a: Point, b: Point) -> i32 {
    if ((a.x == room.left + 2 || a.x == room.right - 2) && a.y == b.y)
        || ((a.y == room.top + 2 || a.y == room.bottom - 2) && a.x == b.x)
    {
        return space_between(a.x, b.x).max(space_between(a.y, b.y));
    }
    (space_between(room.left, a.x) + space_between(room.left, b.x))
        .min(space_between(room.right, a.x) + space_between(room.right, b.x))
        + (space_between(room.top, a.y) + space_between(room.top, b.y))
            .min(space_between(room.bottom, a.y) + space_between(room.bottom, b.y))
        - 1
}

fn fill_between(map: &mut TerrainMap, room: &Room, from: Point, to: Point) {
    if ((from.x == room.left + 2 || from.x == room.right - 2) && from.x == to.x)
        || ((from.y == room.top + 2 || from.y == room.bottom - 2) && from.y == to.y)
    {
        fill_rect(
            map,
            from.x.min(to.x),
            from.y.min(to.y),
            from.x.max(to.x),
            from.y.max(to.y),
            WATER,
        );
        return;
    }

    let corners = [
        Point::new(room.left + 2, room.top + 2),
        Point::new(room.right - 2, room.top + 2),
        Point::new(room.right - 2, room.bottom - 2),
        Point::new(room.left + 2, room.bottom - 2),
    ];
    for corner in corners {
        if (corner.x == from.x || corner.y == from.y) && (corner.x == to.x || corner.y == to.y) {
            draw_line(map, from, corner, WATER);
            draw_line(map, corner, to, WATER);
            return;
        }
    }

    let side = if from.y == room.top + 2 || from.y == room.bottom - 2 {
        let x = if space_between(room.left, from.x) + space_between(room.left, to.x)
            <= space_between(room.right, from.x) + space_between(room.right, to.x)
        {
            room.left + 2
        } else {
            room.right - 2
        };
        Point::new(x, room.top + room.height() / 2)
    } else {
        let y = if space_between(room.top, from.y) + space_between(room.top, to.y)
            <= space_between(room.bottom, from.y) + space_between(room.bottom, to.y)
        {
            room.top + 2
        } else {
            room.bottom - 2
        };
        Point::new(room.left + room.width() / 2, y)
    };
    fill_between(map, room, from, side);
    fill_between(map, room, side, to);
}

fn open_pipe_edges(map: &mut TerrainMap, room: &Room) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if terrain_at(map, x, y) != Some(WATER) {
                continue;
            }
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx != 0 || dy != 0) && terrain_at(map, x + dx, y + dy) == Some(WALL) {
                        set(map, x + dx, y + dy, EMPTY);
                    }
                }
            }
        }
    }
}

fn paint_pipe_neighbours(
    map: &mut TerrainMap,
    rooms: &[Room],
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) {
    for &other in &room.connected {
        if rooms
            .get(other)
            .is_none_or(|room| room.name != "SewerPipeRoom")
        {
            continue;
        }
        let Some(door) = doors.get(room_index, other) else {
            continue;
        };
        fill_rect(map, door.x - 1, door.y - 1, door.x + 1, door.y + 1, EMPTY);
        if door.x == room.left || door.x == room.right {
            fill_rect(map, door.x - 1, door.y, door.x + 1, door.y, WATER);
        } else {
            fill_rect(map, door.x, door.y - 1, door.x, door.y + 1, WATER);
        }
    }
}

fn space_between(a: i32, b: i32) -> i32 {
    (a - b).abs() - 1
}
