//! Shared pinned-SPD `RegularBuilder` setup and branch placement.

use std::collections::HashSet;

use crate::builders::connection;
use crate::builders::place::{angle_between_points, place_room};
use crate::random::Random;
use crate::rooms::room::{Room, DIR_ALL};
use crate::rooms::types::RoomKind;

#[derive(Debug, Clone)]
pub struct BuilderParams {
    pub path_length: f32,
    pub path_len_jitter: [f32; 4],
    pub path_tunnel_chances: [f32; 3],
    pub branch_tunnel_chances: [f32; 3],
    pub extra_connection_chance: f32,
    pub curve_exponent: i32,
    pub curve_intensity: f32,
    pub curve_offset: f32,
}

impl Default for BuilderParams {
    fn default() -> Self {
        Self {
            path_length: 0.25,
            path_len_jitter: [0., 0., 0., 1.],
            path_tunnel_chances: [2., 2., 1.],
            branch_tunnel_chances: [1., 1., 0.],
            extra_connection_chance: 0.30,
            curve_exponent: 2,
            curve_intensity: 0.0,
            curve_offset: 0.0,
        }
    }
}

pub(super) fn target_angle(percent_along: f32, params: &BuilderParams) -> f32 {
    let p = percent_along + params.curve_offset;
    let exponent = params.curve_exponent as f64;
    let curve = 4f64.powf(2.0 * exponent) * ((p as f64 % 0.5) - 0.25).powf(2.0 * exponent + 1.0)
        + 0.25
        + 0.5 * (2.0 * p as f64).floor();
    // Java promotes only the curve term to double, keeps the linear term's
    // float rounding, then casts the combined expression back to float.
    let linear = (1.0f32 - params.curve_intensity) * p;
    let mixed = params.curve_intensity as f64 * curve + linear as f64 - params.curve_offset as f64;
    360.0f32 * mixed as f32
}

pub(super) struct Setup {
    pub entrance: Option<usize>,
    pub exit: Option<usize>,
    pub shop: Option<usize>,
    pub main_path: Vec<usize>,
    pub multi: Vec<usize>,
    pub single: Vec<usize>,
}

pub(super) fn setup_rooms(rooms: &mut [Room], params: &BuilderParams) -> Setup {
    for room in rooms.iter_mut() {
        room.set_empty();
        room.clear_connections();
    }

    let mut entrance = None;
    let mut exit = None;
    let mut shop = None;
    let mut multi = Vec::new();
    let mut single = Vec::new();
    for room in rooms.iter() {
        if room.is_entrance() {
            entrance = Some(room.id);
        } else if room.is_exit() {
            exit = Some(room.id);
        } else if room.kind == RoomKind::Shop && room.max_connections_all == 1 {
            shop = Some(room.id);
        } else if room.max_connections(DIR_ALL) > 1 {
            multi.push(room.id);
        } else if room.max_connections(DIR_ALL) == 1 {
            single.push(room.id);
        }
    }

    weight_rooms(rooms, &mut multi);
    Random::shuffle_list(&mut multi);
    let mut seen = HashSet::new();
    multi.retain(|id| seen.insert(*id));
    Random::shuffle_list(&mut multi);

    let mut rooms_on_main =
        (multi.len() as f32 * params.path_length) as i32 + Random::chances(&params.path_len_jitter);
    let mut main_path = Vec::new();
    while rooms_on_main > 0 && !multi.is_empty() {
        let id = multi.remove(0);
        rooms_on_main -= if rooms[id].kind == RoomKind::Standard {
            rooms[id].size_factor
        } else {
            1
        };
        main_path.push(id);
    }

    Setup {
        entrance,
        exit,
        shop,
        main_path,
        multi,
        single,
    }
}

