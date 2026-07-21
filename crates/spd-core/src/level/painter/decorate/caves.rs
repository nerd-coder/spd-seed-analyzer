//! Pinned `CavesPainter.decorate`.

use crate::level::terrain::{
    TerrainMap, CHASM, EMBERS, EMPTY, EMPTY_DECO, EMPTY_SP, ENTRANCE, EXIT, GRASS, INACTIVE_TRAP,
    PEDESTAL, REGION_DECO, SECRET_TRAP, TRAP, WALL, WALL_DECO, WATER,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::DoorMap;

pub(super) fn decorate(
    map: &mut TerrainMap,
    rooms: &[Room],
    paint_order: &[usize],
    doors: &DoorMap,
) {
    merge_neighbours(map, rooms, paint_order);
    fill_room_corners(map, rooms, paint_order, doors);
    paint_floor_deco(map);
    generate_gold(map);
}

fn merge_neighbours(map: &mut TerrainMap, rooms: &[Room], paint_order: &[usize]) {
    for &room_index in paint_order {
        let Some(room) = rooms.get(room_index).filter(|r| !r.is_empty()) else {
            continue;
        };
        for &neighbour_index in &room.neighbours {
            let Some(neighbour) = rooms.get(neighbour_index).filter(|r| !r.is_empty()) else {
                continue;
            };
            if !room.connected.contains(&neighbour_index) {
                let terrain = if Random::int_max(3) == 0 {
                    REGION_DECO
                } else {
                    CHASM
                };
                let _ = super::super::doors::merge_rooms_with_terrain(
                    map, room, neighbour, None, terrain, 11,
                );
            }
        }
    }
}

fn fill_room_corners(map: &mut TerrainMap, rooms: &[Room], paint_order: &[usize], doors: &DoorMap) {
    for &room_index in paint_order {
        let Some(room) = rooms.get(room_index).filter(|r| !r.is_empty()) else {
            continue;
        };
        if !matches!(
            room.kind,
            RoomKind::Standard | RoomKind::Entrance | RoomKind::Exit
        ) || room.width() <= 4
            || room.height() <= 4
        {
            continue;
        }
        let square = room.square();
        for (cx, cy, wall_dx, wall_dy) in [
            (room.left + 1, room.top + 1, -1, -1),
            (room.right - 1, room.top + 1, 1, -1),
            (room.left + 1, room.bottom - 1, -1, 1),
            (room.right - 1, room.bottom - 1, 1, 1),
        ] {
            if Random::int_max(square) <= 8 {
                continue;
            }
            let Some(corner) = map.point_to_cell(cx, cy) else {
                continue;
            };
            let Some(side_x) = map.point_to_cell(cx + wall_dx, cy) else {
                continue;
            };
            let Some(side_y) = map.point_to_cell(cx, cy + wall_dy) else {
                continue;
            };
            let Some(inward_x) = map.point_to_cell(cx - wall_dx, cy) else {
                continue;
            };
            let Some(inward_y) = map.point_to_cell(cx, cy - wall_dy) else {
                continue;
            };
            if map.is_solid(corner)
                || map.map[side_x] != WALL
                || room_has_door_at(room, room_index, doors, cx + wall_dx, cy)
                || map.map[side_y] != WALL
                || room_has_door_at(room, room_index, doors, cx, cy + wall_dy)
                || map.map[inward_x] == TRAP
                || map.map[inward_y] == TRAP
            {
                continue;
            }
            map.map[corner] = WALL;
            map.trap_destroys_items[corner] = false;
            map.trap_names[corner] = None;
        }
    }
}

fn paint_floor_deco(map: &mut TerrainMap) {
    let width = map.width;
    let length = map.map.len() as i32;
    for cell in (width + 1)..(length - width) {
        let cell = cell as usize;
        if map.map[cell] != EMPTY {
            continue;
        }
        let neighbours = (map.map[cell + 1] == WALL) as i32
            + (map.map[cell - 1] == WALL) as i32
            + (map.map[cell + width as usize] == WALL) as i32
            + (map.map[cell - width as usize] == WALL) as i32;
        if Random::int_max(6) <= neighbours {
            map.map[cell] = EMPTY_DECO;
        }
    }
}

fn generate_gold(map: &mut TerrainMap) {
    let width = map.width;
    let length = map.map.len() as i32;
    for cell in 0..(length - width) {
        let cell = cell as usize;
        if map.map[cell] == WALL
            && is_floor_tile(map.map[cell + width as usize])
            && Random::int_max(4) == 0
        {
            map.map[cell] = WALL_DECO;
        }
    }
}

fn is_floor_tile(terrain: i32) -> bool {
    matches!(
        terrain,
        EMPTY
            | GRASS
            | EMBERS
            | EMPTY_SP
            | ENTRANCE
            | EXIT
            | PEDESTAL
            | SECRET_TRAP
            | TRAP
            | INACTIVE_TRAP
            | EMPTY_DECO
            | WATER
    )
}

fn room_has_door_at(room: &Room, room_index: usize, doors: &DoorMap, x: i32, y: i32) -> bool {
    room.connected.iter().any(|&other| {
        doors
            .get(room_index, other)
            .is_some_and(|door| door.x == x && door.y == y)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cave_room(id: usize, left: i32, right: i32) -> Room {
        let mut room = Room::new(
            id,
            "CircleWallRoom",
            RoomKind::Standard,
            2,
            16,
            10,
            14,
            10,
            14,
        );
        room.left = left;
        room.top = 1;
        room.right = right;
        room.bottom = 10;
        room
    }

    #[test]
    fn merges_unconnected_neighbours_with_region_terrain() {
        let mut left = cave_room(0, 1, 10);
        let mut right = cave_room(1, 10, 19);
        left.neighbours.push(1);
        right.neighbours.push(0);
        let rooms = vec![left, right];
        let mut map = crate::level::terrain::paint_minimal(&rooms).expect("map");

        Random::push_generator_seeded(0xCA7E);
        decorate(&mut map, &rooms, &[1, 0], &DoorMap::new());
        Random::pop_generator();

        assert!((2..10).any(|y| {
            map.point_to_cell(10, y)
                .is_some_and(|cell| matches!(map.map[cell], CHASM | REGION_DECO))
        }));
    }
}
