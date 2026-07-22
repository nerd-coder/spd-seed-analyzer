//! CrystalPath's six temporary-room layout and corridor geometry.

use crate::geom::{Point, Rect};
use crate::level::terrain::{TerrainMap, EMPTY};
use crate::rooms::room::Room;

pub(super) fn build(room: &Room, entry: Point) -> ([Rect; 6], Point, Point) {
    let mut rooms = [Rect::empty(); 6];
    let (prize1, prize2);
    if entry.x == room.left || entry.x == room.right {
        let room_w1 = if room.width() >= 9 { 2 } else { 1 };
        let room_w2 = if room.width() % 2 == 0 { 2 } else { 1 };
        let room_h = if room.height() >= 9 { 2 } else { 1 };
        if entry.x == room.left {
            rooms[0] = rect(room.left + 1, entry.y - room_h - 1, room_w1 - 1, room_h - 1);
            rooms[1] = rect(room.left + 1, entry.y + 2, room_w1 - 1, room_h - 1);
            rooms[2] = rect(
                rooms[1].right + 2,
                entry.y - room_h - 1,
                room_w1 - 1,
                room_h - 1,
            );
            rooms[3] = rect(rooms[1].right + 2, entry.y + 2, room_w1 - 1, room_h - 1);
            rooms[4] = rect(
                rooms[3].right + 2,
                entry.y - room_h - 1,
                room_w2 - 1,
                room_h,
            );
            rooms[5] = rect(rooms[3].right + 2, entry.y + 1, room_w2 - 1, room_h);
            prize1 = Point::new(rooms[4].left, rooms[4].bottom);
            prize2 = Point::new(rooms[5].left, rooms[5].top);
        } else {
            rooms[0] = rect(
                room.right - room_w1,
                entry.y - room_h - 1,
                room_w1 - 1,
                room_h - 1,
            );
            rooms[1] = rect(room.right - room_w1, entry.y + 2, room_w1 - 1, room_h - 1);
            rooms[2] = rect(
                rooms[1].left - room_w1 - 1,
                entry.y - room_h - 1,
                room_w1 - 1,
                room_h - 1,
            );
            rooms[3] = rect(
                rooms[1].left - room_w1 - 1,
                entry.y + 2,
                room_w1 - 1,
                room_h - 1,
            );
            rooms[4] = rect(
                rooms[3].left - room_w2 - 1,
                entry.y - room_h - 1,
                room_w2 - 1,
                room_h,
            );
            rooms[5] = rect(
                rooms[3].left - room_w2 - 1,
                entry.y + 1,
                room_w2 - 1,
                room_h,
            );
            prize1 = Point::new(rooms[4].right, rooms[4].bottom);
            prize2 = Point::new(rooms[5].right, rooms[5].top);
        }
    } else {
        let room_w = if room.width() >= 9 { 2 } else { 1 };
        let room_h1 = if room.height() >= 9 { 2 } else { 1 };
        let room_h2 = if room.height() % 2 == 0 { 2 } else { 1 };
        if entry.y == room.top {
            rooms[0] = rect(entry.x - room_w - 1, room.top + 1, room_w - 1, room_h1 - 1);
            rooms[1] = rect(entry.x + 2, room.top + 1, room_w - 1, room_h1 - 1);
            rooms[2] = rect(
                entry.x - room_w - 1,
                rooms[1].bottom + 2,
                room_w - 1,
                room_h1 - 1,
            );
            rooms[3] = rect(entry.x + 2, rooms[1].bottom + 2, room_w - 1, room_h1 - 1);
            rooms[4] = rect(
                entry.x - room_w - 1,
                rooms[3].bottom + 2,
                room_w,
                room_h2 - 1,
            );
            rooms[5] = rect(entry.x + 1, rooms[3].bottom + 2, room_w, room_h2 - 1);
            prize1 = Point::new(rooms[4].right, rooms[4].top);
            prize2 = Point::new(rooms[5].left, rooms[5].top);
        } else {
            rooms[0] = rect(
                entry.x - room_w - 1,
                room.bottom - room_h1,
                room_w - 1,
                room_h1 - 1,
            );
            rooms[1] = rect(entry.x + 2, room.bottom - room_h1, room_w - 1, room_h1 - 1);
            rooms[2] = rect(
                entry.x - room_w - 1,
                rooms[1].top - room_h1 - 1,
                room_w - 1,
                room_h1 - 1,
            );
            rooms[3] = rect(
                entry.x + 2,
                rooms[1].top - room_h1 - 1,
                room_w - 1,
                room_h1 - 1,
            );
            rooms[4] = rect(
                entry.x - room_w - 1,
                rooms[3].top - room_h2 - 1,
                room_w,
                room_h2 - 1,
            );
            rooms[5] = rect(entry.x + 1, rooms[3].top - room_h2 - 1, room_w, room_h2 - 1);
            prize1 = Point::new(rooms[4].right, rooms[4].bottom);
            prize2 = Point::new(rooms[5].left, rooms[5].bottom);
        }
    }
    (rooms, prize1, prize2)
}

