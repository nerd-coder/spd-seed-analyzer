//! `RegularPainter.mergeRooms` for standard room pairs.

use crate::geom::Point;
use crate::level::terrain::{TerrainMap, CHASM, EMPTY, EMPTY_SP, GRASS};
use crate::rooms::room::{intersect, Room};
use crate::rooms::types::RoomKind;

pub(super) fn is_mergeable_standard(room: &Room) -> bool {
    matches!(
        room.kind,
        RoomKind::Standard | RoomKind::Entrance | RoomKind::Exit
    )
}

pub(super) fn is_normal_size(room: &Room) -> bool {
    room.size_factor <= 1
}

fn point_inside(room: &Room, from: Point, n: i32) -> Point {
    let mut step = from;
    if from.x == room.left {
        step.x += n;
    } else if from.x == room.right {
        step.x -= n;
    } else if from.y == room.top {
        step.y += n;
    } else if from.y == room.bottom {
        step.y -= n;
    }
    step
}

fn can_merge_at(map: &TerrainMap, room: &Room, p: Point) -> bool {
    if !is_mergeable_standard(room) {
        return false;
    }
    let inside = point_inside(room, p, 1);
    match map.point_to_cell(inside.x, inside.y) {
        Some(i) if room.name == "MinefieldRoom" => map.map[i] == EMPTY,
        Some(i) => !map.is_solid(i),
        None => false,
    }
}

fn merge_terrain(room: &Room, other: &Room) -> i32 {
    match room.name.as_str() {
        "PlantsRoom" if matches!(other.name.as_str(), "PlantsRoom" | "GrassyGraveRoom") => GRASS,
        "GrassyGraveRoom" if matches!(other.name.as_str(), "PlantsRoom" | "GrassyGraveRoom") => {
            GRASS
        }
        "PlatformRoom" if matches!(other.name.as_str(), "PlatformRoom" | "ChasmRoom") => CHASM,
        "ChasmRoom" if matches!(other.name.as_str(), "ChasmRoom" | "PlatformRoom") => CHASM,
        "StripedRoom" if other.name == "StripedRoom" => EMPTY_SP,
        _ => EMPTY,
    }
}

/// Open a shared wall strip with the iterated room's merge terrain when wide enough.
///
/// Uses watabou `Rect` math: `height() = bottom - top` (not inclusive).
pub(super) fn merge_rooms(map: &mut TerrainMap, r: &Room, n: &Room, start: Option<Point>) -> bool {
    let inter = intersect(r, n);
    let terrain = merge_terrain(r, n);
    if inter.left == inter.right {
        let mut top = start
            .map(|p| p.y)
            .unwrap_or_else(|| (inter.top + inter.bottom) / 2);
        let mut bottom = top;
        let x = inter.left;
        let mut p = Point::new(x, top);
        while top > inter.top && can_merge_at(map, n, p) && can_merge_at(map, r, p) {
            top -= 1;
            p.y -= 1;
        }
        p.y = bottom;
        while bottom < inter.bottom && can_merge_at(map, n, p) && can_merge_at(map, r, p) {
            bottom += 1;
            p.y += 1;
        }
        if bottom - top >= 3 {
            for y in (top + 1)..bottom {
                if let Some(i) = map.point_to_cell(x, y) {
                    map.map[i] = terrain;
                }
            }
            paint_merge_connector(map, r, n, start, terrain);
            return true;
        }
        false
    } else if inter.top == inter.bottom {
        let mut left = start
            .map(|p| p.x)
            .unwrap_or_else(|| (inter.left + inter.right) / 2);
        let mut right = left;
        let y = inter.top;
        let mut p = Point::new(left, y);
        while left > inter.left && can_merge_at(map, n, p) && can_merge_at(map, r, p) {
            left -= 1;
            p.x -= 1;
        }
        p.x = right;
        while right < inter.right && can_merge_at(map, n, p) && can_merge_at(map, r, p) {
            right += 1;
            p.x += 1;
        }
        if right - left >= 3 {
            for x in (left + 1)..right {
                if let Some(i) = map.point_to_cell(x, y) {
                    map.map[i] = terrain;
                }
            }
            paint_merge_connector(map, r, n, start, terrain);
            return true;
        }
        false
    } else {
        false
    }
}

fn paint_merge_connector(
    map: &mut TerrainMap,
    room: &Room,
    other: &Room,
    door: Option<Point>,
    terrain: i32,
) {
    if terrain != CHASM || !matches!(other.name.as_str(), "PlatformRoom" | "ChasmRoom") {
        return;
    }
    let connector = match room.name.as_str() {
        "PlatformRoom" => EMPTY_SP,
        "ChasmRoom" => EMPTY,
        _ => return,
    };
    if let Some(door) = door {
        if let Some(cell) = map.point_to_cell(door.x, door.y) {
            map.map[cell] = connector;
        }
    }
}
