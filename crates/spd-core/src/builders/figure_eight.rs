//! Pinned SPD v3.3.8 `FigureEightBuilder`.

use crate::builders::connection;
use crate::builders::place::{angle_between_rooms, find_neighbours, place_room};
use crate::builders::regular::{
    create_branches, loop_center, setup_rooms, target_angle, weight_rooms, BranchAngles,
    BuilderParams,
};
use crate::random::Random;
use crate::rooms::room::{connect, Room, DIR_ALL};

#[derive(Debug, Default)]
pub(super) struct FigureEightState {
    // Java keeps this builder field across failed `build` calls.
    landmark: Option<usize>,
}

fn add_tunnels(
    rooms: &mut Vec<Room>,
    source: &[usize],
    tunnel_chances: &mut Vec<f32>,
    defaults: &[f32],
    depth: i32,
) -> Vec<usize> {
    let mut output = Vec::new();
    for &room in source {
        output.push(room);
        let mut tunnels = Random::chances(tunnel_chances);
        if tunnels == -1 {
            *tunnel_chances = defaults.to_vec();
            tunnels = Random::chances(tunnel_chances);
        }
        tunnel_chances[tunnels as usize] -= 1.0;
        for _ in 0..tunnels {
            let id = rooms.len();
            rooms.push(connection::create(id, depth));
            output.push(id);
        }
    }
    output
}

fn place_loop(
    rooms: &mut Vec<Room>,
    loop_ids: &mut Vec<usize>,
    landmark: usize,
    start_angle: f32,
    params: &BuilderParams,
    depth: i32,
) -> Option<()> {
    let mut prev = landmark;
    for i in 1..loop_ids.len() {
        let room = loop_ids[i];
        let angle = start_angle + target_angle(i as f32 / loop_ids.len() as f32, params);
        if place_room(rooms, prev, room, angle) == -1.0 {
            return None;
        }
        prev = room;
    }
    for _ in 0..10_000 {
        if connect(rooms, prev, landmark) {
            return Some(());
        }
        let id = rooms.len();
        rooms.push(connection::create(id, depth));
        let angle = angle_between_rooms(&rooms[prev], &rooms[landmark]);
        if place_room(rooms, prev, id, angle) == -1.0 {
            return None;
        }
        loop_ids.push(id);
        prev = id;
    }
    None
}

/// Restores the insertion order of the room list returned by Java's builder.
///
/// Java creates the connection objects for both loops before placement, but it
/// does not append them to the returned `rooms` list until each loop has been
/// placed. Consequently, a closing stitch created while placing the first loop
/// precedes every connection belonging to the second loop in the final list.
fn restore_java_room_order(
    rooms: &mut Vec<Room>,
    base_len: usize,
    first_loop: &[usize],
    second_loop: &[usize],
) {
    let mut order = Vec::with_capacity(rooms.len());
    let mut included = vec![false; rooms.len()];

    let mut include = |old_id: usize| {
        if !included[old_id] {
            order.push(old_id);
            included[old_id] = true;
        }
    };

    for old_id in 0..base_len {
        include(old_id);
    }
    for &old_id in first_loop {
        if old_id >= base_len {
            include(old_id);
        }
    }
    for &old_id in second_loop {
        if old_id >= base_len {
            include(old_id);
        }
    }
    for old_id in base_len..rooms.len() {
        include(old_id);
    }

    let mut old_to_new = vec![0; rooms.len()];
    for (new_id, &old_id) in order.iter().enumerate() {
        old_to_new[old_id] = new_id;
    }

    let old_rooms = std::mem::take(rooms);
    let mut old_rooms = old_rooms.into_iter().map(Some).collect::<Vec<_>>();
    for (new_id, old_id) in order.into_iter().enumerate() {
        let mut room = old_rooms[old_id]
            .take()
            .expect("room insertion order must contain each room once");
        room.id = new_id;
        room.connected = room
            .connected
            .into_iter()
            .map(|old_id| old_to_new[old_id])
            .collect();
        room.neighbours = room
            .neighbours
            .into_iter()
            .map(|old_id| old_to_new[old_id])
            .collect();
        rooms.push(room);
    }
}

