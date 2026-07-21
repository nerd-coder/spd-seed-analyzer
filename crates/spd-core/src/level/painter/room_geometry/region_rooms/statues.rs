//! City `StatuesRoom` and its entrance/exit variants.

use crate::level::terrain::{
    TerrainMap, EMPTY, EMPTY_SP, ENTRANCE_SP, EXIT, REGION_DECO_ALT, STATUE_SP, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::{center, fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let rows = (room.width() + 1) / 6;
    let cols = (room.height() + 1) / 6;
    let w = (room.width() - 4 - (rows - 1)) / rows;
    let h = (room.height() - 4 - (cols - 1)) / cols;
    let w_spacing = if rows % 2 == room.width() % 2 { 2 } else { 1 };
    let h_spacing = if cols % 2 == room.height() % 2 { 2 } else { 1 };

    for x_index in 0..rows {
        for y_index in 0..cols {
            let left = room.left + 2 + x_index * (w + w_spacing);
            let top = room.top + 2 + y_index * (h + h_spacing);
            fill_rect(map, left, top, left + w - 1, top + h - 1, EMPTY_SP);
            set(map, left, top, STATUE_SP);
            set(map, left + w - 1, top, STATUE_SP);
            set(map, left, top + h - 1, STATUE_SP);
            set(map, left + w - 1, top + h - 1, STATUE_SP);

            if w >= 5 && h >= 5 {
                let mut center_x = left + w / 2;
                if w % 2 == 0 && Random::int_max(2) == 0 {
                    center_x -= 1;
                }
                let mut center_y = top + h / 2;
                if h % 2 == 0 && Random::int_max(2) == 0 {
                    center_y -= 1;
                }
                set(map, center_x, center_y, REGION_DECO_ALT);
            }
        }
    }
    paint_transition(map, room);
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let transition = match room.kind {
        RoomKind::Entrance => ENTRANCE_SP,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    let point = center(room);
    if room.width() <= 10 && room.height() <= 10 {
        fill_margin(map, room, 3, EMPTY_SP);
    }
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if terrain_at(map, point.x + dx, point.y + dy) != Some(STATUE_SP) {
                set(map, point.x + dx, point.y + dy, EMPTY_SP);
            }
        }
    }
    set(map, point.x, point.y, transition);
    if room.kind == RoomKind::Exit {
        if let Some(cell) = map.point_to_cell(point.x, point.y) {
            map.character_allowed[cell] = false;
        }
    }
}
