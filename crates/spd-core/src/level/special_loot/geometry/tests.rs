use super::*;
use crate::geom::Point;
use crate::level::terrain::{self, CHASM, EMPTY, EMPTY_SP, GRASS, HIGH_GRASS, LOCKED_DOOR, WALL};
use crate::random::Random;
use crate::rooms::types::RoomKind;

fn connected_room(name: &str, width: i32, height: i32) -> (Vec<Room>, DoorMap) {
    let mut target = Room::new(
        0,
        name,
        if name == "SecretMazeRoom" {
            RoomKind::Secret
        } else {
            RoomKind::Special
        },
        1,
        1,
        width,
        width,
        height,
        height,
    );
    target.left = 1;
    target.top = 1;
    target.right = width;
    target.bottom = height;

    let mut neighbor = Room::new(
        1,
        "EmptyRoom",
        RoomKind::Standard,
        1,
        16,
        7,
        7,
        height,
        height,
    );
    neighbor.left = width;
    neighbor.top = 1;
    neighbor.right = width + 6;
    neighbor.bottom = height;
    target.connected.push(1);
    neighbor.connected.push(0);

    let mut doors = DoorMap::new();
    doors.insert_test_point(0, 1, Point::new(width, 1 + height / 2));
    (vec![target, neighbor], doors)
}

#[test]
fn secret_maze_paints_full_maze_and_marks_farthest_chest() {
    let (rooms, doors) = connected_room("SecretMazeRoom", 14, 15);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    Random::reset_generators();
    Random::push_generator_seeded(0x5EED);
    let prize_cell = paint(&mut map, &rooms[0], 0, &doors).expect("maze prize cell");
    let tail = Random::int();
    Random::pop_generator();

    let room_wall_count = (rooms[0].left..=rooms[0].right)
        .flat_map(|x| (rooms[0].top..=rooms[0].bottom).map(move |y| (x, y)))
        .filter(|&(x, y)| map.map[map.point_to_cell(x, y).expect("cell")] == WALL)
        .count();
    assert!(room_wall_count > (rooms[0].width() * 2) as usize);
    assert_eq!(map.heap_occupied.iter().filter(|&&v| v).count(), 1);
    assert!(map.heap_occupied[prize_cell]);
    assert_eq!(tail, 295_367_267);
}

#[test]
fn weak_floor_burns_each_row_roll_and_keeps_hidden_well_chasm() {
    let (rooms, doors) = connected_room("WeakFloorRoom", 8, 7);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    Random::reset_generators();
    Random::push_generator_seeded(0xA11CE);
    let _ = paint(&mut map, &rooms[0], 0, &doors);
    let tail = Random::int();
    Random::pop_generator();

    assert!(map.map.contains(&EMPTY_SP));
    assert!(map.map.contains(&CHASM));
    assert_eq!(tail, 43_285_173);
}

#[test]
fn rot_garden_places_heart_and_lashers_without_duplicate_key_rng() {
    let (rooms, doors) = connected_room("RotGardenRoom", 10, 10);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    Random::reset_generators();
    Random::push_generator_seeded(0xB07);
    let _ = paint(&mut map, &rooms[0], 0, &doors);
    let tail = Random::int();
    Random::pop_generator();

    let entrance = map.point_to_cell(10, 6).expect("entrance");
    assert_eq!(map.map[entrance], LOCKED_DOOR);
    assert!(map.map.contains(&HIGH_GRASS));
    let plants = map.mob_occupied.iter().filter(|&&v| v).count();
    assert!((1..=7).contains(&plants));
    let known_plants: Vec<_> = map
        .known_mobs
        .iter()
        .enumerate()
        .filter_map(|(cell, &label)| label.map(|label| (cell, label)))
        .collect();
    assert_eq!(known_plants.len(), plants);
    assert_eq!(
        known_plants
            .iter()
            .filter(|(_, label)| *label == "Rot Heart")
            .count(),
        1
    );
    assert!(known_plants
        .iter()
        .filter(|(_, label)| *label != "Rot Heart")
        .all(|(_, label)| *label == "Rot Lasher"));
    assert!(known_plants
        .iter()
        .all(|&(cell, _)| map.mob_occupied[cell] && map.map[cell] == GRASS));
    assert!(map
        .map
        .iter()
        .zip(&map.mob_occupied)
        .all(|(&tile, &occupied)| !occupied || tile == GRASS));
    assert_eq!(tail, 1_621_242_152);
}

#[test]
fn demon_spawner_burns_center_jitter_and_blocks_ambient_paint() {
    let mut room = Room::new(0, "DemonSpawnerRoom", RoomKind::Special, 1, 1, 6, 6, 8, 8);
    room.left = 1;
    room.top = 1;
    room.right = 6;
    room.bottom = 8;
    let mut map = terrain::paint_minimal(std::slice::from_ref(&room)).expect("map");
    Random::reset_generators();
    Random::push_generator_seeded(0xD3E0);
    let _ = paint(&mut map, &room, 0, &DoorMap::new());
    let tail = Random::int();
    Random::pop_generator();

    assert_eq!(map.mob_occupied.iter().filter(|&&v| v).count(), 1);
    let known_spawner: Vec<_> = map
        .known_mobs
        .iter()
        .enumerate()
        .filter_map(|(cell, &label)| label.map(|label| (cell, label)))
        .collect();
    assert_eq!(known_spawner.len(), 1);
    assert_eq!(known_spawner[0].1, "Demon Spawner");
    assert!(map.mob_occupied[known_spawner[0].0]);
    assert!(map.map.contains(&EMPTY));
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let cell = map.point_to_cell(x, y).expect("cell");
            assert!(!map.water_allowed[cell]);
            assert!(!map.grass_allowed[cell]);
            assert!(!map.trap_allowed[cell]);
        }
    }
    assert_eq!(tail, 581_257_677);
}
