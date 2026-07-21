//! Pinned SPD generic standard-room painters.

mod hazards;
mod nature;
mod structures;

#[cfg(test)]
mod tests;

use crate::generator::GeneratorState;
use crate::geom::{Point, Rect};
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

use super::super::DoorMap;
use super::StandardPaintResult;

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    generator: &mut GeneratorState,
    depth: i32,
) -> Option<StandardPaintResult> {
    match room.name.as_str() {
        "PlantsRoom" => {
            nature::paint_plants(map, room, generator, depth);
            Some(StandardPaintResult::default())
        }
        "AquariumRoom" => {
            nature::paint_aquarium(map, room);
            Some(StandardPaintResult::default())
        }
        "GrassyGraveRoom" => {
            nature::paint_grassy_grave(map, room);
            Some(StandardPaintResult::default())
        }
        "PlatformRoom" => {
            structures::paint_platform(map, room, room_index, doors);
            Some(StandardPaintResult::default())
        }
        "FissureRoom" => {
            structures::paint_fissure(map, room);
            Some(StandardPaintResult::default())
        }
        "StripedRoom" => {
            structures::paint_striped(map, room);
            Some(StandardPaintResult::default())
        }
        "StudyRoom" => Some(StandardPaintResult {
            center_loot: Some(structures::paint_study(map, room, room_index, doors)),
        }),
        "SuspiciousChestRoom" => {
            structures::paint_suspicious_chest_base(map, room);
            Some(StandardPaintResult::default())
        }
        "MinefieldRoom" => {
            hazards::paint_minefield(map, room);
            Some(StandardPaintResult::default())
        }
        _ => None,
    }
}

fn center(room: &Room) -> Point {
    room.as_rect().center_room()
}

fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    fill_rect(map, room.as_rect(), terrain);
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    fill_rect(
        map,
        Rect {
            left: room.left + margin,
            top: room.top + margin,
            right: room.right - margin,
            bottom: room.bottom - margin,
        },
        terrain,
    );
}

fn fill_rect(map: &mut TerrainMap, rect: Rect, terrain: i32) {
    if rect.left > rect.right || rect.top > rect.bottom {
        return;
    }
    for y in rect.top..=rect.bottom {
        for x in rect.left..=rect.right {
            set(map, x, y, terrain);
        }
    }
}

fn draw_inside(map: &mut TerrainMap, room: &Room, from: Point, distance: i32, terrain: i32) {
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
    for step in 1..=distance {
        set(map, from.x + dx * step, from.y + dy * step, terrain);
    }
}
