//! Region-specific structural standard rooms.

mod basin;
mod bridge;
mod caves_fissure;
mod cell_block;
mod chasm_bridge;
mod circles;
mod hallway;
mod library_hall;
mod library_ring;
mod pathing;
mod pillars;
mod ring;
mod segmented;
mod segmented_library;
mod sewer_pipe;
mod standard_bridge;
mod statue_line;
mod statues;
mod water_bridge;

#[cfg(test)]
mod city_tests;
#[cfg(test)]
mod tests;

use crate::geom::Point;
use crate::level::terrain::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

use super::super::DoorMap;
use super::StandardPaintResult;

pub(super) fn paint(
    map: &mut TerrainMap,
    rooms: &[Room],
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    depth: i32,
) -> Option<StandardPaintResult> {
    match room.name.as_str() {
        "SewerPipeRoom" => sewer_pipe::paint(map, rooms, room, room_index, doors),
        "WaterBridgeRoom" | "WaterBridgeEntranceRoom" | "WaterBridgeExitRoom" => {
            water_bridge::paint(map, room, room_index, doors, depth)
        }
        "RingRoom" | "RingEntranceRoom" | "RingExitRoom" => {
            return Some(StandardPaintResult {
                center_loot: ring::paint(map, room),
            });
        }
        "CircleBasinRoom" | "CircleBasinEntranceRoom" | "CircleBasinExitRoom" => {
            basin::paint(map, room, room_index, doors)
        }
        "RegionDecoBridgeRoom" | "RegionDecoBridgeEntranceRoom" | "RegionDecoBridgeExitRoom" => {
            bridge::paint(map, room, room_index, doors)
        }
        "CavesFissureRoom" | "CavesFissureEntranceRoom" | "CavesFissureExitRoom" => {
            caves_fissure::paint(map, room, room_index, doors)
        }
        "CirclePitRoom" | "CircleWallRoom" | "CircleWallEntranceRoom" | "CircleWallExitRoom" => {
            circles::paint(map, room, room_index, doors)
        }
        "RegionDecoLineRoom" | "RegionDecoLineEntranceRoom" | "RegionDecoLineExitRoom" => {
            statue_line::paint(map, room, room_index, doors)
        }
        "SegmentedRoom" => segmented::paint(map, room, room_index, doors),
        "PillarsRoom" | "PillarsEntranceRoom" | "PillarsExitRoom" => pillars::paint(map, room),
        "ChasmBridgeRoom" | "ChasmBridgeEntranceRoom" | "ChasmBridgeExitRoom" => {
            chasm_bridge::paint(map, room, room_index, doors)
        }
        "CellBlockRoom" | "CellBlockEntranceRoom" | "CellBlockExitRoom" => {
            cell_block::paint(map, room)
        }
        "HallwayRoom" | "HallwayEntranceRoom" | "HallwayExitRoom" => {
            hallway::paint(map, room, room_index, doors)
        }
        "LibraryHallRoom" | "LibraryHallEntranceRoom" | "LibraryHallExitRoom" => {
            library_hall::paint(map, room, room_index, doors)
        }
        "LibraryRingRoom" | "LibraryRingEntranceRoom" | "LibraryRingExitRoom" => {
            library_ring::paint(map, room, room_index, doors)
        }
        "StatuesRoom" | "StatuesEntranceRoom" | "StatuesExitRoom" => statues::paint(map, room),
        "SegmentedLibraryRoom" => segmented_library::paint(map, room, room_index, doors),
        _ => return None,
    }
    Some(StandardPaintResult::default())
}

fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}

fn terrain_at(map: &TerrainMap, x: i32, y: i32) -> Option<i32> {
    map.point_to_cell(x, y).map(|cell| map.map[cell])
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    fill_rect(map, room.left, room.top, room.right, room.bottom, terrain);
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    fill_rect(
        map,
        room.left + margin,
        room.top + margin,
        room.right - margin,
        room.bottom - margin,
        terrain,
    );
}

fn fill_rect(map: &mut TerrainMap, left: i32, top: i32, right: i32, bottom: i32, terrain: i32) {
    if left > right || top > bottom {
        return;
    }
    for y in top..=bottom {
        for x in left..=right {
            set(map, x, y, terrain);
        }
    }
}

fn draw_inside(map: &mut TerrainMap, room: &Room, from: Point, n: i32, terrain: i32) {
    let (dx, dy) = if from.x == room.left {
        (1, 0)
    } else if from.x == room.right {
        (-1, 0)
    } else if from.y == room.top {
        (0, 1)
    } else if from.y == room.bottom {
        (0, -1)
    } else {
        return;
    };
    for step in 1..=n {
        set(map, from.x + dx * step, from.y + dy * step, terrain);
    }
}

fn draw_line(map: &mut TerrainMap, from: Point, to: Point, terrain: i32) {
    let dx = (to.x - from.x).signum();
    let dy = (to.y - from.y).signum();
    let steps = (to.x - from.x).abs().max((to.y - from.y).abs());
    for step in 0..=steps {
        set(map, from.x + dx * step, from.y + dy * step, terrain);
    }
}

/// `Rect.center()` as used by pinned SPD rooms.
fn center(room: &Room) -> Point {
    Point::new(
        (room.left + room.right) / 2
            + if (room.right - room.left) % 2 == 1 {
                Random::int_max(2)
            } else {
                0
            },
        (room.top + room.bottom) / 2
            + if (room.bottom - room.top) % 2 == 1 {
                Random::int_max(2)
            } else {
                0
            },
    )
}

fn door_points(room: &Room, room_index: usize, doors: &DoorMap) -> Vec<Point> {
    room.connected
        .iter()
        .filter_map(|&other| doors.get(room_index, other))
        .map(|door| Point::new(door.x, door.y))
        .collect()
}
