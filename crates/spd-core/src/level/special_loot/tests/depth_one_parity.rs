//! Focused draw-shape and geometry coverage for schema-v3 room fixes.

use super::super::crystal_path;
use super::super::special_rooms::treasury_prizes_on_map;
use super::test_room;
use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{paint_minimal, CRYSTAL_DOOR, STATUE};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;
use crate::run::{dungeon_from_run, init_run};

#[test]
fn crystal_path_pins_constructor_center_and_reward_tail() {
    Random::reset_generators();
    let run = init_run(41);
    Random::push_generator_seeded(111);
    let mut dungeon = dungeon_from_run(run);
    dungeon.depth = 1;
    let mut room = test_room("CrystalPathRoom", 8, 8);
    room.connected.push(1);
    let mut neighbour = Room::new(1, "TunnelRoom", RoomKind::Connection, 1, 16, 3, 10, 3, 10);
    neighbour.left = -4;
    neighbour.top = 2;
    neighbour.right = 0;
    neighbour.bottom = 6;
    neighbour.connected.push(0);
    let rooms = vec![room, neighbour];
    let mut map = paint_minimal(&rooms).expect("test map");
    let mut doors = DoorMap::new();
    doors.insert_test_point(0, 1, Point::new(0, 4));
    let mut spawn = Vec::new();

    let loot = crystal_path::paint(&mut dungeon, &rooms, 0, &mut map, &doors, &mut spawn);
    let tail = Random::peek_ints(4);
    Random::pop_generator();

    assert_eq!(loot.len(), 6);
    assert_eq!(spawn.len(), 3);
    assert!(spawn.iter().all(|item| item.class_name == "CrystalKey"));
    assert_eq!(
        map.map.iter().filter(|&&tile| tile == CRYSTAL_DOOR).count(),
        6
    );
    assert_eq!(map.heap_occupied.iter().filter(|&&cell| cell).count(), 6);
    for y in rooms[0].top..=rooms[0].bottom {
        for x in rooms[0].left..=rooms[0].right {
            let cell = map.point_to_cell(x, y).expect("room cell");
            assert!(!map.water_allowed[cell]);
            assert!(!map.grass_allowed[cell]);
            assert!(!map.trap_allowed[cell]);
        }
    }
    assert_eq!(tail, [-455766648, 1806559391, 1474493840, -1826994191]);
}

#[test]
fn treasury_pins_center_item_before_position_and_small_gold_retries() {
    Random::reset_generators();
    let run = init_run(42);
    Random::push_generator_seeded(103);
    let mut dungeon = dungeon_from_run(run);
    dungeon.depth = 1;
    // Inclusive 8x8 room: center() consumes one Int(2) per axis.
    let room = test_room("TreasuryRoom", 7, 7);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("test map");
    let mut spawn = Vec::new();

    let loot = treasury_prizes_on_map(&mut dungeon, &room, &mut map, &mut spawn);
    let tail = Random::peek_ints(4);
    Random::pop_generator();

    let heap_cells: Vec<_> = map
        .heap_occupied
        .iter()
        .enumerate()
        .filter_map(|(cell, &occupied)| occupied.then_some(cell))
        .collect();
    assert!((2..=3).contains(&loot.len()));
    assert_eq!(map.map.iter().filter(|&&tile| tile == STATUE).count(), 1);
    assert_eq!(heap_cells, [23, 24, 32, 34, 43, 46, 53, 77]);
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "IronKey");
    assert_eq!(tail, [-1243442401, -1100848827, -1111774953, -2009378556]);
}
