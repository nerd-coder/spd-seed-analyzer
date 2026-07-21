//! Prison `ChasmBridgeRoom` and its entrance/exit variants.

use crate::geom::Rect;
use crate::level::terrain::{TerrainMap, CHASM, EMPTY, EMPTY_SP, ENTRANCE, EXIT, WALL};
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::standard_bridge;
use super::{door_points, fill_margin, fill_rect, fill_room, set};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let layout = standard_bridge::layout(
        room,
        &door_points(room, room_index, doors),
        max_bridge_width(room.width()),
        max_bridge_width(room.height()),
    );
    fill_rect(
        map,
        layout.space.left,
        layout.space.top,
        layout.space.right,
        layout.space.bottom,
        CHASM,
    );
    fill_rect(
        map,
        layout.bridge.left,
        layout.bridge.top,
        layout.bridge.right,
        layout.bridge.bottom,
        EMPTY_SP,
    );
    apply_place_masks(map, room, layout.logical_space);
    paint_transition(map, room, layout.logical_space);
}

fn max_bridge_width(room_dimension: i32) -> i32 {
    if room_dimension >= 7 {
        2
    } else {
        1
    }
}

fn apply_place_masks(map: &mut TerrainMap, room: &Room, space: Rect) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if !inside(space, x, y) {
                continue;
            }
            if let Some(cell) = map.point_to_cell(x, y) {
                map.item_allowed[cell] = false;
                map.character_allowed[cell] = false;
            }
        }
    }
}

fn paint_transition(map: &mut TerrainMap, room: &Room, space: Rect) {
    let terrain = match room.kind {
        RoomKind::Entrance => ENTRANCE,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    for _ in 0..10_000 {
        let point = room.random_margin(2);
        if inside(space, point.x, point.y) {
            continue;
        }
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx != 0 || dy != 0 {
                    set(map, point.x + dx, point.y + dy, EMPTY);
                }
            }
        }
        set(map, point.x, point.y, terrain);
        if room.kind == RoomKind::Exit {
            if let Some(cell) = map.point_to_cell(point.x, point.y) {
                map.character_allowed[cell] = false;
            }
        }
        return;
    }
}

/// watabou `Rect.inside`: left/top inclusive, right/bottom exclusive.
fn inside(rect: Rect, x: i32, y: i32) -> bool {
    x >= rect.left && x < rect.right && y >= rect.top && y < rect.bottom
}
