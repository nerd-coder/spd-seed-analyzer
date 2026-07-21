//! Sewer `RingRoom` and transition variants.

use crate::geom::Point;
use crate::level::terrain::{
    TerrainMap, DOOR, EMPTY, EMPTY_SP, ENTRANCE_SP, EXIT, REGION_DECO_ALT, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::{center, fill_margin, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room) -> Option<Point> {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let min_dim = room.width().min(room.height());
    let passage_width = ((min_dim + 3) as f32 * 0.2).floor() as i32;
    fill_margin(map, room, passage_width + 1, WALL);
    if min_dim < 10 {
        return None;
    }

    let deco = if matches!(room.kind, RoomKind::Entrance | RoomKind::Exit) {
        EMPTY_SP
    } else {
        REGION_DECO_ALT
    };
    fill_margin(map, room, passage_width + 2, deco);

    let mut center = center(room);
    let (x_dir, y_dir) = pick_center_door_direction(room, center);
    set(map, center.x, center.y, EMPTY_SP);
    match room.kind {
        RoomKind::Entrance => set(map, center.x, center.y, ENTRANCE_SP),
        RoomKind::Exit => {
            set(map, center.x, center.y, EXIT);
            if let Some(cell) = map.point_to_cell(center.x, center.y) {
                map.character_allowed[cell] = false;
            }
        }
        _ => {}
    }

    let prize_center = (room.kind == RoomKind::Standard).then_some(center);
    center.x += x_dir;
    center.y += y_dir;
    while terrain_at(map, center.x, center.y).is_some_and(|terrain| terrain != WALL) {
        set(map, center.x, center.y, EMPTY_SP);
        center.x += x_dir;
        center.y += y_dir;
    }
    set(map, center.x, center.y, DOOR);
    prize_center
}

fn pick_center_door_direction(room: &Room, center: Point) -> (i32, i32) {
    if Random::int_max(2) == 0 {
        let midpoint = (room.left + room.right) as f32 / 2.0;
        let x_dir = if (center.x as f32) < midpoint {
            1
        } else if (center.x as f32) > midpoint {
            -1
        } else if Random::int_max(2) == 0 {
            1
        } else {
            -1
        };
        (x_dir, 0)
    } else {
        let midpoint = (room.top + room.bottom) as f32 / 2.0;
        let y_dir = if (center.y as f32) < midpoint {
            1
        } else if (center.y as f32) > midpoint {
            -1
        } else if Random::int_max(2) == 0 {
            1
        } else {
            -1
        };
        (0, y_dir)
    }
}
