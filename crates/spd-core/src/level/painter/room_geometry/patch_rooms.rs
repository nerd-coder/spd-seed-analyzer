//! SPD `PatchRoom` geometry for the first standard-room painter slice.

use std::collections::VecDeque;

use crate::level::patch;
use crate::level::terrain::{
    TerrainMap, CHASM, EMBERS, EMPTY, ENTRANCE, EXIT, INACTIVE_TRAP, REGION_DECO, SECRET_TRAP,
    TRAP, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::DoorMap;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PatchStyle {
    RegionDeco,
    Cave,
    Ruins,
    Chasm,
    Burned,
}

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) -> bool {
    let Some(style) = style_for_name(&room.name) else {
        return false;
    };

    fill_room(map, room, WALL);
    fill_interior(map, room, EMPTY);

    let (fill, clustering, ensure_path, clean_edges) = patch_params(style, room);
    let mut mask = setup_patch(room, room_index, doors, fill, clustering, ensure_path);
    if clean_edges {
        clean_diagonal_edges(&mut mask, room.width() - 2);
    }

    match style {
        PatchStyle::RegionDeco => fill_patch(map, room, &mask, REGION_DECO),
        PatchStyle::Cave => fill_patch(map, room, &mask, WALL),
        PatchStyle::Chasm => fill_patch(map, room, &mask, CHASM),
        PatchStyle::Ruins => paint_ruins(map, room, &mask),
        PatchStyle::Burned => paint_burned(map, room, &mask),
    }

    paint_transition(map, room, style);
    true
}

fn style_for_name(name: &str) -> Option<PatchStyle> {
    match name {
        "RegionDecoPatchRoom" | "RegionDecoPatchEntranceRoom" | "RegionDecoPatchExitRoom" => {
            Some(PatchStyle::RegionDeco)
        }
        "CaveRoom" | "CaveEntranceRoom" | "CaveExitRoom" => Some(PatchStyle::Cave),
        "RuinsRoom" | "RuinsEntranceRoom" | "RuinsExitRoom" => Some(PatchStyle::Ruins),
        "ChasmRoom" | "ChasmEntranceRoom" | "ChasmExitRoom" => Some(PatchStyle::Chasm),
        "BurnedRoom" => Some(PatchStyle::Burned),
        _ => None,
    }
}

fn patch_params(style: PatchStyle, room: &Room) -> (f32, i32, bool, bool) {
    let area = (room.width() * room.height()).min(18 * 18);
    match style {
        PatchStyle::RegionDeco => {
            let scale = (room.width() * room.height()).min(10 * 10);
            (0.20 + scale as f32 / 1024.0, 1, true, true)
        }
        PatchStyle::Cave => (0.30 + area as f32 / 1024.0, 3, true, true),
        PatchStyle::Ruins => (0.30 + area as f32 / 1024.0, 0, true, true),
        PatchStyle::Chasm => (0.30 + area as f32 / 1024.0, 1, true, true),
        PatchStyle::Burned => (
            (1.48 - (room.width() + room.height()) as f32 * 0.03).min(1.0),
            2,
            false,
            false,
        ),
    }
}

fn setup_patch(
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    mut fill: f32,
    clustering: i32,
    ensure_path: bool,
) -> Vec<bool> {
    let width = room.width() - 2;
    let height = room.height() - 2;
    if !ensure_path || room.connected.is_empty() {
        return patch::generate(width, height, fill, clustering, true);
    }

    let mut attempts = 0;
    loop {
        let mut mask = patch::generate(width, height, fill, clustering, true);
        let mut start = None;
        for &other in &room.connected {
            let Some(door) = doors.get(room_index, other) else {
                continue;
            };
            let points = if door.x == room.left {
                [(door.x + 1, door.y), (door.x + 2, door.y)]
            } else if door.x == room.right {
                [(door.x - 1, door.y), (door.x - 2, door.y)]
            } else if door.y == room.top {
                [(door.x, door.y + 1), (door.x, door.y + 2)]
            } else {
                [(door.x, door.y - 1), (door.x, door.y - 2)]
            };
            for &(x, y) in &points {
                if let Some(i) = patch_index(room, x, y) {
                    mask[i] = false;
                }
            }
            start = patch_index(room, points[0].0, points[0].1);
        }

        if start.is_some_and(|cell| all_open_cells_reachable(&mask, width, cell)) {
            return mask;
        }

        attempts += 1;
        if attempts > 100 {
            fill -= 0.01;
            attempts = 0;
        }
    }
}

fn all_open_cells_reachable(mask: &[bool], width: i32, start: usize) -> bool {
    if start >= mask.len() || mask[start] {
        return false;
    }
    let height = mask.len() as i32 / width;
    let mut reached = vec![false; mask.len()];
    let mut queue = VecDeque::from([start]);
    reached[start] = true;

    while let Some(cell) = queue.pop_front() {
        let x = cell as i32 % width;
        let y = cell as i32 / width;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx >= width || ny >= height {
                    continue;
                }
                let next = (nx + ny * width) as usize;
                if !mask[next] && !reached[next] {
                    reached[next] = true;
                    queue.push_back(next);
                }
            }
        }
    }

    mask.iter()
        .zip(reached)
        .all(|(&filled, seen)| filled || seen)
}

