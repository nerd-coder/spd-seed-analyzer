use crate::geom::{Point, Rect};
use crate::level::terrain::{TerrainMap, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

use crate::level::painter::DoorMap;

use super::common::{door_points, draw_line, fill_rect, set, terrain_at};

fn door_center(room: &Room, points: &[Point]) -> Point {
    let sum_x: i32 = points.iter().map(|point| point.x).sum();
    let sum_y: i32 = points.iter().map(|point| point.y).sum();
    let mut center = Point::new(sum_x / points.len() as i32, sum_y / points.len() as i32);
    // Pinned code casts its integer-valued PointF sums before division, but
    // still evaluates both fractional rounding Floats.
    let _ = Random::float();
    let _ = Random::float();
    center.x = center.x.clamp(room.left + 1, room.right - 1);
    center.y = center.y.clamp(room.top + 1, room.bottom - 1);
    center
}

pub(super) fn connection_space(room: &Room, points: &[Point], ring: bool) -> Rect {
    let mut center = door_center(room, points);
    if ring {
        center.x = center.x.clamp(room.left + 2, room.right - 2);
        center.y = center.y.clamp(room.top + 2, room.bottom - 2);
        Rect {
            left: center.x - 1,
            top: center.y - 1,
            right: center.x + 1,
            bottom: center.y + 1,
        }
    } else {
        Rect {
            left: center.x,
            top: center.y,
            right: center.x,
            bottom: center.y,
        }
    }
}

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    floor: i32,
    ring: bool,
) {
    let points = door_points(room, room_index, doors);
    if points.is_empty() {
        return;
    }
    let connection = connection_space(room, &points, ring);
    for door in points {
        let mut start = door;
        if start.x == room.left {
            start.x += 1;
        } else if start.y == room.top {
            start.y += 1;
        } else if start.x == room.right {
            start.x -= 1;
        } else if start.y == room.bottom {
            start.y -= 1;
        }
        let right_shift = if start.x < connection.left {
            connection.left - start.x
        } else if start.x > connection.right {
            connection.right - start.x
        } else {
            0
        };
        let down_shift = if start.y < connection.top {
            connection.top - start.y
        } else if start.y > connection.bottom {
            connection.bottom - start.y
        } else {
            0
        };
        let (mid, end) = if door.x == room.left || door.x == room.right {
            let mid = Point::new(start.x + right_shift, start.y);
            (mid, Point::new(mid.x, mid.y + down_shift))
        } else {
            let mid = Point::new(start.x, start.y + down_shift);
            (mid, Point::new(mid.x + right_shift, mid.y))
        };
        draw_line(map, start, mid, floor);
        draw_line(map, mid, end, floor);
    }

    if ring {
        fill_rect(map, connection, floor);
        set(
            map,
            Point::new(connection.left + 1, connection.top + 1),
            WALL,
        );
    } else {
        paint_optional_diagonal(map, room, connection, floor);
    }
}

fn paint_optional_diagonal(map: &mut TerrainMap, room: &Room, connection: Rect, floor: i32) {
    if room.width() < 7 || room.height() < 7 || room.connected.len() < 4 {
        return;
    }
    let offset = 2 * Random::int_max(4);
    let circle = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];
    let center = Point::new(connection.left, connection.top);
    let before = circle[((offset + 7) % 8) as usize];
    let after = circle[((offset + 1) % 8) as usize];
    if terrain_at(map, center.x + before.0, center.y + before.1) == Some(floor)
        && terrain_at(map, center.x + after.0, center.y + after.1) == Some(floor)
    {
        let chosen = circle[offset as usize];
        set(
            map,
            Point::new(center.x + chosen.0, center.y + chosen.1),
            floor,
        );
    }
}
