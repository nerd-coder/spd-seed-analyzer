//! Pinned `RegularLevel.createMobs` for supported non-boss floors.

mod navigation;
mod rotation;

#[cfg(test)]
mod tests;

use crate::level::terrain::{self, TerrainMap, ENTRANCE, ENTRANCE_SP, EXIT, GRASS, HIGH_GRASS};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use navigation::{distance_limited, shadow_cast};
use rotation::{next_mob, MobKind};

/// Runs the pinned population path. Exact final parity still depends on every
/// preceding builder, painter, and quest hook reaching the same state.
pub(crate) fn create_regular(
    depth: i32,
    large_feeling: bool,
    rooms: &[Room],
    map: &mut TerrainMap,
) -> bool {
    let entrance_room = rooms
        .iter()
        .find(|room| room.is_entrance() && !room.is_empty());
    let Some(entrance_room) = entrance_room else {
        return false;
    };
    let Some(entrance_cell) = map
        .map
        .iter()
        .position(|&tile| tile == ENTRANCE || tile == ENTRANCE_SP)
    else {
        return false;
    };

    // RegularLevel.createMobs calls mobLimit before collecting and shuffling
    // the weighted StandardRoom list.
    let mut remaining = mob_limit(depth, large_feeling);

    let mut spawn_rooms = Vec::new();
    for (index, room) in rooms.iter().enumerate() {
        if matches!(
            room.kind,
            RoomKind::Entrance | RoomKind::Exit | RoomKind::Standard
        ) {
            let weight = if room.is_entrance() {
                1
            } else {
                room.size_factor.max(0)
            };
            for _ in 0..weight {
                spawn_rooms.push(index);
            }
        }
    }
    if spawn_rooms.is_empty() {
        return false;
    }
    Random::shuffle_list(&mut spawn_rooms);

    let entrance_fov = shadow_cast(entrance_cell, map);
    let mut entrance_walkable: Vec<bool> = (0..map.len()).map(|i| !map.is_solid(i)).collect();
    for y in (entrance_room.top + 1)..entrance_room.bottom {
        for x in (entrance_room.left + 1)..entrance_room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                if map.passable[cell] {
                    entrance_walkable[cell] = true;
                }
            }
        }
    }
    let entrance_distance = distance_limited(map, entrance_cell, &entrance_walkable, 8);

    let mut rotation = Vec::new();
    let mut room_cursor = 0usize;
    let mut current_mob = None;
    let mut failed_placements = 0usize;
    while remaining > 0 {
        if current_mob.is_none() {
            current_mob = Some(next_mob(depth, &mut rotation));
        }
        let room_index = spawn_rooms[room_cursor % spawn_rooms.len()];
        room_cursor += 1;
        let room = &rooms[room_index];
        if let Some(cell) = find_position(
            room,
            current_mob.expect("mob exists"),
            map,
            &entrance_fov,
            &entrance_distance,
        ) {
            failed_placements = 0;
            let mob = current_mob.take().expect("mob is present while placing");
            place_mob(map, cell, mob);
            remaining -= 1;

            // Java may immediately place one more mob in the same room. A
            // failed second placement retains that mob for the next room.
            if depth > 1 && remaining > 0 && Random::int_max(4) == 0 {
                current_mob = Some(next_mob(depth, &mut rotation));
                if let Some(cell) = find_position(
                    room,
                    current_mob.expect("second mob exists"),
                    map,
                    &entrance_fov,
                    &entrance_distance,
                ) {
                    let mob = current_mob.take().expect("second mob is present");
                    place_mob(map, cell, mob);
                    remaining -= 1;
                }
            }
        } else {
            failed_placements += 1;
            // Compatibility guard for the partial painter: pinned SPD retries
            // forever because a fully painted valid level guarantees at least
            // one legal spawn cell. Our incomplete painter cannot guarantee that
            // invariant yet. Remove this guard once every supported layout is
            // fully painted and validity-checked like the pinned game.
            // One traversal gives every weighted room 31 placement probes.
            if failed_placements >= spawn_rooms.len() {
                return false;
            }
        }
    }

    for cell in 0..map.len() {
        if map.mob_occupied[cell] && map.map[cell] == HIGH_GRASS {
            map.map[cell] = GRASS;
        }
    }
    true
}

fn mob_limit(depth: i32, large_feeling: bool) -> i32 {
    if depth == 1 {
        return 8;
    }
    let base = 3 + depth % 5 + Random::int_max(3);
    if large_feeling {
        (base as f32 * 1.33).ceil() as i32
    } else {
        base
    }
}

fn find_position(
    room: &Room,
    mob: MobKind,
    map: &TerrainMap,
    entrance_fov: &[bool],
    entrance_distance: &[i32],
) -> Option<usize> {
    let mut tries = 30;
    loop {
        let point = room.random();
        let cell = map.point_to_cell(point.x, point.y);
        tries -= 1;
        let invalid = cell.is_none_or(|cell| {
            map.mob_occupied[cell]
                || entrance_fov[cell]
                || entrance_distance[cell] != i32::MAX
                || !map.passable[cell]
                || map.is_solid(cell)
                || !map.character_allowed[cell]
                || (room.name == "AquariumRoom" && map.map[cell] == terrain::WATER)
                || map.plant_occupied[cell]
                || map.map[cell] == EXIT
                || map.trap_names[cell].is_some()
                || (mob.is_large() && !map.is_open_space(cell))
        });
        if !invalid {
            return (tries >= 0).then_some(cell.expect("valid cell"));
        }
        if tries < 0 {
            return None;
        }
    }
}

fn place_mob(map: &mut TerrainMap, cell: usize, mob: MobKind) {
    map.mob_occupied[cell] = true;
    map.known_mobs[cell] = Some(mob.label());
}
