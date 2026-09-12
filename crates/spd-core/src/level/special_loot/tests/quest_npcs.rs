use super::super::quest_rooms::blacksmith_room_prizes;
use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{
    paint_minimal, CUSTOM_DECO_WTR, EMPTY, EMPTY_SP, EXIT, PEDESTAL, TRAP,
};
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
    assert!(!map.map.contains(&TRAP));

    let npc = map.point_to_cell(3, 3).expect("NPC cell");
    assert!(map.mob_occupied[npc]);
    assert_eq!(map.known_mobs[npc], Some("Blacksmith"));
    assert_eq!(
        map.known_mobs.iter().flatten().copied().collect::<Vec<_>>(),
        ["Blacksmith"]
    );

    let furnace = map.point_to_cell(4, 3).expect("furnace cell");
    assert_eq!(map.map[furnace], CUSTOM_DECO_WTR);
    let left_pedestal = map.point_to_cell(3, 5).expect("left pedestal");
    let right_pedestal = map.point_to_cell(4, 5).expect("right pedestal");
    assert_eq!(map.map[left_pedestal], PEDESTAL);
    assert_eq!(map.map[right_pedestal], PEDESTAL);
    assert!(map.heap_occupied[left_pedestal] && map.heap_occupied[right_pedestal]);

    let left_exit = map.point_to_cell(1, 1).expect("left entrance");
    let right_exit = map.point_to_cell(6, 1).expect("right entrance");
    assert!(map.map[left_exit] == EXIT || map.map[right_exit] == EXIT);
    assert_eq!(map.branch_exits.len(), 1);

    assert_eq!(
        map.custom_tiles
            .iter()
            .map(|tile| tile.class_name.as_str())
            .collect::<Vec<_>>(),
        ["QuestEntrance", "SmithyVisuals"]
    );
    assert_eq!(
        map.custom_tiles[1].static_data,
        [7, 16, 17, 10, 8, -1, 18, 10, 8, -1, -1, 10, 12, 20, 20, 14]
    );
    assert_eq!(map.custom_walls.len(), 1);
    assert_eq!(map.custom_walls[0].class_name, "FurnaceOverhang");
    assert_eq!(map.custom_walls[0].static_data, [3]);

    for y in 0..=7 {
        for x in 0..=7 {
            let cell = map.point_to_cell(x, y).expect("room cell");
            assert!(!map.character_allowed[cell]);
            assert_eq!(map.item_allowed[cell], map.map[cell] == EMPTY);
        }
    }
}
