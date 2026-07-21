//! Incremental ports of `StandardRoom.paint` geometry.

mod patch_rooms;

use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

use super::DoorMap;

/// Paint a supported standard-room family in RegularPainter room order.
pub(crate) fn paint_standard_room(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) {
    let _ = patch_rooms::paint(map, room, room_index, doors);
}
