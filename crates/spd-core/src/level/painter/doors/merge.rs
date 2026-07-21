//! `RegularPainter.mergeRooms` for standard room pairs.

use crate::geom::{Point, Rect};
use crate::level::terrain::{TerrainMap, CHASM, EMPTY, EMPTY_SP, GRASS, REGION_DECO_ALT, WATER};
use crate::random::Random;
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

fn can_merge_at(
    map: &TerrainMap,
    room: &Room,
    other: &Room,
    p: Point,
    merge_terrain: i32,
    depth: i32,
) -> bool {
    if room.kind == RoomKind::Connection
        && matches!(
            room.name.as_str(),
            "BridgeRoom" | "WalkwayRoom" | "RingBridgeRoom"
        )
    {
        return merge_terrain == CHASM;
    }
    if !is_mergeable_standard(room) {
        return false;
    }
    let inside = point_inside(room, p, 1);
    match map.point_to_cell(inside.x, inside.y) {
        Some(_) if room.name == "SewerPipeRoom" => false,
        Some(_)
            if matches!(
                room.name.as_str(),
                "HallwayRoom" | "HallwayEntranceRoom" | "HallwayExitRoom"
            ) && !matches!(
                other.name.as_str(),
                "HallwayRoom" | "HallwayEntranceRoom" | "HallwayExitRoom"
            ) =>
        {
            false
        }
        Some(_) if room.name == "WaterBridgeEntranceRoom" && depth <= 2 => false,
        Some(i)
            if matches!(
                room.name.as_str(),
                "WaterBridgeRoom" | "WaterBridgeEntranceRoom" | "WaterBridgeExitRoom"
            ) =>
        {
            map.map[i] != WATER
        }
        Some(i) if matches!(room.name.as_str(), "BurnedRoom" | "MinefieldRoom") => {
            map.map[i] == EMPTY
        }
        Some(i)
            if room.name == "CavesFissureRoom"
                || room.name == "CavesFissureEntranceRoom"
                || room.name == "CavesFissureExitRoom" =>
        {
            merge_terrain == CHASM || map.map[i] != CHASM
        }
        Some(i)
            if matches!(
                room.name.as_str(),
                "RegionDecoBridgeRoom"
                    | "RegionDecoBridgeEntranceRoom"
                    | "RegionDecoBridgeExitRoom"
            ) =>
        {
            map.map[i] != REGION_DECO_ALT
        }
        Some(i)
            if matches!(
                room.name.as_str(),
                "ChasmBridgeRoom" | "ChasmBridgeEntranceRoom" | "ChasmBridgeExitRoom"
            ) =>
        {
            map.map[i] != CHASM
        }
        Some(i) => !map.is_solid(i),
        None => false,
    }
}

fn effective_merge_terrain(room: &Room, other: &Room, merge_terrain: i32) -> i32 {
    match room.name.as_str() {
        "PlantsRoom"
            if merge_terrain == EMPTY
                && matches!(other.name.as_str(), "PlantsRoom" | "GrassyGraveRoom") =>
        {
            GRASS
        }
        "GrassyGraveRoom"
            if merge_terrain == EMPTY
                && matches!(other.name.as_str(), "PlantsRoom" | "GrassyGraveRoom") =>
        {
            GRASS
        }
        "PlatformRoom"
            if merge_terrain != CHASM
                && room.connected.contains(&other.id)
                && matches!(other.name.as_str(), "PlatformRoom" | "ChasmRoom") =>
        {
            CHASM
        }
        "ChasmRoom"
            if merge_terrain == EMPTY
                && matches!(other.name.as_str(), "ChasmRoom" | "PlatformRoom") =>
        {
            CHASM
        }
        "StripedRoom" if merge_terrain == EMPTY && other.name == "StripedRoom" => EMPTY_SP,
        _ => merge_terrain,
    }
}

/// Open a shared wall strip with the iterated room's merge terrain when wide enough.
///
/// Uses watabou `Rect` math: `height() = bottom - top` (not inclusive).
pub(super) fn merge_rooms(
    map: &mut TerrainMap,
    r: &Room,
    n: &Room,
    start: Option<Point>,
    depth: i32,
) -> bool {
    merge_rooms_with_terrain(map, r, n, start, EMPTY, depth)
}

