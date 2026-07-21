//! Sewer `CircleBasinRoom` and transition variants.

use crate::geom::Point;
use crate::level::patch;
use crate::level::terrain::{
    TerrainMap, CHASM, EMPTY, EMPTY_SP, ENTRANCE_SP, EXIT, WALL, WALL_DECO, WATER,
};
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::circles::{draw_line, fill_ellipse};
use super::{center, door_points, draw_inside, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_ellipse(map, room, 1, EMPTY);

    for door in door_points(room, room_index, doors) {
        let distance = if door.x == room.left || door.x == room.right {
            room.width() / 2
        } else {
            room.height() / 2
        };
        draw_inside(map, room, door, distance, EMPTY);
    }

    fill_ellipse(map, room, 3, CHASM);
    draw_line(
        map,
        Point::new(room.left + room.width() / 2, room.top + 3),
        Point::new(room.left + room.width() / 2, room.bottom - 3),
        EMPTY_SP,
    );
    draw_line(
        map,
        Point::new(room.left + 3, room.top + room.height() / 2),
        Point::new(room.right - 3, room.top + room.height() / 2),
        EMPTY_SP,
    );

    if room.width() > 11 || room.height() > 11 {
        let center = center(room);
        fill_rect(
            map,
            center.x - 1,
            center.y - 1,
            center.x + 1,
            center.y + 1,
            EMPTY_SP,
        );
        set(map, center.x, center.y, WALL);
    }

    paint_water_patch(map, room);
    paint_transition(map, room);
}

fn paint_water_patch(map: &mut TerrainMap, room: &Room) {
    let width = room.width() - 2;
    let mask = patch::generate(width, room.height() - 2, 0.5, 5, true);
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            let patch_cell = ((x - room.left - 1) + (y - room.top - 1) * width) as usize;
            if !mask.get(patch_cell).copied().unwrap_or(false)
                || terrain_at(map, x, y) != Some(EMPTY)
            {
                continue;
            }
            set(map, x, y, WATER);
            if terrain_at(map, x, y - 1) == Some(WALL) {
                set(map, x, y - 1, WALL_DECO);
            }
        }
    }
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let terrain = match room.kind {
        RoomKind::Entrance => ENTRANCE_SP,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    let center = center(room);
    set(map, center.x, center.y, terrain);
    if room.kind == RoomKind::Exit {
        if let Some(cell) = map.point_to_cell(center.x, center.y) {
            map.character_allowed[cell] = false;
        }
    }
}
