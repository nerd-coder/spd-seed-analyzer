//! Sewer `WaterBridgeRoom` and its entrance/exit variants.

use crate::geom::Rect;
use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_SP, ENTRANCE, EXIT, WALL, WATER};
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::standard_bridge;
use super::{door_points, fill_margin, fill_rect, fill_room, set};

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    depth: i32,
) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let layout = standard_bridge::layout(
        room,
        &door_points(room, room_index, doors),
        if room.width() >= 8 { 3 } else { 2 },
        if room.height() >= 8 { 3 } else { 2 },
    );
    fill_rect(
        map,
        layout.space.left,
        layout.space.top,
        layout.space.right,
        layout.space.bottom,
        WATER,
    );
    fill_rect(
        map,
        layout.bridge.left,
        layout.bridge.top,
        layout.bridge.right,
        layout.bridge.bottom,
        EMPTY_SP,
    );

    apply_place_masks(map, room, layout.logical_space, depth);
    if matches!(room.kind, RoomKind::Entrance | RoomKind::Exit) {
        paint_transition(map, room, layout.logical_space);
    }
}

fn apply_place_masks(map: &mut TerrainMap, room: &Room, space: Rect, depth: i32) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let Some(cell) = map.point_to_cell(x, y) else {
                continue;
            };
            map.water_allowed[cell] = false;
            if inside(space, x, y) {
                map.item_allowed[cell] = false;
                map.character_allowed[cell] = false;
            }
            if room.kind == RoomKind::Entrance && depth == 1 {
                map.trap_allowed[cell] = false;
            }
        }
    }
}

fn paint_transition(map: &mut TerrainMap, room: &Room, space: Rect) {
    let mut selected = None;
    for _ in 0..10_000 {
        let point = room.random_margin(2);
        if !inside(space, point.x, point.y) {
            selected = Some(point);
            break;
        }
    }
    let Some(point) = selected else {
        return;
    };

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                set(map, point.x + dx, point.y + dy, EMPTY);
            }
        }
    }
    let terrain = if room.kind == RoomKind::Entrance {
        ENTRANCE
    } else {
        EXIT
    };
    set(map, point.x, point.y, terrain);
    if room.kind == RoomKind::Exit {
        if let Some(cell) = map.point_to_cell(point.x, point.y) {
            map.character_allowed[cell] = false;
        }
    }
}

/// watabou `Rect.inside`: left/top inclusive, right/bottom exclusive.
fn inside(rect: Rect, x: i32, y: i32) -> bool {
    x >= rect.left && x < rect.right && y >= rect.top && y < rect.bottom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_rect_inside_includes_left_and_top_edges() {
        let rect = Rect {
            left: 2,
            top: 3,
            right: 6,
            bottom: 8,
        };
        assert!(inside(rect, 2, 3));
        assert!(inside(rect, 5, 7));
        assert!(!inside(rect, 6, 3));
        assert!(!inside(rect, 2, 8));
    }
}
