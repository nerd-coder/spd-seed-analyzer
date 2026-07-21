use crate::level::painter::{place_doors_for_room, DoorMap};
use crate::level::terrain::{
    self, BOOKSHELF, EMPTY, EMPTY_SP, ENTRANCE_SP, EXIT, REGION_DECO, REGION_DECO_ALT, STATUE_SP,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::paint;

fn room(name: &str, kind: RoomKind, size_factor: i32, width: i32, height: i32) -> Room {
    let mut room = Room::new(0, name, kind, size_factor, 16, 4, 18, 4, 18);
    room.left = 1;
    room.top = 1;
    room.right = room.left + width - 1;
    room.bottom = room.top + height - 1;
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
fn hallway_entrance_replaces_center_detail_without_extra_center_rolls() {
    Random::push_generator_seeded(0xC17_11A11);
    let entrance = room("HallwayEntranceRoom", RoomKind::Entrance, 1, 10, 8);
    let (rooms, doors) = with_east_connection(entrance);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 18).is_some());
    let next = Random::int();
    Random::pop_generator();

    assert_eq!(next, -702_599_100);
    assert!(map.map.contains(&ENTRANCE_SP));
    assert!(!map.map.contains(&STATUE_SP));
    assert!(!map.map.contains(&REGION_DECO_ALT));
    assert!(map.map.iter().filter(|&&tile| tile == EMPTY_SP).count() >= 9);
}

#[test]
fn library_hall_exit_preserves_orientation_then_center_rng_order() {
    Random::push_generator_seeded(0x11B_AA11);
    let exit = room("LibraryHallExitRoom", RoomKind::Exit, 2, 12, 12);
    let (rooms, doors) = with_east_connection(exit);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 18).is_some());
    let next = Random::int();
    Random::pop_generator();

    assert_eq!(next, 1_211_676_499);
    assert!(map.map.contains(&BOOKSHELF));
    assert!(map.map.contains(&REGION_DECO));
    let exit_cell = map.map.iter().position(|&tile| tile == EXIT).expect("exit");
    assert!(!map.character_allowed[exit_cell]);
}

#[test]
fn library_ring_entrance_carves_from_center_to_outer_empty_ring() {
    Random::push_generator_seeded(0x11B_21A6);
    let entrance = room("LibraryRingEntranceRoom", RoomKind::Entrance, 2, 14, 13);
    let (rooms, doors) = with_east_connection(entrance);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 18).is_some());
    let next = Random::int();
    Random::pop_generator();

    assert_eq!(next, -117_911_075);
    assert!(map.map.contains(&BOOKSHELF));
    assert!(map.map.contains(&ENTRANCE_SP));
    assert!(map.map.contains(&EMPTY_SP));
    let transition = map
        .map
        .iter()
        .position(|&tile| tile == ENTRANCE_SP)
        .expect("entrance");
    let width = map.width as usize;
    assert!([
        transition - 1,
        transition + 1,
        transition - width,
        transition + width
    ]
    .into_iter()
    .any(|cell| map.map[cell] == EMPTY_SP));
}

#[test]
fn giant_library_ring_uses_even_dimensions_and_cross_openings() {
    Random::push_generator_seeded(0x61A67);
    let giant = room("LibraryRingRoom", RoomKind::Standard, 3, 16, 18);
    let (rooms, doors) = with_east_connection(giant);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 18).is_some());
    Random::pop_generator();

    let center_x = (rooms[0].left + rooms[0].right) / 2;
    let center_y = (rooms[0].top + rooms[0].bottom) / 2;
    for point in [(center_x - 4, center_y), (center_x, center_y - 4)] {
        let cell = map.point_to_cell(point.0, point.1).expect("cross cell");
        assert_eq!(map.map[cell], EMPTY);
    }
}

#[test]
fn statues_exit_burns_transition_center_rolls_and_blocks_exit() {
    Random::push_generator_seeded(0x57A_7E5);
    let exit = room("StatuesExitRoom", RoomKind::Exit, 2, 14, 12);
    let (rooms, doors) = with_east_connection(exit);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 18).is_some());
    let next = Random::int();
    Random::pop_generator();

    assert_eq!(next, 943_703_656);
    assert!(map.map.contains(&STATUE_SP));
    let exit_cell = map.map.iter().position(|&tile| tile == EXIT).expect("exit");
    assert!(!map.character_allowed[exit_cell]);
    assert!(map.is_solid(
        map.map
            .iter()
            .position(|&tile| tile == STATUE_SP)
            .expect("statue")
    ));
}

#[test]
fn statues_even_pedestal_coordinates_burn_separate_rolls() {
    Random::push_generator_seeded(0x57A_7E5);
    let statues = room("StatuesRoom", RoomKind::Standard, 1, 10, 10);
    let (rooms, doors) = with_east_connection(statues);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 18).is_some());
    let next = Random::int();
    Random::pop_generator();

    assert_eq!(next, 943_703_656);
    assert!(map.map.contains(&REGION_DECO_ALT));
}

#[test]
fn city_variant_families_paint_entrances_and_protect_exits() {
    let cases = [
        ("HallwayExitRoom", RoomKind::Exit, 1, 9, 9, EXIT),
        (
            "LibraryHallEntranceRoom",
            RoomKind::Entrance,
            2,
            12,
            12,
            crate::level::terrain::ENTRANCE,
        ),
        ("LibraryRingExitRoom", RoomKind::Exit, 2, 14, 13, EXIT),
        (
            "StatuesEntranceRoom",
            RoomKind::Entrance,
            1,
            9,
            9,
            ENTRANCE_SP,
        ),
    ];
    for (index, (name, kind, size, width, height, transition)) in cases.into_iter().enumerate() {
        Random::push_generator_seeded(0xC17_000 + index as i64);
        let variant = room(name, kind, size, width, height);
        let (rooms, doors) = with_east_connection(variant);
        let mut map = terrain::paint_minimal(&rooms).expect("map");
        assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 18).is_some());
        Random::pop_generator();

        let cell = map
            .map
            .iter()
            .position(|&tile| tile == transition)
            .unwrap_or_else(|| panic!("missing transition for {name}"));
        if kind == RoomKind::Exit {
            assert!(!map.character_allowed[cell], "unprotected exit for {name}");
        }
    }
}

#[test]
fn segmented_library_uses_bookshelf_walls_and_single_cell_gaps() {
    Random::push_generator_seeded(0x5E6_11B);
    let library = room("SegmentedLibraryRoom", RoomKind::Standard, 2, 14, 13);
    let (rooms, doors) = with_east_connection(library);
    let door = doors.get(0, 1).expect("door");
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms, &rooms[0], 0, &doors, 18).is_some());
    let next = Random::int();
    Random::pop_generator();

    assert_eq!(next, 1_386_148_211);
    assert!(map.map.contains(&BOOKSHELF));
    assert!(map.map.contains(&EMPTY_SP));
    let first_inside = if door.x == rooms[0].right {
        (door.x - 1, door.y)
    } else {
        (door.x, door.y + 1)
    };
    let cell = map
        .point_to_cell(first_inside.0, first_inside.1)
        .expect("inside door");
    assert_eq!(map.map[cell], EMPTY_SP);
}
