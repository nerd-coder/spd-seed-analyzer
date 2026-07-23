use super::super::special_rooms::storage_prizes;
use super::test_room;
use crate::level::terrain::paint_minimal;
use crate::random::Random;
use crate::run::{dungeon_from_run, init_run};

#[test]
fn storage_room_records_each_drop_and_queues_liquid_flame() {
    Random::reset_generators();
    Random::push_generator_seeded(0x570A6E);
    let mut dungeon = dungeon_from_run(init_run(40));
    dungeon.depth = 7;
    let room = test_room("StorageRoom", 8, 8);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("StorageRoom map");
    let mut spawn = Vec::new();

    let loot = storage_prizes(&mut dungeon, &room, &mut map, &mut spawn);
    Random::pop_generator();

    assert!((3..=4).contains(&loot.len()));
    let heaps: Vec<_> = map.known_heaps.iter().flatten().collect();
    assert_eq!(heaps.len(), loot.len());
    assert!(heaps.iter().all(|heap| {
        heap.heap_type == "heap"
            && heap.items.len() == 1
            && heap.items[0].source.as_deref() == Some("StorageRoom")
    }));
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            let cell = map.point_to_cell(x, y).expect("StorageRoom interior");
            assert_eq!(map.map[cell], crate::level::terrain::EMPTY_SP);
        }
    }
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "PotionOfLiquidFlame");
}
