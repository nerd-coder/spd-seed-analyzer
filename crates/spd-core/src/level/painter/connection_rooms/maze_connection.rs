use crate::level::maze;
use crate::level::painter::DoorMap;
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

use super::common::door_points;

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    let points = door_points(room, room_index, doors);
    let mut generated = maze::generate_for_room(room, &points);
    // Java retries small mazes until their center is filled. The cap only
    // protects malformed browser callers; valid rooms converge quickly.
    for _ in 0..10_000 {
        let (cells, width, height) = &generated;
        let small = *width >= 5 && *height >= 5 && (*width <= 7 || *height <= 7);
        let center = (*width / 2 + (*height / 2) * *width) as usize;
        if !small || cells[center] {
            maze::paint_cells(map, room, cells, *width, *height);
            return;
        }
        generated = maze::generate_for_room(room, &points);
    }
    let (cells, width, height) = generated;
    maze::paint_cells(map, room, &cells, width, height);
}
