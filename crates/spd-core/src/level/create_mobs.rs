//! `RegularLevel.createMobs`: oracle-exact at depth 1 and a source-aligned
//! partial port for Sewer floors 2–4 and Prison floors 6–8.

mod navigation;

#[cfg(test)]
mod tests;

use crate::level::terrain::{self, TerrainMap, ENTRANCE, ENTRANCE_SP, EXIT, GRASS, HIGH_GRASS};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use navigation::{distance_limited, shadow_cast};

#[derive(Clone, Copy)]
enum MobKind {
    Rat,
    Albino,
    Snake,
    Gnoll,
    GnollExile,
    Swarm,
    Crab,
    HermitCrab,
    Slime,
    CausticSlime,
    Skeleton,
    Thief,
    Bandit,
    Dm100,
    Guard,
    Necromancer,
    SpectralNecromancer,
}

impl MobKind {
    fn label(self) -> &'static str {
        match self {
            Self::Rat => "Rat",
            Self::Albino => "Albino",
            Self::Snake => "Snake",
            Self::Gnoll => "Gnoll",
            Self::GnollExile => "GnollExile",
            Self::Swarm => "Swarm",
            Self::Crab => "Crab",
            Self::HermitCrab => "HermitCrab",
            Self::Slime => "Slime",
            Self::CausticSlime => "CausticSlime",
            Self::Skeleton => "Skeleton",
            Self::Thief => "Thief",
            Self::Bandit => "Bandit",
            Self::Dm100 => "DM100",
            Self::Guard => "Guard",
            Self::Necromancer => "Necromancer",
            Self::SpectralNecromancer => "SpectralNecromancer",
        }
    }

    fn is_large(self) -> bool {
        false
    }
}

/// Runs the exact depth-one path and the partial deeper-floor path for depths
/// 2–4 and 6–8. The latter retains pinned rotations and placement semantics but
/// is not yet a lifecycle or cell-for-cell parity claim.
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
    let mut remaining = if depth == 1 {
        8
    } else {
        let base = 3 + depth % 5 + Random::int_max(3);
        if large_feeling {
            (base as f32 * 1.33).ceil() as i32
        } else {
            base
        }
    };

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
        }
    }

    for cell in 0..map.len() {
        if map.mob_occupied[cell] && map.map[cell] == HIGH_GRASS {
            map.map[cell] = GRASS;
        }
    }
    true
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

fn next_mob(depth: i32, rotation: &mut Vec<MobKind>) -> MobKind {
    if rotation.is_empty() {
        rotation.extend(match depth {
            2 => vec![
                MobKind::Rat,
                MobKind::Rat,
                MobKind::Snake,
                MobKind::Gnoll,
                MobKind::Gnoll,
            ],
            3 => vec![
                MobKind::Rat,
                MobKind::Snake,
                MobKind::Gnoll,
                MobKind::Gnoll,
                MobKind::Gnoll,
                MobKind::Swarm,
                MobKind::Crab,
            ],
            4 => vec![
                MobKind::Gnoll,
                MobKind::Swarm,
                MobKind::Crab,
                MobKind::Crab,
                MobKind::Slime,
                MobKind::Slime,
            ],
            6 => vec![
                MobKind::Skeleton,
                MobKind::Skeleton,
                MobKind::Skeleton,
                MobKind::Thief,
                MobKind::Swarm,
            ],
            7 => vec![
                MobKind::Skeleton,
                MobKind::Skeleton,
                MobKind::Skeleton,
                MobKind::Thief,
                MobKind::Dm100,
                MobKind::Guard,
            ],
            8 => vec![
                MobKind::Skeleton,
                MobKind::Skeleton,
                MobKind::Thief,
                MobKind::Dm100,
                MobKind::Dm100,
                MobKind::Guard,
                MobKind::Guard,
                MobKind::Necromancer,
            ],
            _ => vec![MobKind::Rat, MobKind::Rat, MobKind::Rat, MobKind::Snake],
        });
        if depth == 4 && Random::float() < 0.025 {
            rotation.push(MobKind::Thief);
        }
        for mob in &mut *rotation {
            // MobSpawner swaps alternatives before its list shuffle, and the
            // roll is made for every entry even when no alternative exists.
            let alt = Random::float() < 1.0 / 50.0;
            if alt {
                *mob = match *mob {
                    MobKind::Rat => MobKind::Albino,
                    MobKind::Gnoll => MobKind::GnollExile,
                    MobKind::Crab => MobKind::HermitCrab,
                    MobKind::Slime => MobKind::CausticSlime,
                    MobKind::Thief => MobKind::Bandit,
                    MobKind::Necromancer => MobKind::SpectralNecromancer,
                    other => other,
                };
            }
        }
        Random::shuffle_list(rotation);
    }
    let mob = rotation.remove(0);
    if matches!(mob, MobKind::Thief | MobKind::Bandit) {
        // Thief's instance initializer chooses Ring versus Artifact loot.
        let _ = Random::int_max(2);
    }
    // ChampionEnemy.rollForChampion always burns Random.Int(6), even with
    // challenges disabled. This is the stream edge that matters.
    let _ = Random::int_max(6);
    mob
}
