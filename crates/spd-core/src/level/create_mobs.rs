//! The depth-one `RegularLevel.createMobs` pass.
//!
//! Depth one is the only floor with a fixed eight-mob population.  Keeping
//! this in its own module makes the exact RNG-sensitive path auditable while
//! later floors can still be expanded without making `level/mod.rs` a god
//! file.

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
}

impl MobKind {
    fn label(self) -> &'static str {
        match self {
            Self::Rat => "Rat",
            Self::Albino => "Albino",
            Self::Snake => "Snake",
        }
    }

    fn is_large(self) -> bool {
        false
    }
}

/// Port `RegularLevel.createMobs` for the fixed depth-one population.
///
/// Returns `true` when this module consumed the floor's ambient mob stream;
/// deeper floors intentionally remain pending until their region rotations,
/// mob limits, and room-painted NPCs are represented in the map model.
pub(crate) fn create_depth_one(rooms: &[Room], map: &mut TerrainMap) -> bool {
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
    let mut remaining = 8;
    while remaining > 0 {
        if current_mob.is_none() {
            current_mob = Some(next_mob(&mut rotation));
        }
        let room_index = spawn_rooms[room_cursor % spawn_rooms.len()];
        room_cursor += 1;
        let room = &rooms[room_index];
        let mut tries = 30;
        let mut position = None;
        loop {
            let point = room.random();
            let Some(cell) = map.point_to_cell(point.x, point.y) else {
                tries -= 1;
                if tries < 0 {
                    break;
                }
                continue;
            };
            tries -= 1;
            let invalid = map.mob_occupied[cell]
                || entrance_fov[cell]
                || entrance_distance[cell] != i32::MAX
                || !map.passable[cell]
                || map.is_solid(cell)
                || !map.character_allowed[cell]
                || (room.name == "AquariumRoom" && map.map[cell] == terrain::WATER)
                || map.plant_occupied[cell]
                || map.map[cell] == EXIT
                || map.trap_names[cell].is_some()
                || (current_mob.is_some_and(MobKind::is_large) && !map.is_open_space(cell));
            if !invalid || tries < 0 {
                if !invalid && tries >= 0 {
                    position = Some(cell);
                }
                break;
            }
        }

        if let Some(cell) = position {
            let mob = current_mob.take().expect("mob is present while placing");
            map.mob_occupied[cell] = true;
            map.known_mobs[cell] = Some(mob.label());
            if map.map[cell] == HIGH_GRASS {
                map.map[cell] = GRASS;
            }
            remaining -= 1;
        }
    }
    true
}

fn next_mob(rotation: &mut Vec<MobKind>) -> MobKind {
    if rotation.is_empty() {
        rotation.extend([MobKind::Rat, MobKind::Rat, MobKind::Rat, MobKind::Snake]);
        for mob in &mut *rotation {
            // MobSpawner swaps alternatives before its list shuffle, and the
            // roll is made for every entry even when no alternative exists.
            let alt = Random::float() < 1.0 / 50.0;
            if alt && matches!(*mob, MobKind::Rat) {
                *mob = MobKind::Albino;
            }
        }
        Random::shuffle_list(rotation);
    }
    let mob = rotation.remove(0);
    // ChampionEnemy.rollForChampion always burns Random.Int(6), even with
    // challenges disabled. This is the stream edge that matters.
    let _ = Random::int_max(6);
    mob
}
