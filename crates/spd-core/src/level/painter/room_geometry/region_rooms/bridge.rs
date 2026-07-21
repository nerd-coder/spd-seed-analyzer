//! `RegionDecoBridgeRoom` and its cave entrance/exit variants.

use crate::geom::Rect;
use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_SP, ENTRANCE, EXIT, REGION_DECO_ALT, WALL};
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::standard_bridge;
use super::{door_points, fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let door_points = door_points(room, room_index, doors);
    let layout = standard_bridge::layout(room, &door_points, 1, 1);
    let space = layout.space;
    let bridge = layout.bridge;

    fill_rect(
        map,
        space.left,
        space.top,
        space.right,
        space.bottom,
        REGION_DECO_ALT,
    );
    fill_rect(
        map,
        bridge.left,
        bridge.top,
        bridge.right,
        bridge.bottom,
        EMPTY_SP,
    );
    for y in space.top..=space.bottom {
        for x in space.left..=space.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.item_allowed[cell] = false;
                map.character_allowed[cell] = false;
            }
        }
    }

    if matches!(room.kind, RoomKind::Entrance | RoomKind::Exit) {
        paint_transition(map, room, space);
    }
}

fn paint_transition(map: &mut TerrainMap, room: &Room, space: Rect) {
    let terrain = if room.kind == RoomKind::Entrance {
        ENTRANCE
    } else {
        EXIT
    };
    for _ in 0..10_000 {
        let p = room.random_margin(2);
        if p.x >= space.left && p.x <= space.right && p.y >= space.top && p.y <= space.bottom {
            continue;
        }
        let touches_deco = (-1..=1).any(|dy| {
            (-1..=1).any(|dx| {
                (dx != 0 || dy != 0) && terrain_at(map, p.x + dx, p.y + dy) == Some(REGION_DECO_ALT)
            })
        });
        if !touches_deco {
            set(map, p.x, p.y, terrain);
            return;
        }
    }
}
