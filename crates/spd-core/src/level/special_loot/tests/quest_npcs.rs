use super::super::quest_rooms::blacksmith_room_prizes;
use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{paint_minimal, EMPTY, EMPTY_SP, EXIT, TRAP};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;
use crate::run::{dungeon_from_run, init_run};

#[test]
fn blacksmith_paints_heap_cells_and_npc_before_ambient_mobs() {
    let mut smith = Room::new(0, "BlacksmithRoom", RoomKind::Standard, 1, 16, 8, 8, 8, 8);
    smith.left = 0;
    smith.top = 0;
    smith.right = 7;
    smith.bottom = 7;
    smith.connected.push(1);

    let mut tunnel = Room::new(1, "TunnelRoom", RoomKind::Connection, 1, 16, 3, 8, 3, 8);
    tunnel.left = -3;
    tunnel.top = 2;
    tunnel.right = 0;
    tunnel.bottom = 5;
    tunnel.connected.push(0);

    let rooms = vec![smith, tunnel];
    let mut map = paint_minimal(&rooms).expect("test map");
    let mut doors = DoorMap::new();
    doors.insert_test_point(0, 1, Point::new(0, 3));
    let mut dungeon = dungeon_from_run(init_run(42));
    dungeon.depth = 12;

    Random::reset_generators();
    Random::push_generator_seeded(1234);
    let loot = blacksmith_room_prizes(&mut dungeon, &rooms, 0, &mut map, &doors);
    Random::pop_generator();

    let first_corridor = map.point_to_cell(1, 3).expect("first corridor cell");
    let second_corridor = map.point_to_cell(2, 3).expect("second corridor cell");
    assert_eq!(map.map[first_corridor], EMPTY);
    assert_eq!(map.map[second_corridor], EMPTY_SP);
    assert_eq!(loot.len(), 2);
    assert_eq!(
        map.mob_occupied
            .iter()
            .filter(|&&occupied| occupied)
            .count(),
        1
    );
    assert_eq!(
        map.known_mobs.iter().flatten().copied().collect::<Vec<_>>(),
        ["Blacksmith"]
    );
    assert!(map.map.contains(&EXIT));
    assert!(map.map.contains(&TRAP));
}
