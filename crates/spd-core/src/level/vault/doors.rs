//! Vault `paintDoors`: hidden-door chance 0, no room merges.

use crate::level::painter::{self, DoorMap};
use crate::level::terrain::TerrainMap;
use crate::level::Feeling;
use crate::rooms::room::Room;

pub(super) fn paint(
    map: &mut TerrainMap,
    rooms: &[Room],
    order: &[usize],
    doors: &mut DoorMap,
    depth: i32,
) {
    painter::paint_doors_with_hidden_chance(map, rooms, order, depth, Feeling::None, doors, 0.0);
}
