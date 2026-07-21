//! City `LibraryHallRoom` and its entrance/exit variants.

use crate::geom::Point;
use crate::level::terrain::{TerrainMap, BOOKSHELF, EMPTY, ENTRANCE, EXIT, REGION_DECO, WALL};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::{center, door_points, draw_inside, draw_line, fill_margin, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let mut top_bottom_books = 0.0f32;
    let mut left_right_books = 0.0f32;
    if room.width() > room.height() {
        top_bottom_books += (room.width() - room.height()) as f32;
    } else if room.height() > room.width() {
        left_right_books += (room.height() - room.width()) as f32;
    }
    if room.width() % 2 == 0 && room.height() % 2 != 0 {
        top_bottom_books += 2.0;
    } else if room.width() % 2 != 0 && room.height() % 2 == 0 {
        left_right_books += 2.0;
    }
    let doors = door_points(room, room_index, doors);
    for door in &doors {
        if door.x == room.left || door.x == room.right {
            top_bottom_books += 0.1;
        } else {
            left_right_books += 0.1;
        }
    }

    let left_to_right = top_bottom_books > left_right_books
        || (top_bottom_books == left_right_books && Random::int_max(2) == 0);
    // These counterintuitive names match the pinned Java exactly.
    let major_dim = if left_to_right {
        room.height()
    } else {
        room.width()
    };
    let minor_dim = if left_to_right {
        room.width()
    } else {
        room.height()
    };

    if !(9..11).contains(&major_dim) {
        if left_to_right {
            draw_line(
                map,
                Point::new(room.left + 1, room.top + 1),
                Point::new(room.right - 1, room.top + 1),
                BOOKSHELF,
            );
            draw_line(
                map,
                Point::new(room.left + 1, room.bottom - 1),
                Point::new(room.right - 1, room.bottom - 1),
                BOOKSHELF,
            );
        } else {
            draw_line(
                map,
                Point::new(room.left + 1, room.top + 1),
                Point::new(room.left + 1, room.bottom - 1),
                BOOKSHELF,
            );
            draw_line(
                map,
                Point::new(room.right - 1, room.top + 1),
                Point::new(room.right - 1, room.bottom - 1),
                BOOKSHELF,
            );
        }
    }

    let center = center(room);
    if major_dim >= 9 {
        let length_inset = if minor_dim >= 13 { 3 } else { 2 };
        if left_to_right {
            draw_line(
                map,
                Point::new(room.left + length_inset, center.y - 2),
                Point::new(room.right - length_inset, center.y - 2),
                BOOKSHELF,
            );
            draw_line(
                map,
                Point::new(room.left + length_inset, center.y + 2),
                Point::new(room.right - length_inset, center.y + 2),
                BOOKSHELF,
            );
        } else {
            draw_line(
                map,
                Point::new(center.x - 2, room.top + length_inset),
                Point::new(center.x - 2, room.bottom - length_inset),
                BOOKSHELF,
            );
            draw_line(
                map,
                Point::new(center.x + 2, room.top + length_inset),
                Point::new(center.x + 2, room.bottom - length_inset),
                BOOKSHELF,
            );
        }
    }

    if minor_dim % 2 == 1 && minor_dim < 9 {
        set(map, center.x, center.y, REGION_DECO);
    } else {
        let pedestal_inset = if minor_dim >= 13 {
            4
        } else if minor_dim >= 10 {
            3
        } else {
            2
        };
        if left_to_right {
            set(map, room.left + pedestal_inset, center.y, REGION_DECO);
            set(map, room.right - pedestal_inset, center.y, REGION_DECO);
        } else {
            set(map, center.x, room.top + pedestal_inset, REGION_DECO);
            set(map, center.x, room.bottom - pedestal_inset, REGION_DECO);
        }
    }

    for door in doors {
        draw_inside(map, room, door, 1, EMPTY);
    }
    paint_transition(map, room);
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let transition = match room.kind {
        RoomKind::Entrance => ENTRANCE,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    for _ in 0..10_000 {
        let point = room.random_margin(2);
        if terrain_at(map, point.x, point.y) == Some(REGION_DECO) {
            set(map, point.x, point.y, transition);
            if room.kind == RoomKind::Exit {
                if let Some(cell) = map.point_to_cell(point.x, point.y) {
                    map.character_allowed[cell] = false;
                }
            }
            return;
        }
    }
}
