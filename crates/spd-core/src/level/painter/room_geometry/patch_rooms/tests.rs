use super::*;
use crate::level::painter::{place_doors_for_room, DoorMap};
use crate::level::terrain;

fn room(name: &str, kind: RoomKind) -> Room {
    let mut room = Room::new(0, name, kind, 1, 16, 7, 10, 7, 10);
    room.left = 1;
    room.top = 1;
    room.right = 9;
    room.bottom = 9;
    room
}

#[test]
fn cave_patch_is_deterministic_and_paints_interior_walls() {
    let rooms = vec![room("CaveRoom", RoomKind::Standard)];
    let paint_once = || {
        Random::push_generator_seeded(0xCAFE);
        let mut map = terrain::paint_minimal(&rooms).expect("map");
        assert!(paint(&mut map, &rooms[0], 0, &DoorMap::new()));
        Random::pop_generator();
        map.map
    };

    let first = paint_once();
    let second = paint_once();
    assert_eq!(first, second);
    assert!(first.iter().filter(|&&t| t == WALL).count() > 32);
}

#[test]
fn burned_patch_blocks_later_terrain_and_tracks_burning_traps() {
    Random::push_generator_seeded(0xB0A7);
    let rooms = vec![room("BurnedRoom", RoomKind::Standard)];
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms[0], 0, &DoorMap::new()));
    Random::pop_generator();

    let protected = map.trap_allowed.iter().filter(|&&allowed| !allowed).count();
    assert!(protected > 0);
    assert!(map.trap_names.contains(&Some("BurningTrap")));
    assert!(map
        .trap_destroys_items
        .iter()
        .zip(&map.trap_names)
        .all(|(&destroys, name)| name.is_none() || destroys));
}

#[test]
fn cave_entrance_replaces_minimal_center_transition() {
    Random::push_generator_seeded(77);
    let rooms = vec![room("CaveEntranceRoom", RoomKind::Entrance)];
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    assert!(paint(&mut map, &rooms[0], 0, &DoorMap::new()));
    Random::pop_generator();

    assert_eq!(map.map.iter().filter(|&&t| t == ENTRANCE).count(), 1);
}

#[test]
fn connected_patch_keeps_all_open_cells_reachable() {
    Random::push_generator_seeded(123);
    let mut left = room("ChasmRoom", RoomKind::Standard);
    let mut right = room("CaveRoom", RoomKind::Standard);
    right.id = 1;
    right.left = 9;
    right.right = 17;
    left.connected.push(1);
    right.connected.push(0);
    let rooms = vec![left, right];
    let mut doors = DoorMap::new();
    place_doors_for_room(&rooms, 0, &mut doors);
    let mask = setup_patch(&rooms[0], 0, &doors, 0.45, 1, true);
    Random::pop_generator();

    let door = doors.get(0, 1).expect("door");
    let start = patch_index(&rooms[0], door.x - 1, door.y).expect("inside door");
    assert!(all_open_cells_reachable(&mask, rooms[0].width() - 2, start));
}

#[test]
fn connected_patch_consumes_center_jitter_before_door_start_on_retries() {
    Random::push_generator_seeded(0xCE47);
    let mut left = room("ChasmRoom", RoomKind::Standard);
    left.right += 1;
    left.bottom += 1;
    let mut right = room("CaveRoom", RoomKind::Standard);
    right.id = 1;
    right.left = left.right;
    right.right = right.left + 8;
    right.bottom += 1;
    left.connected.push(1);
    right.connected.push(0);
    let rooms = vec![left, right];
    let mut doors = DoorMap::new();
    place_doors_for_room(&rooms, 0, &mut doors);
    let _ = setup_patch(&rooms[0], 0, &doors, 0.72, 3, true);
    let next = Random::long();
    Random::pop_generator();

    assert_eq!(next, 5_706_832_454_276_186_911);
}
