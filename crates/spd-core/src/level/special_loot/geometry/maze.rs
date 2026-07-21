//! `SecretMazeRoom.paint` geometry using the shared pinned Maze feature.

use crate::geom::Point;
use crate::level::maze;
use crate::level::painter::DoorMap;
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    let door_points: Vec<Point> = room
        .connected
        .iter()
        .filter_map(|&other| {
            doors
                .get(room_index, other)
                .map(|door| Point::new(door.x, door.y))
        })
        .collect();
    let (cells, width, height) = maze::generate_for_room(room, &door_points);
    maze::paint_cells(map, room, &cells, width, height);

    if let Some(entrance) = door_points.first() {
        let entrance = (entrance.x - room.left, entrance.y - room.top);
        let passable: Vec<bool> = cells.iter().map(|&filled| !filled).collect();
        let distances = maze::distance_map(
            width,
            height,
            maze::index(width, entrance.0, entrance.1),
            &passable,
        );
        let mut best_distance = 0;
        let mut best_cell = 0;
        for (cell, &distance) in distances.iter().enumerate() {
            if distance != i32::MAX && distance > best_distance {
                best_distance = distance;
                best_cell = cell;
            }
        }
        if best_distance > 0 {
            if let Some(prize_cell) = map.point_to_cell(
                room.left + best_cell as i32 % width,
                room.top + best_cell as i32 / width,
            ) {
                map.heap_occupied[prize_cell] = true;
            }
        }
    }
}
