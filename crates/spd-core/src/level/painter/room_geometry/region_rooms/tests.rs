use crate::level::painter::{place_doors_for_room, DoorMap};
use crate::level::terrain::{
    self, CHASM, DOOR, EMPTY_SP, ENTRANCE, ENTRANCE_SP, EXIT, REGION_DECO_ALT, WATER,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::paint;

fn room(name: &str, kind: RoomKind, size_factor: i32, side: i32) -> Room {
    let mut room = Room::new(0, name, kind, size_factor, 16, 4, 18, 4, 18);
    room.left = 1;
    room.top = 1;
    room.right = side;
    room.bottom = side;
    room
}

fn with_east_connection(mut room: Room) -> (Vec<Room>, DoorMap) {
    let mut other = Room::new(1, "EmptyRoom", RoomKind::Standard, 1, 16, 4, 10, 4, 10);
    other.left = room.right;
    other.top = room.top + 2;
    other.right = room.right + 5;
    other.bottom = room.bottom - 2;
    room.connected.push(1);
    room.neighbours.push(1);
    other.connected.push(0);
    other.neighbours.push(0);
    let rooms = vec![room, other];
    let mut doors = DoorMap::new();
    place_doors_for_room(&rooms, 0, &mut doors);
    (rooms, doors)
}

#[test]
fn region_deco_bridge_transition_avoids_solid_strip() {
    Random::push_generator_seeded(0xB12D6E);
    let room = room("RegionDecoBridgeEntranceRoom", RoomKind::Entrance, 2, 13);
    let (rooms, doors) = with_east_connection(room);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 13).is_some());
    Random::pop_generator();

    assert!(map.map.contains(&REGION_DECO_ALT));
    assert!(map.map.contains(&EMPTY_SP));
    assert!(map.map.contains(&ENTRANCE));
    for (cell, &terrain) in map.map.iter().enumerate() {
        if terrain == REGION_DECO_ALT {
            assert!(!map.item_allowed[cell]);
            assert!(!map.character_allowed[cell]);
        }
    }
}

#[test]
fn caves_fissure_paints_bridged_chasms_and_safe_entrance() {
    Random::push_generator_seeded(0xCAFE5);
    let room = room("CavesFissureEntranceRoom", RoomKind::Entrance, 2, 13);
    let (rooms, doors) = with_east_connection(room);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 13).is_some());
    Random::pop_generator();

    assert!(map.map.contains(&CHASM));
    assert!(map.map.contains(&EMPTY_SP));
    let entrance = map
        .map
        .iter()
        .position(|&terrain| terrain == ENTRANCE)
        .expect("entrance");
    let width = map.width as usize;
    for neighbour in [
        entrance - width,
        entrance - 1,
        entrance + 1,
        entrance + width,
    ] {
        assert_ne!(map.map[neighbour], CHASM);
    }
}

#[test]
fn cave_circle_rooms_paint_pit_and_center_transition() {
    Random::push_generator_seeded(0xC1AC1E);
    let pit = room("CirclePitRoom", RoomKind::Standard, 3, 17);
    let (pit_rooms, pit_doors) = with_east_connection(pit);
    let mut pit_map = terrain::paint_minimal(&pit_rooms).expect("map");
    assert!(paint(&mut pit_map, &pit_rooms, &pit_rooms[0], 0, &pit_doors, 13).is_some());
    assert!(pit_map.map.contains(&CHASM));

    let entrance = room("CircleWallEntranceRoom", RoomKind::Entrance, 2, 13);
    let (entrance_rooms, entrance_doors) = with_east_connection(entrance);
    let mut entrance_map = terrain::paint_minimal(&entrance_rooms).expect("map");
    assert!(paint(
        &mut entrance_map,
        &entrance_rooms,
        &entrance_rooms[0],
        0,
        &entrance_doors,
        13
    )
    .is_some());
    Random::pop_generator();

    assert!(entrance_map.map.contains(&ENTRANCE));
}

#[test]
fn sewer_pipe_paints_water_paths_and_disables_later_lakes() {
    Random::push_generator_seeded(0x515E);
    let pipe = room("SewerPipeRoom", RoomKind::Standard, 1, 9);
    let (rooms, doors) = with_east_connection(pipe);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 3).is_some());
    Random::pop_generator();

    assert!(map.map.contains(&WATER));
    let water_cell = map
        .map
        .iter()
        .position(|&terrain| terrain == WATER)
        .expect("water cell");
    assert!(map.item_allowed[water_cell]);
    for y in rooms[0].top..=rooms[0].bottom {
        for x in rooms[0].left..=rooms[0].right {
            let cell = map.point_to_cell(x, y).expect("pipe cell");
            assert!(!map.water_allowed[cell]);
        }
    }
}

