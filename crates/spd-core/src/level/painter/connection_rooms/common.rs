use crate::geom::{Point, Rect};
use crate::level::terrain::{TerrainMap, CHASM};
use crate::rooms::room::{intersect, Room};

use crate::level::painter::DoorMap;

pub(super) fn door_points(room: &Room, room_index: usize, doors: &DoorMap) -> Vec<Point> {
    room.connected
        .iter()
        .filter_map(|&other| {
            doors
                .get(room_index, other)
                .map(|door| Point::new(door.x, door.y))
        })
        .collect()
}

pub(super) fn set(map: &mut TerrainMap, point: Point, terrain: i32) {
    if let Some(cell) = map.point_to_cell(point.x, point.y) {
        map.map[cell] = terrain;
    }
}

pub(super) fn fill_rect(map: &mut TerrainMap, rect: Rect, terrain: i32) {
    for x in rect.left..=rect.right {
        for y in rect.top..=rect.bottom {
            set(map, Point::new(x, y), terrain);
        }
    }
}

pub(super) fn draw_line(map: &mut TerrainMap, from: Point, to: Point, terrain: i32) {
    if from.x == to.x {
        for y in from.y.min(to.y)..=from.y.max(to.y) {
            set(map, Point::new(from.x, y), terrain);
        }
    } else {
        for x in from.x.min(to.x)..=from.x.max(to.x) {
            set(map, Point::new(x, from.y), terrain);
        }
    }
}

pub(super) fn terrain_at(map: &TerrainMap, x: i32, y: i32) -> Option<i32> {
    map.point_to_cell(x, y).map(|cell| map.map[cell])
}

pub(super) fn paint_chasm_interior(map: &mut TerrainMap, room: &Room, force: bool) {
    if force || room.width().min(room.height()) > 3 {
        fill_rect(
            map,
            Rect {
                left: room.left + 1,
                top: room.top + 1,
                right: room.right - 1,
                bottom: room.bottom - 1,
            },
            CHASM,
        );
    }
}

pub(super) fn paint_bridge_neighbours(map: &mut TerrainMap, rooms: &[Room], room: &Room) {
    for &other in &room.neighbours {
        let Some(other) = rooms.get(other) else {
            continue;
        };
        if !matches!(
            other.name.as_str(),
            "BridgeRoom" | "RingBridgeRoom" | "WalkwayRoom"
        ) {
            continue;
        }
        let mut shared = intersect(room, other);
        if shared.raw_width() != 0 {
            shared.left += 1;
            shared.right -= 1;
        } else {
            shared.top += 1;
            shared.bottom -= 1;
        }
        fill_rect(map, shared, CHASM);
    }
}
