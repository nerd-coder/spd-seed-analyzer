//! Pinned SPD v3.3.8 `LoopBuilder`.

use crate::builders::connection;
use crate::builders::place::{angle_between_rooms, find_neighbours, place_room};
use crate::builders::regular::{
    create_branches, loop_center, setup_rooms, target_angle, weight_rooms, BranchAngles,
    BuilderParams,
};
use crate::random::Random;
use crate::rooms::room::{connect, Room};

pub(super) fn build(
    rooms: &mut Vec<Room>,
    params: &BuilderParams,
    depth: i32,
    prepare_shop: &mut impl FnMut(&mut Room),
) -> Option<()> {
    let setup = setup_rooms(rooms, params);
    let entrance = setup.entrance?;
    let mut main_path = setup.main_path;

    rooms[entrance].set_size();
    rooms[entrance].set_pos(0, 0);
    let start_angle = Random::float_max(360.0);

    main_path.insert(0, entrance);
    if let Some(exit) = setup.exit {
        main_path.insert(main_path.len().div_ceil(2), exit);
    }

    let mut loop_ids = Vec::new();
    let mut tunnel_chances = params.path_tunnel_chances.to_vec();
    for room in main_path {
        loop_ids.push(room);
        let mut tunnels = Random::chances(&tunnel_chances);
        if tunnels == -1 {
            tunnel_chances = params.path_tunnel_chances.to_vec();
            tunnels = Random::chances(&tunnel_chances);
        }
        tunnel_chances[tunnels as usize] -= 1.0;
        for _ in 0..tunnels {
            let id = rooms.len();
            rooms.push(connection::create(id, depth));
            loop_ids.push(id);
        }
    }

    let mut prev = entrance;
    for i in 1..loop_ids.len() {
        let room = loop_ids[i];
        let angle = start_angle + target_angle(i as f32 / loop_ids.len() as f32, params);
        if place_room(rooms, prev, room, angle) == -1.0 {
            return None;
        }
        prev = room;
    }

    let mut stitched = false;
    for _ in 0..10_000 {
        if connect(rooms, prev, entrance) {
            stitched = true;
            break;
        }
        let id = rooms.len();
        rooms.push(connection::create(id, depth));
        let angle = angle_between_rooms(&rooms[prev], &rooms[entrance]);
        if place_room(rooms, prev, id, angle) == -1.0 {
            return None;
        }
        loop_ids.push(id);
        prev = id;
    }
    if !stitched {
        return None;
    }

    if let Some(shop) = setup.shop {
        prepare_shop(&mut rooms[shop]);
        let mut placed = false;
        // Java's do/while with `tries >= 0` makes eleven attempts.
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

    let center = loop_center(rooms, &loop_ids);
    let mut branchable = loop_ids;
    let mut rooms_to_branch = setup.multi;
    rooms_to_branch.extend(setup.single);
    weight_rooms(rooms, &mut branchable);
    if !create_branches(
        rooms,
        &mut branchable,
        &rooms_to_branch,
        &params.branch_tunnel_chances,
        depth,
        BranchAngles::Around(center),
    ) {
        return None;
    }

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
