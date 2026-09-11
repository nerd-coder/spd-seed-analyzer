//! CityPainter water / grass / decorate with vault fills and `nTraps=0`.

use crate::level::painter::{self, DoorMap};
use crate::level::terrain::TerrainMap;
use crate::level::Feeling;
use crate::rooms::room::Room;

pub(super) fn paint(
    map: &mut TerrainMap,
    rooms: &[Room],
    order: &[usize],
    doors: &DoorMap,
    depth: i32,
) {
    painter::paint_environment_with(
        map,
        rooms,
        order,
        doors,
        depth,
        Feeling::None,
        0,
        (0.15, 12),
        (0.30, 3),
    );
}
