use crate::geom::{Point, Rect};
use crate::level::painter::DoorMap;
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

use super::common::{door_points, draw_line, fill_rect};

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    floor: i32,
) {
    let mut pending = door_points(room, room_index, doors);
    for point in &mut pending {
        if point.y == room.top {
            point.y += 1;
        } else if point.y == room.bottom {
            point.y -= 1;
        } else if point.x == room.left {
            point.x += 1;
        } else {
            point.x -= 1;
        }
    }
    if pending.is_empty() {
        return;
    }
    let mut filled = vec![pending.remove(0)];
    while !pending.is_empty() {
        let mut best = None;
        let mut shortest = i32::MAX;
        for &from in &filled {
            for (index, &to) in pending.iter().enumerate() {
                let distance = distance(room, from, to);
                if distance < shortest {
                    shortest = distance;
                    best = Some((from, to, index));
                }
            }
        }
        let Some((from, to, index)) = best else {
            break;
        };
        fill_between(map, room, from, to, floor);
        filled.push(to);
        pending.remove(index);
    }
}

fn space_between(a: i32, b: i32) -> i32 {
    (a - b).abs() - 1
}

fn distance(room: &Room, a: Point, b: Point) -> i32 {
    if ((a.x == room.left + 1 || a.x == room.right - 1) && a.y == b.y)
        || ((a.y == room.top + 1 || a.y == room.bottom - 1) && a.x == b.x)
    {
        return space_between(a.x, b.x).max(space_between(a.y, b.y));
    }
    (space_between(room.left, a.x) + space_between(room.left, b.x))
        .min(space_between(room.right, a.x) + space_between(room.right, b.x))
        + (space_between(room.top, a.y) + space_between(room.top, b.y))
            .min(space_between(room.bottom, a.y) + space_between(room.bottom, b.y))
        - 1
}

fn fill_between(map: &mut TerrainMap, room: &Room, from: Point, to: Point, floor: i32) {
    if ((from.x == room.left + 1 || from.x == room.right - 1) && from.x == to.x)
        || ((from.y == room.top + 1 || from.y == room.bottom - 1) && from.y == to.y)
    {
        fill_rect(
            map,
            Rect {
                left: from.x.min(to.x),
                top: from.y.min(to.y),
                right: from.x.max(to.x),
                bottom: from.y.max(to.y),
            },
            floor,
        );
        return;
    }

    for corner in [
        Point::new(room.left + 1, room.top + 1),
        Point::new(room.right - 1, room.top + 1),
        Point::new(room.right - 1, room.bottom - 1),
        Point::new(room.left + 1, room.bottom - 1),
    ] {
        if (corner.x == from.x || corner.y == from.y) && (corner.x == to.x || corner.y == to.y) {
            draw_line(map, from, corner, floor);
            draw_line(map, corner, to, floor);
            return;
        }
    }

    let side = if from.y == room.top + 1 || from.y == room.bottom - 1 {
        if space_between(room.left, from.x) + space_between(room.left, to.x)
            <= space_between(room.right, from.x) + space_between(room.right, to.x)
        {
            Point::new(room.left + 1, room.top + room.height() / 2)
        } else {
            Point::new(room.right - 1, room.top + room.height() / 2)
        }
    } else if space_between(room.top, from.y) + space_between(room.top, to.y)
        <= space_between(room.bottom, from.y) + space_between(room.bottom, to.y)
    {
        Point::new(room.left + room.width() / 2, room.top + 1)
    } else {
        Point::new(room.left + room.width() / 2, room.bottom - 1)
    };
    fill_between(map, room, from, side, floor);
    fill_between(map, room, side, to, floor);
}
