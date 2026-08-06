//! Mining-specific door resolution and border overlays.

use std::collections::{HashMap, VecDeque};

use crate::geom::Point;
use crate::level::painter::{merge_rooms_with_terrain, DoorMap, DoorType};
use crate::level::terrain::{TerrainMap, EMPTY, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

pub(super) fn paint(
    map: &mut TerrainMap,
    rooms: &[Room],
    order: &[usize],
    doors: &mut DoorMap,
    depth: i32,
) {
    let mut merged = HashMap::new();
    for &room_index in order {
        for &other in &rooms[room_index].connected {
            let Some(door) = doors.get(room_index, other) else {
                continue;
            };
            let (x, y, door_type) = (door.x, door.y, door.door_type);
            let cell = map.point_to_cell(x, y).unwrap();
            match door_type {
                DoorType::Wall | DoorType::Hidden => map.map[cell] = WALL,
                _ => {
                    if Random::float() < 0.90 {
                        doors.get_mut(room_index, other).unwrap().door_type = DoorType::Hidden;
                        if !reachable(rooms, doors, room_index, other) {
                            doors.get_mut(room_index, other).unwrap().door_type = DoorType::Empty;
                            map.map[cell] = EMPTY;
                        } else {
                            map.map[cell] = WALL;
                        }
                    } else {
                        doors.get_mut(room_index, other).unwrap().door_type = DoorType::Empty;
                        map.map[cell] = EMPTY;
                    }
                }
            }
            if map.map[cell] == EMPTY
                && merged.get(&room_index) != Some(&other)
                && merged.get(&other) != Some(&room_index)
                && merge_rooms_with_terrain(
                    map,
                    &rooms[room_index],
                    &rooms[other],
                    Some(Point::new(x, y)),
                    EMPTY,
                    depth,
                )
            {
                merged.insert(room_index, other);
                merged.insert(other, room_index);
            }
        }
    }
}

fn reachable(rooms: &[Room], doors: &DoorMap, from: usize, target: usize) -> bool {
    let mut seen = vec![false; rooms.len()];
    let mut queue = VecDeque::from([from]);
    seen[from] = true;
    while let Some(room) = queue.pop_front() {
        for &other in &rooms[room].connected {
            let Some(door) = doors.get(room, other) else {
                continue;
            };
            if !matches!(
                door.door_type,
                DoorType::Empty | DoorType::Tunnel | DoorType::Regular
            ) {
                continue;
            }
            if other == target {
                return true;
            }
            if !seen[other] {
                seen[other] = true;
                queue.push_back(other);
            }
        }
    }
    false
}

pub(super) fn add_border_overlays(map: &mut TerrainMap) {
    map.record_custom_tile(
        "BorderTopDarken",
        "environment/custom_tiles/caves_quest.png",
        (0, 0, map.width, 1),
        vec![1; map.width as usize],
    );
    let mut data = vec![-1; map.len()];
    for (cell, value) in data.iter_mut().enumerate() {
        let x = cell as i32 % map.width;
        *value = if x == 0 || x == map.width - 1 {
            1
        } else if cell as i32 + 2 * map.width > map.len() as i32 {
            2
        } else {
            -1
        };
    }
    map.custom_walls.push(crate::report::MapCustomTile {
        class_name: "BorderWallsDarken".into(),
        texture: "environment/custom_tiles/caves_quest.png".into(),
        x: 0,
        y: 0,
        width: map.width as u32,
        height: map.height as u32,
        static_data: data,
    });
}