pub(super) fn build(
    rooms: &mut Vec<Room>,
    params: &BuilderParams,
    depth: i32,
    state: &mut FigureEightState,
    prepare_shop: &mut impl FnMut(&mut Room),
) -> Option<()> {
    let base_len = rooms.len();
    let mut setup = setup_rooms(rooms, params);

    if state.landmark.is_none() {
        for &room in &setup.main_path {
            if rooms[room].max_connections(DIR_ALL) >= 4
                && state.landmark.is_none_or(|current| {
                    rooms[current].min_width() * rooms[current].min_height()
                        < rooms[room].min_width() * rooms[room].min_height()
                })
            {
                state.landmark = Some(room);
            }
        }
        // This compensation happens only on the first build call, matching the
        // persistent Java builder field after a failed inner attempt.
        if !setup.multi.is_empty() {
            setup.main_path.push(setup.multi.remove(0));
        }
    }
    let landmark = state.landmark?;
    setup.main_path.retain(|&room| room != landmark);
    setup.multi.retain(|&room| room != landmark);

    let start_angle = Random::float_max(360.0);
    let mut on_first = setup.main_path.len() / 2;
    if setup.main_path.len() % 2 == 1 {
        on_first += Random::int_max(2) as usize;
    }
    let mut rooms_to_loop = setup.main_path;

    let mut first_temp = vec![landmark];
    first_temp.extend(rooms_to_loop.drain(..on_first));
    let entrance = setup.entrance?;
    first_temp.insert(first_temp.len().div_ceil(2), entrance);

    let mut tunnel_chances = params.path_tunnel_chances.to_vec();
    let mut first_loop = add_tunnels(
        rooms,
        &first_temp,
        &mut tunnel_chances,
        &params.path_tunnel_chances,
        depth,
    );

    let mut second_temp = vec![landmark];
    second_temp.extend(rooms_to_loop);
    if let Some(exit) = setup.exit {
        second_temp.insert(second_temp.len().div_ceil(2), exit);
    }
    let mut second_loop = add_tunnels(
        rooms,
        &second_temp,
        &mut tunnel_chances,
        &params.path_tunnel_chances,
        depth,
    );

    rooms[landmark].set_size();
    rooms[landmark].set_pos(0, 0);
    place_loop(rooms, &mut first_loop, landmark, start_angle, params, depth)?;
    place_loop(
        rooms,
        &mut second_loop,
        landmark,
        start_angle + 180.0,
        params,
        depth,
    )?;

    if let Some(shop) = setup.shop {
        prepare_shop(&mut rooms[shop]);
        let mut placed = false;
        for _ in 0..11 {
            if place_room(rooms, entrance, shop, Random::float_max(360.0)) != -1.0 {
                placed = true;
                break;
            }
        }
        if !placed {
            return None;
        }
    }

    let first_center = loop_center(rooms, &first_loop);
    let second_center = loop_center(rooms, &second_loop);
    let mut branchable = first_loop.clone();
    branchable.extend(&second_loop);
    if let Some(position) = branchable.iter().position(|&room| room == landmark) {
        branchable.remove(position);
    }
    let mut rooms_to_branch = setup.multi;
    rooms_to_branch.extend(setup.single);
    weight_rooms(rooms, &mut branchable);
    if !create_branches(
        rooms,
        &mut branchable,
        &rooms_to_branch,
        &params.branch_tunnel_chances,
        depth,
        BranchAngles::FigureEight {
            first_ids: &first_loop,
            first_center,
            second_center,
        },
    ) {
        return None;
    }

    restore_java_room_order(rooms, base_len, &first_loop, &second_loop);
    find_neighbours(rooms);
    for room in 0..rooms.len() {
        let neighbours = rooms[room].neighbours.clone();
        for other in neighbours {
            if !rooms[other].connected.contains(&room)
                && Random::float() < params.extra_connection_chance
            {
                let _ = connect(rooms, room, other);
            }
        }
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rooms::types::RoomKind;

    fn room(id: usize, name: &str, kind: RoomKind) -> Room {
        Room::new(id, name, kind, 1, 16, 3, 10, 3, 10)
    }

    #[test]
    fn java_order_places_first_loop_stitch_before_second_loop_connections() {
        let mut rooms = vec![
            room(0, "BaseA", RoomKind::Standard),
            room(1, "BaseB", RoomKind::Standard),
            room(2, "BaseC", RoomKind::Standard),
            room(3, "FirstPrecreated", RoomKind::Connection),
            room(4, "SecondPrecreatedA", RoomKind::Connection),
            room(5, "SecondPrecreatedB", RoomKind::Connection),
            room(6, "FirstStitch", RoomKind::Connection),
            room(7, "SecondStitch", RoomKind::Connection),
            room(8, "Branch", RoomKind::Connection),
        ];
        rooms[0].connected = vec![3];
        rooms[3].connected = vec![0, 6];
        rooms[6].connected = vec![3, 1];
        rooms[1].connected = vec![6];
        rooms[0].neighbours = vec![4, 8];
        rooms[4].neighbours = vec![0, 5];
        rooms[5].neighbours = vec![4, 7];
        rooms[7].neighbours = vec![5];
        rooms[8].neighbours = vec![0];

        restore_java_room_order(&mut rooms, 3, &[0, 3, 1, 6], &[0, 4, 2, 5, 7]);

        assert_eq!(
            rooms
                .iter()
                .map(|room| room.name.as_str())
                .collect::<Vec<_>>(),
            [
                "BaseA",
                "BaseB",
                "BaseC",
                "FirstPrecreated",
                "FirstStitch",
                "SecondPrecreatedA",
                "SecondPrecreatedB",
                "SecondStitch",
                "Branch",
            ]
        );
        assert!(rooms.iter().enumerate().all(|(id, room)| room.id == id));
        assert!(rooms.iter().all(|room| room
            .connected
            .iter()
            .chain(&room.neighbours)
            .all(|&other| other < rooms.len())));
        for room in &rooms {
            assert!(room
                .connected
                .iter()
                .all(|&other| rooms[other].connected.contains(&room.id)));
            assert!(room
                .neighbours
                .iter()
                .all(|&other| rooms[other].neighbours.contains(&room.id)));
        }
        assert_eq!(rooms[0].connected, [3]);
        assert_eq!(rooms[3].connected, [0, 4]);
        assert_eq!(rooms[4].connected, [3, 1]);
        assert_eq!(rooms[0].neighbours, [5, 8]);
        assert_eq!(rooms[5].neighbours, [0, 6]);
    }
}
