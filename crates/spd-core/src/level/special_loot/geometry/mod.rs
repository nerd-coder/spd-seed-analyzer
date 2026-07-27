//! RNG-visible geometry for special and secret rooms.

mod basic;
mod crypt;
mod maze;
mod rot_garden;
mod sacrifice;
mod summoning;

#[cfg(test)]
mod tests;

use crate::level::painter::DoorMap;
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

/// Paint supported rooms at the point their Java `Room.paint` runs.
/// Returns the room painter's exact prize cell when geometry chooses one.
#[cfg(test)]
pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) -> Option<usize> {
    paint_at_depth(map, room, room_index, doors, 0)
}

pub(super) fn paint_at_depth(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    depth: i32,
) -> Option<usize> {
    match room.name.as_str() {
        "CryptRoom" => Some(crypt::paint(map, room, room_index, doors)),
        "SecretMazeRoom" => maze::paint(map, room, room_index, doors),
        "SecretSummoningRoom" => Some(summoning::paint(map, room)),
        "RotGardenRoom" => {
            rot_garden::paint(map, room, room_index, doors);
            None
        }
        "SacrificeRoom" => {
            sacrifice::paint(map, room, room_index, doors, depth);
            None
        }
        "GardenRoom" => {
            basic::paint_garden(map, room);
            None
        }
        "LibraryRoom" => {
            basic::paint_library(map, room, room_index, doors);
            None
        }
        "PoolRoom" => basic::paint_pool(map, room, room_index, doors),
        "PitRoom" => {
            basic::paint_pit(map, room, room_index, doors);
            None
        }
        "RunestoneRoom" => {
            basic::paint_runestone(map, room, room_index, doors);
            None
        }
        "SecretRunestoneRoom" => {
            basic::paint_secret_runestone(map, room, room_index, doors);
            None
        }
        "SecretLibraryRoom" => {
            basic::paint_secret_library(map, room, room_index, doors);
            None
        }
        "WeakFloorRoom" => {
            basic::paint_weak_floor(map, room, room_index, doors);
            None
        }
        "DemonSpawnerRoom" => {
            basic::paint_demon_spawner(map, room);
            None
        }
        "AmbitiousImpRoom" => {
            basic::paint_ambitious_imp(map, room, room_index, doors);
            None
        }
        _ => None,
    }
}
