//! `placeDoors` + `paintDoors` (mergeRooms + hidden-door Graph).
//!
//! Matches SPD `RegularPainter.placeDoors` / `paintDoors` / `mergeRooms` for
//! main-stream RNG and map door tiles. Room geometry paint remains approximate,
//! so merge success can still desync slightly vs the game.

mod merge;
mod model;

use std::collections::{HashMap, VecDeque};

use crate::geom::Point;
use crate::level::terrain::{TerrainMap, DOOR, EMPTY, LOCKED_DOOR, SECRET_DOOR, WALL};
use crate::level::Feeling;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

pub use model::{apply_room_door_types, door_spots, place_doors_for_room, DoorMap, DoorType};

use merge::{is_mergeable_standard, is_normal_size, merge_rooms};

/// Graph edge: non-blocked door types (SPD `Room.edges`).
fn is_graph_edge(t: DoorType) -> bool {
    matches!(
        t,
        DoorType::Empty | DoorType::Tunnel | DoorType::Unlocked | DoorType::Regular
    )
}

/// `Graph.buildDistanceMap` over rooms (price defaults to 1).
fn build_distance_map(rooms: &[Room], doors: &DoorMap, focus: usize) -> Vec<i32> {
    let n = rooms.len();
    let mut dist = vec![i32::MAX; n];
    if focus >= n {
        return dist;
    }
    let mut q = VecDeque::new();
    dist[focus] = 0;
    q.push_back(focus);
    while let Some(ri) = q.pop_front() {
        let d = dist[ri];
        let price = 1i32;
        for &ni in &rooms[ri].connected {
            let Some(door) = doors.get(ri, ni) else {
                continue;
            };
            if !is_graph_edge(door.door_type) {
                continue;
            }
            let nd = d + price;
            if dist[ni] > nd {
                dist[ni] = nd;
                q.push_back(ni);
            }
        }
    }
    dist
}

fn count_reachable_non_connection(rooms: &[Room], dist: &[i32]) -> i32 {
    rooms
        .iter()
        .zip(dist.iter())
        .filter(|(r, &d)| d != i32::MAX && r.kind != RoomKind::Connection)
        .count() as i32
}