fn rect(left: i32, top: i32, width: i32, height: i32) -> Rect {
    Rect {
        left,
        top,
        right: left + width,
        bottom: top + height,
    }
}

pub(super) fn internal_doors(room: &Room, entry: Point, rooms: &[Rect; 6]) -> [Point; 6] {
    if entry.x == room.left {
        [
            Point::new(rooms[0].left, rooms[0].bottom + 1),
            Point::new(rooms[1].left, rooms[1].top - 1),
            Point::new(rooms[2].left, rooms[2].bottom + 1),
            Point::new(rooms[3].left, rooms[3].top - 1),
            Point::new(rooms[4].left - 1, rooms[4].bottom - 1),
            Point::new(rooms[5].left - 1, rooms[5].top + 1),
        ]
    } else if entry.x == room.right {
        [
            Point::new(rooms[0].right, rooms[0].bottom + 1),
            Point::new(rooms[1].right, rooms[1].top - 1),
            Point::new(rooms[2].right, rooms[2].bottom + 1),
            Point::new(rooms[3].right, rooms[3].top - 1),
            Point::new(rooms[4].right + 1, rooms[4].bottom - 1),
            Point::new(rooms[5].right + 1, rooms[5].top + 1),
        ]
    } else if entry.y == room.top {
        [
            Point::new(rooms[0].right + 1, rooms[0].top),
            Point::new(rooms[1].left - 1, rooms[1].top),
            Point::new(rooms[2].right + 1, rooms[2].top),
            Point::new(rooms[3].left - 1, rooms[3].top),
            Point::new(rooms[4].right - 1, rooms[4].top - 1),
            Point::new(rooms[5].left + 1, rooms[5].top - 1),
        ]
    } else {
        [
            Point::new(rooms[0].right + 1, rooms[0].bottom),
            Point::new(rooms[1].left - 1, rooms[1].bottom),
            Point::new(rooms[2].right + 1, rooms[2].bottom),
            Point::new(rooms[3].left - 1, rooms[3].bottom),
            Point::new(rooms[4].right - 1, rooms[4].bottom + 1),
            Point::new(rooms[5].left + 1, rooms[5].bottom + 1),
        ]
    }
}

pub(super) fn draw_inside(map: &mut TerrainMap, room: &Room, entry: Point, distance: i32) {
    let (dx, dy) = if entry.x == room.left {
        (1, 0)
    } else if entry.x == room.right {
        (-1, 0)
    } else if entry.y == room.top {
        (0, 1)
    } else {
        (0, -1)
    };
    for step in 1..=distance {
        set(
            map,
            Point::new(entry.x + dx * step, entry.y + dy * step),
            EMPTY,
        );
    }
}

fn set(map: &mut TerrainMap, point: Point, terrain: i32) {
    if let Some(cell) = map.point_to_cell(point.x, point.y) {
        map.map[cell] = terrain;
    }
}
