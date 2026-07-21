//! RNG-visible geometry for special and secret rooms.

mod basic;
mod maze;
mod rot_garden;

#[cfg(test)]
mod tests;

use crate::level::painter::DoorMap;
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

/// Paint supported rooms at the point their Java `Room.paint` runs.
pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    match room.name.as_str() {
        "SecretMazeRoom" => maze::paint(map, room, room_index, doors),
        "RotGardenRoom" => rot_garden::paint(map, room, room_index, doors),
        "WeakFloorRoom" => basic::paint_weak_floor(map, room, room_index, doors),
        "DemonSpawnerRoom" => basic::paint_demon_spawner(map, room),
        _ => {}
    }
}
