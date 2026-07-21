//! Prison `RegionDecoLineRoom` and its entrance/exit variants.

use crate::level::terrain::{TerrainMap, EMPTY, ENTRANCE, EXIT, REGION_DECO, WALL};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::{door_points, draw_inside, fill_margin, fill_rect, fill_room, set};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let doors = door_points(room, room_index, doors);
    let mut side_preferences = [1.0f32; 4];
    for door in &doors {
        if door.y == room.top {
            side_preferences[0] -= 2.0;
        }
        if door.y == room.top + 1 {
            side_preferences[0] -= 1.0;
        }
        if door.y == room.bottom {
            side_preferences[2] -= 2.0;
        }
        if door.y == room.bottom - 1 {
            side_preferences[2] -= 1.0;
        }
        if door.x == room.left {
            side_preferences[3] -= 2.0;
        }
        if door.x == room.left + 1 {
            side_preferences[3] -= 1.0;
        }
        if door.x == room.right {
            side_preferences[1] -= 2.0;
        }
        if door.x == room.right - 1 {
            side_preferences[1] -= 1.0;
        }
    }

    let chosen_side = loop {
        let side = Random::chances(&side_preferences);
        if side >= 0 {
            break side;
        }
        for preference in &mut side_preferences {
            *preference += 1.0;
        }
    };
    match chosen_side {
        0 => fill_rect(
            map,
            room.left + 1,
            room.top + 1,
            room.right - 1,
            room.top + 1,
            REGION_DECO,
        ),
        1 => fill_rect(
            map,
            room.right - 1,
            room.top + 1,
            room.right - 1,
            room.bottom - 1,
            REGION_DECO,
        ),
        2 => fill_rect(
            map,
            room.left + 1,
            room.bottom - 1,
            room.right - 1,
            room.bottom - 1,
            REGION_DECO,
        ),
        3 => fill_rect(
            map,
            room.left + 1,
            room.top + 1,
            room.left + 1,
            room.bottom - 1,
            REGION_DECO,
        ),
        _ => return,
    }

    for door in doors {
        draw_inside(map, room, door, 1, EMPTY);
    }
    paint_transition(map, room);
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let terrain = match room.kind {
        RoomKind::Entrance => ENTRANCE,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    // There are no mobs during this analyzer's room-paint pass, so the pinned
    // do/while succeeds on its first Room.random(3) candidate.
    let point = room.random_margin(3);
    set(map, point.x, point.y, terrain);
    if room.kind == RoomKind::Exit {
        if let Some(cell) = map.point_to_cell(point.x, point.y) {
            map.character_allowed[cell] = false;
        }
    }
}