pub(in crate::level::painter) fn merge_rooms_with_terrain(
    map: &mut TerrainMap,
    r: &Room,
    n: &Room,
    start: Option<Point>,
    requested_terrain: i32,
    depth: i32,
) -> bool {
    let inter = intersect(r, n);
    let terrain = effective_merge_terrain(r, n, requested_terrain);
    if inter.left == inter.right {
        let merge_start = start.unwrap_or_else(|| random_rect_center(inter));
        let mut top = merge_start.y;
        let mut bottom = top;
        let x = inter.left;
        let mut p = Point::new(x, top);
        while top > inter.top
            && can_merge_at(map, n, r, p, requested_terrain, depth)
            && can_merge_at(map, r, n, p, requested_terrain, depth)
        {
            top -= 1;
            p.y -= 1;
        }
        p.y = bottom;
        while bottom < inter.bottom
            && can_merge_at(map, n, r, p, requested_terrain, depth)
            && can_merge_at(map, r, n, p, requested_terrain, depth)
        {
            bottom += 1;
            p.y += 1;
        }
        if bottom - top >= 3 {
            for y in (top + 1)..bottom {
                if let Some(i) = map.point_to_cell(x, y) {
                    map.map[i] = terrain;
                }
            }
            paint_merge_connector(map, r, n, start, requested_terrain);
            return true;
        }
        false
    } else if inter.top == inter.bottom {
        let merge_start = start.unwrap_or_else(|| random_rect_center(inter));
        let mut left = merge_start.x;
        let mut right = left;
        let y = inter.top;
        let mut p = Point::new(left, y);
        while left > inter.left
            && can_merge_at(map, n, r, p, requested_terrain, depth)
            && can_merge_at(map, r, n, p, requested_terrain, depth)
        {
            left -= 1;
            p.x -= 1;
        }
        p.x = right;
        while right < inter.right
            && can_merge_at(map, n, r, p, requested_terrain, depth)
            && can_merge_at(map, r, n, p, requested_terrain, depth)
        {
            right += 1;
            p.x += 1;
        }
        if right - left >= 3 {
            for x in (left + 1)..right {
                if let Some(i) = map.point_to_cell(x, y) {
                    map.map[i] = terrain;
                }
            }
            paint_merge_connector(map, r, n, start, requested_terrain);
            return true;
        }
        false
    } else {
        false
    }
}

/// watabou `Rect.center()` consumes x jitter before y jitter, even when the
/// caller only reads one coordinate.
fn random_rect_center(rect: Rect) -> Point {
    Point::new(
        (rect.left + rect.right) / 2
            + if rect.raw_width() % 2 == 0 {
                Random::int_max(2)
            } else {
                0
            },
        (rect.top + rect.bottom) / 2
            + if rect.raw_height() % 2 == 0 {
                Random::int_max(2)
            } else {
                0
            },
    )
}

fn paint_merge_connector(
    map: &mut TerrainMap,
    room: &Room,
    other: &Room,
    door: Option<Point>,
    requested_terrain: i32,
) {
    if matches!(
        room.name.as_str(),
        "HallwayRoom" | "HallwayEntranceRoom" | "HallwayExitRoom"
    ) {
        if let Some(door) = door {
            if let Some(cell) = map.point_to_cell(door.x, door.y) {
                map.map[cell] = EMPTY_SP;
            }
        }
        return;
    }
    if !matches!(other.name.as_str(), "PlatformRoom" | "ChasmRoom") {
        return;
    }
    let connector = match room.name.as_str() {
        "PlatformRoom" if requested_terrain != CHASM && room.connected.contains(&other.id) => {
            EMPTY_SP
        }
        "ChasmRoom" if requested_terrain == EMPTY => EMPTY,
        _ => return,
    };
    if let Some(door) = door {
        if let Some(cell) = map.point_to_cell(door.x, door.y) {
            map.map[cell] = connector;
        }
    }
}