fn clean_diagonal_edges(mask: &mut [bool], width: i32) {
    let width = width as usize;
    for i in 0..mask.len().saturating_sub(width) {
        if !mask[i] {
            continue;
        }
        if !i.is_multiple_of(width) && mask[i - 1 + width] && !(mask[i - 1] || mask[i + width]) {
            mask[i - 1 + width] = false;
        }
        if !(i + 1).is_multiple_of(width)
            && mask[i + 1 + width]
            && !(mask[i + 1] || mask[i + width])
        {
            mask[i + 1 + width] = false;
        }
    }
}

fn paint_ruins(map: &mut TerrainMap, room: &Room, mask: &[bool]) {
    let patch_width = room.width() - 2;
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            let pi = ((x - room.left - 1) + (y - room.top - 1) * patch_width) as usize;
            if !mask[pi] {
                continue;
            }
            let wall = if y > room.top + 1
                && y < room.bottom - 1
                && x > room.left + 1
                && x < room.right - 1
            {
                let adjacent = [
                    pi - 1,
                    pi + 1,
                    pi - patch_width as usize,
                    pi + patch_width as usize,
                ]
                .into_iter()
                .filter(|&i| mask[i])
                .count() as i32;
                Random::int_max(2) < adjacent
            } else {
                true
            };
            set(map, x, y, if wall { WALL } else { REGION_DECO });
        }
    }
}

fn paint_burned(map: &mut TerrainMap, room: &Room, mask: &[bool]) {
    let patch_width = room.width() - 2;
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            let pi = ((x - room.left - 1) + (y - room.top - 1) * patch_width) as usize;
            if !mask[pi] {
                continue;
            }
            let terrain = match Random::int_max(5) {
                1 => EMBERS,
                2 => TRAP,
                3 => SECRET_TRAP,
                4 => INACTIVE_TRAP,
                _ => EMPTY,
            };
            let Some(cell) = map.point_to_cell(x, y) else {
                continue;
            };
            map.map[cell] = terrain;
            map.water_allowed[cell] = false;
            map.grass_allowed[cell] = false;
            map.trap_allowed[cell] = false;
            if matches!(terrain, TRAP | SECRET_TRAP | INACTIVE_TRAP) {
                map.trap_destroys_items[cell] = true;
                map.trap_names[cell] = Some("BurningTrap");
            }
        }
    }
}

fn paint_transition(map: &mut TerrainMap, room: &Room, style: PatchStyle) {
    let terrain = match room.kind {
        RoomKind::Entrance => ENTRANCE,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    let blocked = match style {
        PatchStyle::RegionDeco => REGION_DECO,
        PatchStyle::Cave | PatchStyle::Ruins => WALL,
        PatchStyle::Chasm => CHASM,
        PatchStyle::Burned => return,
    };

    // Pinned entrance/exit subclasses use 30 strict Room.random(2) attempts,
    // then accept a cramped cell when one of its cardinal neighbours is open.
    // There are no mobs in this analyzer pass.
    let mut tries = 30;
    for _ in 0..1_000 {
        let x = Random::int_range_inclusive(room.left + 2, room.right - 2);
        let y = Random::int_range_inclusive(room.top + 2, room.bottom - 2);
        let Some(cell) = map.point_to_cell(x, y) else {
            continue;
        };
        let valid = if tries > 0 {
            tries -= 1;
            map.map[cell] != blocked
        } else {
            cardinal_neighbour_open(map, cell, style)
        };
        if !valid {
            continue;
        }
        map.map[cell] = terrain;
        clear_neighbours8(map, cell);
        return;
    }
}

fn cardinal_neighbour_open(map: &TerrainMap, cell: usize, style: PatchStyle) -> bool {
    let width = map.width;
    let x = cell as i32 % width;
    let y = cell as i32 / width;
    [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .into_iter()
        .any(|(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= width || ny >= map.height {
                return false;
            }
            let terrain = map.map[(nx + ny * width) as usize];
            match style {
                PatchStyle::RegionDeco => terrain != REGION_DECO,
                PatchStyle::Cave => terrain != WALL,
                PatchStyle::Ruins => terrain != WALL && terrain != REGION_DECO,
                PatchStyle::Chasm => terrain != CHASM,
                PatchStyle::Burned => false,
            }
        })
}

fn clear_neighbours8(map: &mut TerrainMap, cell: usize) {
    let width = map.width;
    let x = cell as i32 % width;
    let y = cell as i32 / width;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && ny >= 0 && nx < width && ny < map.height {
                map.map[(nx + ny * width) as usize] = EMPTY;
            }
        }
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            set(map, x, y, terrain);
        }
    }
}

fn fill_interior(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            set(map, x, y, terrain);
        }
    }
}

fn fill_patch(map: &mut TerrainMap, room: &Room, mask: &[bool], terrain: i32) {
    let patch_width = room.width() - 2;
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            let pi = ((x - room.left - 1) + (y - room.top - 1) * patch_width) as usize;
            if mask[pi] {
                set(map, x, y, terrain);
            }
        }
    }
}

fn patch_index(room: &Room, x: i32, y: i32) -> Option<usize> {
    let width = room.width() - 2;
    let height = room.height() - 2;
    let px = x - room.left - 1;
    let py = y - room.top - 1;
    (px >= 0 && py >= 0 && px < width && py < height).then_some((px + py * width) as usize)
}

fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}
