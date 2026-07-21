//! Pinned SPD `RotGardenRoom.paint` terrain, heart, and lasher placement.

use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, GRASS, HIGH_GRASS, LOCKED_DOOR, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

use crate::level::maze::distance_map;

const GARDEN_RETRY_LIMIT: usize = 10_000;

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    let Some(entrance) = entrance(room, room_index, doors) else {
        return;
    };
    fill_room(map, room, WALL);
    set(map, entrance, LOCKED_DOOR);

    let Some(entry_cell) = map.point_to_cell(entrance.x, entrance.y) else {
        return;
    };
    let mut passable = vec![false; map.len()];
    let mut candidates = Vec::new();
    let mut found_layout = false;

    // Java retries without a bound. Valid 10x10 builder rooms settle quickly;
    // this cap prevents a malformed browser input from hanging indefinitely.
    for _ in 0..GARDEN_RETRY_LIMIT {
        fill_margin(map, room, 1, HIGH_GRASS);
        scatter_walls(map, room, 1, 12);
        scatter_walls(map, room, 2, 8);
        scatter_walls(map, room, 3, 4);
        draw_inside(map, room, entrance, 3, HIGH_GRASS);

        for (cell, passable_cell) in passable.iter_mut().enumerate() {
            *passable_cell = map.map[cell] != WALL;
        }
        let distances = distance_map(map.width, map.height, entry_cell, &passable);
        candidates.clear();
        let mut open_cells = 0;
        for x in room.left..=room.right {
            for y in room.top..=room.bottom {
                let Some(cell) = map.point_to_cell(x, y) else {
                    continue;
                };
                if distances[cell] != i32::MAX {
                    open_cells += 1;
                    if distances[cell] >= 7 {
                        candidates.push(cell);
                    }
                } else if map.map[cell] == HIGH_GRASS {
                    map.map[cell] = WALL;
                }
            }
        }

        // candidates is a Java ArrayList, so this is Collections.shuffle,
        // followed by distance-tier removal from a snapshot of the list.
        Random::shuffle_list(&mut candidates);
        let mut closest = 7;
        while candidates.len() > 5 {
            let snapshot = candidates.clone();
            for cell in snapshot {
                if candidates.len() > 5 && distances[cell] == closest {
                    if let Some(index) = candidates.iter().position(|&candidate| candidate == cell)
                    {
                        candidates.remove(index);
                    }
                }
            }
            closest += 1;
        }

        if !candidates.is_empty() && open_cells >= 35 {
            found_layout = true;
            break;
        }
    }
    if !found_layout {
        return;
    }

    let heart_cell = *Random::element(&candidates).expect("non-empty RotGarden candidates");
    place_plant(map, heart_cell, "Rot Heart");

    let mut new_passable = passable.clone();
    for _ in 0..6 {
        let mut placed = None;
        let mut tries = 50;
        loop {
            let point = room.random();
            let Some(cell) = map.point_to_cell(point.x, point.y) else {
                tries -= 1;
                if tries <= 0 {
                    break;
                }
                continue;
            };
            tries -= 1;
            // The 50th Java draw is never validated because `tries > 0` is
            // evaluated before validPlantPos in the do/while condition.
            if tries <= 0 {
                break;
            }
            if valid_plant_pos(
                map,
                cell,
                heart_cell,
                entry_cell,
                &mut passable,
                &mut new_passable,
            ) {
                placed = Some(cell);
                break;
            }
        }
        let Some(cell) = placed else {
            break;
        };
        place_plant(map, cell, "Rot Lasher");
    }

    // CIRCLE8 pairs: NW→N, NE→E, SE→S, SW→W.
    let width = map.width as isize;
    let circle8 = [
        -width - 1,
        -width,
        -width + 1,
        1,
        width + 1,
        width,
        width - 1,
        -1,
    ];
    for i in (0..8).step_by(2) {
        let diagonal = (heart_cell as isize + circle8[i]) as usize;
        if map.map[diagonal] != WALL {
            let cardinal = (heart_cell as isize + circle8[i + 1]) as usize;
            map.map[cardinal] = HIGH_GRASS;
        }
    }
}

fn valid_plant_pos(
    map: &TerrainMap,
    cell: usize,
    heart_cell: usize,
    entry_cell: usize,
    passable: &mut [bool],
    new_passable: &mut [bool],
) -> bool {
    if map.map[cell] != HIGH_GRASS {
        return false;
    }

    let width = map.width as isize;
    let neighbours9 = [
        -width - 1,
        -width,
        -width + 1,
        -1,
        0,
        1,
        width - 1,
        width,
        width + 1,
    ];
    if neighbours9.iter().any(|&offset| {
        let neighbor = cell as isize + offset;
        neighbor >= 0 && (neighbor as usize) < map.len() && map.mob_occupied[neighbor as usize]
    }) {
        return false;
    }

    new_passable[cell] = false;
    let cx = cell as i32 % map.width;
    let cy = cell as i32 / map.width;
    let hx = heart_cell as i32 % map.width;
    let hy = heart_cell as i32 / map.width;
    let near_heart = (cx - hx).abs().max((cy - hy).abs()) <= 2;
    let offsets: &[isize] = if near_heart {
        &[
            -width - 1,
            -width,
            -width + 1,
            -1,
            1,
            width - 1,
            width,
            width + 1,
        ]
    } else {
        &[-width, -1, 1, width]
    };
    for &offset in offsets {
        let neighbor = cell as isize + offset;
        if neighbor >= 0 && (neighbor as usize) < new_passable.len() {
            new_passable[neighbor as usize] = false;
        }
    }

    let distances = distance_map(map.width, map.height, heart_cell, new_passable);
    if distances[entry_cell] == i32::MAX {
        new_passable.copy_from_slice(passable);
        false
    } else {
        passable.copy_from_slice(new_passable);
        true
    }
}

fn place_plant(map: &mut TerrainMap, cell: usize, label: &'static str) {
    map.mob_occupied[cell] = true;
    map.known_mobs[cell] = Some(label);
    map.map[cell] = GRASS;
}

fn scatter_walls(map: &mut TerrainMap, room: &Room, margin: i32, count: usize) {
    for _ in 0..count {
        set(map, room.random_margin(margin), WALL);
    }
}

fn entrance(room: &Room, room_index: usize, doors: &DoorMap) -> Option<Point> {
    room.connected.first().and_then(|&other| {
        doors
            .get(room_index, other)
            .map(|door| Point::new(door.x, door.y))
    })
}

fn set(map: &mut TerrainMap, point: Point, terrain: i32) {
    if let Some(cell) = map.point_to_cell(point.x, point.y) {
        map.map[cell] = terrain;
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            set(map, Point::new(x, y), terrain);
        }
    }
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    for y in (room.top + margin)..=(room.bottom - margin) {
        for x in (room.left + margin)..=(room.right - margin) {
            set(map, Point::new(x, y), terrain);
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
    } else {
        (0, -1)
    };
    for step in 1..=distance {
        set(
            map,
            Point::new(from.x + dx * step, from.y + dy * step),
            terrain,
        );
    }
}
