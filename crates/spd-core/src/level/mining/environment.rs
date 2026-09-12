//! Mining painter's isolated water, grass, caves decoration, and dark-gold walls.

use crate::geom::Point;
use crate::level::painter::{merge_rooms_with_terrain, DoorMap};
use crate::level::patch;
use crate::level::terrain::{
    TerrainMap, CHASM, EMPTY, EMPTY_DECO, GRASS, HIGH_GRASS, REGION_DECO, TRAP, WALL, WALL_DECO,
    WATER,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

pub(super) fn paint(
    map: &mut TerrainMap,
    rooms: &[Room],
    order: &mut [usize],
    doors: &DoorMap,
    mut gold: i32,
) {
    let seed = Random::long();
    Random::push_generator_seeded(seed);
    paint_water(map, rooms, order);
    paint_grass(map, rooms, order);
    merge_neighbours(map, rooms, order);
    fill_corners(map, rooms, order, doors);
    floor_deco(map);
    gold -= map.map.iter().filter(|&&tile| tile == WALL_DECO).count() as i32;
    generate_gold(map, rooms, order, &mut gold);
    for tile in &mut map.map {
        if *tile == CHASM {
            *tile = EMPTY;
        }
    }
    Random::pop_generator();
}

fn paint_water(map: &mut TerrainMap, rooms: &[Room], order: &[usize]) {
    let mask = patch::generate(map.width, map.height, 0.35, 6, true);
    for &room_index in order {
        let room = &rooms[room_index];
        for x in room.left..=room.right {
            for y in room.top..=room.bottom {
                let cell = map.point_to_cell(x, y).unwrap();
                if mask[cell] && map.map[cell] == EMPTY {
                    map.map[cell] = WATER;
                }
            }
        }
    }
}

fn paint_grass(map: &mut TerrainMap, rooms: &[Room], order: &[usize]) {
    let mask = patch::generate(map.width, map.height, 0.10, 3, true);
    let mut cells = Vec::new();
    for &room_index in order {
        let room = &rooms[room_index];
        for x in room.left..=room.right {
            for y in room.top..=room.bottom {
                let cell = map.point_to_cell(x, y).unwrap();
                if mask[cell] && map.map[cell] == EMPTY {
                    cells.push(cell);
                }
            }
        }
    }
    let w = map.width as isize;
    let neighbours = [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1];
    for cell in cells {
        if map.heap_occupied[cell] || map.mob_occupied[cell] {
            map.map[cell] = GRASS;
            continue;
        }
        let count = 1 + neighbours
            .iter()
            .filter(|offset| mask[(cell as isize + **offset) as usize])
            .count();
        map.map[cell] = if Random::float() < count as f32 / 12.0 {
            HIGH_GRASS
        } else {
            GRASS
        };
    }
}

fn merge_neighbours(map: &mut TerrainMap, rooms: &[Room], order: &[usize]) {
    for &room_index in order {
        let room = &rooms[room_index];
        for &other in &room.neighbours {
            if !room.connected.contains(&other) {
                let terrain = if Random::int_max(3) == 0 {
                    REGION_DECO
                } else {
                    CHASM
                };
                let _ = merge_rooms_with_terrain(map, room, &rooms[other], None, terrain, 12);
            }
        }
    }
}

fn fill_corners(map: &mut TerrainMap, rooms: &[Room], order: &[usize], doors: &DoorMap) {
    for &room_index in order {
        let room = &rooms[room_index];
        if !matches!(room.kind, RoomKind::Standard | RoomKind::Entrance)
            || room.width() <= 4
            || room.height() <= 4
        {
            continue;
        }
        for (x, y, dx, dy) in [
            (room.left + 1, room.top + 1, -1, -1),
            (room.right - 1, room.top + 1, 1, -1),
            (room.left + 1, room.bottom - 1, -1, 1),
            (room.right - 1, room.bottom - 1, 1, 1),
        ] {
            if Random::int_max(room.square()) <= 8 {
                continue;
            }
            let cell = map.point_to_cell(x, y).unwrap();
            let side_x = map.point_to_cell(x + dx, y).unwrap();
            let side_y = map.point_to_cell(x, y + dy).unwrap();
            let inner_x = map.point_to_cell(x - dx, y).unwrap();
            let inner_y = map.point_to_cell(x, y - dy).unwrap();
            let blocks_door = room.connected.iter().any(|&other| {
                doors.get(room_index, other).is_some_and(|door| {
                    (door.x == x + dx && door.y == y) || (door.x == x && door.y == y + dy)
                })
            });
            if !map.is_solid(cell)
                && map.map[side_x] == WALL
                && map.map[side_y] == WALL
                && !blocks_door
                && map.map[inner_x] != TRAP
                && map.map[inner_y] != TRAP
            {
                map.map[cell] = WALL;
                map.trap_names[cell] = None;
            }
        }
    }
}

fn floor_deco(map: &mut TerrainMap) {
    let w = map.width as usize;
    for cell in w + 1..map.len() - w {
        if map.map[cell] != EMPTY {
            continue;
        }
        let walls = (map.map[cell + 1] == WALL) as i32
            + (map.map[cell - 1] == WALL) as i32
            + (map.map[cell + w] == WALL) as i32
            + (map.map[cell - w] == WALL) as i32;
        if Random::int_max(6) <= walls {
            map.map[cell] = EMPTY_DECO;
        }
    }
}

fn generate_gold(map: &mut TerrainMap, rooms: &[Room], order: &mut [usize], gold: &mut i32) {
    while *gold > 0 {
        Random::shuffle_list(order);
        for &room_index in order.iter() {
            if rooms[room_index].kind == RoomKind::Secret {
                continue;
            }
            let room = &rooms[room_index];
            let candidates = gold_pos_candidates(map, room, *gold);
            let Some(&cell) = Random::element(&candidates) else {
                continue;
            };
            map.map[cell] = WALL_DECO;
            *gold -= 1;
            if *gold > 0 {
                let offsets = [-map.width as isize, -1, 1, map.width as isize];
                let next = cell as isize + offsets[Random::int_max(4) as usize];
                if inside_map(map, next) && map.map[next as usize] == WALL {
                    map.map[next as usize] = WALL_DECO;
                    *gold -= 1;
                }
                if Random::int_max(2) == 0 {
                    let next = cell as isize + offsets[Random::int_max(4) as usize];
                    if inside_map(map, next) && map.map[next as usize] == WALL {
                        map.map[next as usize] = WALL_DECO;
                        *gold -= 1;
                    }
                }
            }
        }
    }
}

fn gold_pos_candidates(map: &TerrainMap, room: &Room, gold: i32) -> Vec<usize> {
    let mut candidates = Vec::new();
    for x in room.left..=room.right {
        for y in room.top..=room.bottom {
            let cell = map.point_to_cell(x, y).unwrap();
            if gold > 0
                && inside_map(map, cell as isize)
                && map.map[cell] == WALL
                && cardinal_open_neighbour_inside_room(map, room, cell)
            {
                candidates.push(cell);
            }
        }
    }
    candidates
}

fn cardinal_open_neighbour_inside_room(map: &TerrainMap, room: &Room, cell: usize) -> bool {
    let w = map.width as isize;
    for offset in [-w, -1, 1, w] {
        let next = cell as isize + offset;
        if inside_map(map, next) && map.map[next as usize] != WALL {
            let point = cell_point(map, next as usize);
            // SPD `Room.inside` excludes the 1-tile perimeter.
            if point.x > room.left
                && point.y > room.top
                && point.x < room.right
                && point.y < room.bottom
            {
                return true;
            }
        }
    }
    false
}

fn inside_map(map: &TerrainMap, cell: isize) -> bool {
    let w = map.width as isize;
    let len = map.len() as isize;
    !(cell < w || cell >= len - w || cell % w == 0 || cell % w == w - 1)
}

fn cell_point(map: &TerrainMap, cell: usize) -> Point {
    let w = map.width as usize;
    Point::new(
        (cell % w) as i32 + map.origin_x,
        (cell / w) as i32 + map.origin_y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blank_map(width: i32, height: i32) -> TerrainMap {
        let len = (width * height) as usize;
        TerrainMap {
            width,
            height,
            origin_x: 0,
            origin_y: 0,
            map: vec![WALL; len],
            passable: vec![false; len],
            water_allowed: vec![true; len],
            grass_allowed: vec![true; len],
            trap_allowed: vec![true; len],
            item_allowed: vec![true; len],
            character_allowed: vec![true; len],
            mob_occupied: vec![false; len],
            plant_occupied: vec![false; len],
            known_plants: vec![None; len],
            known_mobs: vec![None; len],
            heap_occupied: vec![false; len],
            known_heaps: vec![None; len],
            known_blobs: Vec::new(),
            trap_destroys_items: vec![false; len],
            trap_names: vec![None; len],
            branch_exits: Vec::new(),
            branch_entrances: Vec::new(),
            custom_tiles: Vec::new(),
            custom_terrain: Vec::new(),
            custom_walls: Vec::new(),
        }
    }

    fn placed_room(
        name: &str,
        kind: RoomKind,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    ) -> Room {
        let mut room = Room::new(0, name, kind, 1, 16, 4, 14, 4, 14);
        room.left = left;
        room.top = top;
        room.right = right;
        room.bottom = bottom;
        room
    }

    fn set_tile(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
        let cell = map.point_to_cell(x, y).unwrap();
        map.map[cell] = terrain;
    }

    fn fill_interior(map: &mut TerrainMap, room: &Room, terrain: i32) {
        for x in room.left + 1..room.right {
            for y in room.top + 1..room.bottom {
                set_tile(map, x, y, terrain);
            }
        }
    }

    fn wall_deco_cells(map: &TerrainMap) -> Vec<usize> {
        map.map
            .iter()
            .enumerate()
            .filter(|(_, &tile)| tile == WALL_DECO)
            .map(|(cell, _)| cell)
            .collect()
    }

    #[test]
    fn gold_candidates_need_an_open_neighbour_inside_the_room() {
        let mut map = blank_map(12, 12);
        let room = placed_room("MineSmallRoom", RoomKind::Standard, 3, 3, 9, 9);
        fill_interior(&mut map, &room, EMPTY);
        // Left perimeter wall whose only opening faces out of the room.
        set_tile(&mut map, 4, 6, WALL);
        set_tile(&mut map, 2, 6, EMPTY);

        let candidates = gold_pos_candidates(&map, &room, 1);
        let outward = map.point_to_cell(3, 6).unwrap();
        let inward = map.point_to_cell(3, 5).unwrap();
        assert!(
            !candidates.contains(&outward),
            "wall that only faces out-of-room empty must not be a candidate"
        );
        assert!(candidates.contains(&inward));
    }

    #[test]
    fn generate_gold_places_only_on_walls_with_in_room_open_neighbours() {
        let mut map = blank_map(12, 12);
        let room = placed_room("MineSmallRoom", RoomKind::Standard, 3, 3, 9, 9);
        fill_interior(&mut map, &room, EMPTY);
        set_tile(&mut map, 4, 6, WALL);
        set_tile(&mut map, 2, 6, EMPTY);
        let outward = map.point_to_cell(3, 6).unwrap();

        Random::push_generator_seeded(1);
        let mut gold = 1;
        generate_gold(&mut map, std::slice::from_ref(&room), &mut [0], &mut gold);
        Random::pop_generator();

        assert_eq!(gold, 0);
        assert_ne!(map.map[outward], WALL_DECO);
        let placed = wall_deco_cells(&map);
        assert_eq!(placed.len(), 1);
        assert!(cardinal_open_neighbour_inside_room(&map, &room, placed[0]));
    }

    #[test]
    fn generate_gold_skips_secret_rooms() {
        let mut map = blank_map(12, 18);
        let mut standard = placed_room("MineSmallRoom", RoomKind::Standard, 3, 3, 9, 9);
        let mut secret = placed_room("MineSecretRoom", RoomKind::Secret, 3, 11, 9, 16);
        standard.id = 0;
        secret.id = 1;
        fill_interior(&mut map, &standard, EMPTY);
        fill_interior(&mut map, &secret, EMPTY);
        let rooms = vec![standard, secret];

        Random::push_generator_seeded(2);
        let mut gold = 1;
        generate_gold(&mut map, &rooms, &mut [0, 1], &mut gold);
        Random::pop_generator();

        let secret = &rooms[1];
        for x in secret.left..=secret.right {
            for y in secret.top..=secret.bottom {
                let cell = map.point_to_cell(x, y).unwrap();
                assert_ne!(map.map[cell], WALL_DECO);
            }
        }
        assert!(wall_deco_cells(&map)
            .iter()
            .any(|&cell| cardinal_open_neighbour_inside_room(&map, &rooms[0], cell)));
    }
}