#[test]
fn sewer_pipe_multi_door_center_burns_both_fractional_float_tests() {
    let mut pipe = room("SewerPipeRoom", RoomKind::Standard, 2, 8);
    pipe.left = 0;
    pipe.top = 0;
    let doors = [crate::geom::Point::new(0, 3), crate::geom::Point::new(8, 5)];

    Random::push_generator_seeded(0x51_50_45);
    let _x_fraction_test = Random::float();
    let _y_fraction_test = Random::float();
    let expected_next = Random::int();
    Random::pop_generator();

    Random::push_generator_seeded(0x51_50_45);
    assert_eq!(
        super::sewer_pipe::connection_center(&pipe, &doors),
        crate::geom::Point::new(4, 4)
    );
    let actual_next = Random::int();
    Random::pop_generator();
    assert_eq!(actual_next, expected_next);
}

#[test]
fn water_bridge_variants_paint_transition_and_placement_masks() {
    Random::push_generator_seeded(0xB41D63);
    let entrance = room("WaterBridgeEntranceRoom", RoomKind::Entrance, 1, 10);
    let (rooms, doors) = with_east_connection(entrance);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 1).is_some());
    Random::pop_generator();

    assert!(map.map.contains(&WATER));
    assert!(map.map.contains(&EMPTY_SP));
    assert!(map.map.contains(&ENTRANCE));
    assert!(map.item_allowed.iter().any(|&allowed| !allowed));
    assert!(map.character_allowed.iter().any(|&allowed| !allowed));
    for y in rooms[0].top..=rooms[0].bottom {
        for x in rooms[0].left..=rooms[0].right {
            let cell = map.point_to_cell(x, y).expect("bridge cell");
            assert!(!map.water_allowed[cell]);
            assert!(!map.trap_allowed[cell]);
        }
    }

    Random::push_generator_seeded(0xE817);
    let exit = room("WaterBridgeExitRoom", RoomKind::Exit, 1, 10);
    let (rooms, doors) = with_east_connection(exit);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 4).is_some());
    Random::pop_generator();
    let exit = map
        .map
        .iter()
        .position(|&terrain| terrain == EXIT)
        .expect("exit");
    assert!(!map.character_allowed[exit]);
}

#[test]
fn ring_variants_paint_center_detail_and_inner_door() {
    Random::push_generator_seeded(0xA11CE);
    let ring = room("RingRoom", RoomKind::Standard, 2, 13);
    let (rooms, doors) = with_east_connection(ring);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    let result = paint(&mut map, &rooms, &rooms[0], 0, &doors, 3).expect("ring paint");
    Random::pop_generator();
    assert!(result.center_loot.is_some());
    assert!(map.map.contains(&REGION_DECO_ALT));
    assert!(map.map.contains(&DOOR));

    Random::push_generator_seeded(0xA11CE);
    let entrance = room("RingEntranceRoom", RoomKind::Entrance, 2, 13);
    let (rooms, doors) = with_east_connection(entrance);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    let result = paint(&mut map, &rooms, &rooms[0], 0, &doors, 3).expect("ring entrance");
    Random::pop_generator();
    assert!(result.center_loot.is_none());
    assert!(map.map.contains(&ENTRANCE_SP));
}

#[test]
fn circle_basin_paints_cross_chasm_patch_and_center_transition() {
    Random::push_generator_seeded(0xBA51);
    let basin = room("CircleBasinEntranceRoom", RoomKind::Entrance, 2, 13);
    let (rooms, doors) = with_east_connection(basin);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 4).is_some());
    Random::pop_generator();

    assert!(map.map.contains(&CHASM));
    assert!(map.map.contains(&WATER));
    let water_cell = map
        .map
        .iter()
        .position(|&terrain| terrain == WATER)
        .expect("water cell");
    assert!(map.item_allowed[water_cell]);
    assert!(map.map.contains(&EMPTY_SP));
    assert!(map.map.contains(&ENTRANCE_SP));
}