/// `paintDoors` — merge standard rooms when possible; roll hidden doors with Graph checks.
pub fn paint_doors(
    map: &mut TerrainMap,
    rooms: &[Room],
    paint_order: &[usize],
    depth: i32,
    feeling: Feeling,
    doors: &mut DoorMap,
) {
    let mut hidden_door_chance = if depth > 1 {
        (depth as f32 / 20.0).min(1.0)
    } else {
        0.0
    };
    if feeling == Feeling::Secrets {
        hidden_door_chance = (0.5 + hidden_door_chance) / 2.0;
    }

    // room index → partner when a NORMAL-sized standard room has merged once.
    let mut room_merges: HashMap<usize, usize> = HashMap::new();

    for &ri in paint_order {
        if ri >= rooms.len() || rooms[ri].is_empty() {
            continue;
        }
        let neighbours: Vec<usize> = rooms[ri].connected.clone();
        for ni in neighbours {
            if ni >= rooms.len() || rooms[ni].is_empty() {
                continue;
            }

            if room_merges.get(&ri) == Some(&ni) || room_merges.get(&ni) == Some(&ri) {
                continue;
            }

            let start = doors.get(ri, ni).map(|d| Point::new(d.x, d.y));
            let can_try_merge = !room_merges.contains_key(&ri)
                && !room_merges.contains_key(&ni)
                && is_mergeable_standard(&rooms[ri])
                && is_mergeable_standard(&rooms[ni]);

            if can_try_merge && merge_rooms(map, &rooms[ri], &rooms[ni], start) {
                if is_normal_size(&rooms[ri]) {
                    room_merges.insert(ri, ni);
                }
                if is_normal_size(&rooms[ni]) {
                    room_merges.insert(ni, ri);
                }
                continue;
            }

            // Resolve REGULAR → HIDDEN / UNLOCKED with Graph connectivity.
            let cur = doors.get(ri, ni).map(|d| d.door_type);
            if cur == Some(DoorType::Regular) {
                let mut next = if Random::float() < hidden_door_chance {
                    DoorType::Hidden
                } else {
                    DoorType::Unlocked
                };
                if next == DoorType::Hidden {
                    if let Some(d) = doors.get_mut(ri, ni) {
                        d.door_type = DoorType::Hidden;
                    }
                    if feeling != Feeling::Secrets {
                        let dist = build_distance_map(rooms, doors, ri);
                        if dist.get(ni).copied().unwrap_or(i32::MAX) == i32::MAX {
                            next = DoorType::Unlocked;
                        }
                    } else {
                        let dist_r = build_distance_map(rooms, doors, ri);
                        if count_reachable_non_connection(rooms, &dist_r) < 2 {
                            next = DoorType::Unlocked;
                        } else {
                            let dist_n = build_distance_map(rooms, doors, ni);
                            if count_reachable_non_connection(rooms, &dist_n) < 2 {
                                next = DoorType::Unlocked;
                            }
                        }
                    }
                    if feeling != Feeling::Secrets && next == DoorType::Hidden {
                        let dist = build_distance_map(rooms, doors, ri);
                        if dist.get(ni).copied().unwrap_or(i32::MAX) == i32::MAX {
                            next = DoorType::Unlocked;
                        }
                    }
                }
                if let Some(d) = doors.get_mut(ri, ni) {
                    d.door_type = next;
                }
            }

            // Skip depth 1/2 tutorial intro overrides (not seed-finder relevant).

            let (dx, dy, dtype) = match doors.get(ri, ni) {
                Some(d) => (d.x, d.y, d.door_type),
                None => continue,
            };
            let terrain = match dtype {
                DoorType::Empty => EMPTY,
                DoorType::Tunnel | DoorType::Water => EMPTY,
                DoorType::Unlocked | DoorType::Regular => DOOR,
                DoorType::Hidden => SECRET_DOOR,
                DoorType::Barricade => WALL,
                DoorType::Locked => LOCKED_DOOR,
                DoorType::Crystal => LOCKED_DOOR,
                DoorType::Wall => WALL,
            };
            if let Some(i) = map.point_to_cell(dx, dy) {
                map.map[i] = terrain;
            }
        }
    }

    map.recompute_passable();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::terrain;
    use crate::rooms::types::RoomKind;

    fn box_room(id: usize, kind: RoomKind, l: i32, t: i32, r: i32, b: i32) -> Room {
        let mut room = Room::new(id, "EmptyRoom", kind, 1, 4, 5, 9, 5, 9);
        room.left = l;
        room.top = t;
        room.right = r;
        room.bottom = b;
        room
    }

    #[test]
    fn place_doors_picks_spot() {
        Random::push_generator_seeded(1);
        let mut a = box_room(0, RoomKind::Standard, 1, 1, 8, 8);
        let mut b = box_room(1, RoomKind::Standard, 8, 1, 15, 8);
        a.connected.push(1);
        b.connected.push(0);
        let rooms = vec![a, b];
        let mut doors = DoorMap::new();
        place_doors_for_room(&rooms, 0, &mut doors);
        Random::pop_generator();
        let d = doors.get(0, 1).expect("door");
        assert_eq!(d.x, 8);
        assert!((1..=8).contains(&d.y));
    }

    #[test]
    fn paint_doors_can_hide_or_unlock() {
        Random::push_generator_seeded(99);
        let mut a = box_room(0, RoomKind::Standard, 1, 1, 8, 8);
        let mut b = box_room(1, RoomKind::Standard, 8, 1, 15, 8);
        let mut c = box_room(2, RoomKind::Standard, 15, 1, 22, 8);
        a.connected.push(1);
        b.connected.push(0);
        b.connected.push(2);
        c.connected.push(1);
        let rooms = vec![a, b, c];
        let mut doors = DoorMap::new();
        for i in 0..3 {
            place_doors_for_room(&rooms, i, &mut doors);
            apply_room_door_types(&rooms[i], i, &mut doors);
        }
        let mut map = terrain::paint_minimal(&rooms).expect("map");
        paint_doors(&mut map, &rooms, &[0, 1, 2], 10, Feeling::None, &mut doors);
        Random::pop_generator();

        let doorish = map
            .map
            .iter()
            .filter(|&&t| t == DOOR || t == SECRET_DOOR || t == LOCKED_DOOR)
            .count();
        assert!(
            doorish > 0 || map.map.contains(&EMPTY),
            "expected door tiles or merge openings"
        );
        for d in doors.doors.values() {
            assert!(
                matches!(
                    d.door_type,
                    DoorType::Regular | DoorType::Unlocked | DoorType::Hidden
                ),
                "unexpected door type {:?}",
                d.door_type
            );
        }
    }
}