pub(super) fn weight_rooms(rooms: &[Room], list: &mut Vec<usize>) {
    let original = list.clone();
    for id in original {
        if rooms[id].kind == RoomKind::Standard {
            for _ in 1..rooms[id].connection_weight() {
                list.push(id);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum BranchAngles<'a> {
    Around((f32, f32)),
    FigureEight {
        first_ids: &'a [usize],
        first_center: (f32, f32),
        second_center: (f32, f32),
    },
}

impl BranchAngles<'_> {
    fn next(self, rooms: &[Room], room: usize) -> f32 {
        let center = match self {
            Self::Around(center) => center,
            Self::FigureEight {
                first_ids,
                first_center,
                second_center,
            } => {
                if first_ids.contains(&room) {
                    first_center
                } else {
                    second_center
                }
            }
        };
        let room_center = (
            (rooms[room].left + rooms[room].right) as f32 / 2.0,
            (rooms[room].top + rooms[room].bottom) as f32 / 2.0,
        );
        let mut to_center = angle_between_points(room_center, center);
        if to_center < 0.0 {
            to_center += 360.0;
        }
        let mut angle = Random::float_max(360.0);
        for _ in 0..4 {
            let candidate = Random::float_max(360.0);
            if (to_center - candidate).abs() < (to_center - angle).abs() {
                angle = candidate;
            }
        }
        angle
    }
}

fn disconnect_room(rooms: &mut [Room], id: usize) {
    for room in rooms.iter_mut() {
        room.connected.retain(|&other| other != id);
        room.neighbours.retain(|&other| other != id);
    }
    if let Some(room) = rooms.get_mut(id) {
        room.clear_connections();
    }
}

pub(super) fn create_branches(
    rooms: &mut Vec<Room>,
    branchable: &mut Vec<usize>,
    rooms_to_branch: &[usize],
    conn_chances: &[f32],
    depth: i32,
    angles: BranchAngles<'_>,
) -> bool {
    let mut i = 0;
    let mut failed = 0;
    let mut connection_chances = conn_chances.to_vec();
    while i < rooms_to_branch.len() {
        if failed > 100 || branchable.is_empty() {
            return false;
        }
        let room = rooms_to_branch[i];
        let branch_start = rooms.len();
        let mut connecting = Vec::new();

        let mut curr = None;
        // Valid regular layouts always have a non-connection main-path room.
        // Bound the rejection loop so malformed callers cannot hang WASM.
        for _ in 0..10_000 {
            let candidate = branchable[Random::int_max(branchable.len() as i32) as usize];
            if rooms[room].kind != RoomKind::Secret || rooms[candidate].kind != RoomKind::Connection
            {
                curr = Some(candidate);
                break;
            }
        }
        let Some(mut curr) = curr else {
            return false;
        };

        let mut connecting_count = Random::chances(&connection_chances);
        if connecting_count == -1 {
            connection_chances = conn_chances.to_vec();
            connecting_count = Random::chances(&connection_chances);
        }
        connection_chances[connecting_count as usize] -= 1.0;

        for _ in 0..connecting_count {
            let id = rooms.len();
            rooms.push(if rooms[room].kind == RoomKind::Secret {
                connection::maze(id)
            } else {
                connection::create(id, depth)
            });
            let mut placed = false;
            for _ in 0..3 {
                let angle = angles.next(rooms, curr);
                if place_room(rooms, curr, id, angle) != -1.0 {
                    placed = true;
                    break;
                }
            }
            if !placed {
                disconnect_room(rooms, id);
                for &connection in &connecting {
                    disconnect_room(rooms, connection);
                }
                rooms.truncate(branch_start);
                connecting.clear();
                break;
            }
            connecting.push(id);
            curr = id;
        }

        if connecting.len() as i32 != connecting_count {
            failed += 1;
            continue;
        }

        let mut placed = false;
        for _ in 0..10 {
            let angle = angles.next(rooms, curr);
            if place_room(rooms, curr, room, angle) != -1.0 {
                placed = true;
                break;
            }
        }
        if !placed {
            disconnect_room(rooms, room);
            for &connection in &connecting {
                disconnect_room(rooms, connection);
            }
            rooms.truncate(branch_start);
            failed += 1;
            continue;
        }

        for &connection in &connecting {
            if Random::int_max(3) <= 1 {
                branchable.push(connection);
            }
        }
        if rooms[room].max_connections(DIR_ALL) > 1 && Random::int_max(3) == 0 {
            let copies = if rooms[room].kind == RoomKind::Standard {
                rooms[room].connection_weight()
            } else {
                1
            };
            for _ in 0..copies {
                branchable.push(room);
            }
        }
        i += 1;
    }
    true
}

pub(super) fn loop_center(rooms: &[Room], ids: &[usize]) -> (f32, f32) {
    let mut center = (0.0, 0.0);
    for &id in ids {
        center.0 += (rooms[id].left + rooms[id].right) as f32 / 2.0;
        center.1 += (rooms[id].top + rooms[id].bottom) as f32 / 2.0;
    }
    center.0 /= ids.len() as f32;
    center.1 /= ids.len() as f32;
    center
}
