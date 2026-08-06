//! Mining painter's isolated water, grass, caves decoration, and dark-gold walls.

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
            let mut candidates = Vec::new();
            for x in room.left..=room.right {
                for y in room.top..=room.bottom {
                    let cell = map.point_to_cell(x, y).unwrap();
                    if *gold > 0
                        && map.map[cell] == WALL
                        && cardinal_neighbour_is_not_wall(map, cell)
                    {
                        candidates.push(cell);
                    }
                }
            }
            let Some(&cell) = Random::element(&candidates) else {
                continue;
            };
            map.map[cell] = WALL_DECO;
            *gold -= 1;
            if *gold > 0 {
                let offsets = [-map.width as isize, -1, 1, map.width as isize];
                let next = (cell as isize + offsets[Random::int_max(4) as usize]) as usize;
                if map.map[next] == WALL {
                    map.map[next] = WALL_DECO;
                    *gold -= 1;
                }
                if Random::int_max(2) == 0 {
                    let next = (cell as isize + offsets[Random::int_max(4) as usize]) as usize;
                    if map.map[next] == WALL {
                        map.map[next] = WALL_DECO;
                        *gold -= 1;
                    }
                }
            }
        }
    }
}

fn cardinal_neighbour_is_not_wall(map: &TerrainMap, cell: usize) -> bool {
    let w = map.width as isize;
    [-1, -w, 1, w]
        .iter()
        .any(|offset| map.map[(cell as isize + offset) as usize] != WALL)
}
