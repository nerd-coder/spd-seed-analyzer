//! City `LibraryRingRoom` and its entrance/exit variants.

use crate::level::terrain::{TerrainMap, BOOKSHELF, EMPTY, EMPTY_SP, ENTRANCE_SP, EXIT, WALL};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::{center, door_points, draw_inside, fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, BOOKSHELF);
    fill_margin(map, room, 2, EMPTY);
    fill_margin(map, room, 4, BOOKSHELF);

    if room.size_factor == 3 {
        let center_x = (room.left + room.right) / 2;
        let center_y = (room.top + room.bottom) / 2;
        fill_rect(
            map,
            center_x - 4,
            center_y,
            center_x + 5,
            center_y + 1,
            EMPTY,
        );
        fill_rect(
            map,
            center_x,
            center_y - 4,
            center_x + 1,
            center_y + 5,
            EMPTY,
        );
    }
    for door in door_points(room, room_index, doors) {
        draw_inside(map, room, door, 2, EMPTY);
    }
    paint_transition(map, room);
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let transition = match room.kind {
        RoomKind::Entrance => ENTRANCE_SP,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    fill_margin(map, room, 5, EMPTY_SP);

    let mut point = center(room);
    set(map, point.x, point.y, transition);
    if room.kind == RoomKind::Exit {
        if let Some(cell) = map.point_to_cell(point.x, point.y) {
            map.character_allowed[cell] = false;
        }
    }

    let (dx, dy) = if Random::int_max(2) == 0 {
        (if Random::int_max(2) == 0 { 1 } else { -1 }, 0)
    } else {
        (0, if Random::int_max(2) == 0 { 1 } else { -1 })
    };
    point.x += dx;
    point.y += dy;
    while terrain_at(map, point.x, point.y).is_some_and(|terrain| terrain != EMPTY) {
        set(map, point.x, point.y, EMPTY_SP);
        point.x += dx;
        point.y += dy;
    }
}
