//! Incremental ports of `StandardRoom.paint` geometry.

mod generic_rooms;
mod patch_rooms;

use crate::generator::GeneratorState;
use crate::geom::Point;
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

use super::DoorMap;

#[derive(Debug, Default)]
pub(crate) struct StandardPaintResult {
    /// Center already selected by `Room.center()` for center-loot rooms.
    pub center_loot: Option<Point>,
}

/// Paint a supported standard-room family in RegularPainter room order.
pub(crate) fn paint_standard_room(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    generator: &mut GeneratorState,
    depth: i32,
) -> StandardPaintResult {
    if patch_rooms::paint(map, room, room_index, doors) {
        StandardPaintResult::default()
    } else {
        generic_rooms::paint(map, room, room_index, doors, generator, depth).unwrap_or_default()
    }
}
