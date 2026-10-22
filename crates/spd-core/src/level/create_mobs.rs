//! Pinned `RegularLevel.createMobs` for supported non-boss floors.

mod champion;
mod navigation;
mod rotation;

#[cfg(test)]
mod tests;

pub(crate) use champion::ChampionCounter;

use crate::level::terrain::{
    self, TerrainMap, ENTRANCE, ENTRANCE_SP, EXIT, FURROWED_GRASS, GRASS, HIGH_GRASS,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use champion::GiantState;
use navigation::{distance_limited, shadow_cast};
use rotation::next_mob;

/// A mob the population loop places: map label and `Char.Property.LARGE`.
#[derive(Clone, Copy)]
struct Spawn {
    label: &'static str,
    large: bool,
    /// An unknown champion counter could make this mob a `LARGE` Giant.
    maybe_large: bool,
}

/// Outcome of one floor's `createMobs`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Population {
    /// Every mob found a legal cell.
    pub complete: bool,
    /// A possible Giant champion sits outside open space, so Java may have
    /// placed it elsewhere and drawn different RNG for the rest of the floor.
    pub may_differ: bool,
}

/// Runs the pinned population path. Exact final parity still depends on every
/// preceding builder, painter, and quest hook reaching the same state.
pub(crate) fn create_regular(
    depth: i32,
    large_feeling: bool,
    mut champions: ChampionCounter,
    rooms: &[Room],
    map: &mut TerrainMap,
) -> Population {
    let Some(entrance_cell) = map
        .map
        .iter()
        .position(|&tile| tile == ENTRANCE || tile == ENTRANCE_SP)
    else {
        return Population::default();
    };
    // RegularLevel.createMobs calls mobLimit before collecting and shuffling
    // the weighted StandardRoom list.
    let remaining = mob_limit(depth, large_feeling);
    let order: Vec<usize> = (0..rooms.len()).collect();
    let spawn_rooms = spawn_room_indices(rooms, &order, |room| room.size_factor);
    let mut rotation = Vec::new();
    populate(
        depth,
        rooms,
        map,
        entrance_cell,
        remaining,
        spawn_rooms,
        || {
            let (mob, roll) = next_mob(depth, &mut rotation);
            let giant = champions.roll(mob, roll, depth);
            Spawn {
                label: mob.label(),
                large: mob.is_large() || giant == GiantState::Yes,
                maybe_large: giant == GiantState::Maybe,
            }
        },
    )
}

/// `RegularLevel.createMobs` on a `MiningLevel`: one fewer mob than usual and
/// `MiningLevel.createMob` (no champion roll). `order` is Java `level.rooms`.
pub(crate) fn create_mining(
    depth: i32,
    rooms: &[Room],
    order: &[usize],
    map: &mut TerrainMap,
    entrance_cell: usize,
    crystal: bool,
) -> bool {
    let remaining = mob_limit(depth, false) - 1;
    // MineLargeRoom and MineGiantRoom spawn their own enemies, so weigh 1.
    let spawn_rooms = spawn_room_indices(rooms, order, |room| {
        if matches!(room.name.as_str(), "MineLargeRoom" | "MineGiantRoom") {
            1
        } else {
            room.size_factor
        }
    });
    populate(
        depth,
        rooms,
        map,
        entrance_cell,
        remaining,
        spawn_rooms,
        || {
            let label = if crystal {
                // `CrystalWisp()` picks its sprite colour.
                let _ = Random::int_max(3);
                "CrystalWisp"
            } else {
                "GnollGuard"
            };
            Spawn {
                label,
                large: false,
                maybe_large: false,
            }
        },
    )
    .complete
}

fn populate(
    depth: i32,
    rooms: &[Room],
    map: &mut TerrainMap,
    entrance_cell: usize,
    mut remaining: i32,
    mut spawn_rooms: Vec<usize>,
    mut create_mob: impl FnMut() -> Spawn,
) -> Population {
    let mut population = Population::default();
    let entrance_room = rooms
        .iter()
        .find(|room| room.is_entrance() && !room.is_empty());
    let Some(entrance_room) = entrance_room else {
        return population;
    };
    if spawn_rooms.is_empty() {
        return population;
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

    let mut room_cursor = 0usize;
    let mut current_mob: Option<Spawn> = None;
    let mut failed_placements = 0usize;
    while remaining > 0 {
        let mob = *current_mob.get_or_insert_with(&mut create_mob);
        let room_index = spawn_rooms[room_cursor % spawn_rooms.len()];
        room_cursor += 1;
        let room = &rooms[room_index];
        if let Some(cell) = find_position(room, mob, map, &entrance_fov, &entrance_distance) {
            failed_placements = 0;
            current_mob = None;
            population.may_differ |= place_mob(map, cell, mob);
            remaining -= 1;

            // Java may immediately place one more mob in the same room. A
            // failed second placement retains that mob for the next room.
            if depth > 1 && remaining > 0 && Random::int_max(4) == 0 {
                let second = create_mob();
                current_mob = Some(second);
                if let Some(cell) =
                    find_position(room, second, map, &entrance_fov, &entrance_distance)
                {
                    current_mob = None;
                    population.may_differ |= place_mob(map, cell, second);
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
                return population;
            }
        }
    }

    for cell in 0..map.len() {
        if map.mob_occupied[cell] && matches!(map.map[cell], HIGH_GRASS | FURROWED_GRASS) {
            map.map[cell] = GRASS;
        }
    }
    population.complete = true;
    population
}

/// Java `RegularLevel.createMobs` duplicates each `StandardRoom` according to
/// `mobSpawnWeight()` before shuffling. The regular entrance/exit hierarchy
/// extends `StandardRoom`: an entrance has weight one, while an exit retains
/// its size-category weight.
fn spawn_room_indices(
    rooms: &[Room],
    order: &[usize],
    size_weight: impl Fn(&Room) -> i32,
) -> Vec<usize> {
    let mut spawn_rooms = Vec::new();
    for &index in order {
        let room = &rooms[index];
        // `EntranceRoom` / `ExitRoom` inherit `StandardRoom` in pinned SPD.
        // The Rust model stores their role separately, so preserve their
        // inherited Java spawn weights explicitly.
        if matches!(
            room.kind,
            RoomKind::Entrance | RoomKind::Exit | RoomKind::Standard
        ) {
            let weight = if room.is_entrance() {
                1
            } else {
                size_weight(room).max(0)
            };
            for _ in 0..weight {
                spawn_rooms.push(index);
            }
        }
    }
    spawn_rooms
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
    mob: Spawn,
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
                || (mob.large && !map.is_open_space(cell))
        });
        if !invalid {
            return (tries >= 0).then_some(cell.expect("valid cell"));
        }
        if tries < 0 {
            return None;
        }
    }
}

/// Places `mob` and reports whether a Giant could not have stood there. A
/// `LARGE` check only rejects more probes, so an open-space cell is the one
/// Java picks either way.
fn place_mob(map: &mut TerrainMap, cell: usize, mob: Spawn) -> bool {
    map.mob_occupied[cell] = true;
    map.place_mob(cell, mob.label);
    mob.maybe_large && !map.is_open_space(cell)
}
