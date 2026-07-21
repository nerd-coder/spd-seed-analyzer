use crate::level::painter::{place_doors_for_room, DoorMap};
use crate::level::terrain::{self, CHASM, EMPTY_SP, ENTRANCE, REGION_DECO_ALT};
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
    assert!(paint(&mut map, &rooms[0], 0, &doors));
    Random::pop_generator();

    assert!(map.map.contains(&REGION_DECO_ALT));
    assert!(map.map.contains(&EMPTY_SP));
    assert!(map.map.contains(&ENTRANCE));
    for (cell, &terrain) in map.map.iter().enumerate() {
        if terrain == REGION_DECO_ALT {
            assert!(!map.item_allowed[cell]);
        }
    }
}

#[test]
fn caves_fissure_paints_bridged_chasms_and_safe_entrance() {
    Random::push_generator_seeded(0xCAFE5);
    let room = room("CavesFissureEntranceRoom", RoomKind::Entrance, 2, 13);
    let (rooms, doors) = with_east_connection(room);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms[0], 0, &doors));
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
    assert!(paint(&mut pit_map, &pit_rooms[0], 0, &pit_doors));
    assert!(pit_map.map.contains(&CHASM));

    let entrance = room("CircleWallEntranceRoom", RoomKind::Entrance, 2, 13);
    let (entrance_rooms, entrance_doors) = with_east_connection(entrance);
    let mut entrance_map = terrain::paint_minimal(&entrance_rooms).expect("map");
    assert!(paint(
        &mut entrance_map,
        &entrance_rooms[0],
        0,
        &entrance_doors
    ));
    Random::pop_generator();

    assert!(entrance_map.map.contains(&ENTRANCE));
}
