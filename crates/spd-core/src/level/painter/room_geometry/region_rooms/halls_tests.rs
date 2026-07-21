use super::*;
use crate::level::painter::{place_doors_for_room, DoorMap};
use crate::level::terrain::{self, EMBERS, EMPTY, ENTRANCE, EXIT, PEDESTAL, STATUE, WALL};
use crate::rooms::types::RoomKind;

fn room(id: usize, name: &str, kind: RoomKind, left: i32, width: i32, height: i32) -> Room {
    let mut room = Room::new(id, name, kind, 2, 16, width, width, height, height);
    room.left = left;
    room.top = 1;
    room.right = left + width - 1;
    room.bottom = height;
    room
}

#[test]
fn skulls_paints_elliptical_floor_statues_and_wall_core() {
    let mut skulls = room(0, "SkullsRoom", RoomKind::Standard, 1, 14, 14);
    let mut neighbour = room(1, "EmptyRoom", RoomKind::Standard, 14, 9, 14);
    skulls.connected.push(1);
    neighbour.connected.push(0);
    let rooms = vec![skulls, neighbour];
    Random::push_generator_seeded(0x5A11);
    let mut doors = DoorMap::new();
    place_doors_for_room(&rooms, 0, &mut doors);
    let mut map = terrain::paint_minimal(&rooms).expect("map");
    let result = paint(&mut map, &rooms, &rooms[0], 0, &doors, 23).expect("skulls");
    Random::pop_generator();

    assert!(result.center_loot.is_none());
    assert!(map.map.contains(&EMPTY));
    assert!(map.map.contains(&STATUE));
    let center = rooms[0].as_rect().center_deterministic();
    assert_eq!(
        map.map[map.point_to_cell(center.x, center.y).expect("center")],
        WALL
    );
    let door = doors.get(0, 1).expect("door");
    assert_eq!(
        map.map[map.point_to_cell(door.x - 3, door.y).expect("corridor")],
        EMPTY
    );
}

#[test]
fn ritual_paints_altar_and_reports_only_standard_center_loot() {
    let standard = room(0, "RitualRoom", RoomKind::Standard, 1, 9, 9);
    Random::push_generator_seeded(0x7110);
    let mut map = terrain::paint_minimal(std::slice::from_ref(&standard)).expect("map");
    let result = paint(
        &mut map,
        std::slice::from_ref(&standard),
        &standard,
        0,
        &DoorMap::new(),
        23,
    )
    .expect("ritual");
    Random::pop_generator();

    let center = result.center_loot.expect("standard ritual prize");
    assert_eq!(map.map.iter().filter(|&&tile| tile == STATUE).count(), 8);
    assert_eq!(map.map.iter().filter(|&&tile| tile == EMBERS).count(), 8);
    assert_eq!(map.map.iter().filter(|&&tile| tile == PEDESTAL).count(), 1);
    assert_eq!(
        map.map[map.point_to_cell(center.x, center.y).expect("center")],
        PEDESTAL
    );
}

#[test]
fn ritual_variants_replace_loot_with_protected_transitions() {
    for (name, kind, terrain) in [
        ("RitualEntranceRoom", RoomKind::Entrance, ENTRANCE),
        ("RitualExitRoom", RoomKind::Exit, EXIT),
    ] {
        let room = room(0, name, kind, 1, 10, 12);
        Random::push_generator_seeded(0xE117);
        let mut map = terrain::paint_minimal(std::slice::from_ref(&room)).expect("map");
        let result = paint(
            &mut map,
            std::slice::from_ref(&room),
            &room,
            0,
            &DoorMap::new(),
            23,
        )
        .expect("ritual variant");
        Random::pop_generator();

        assert!(result.center_loot.is_none());
        assert_eq!(map.map.iter().filter(|&&tile| tile == terrain).count(), 1);
        if kind == RoomKind::Exit {
            let cell = map.map.iter().position(|&tile| tile == EXIT).expect("exit");
            assert!(!map.character_allowed[cell]);
        }
    }
}
