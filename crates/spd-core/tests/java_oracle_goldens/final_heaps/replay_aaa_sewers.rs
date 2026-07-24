use super::*;

use std::ffi::OsStr;

#[test]
fn aaa_replay_pins_floor_two_and_floor_three_painter_boundary() {
    let fixtures: Vec<_> = (2..=4)
        .map(|depth| {
            let name = format!("aaa-aaa-aaa-final-heaps-floor-{depth}.json");
            let path = fixture_paths()
                .into_iter()
                .find(|path| {
                    path.file_name()
                        .is_some_and(|file| file == OsStr::new(&name))
                })
                .unwrap_or_else(|| panic!("missing AAA floor-{depth} fixture"));
            read_fixture(&path)
        })
        .collect();

    let mut dungeon = dungeon_from_run(init_run(fixtures[0].input.numeric));
    for depth in 1_u32..=4 {
        dungeon.depth = depth as i32;
        let actual = create_level_partial(&mut dungeon);
        if depth == 1 {
            continue;
        }
        let expected = fixture_and_floor(&fixtures[(depth - 2) as usize], depth);
        let context = format!("AAA floor-{depth}");
        let mut rooms = actual.rooms.clone();
        rooms.sort();
        assert_eq!(rooms, expected.rooms, "{context} room classes");
        let bounds: Vec<_> = actual
            .room_bounds
            .iter()
            .map(|room| OracleRoomFact {
                class_name: room.class_name.clone(),
                left: room.left,
                top: room.top,
                right: room.right,
                bottom: room.bottom,
            })
            .collect();
        assert_eq!(bounds, expected.room_bounds, "{context} room bounds");
        assert_eq!(
            actual.pre_paint_rng_probe, expected.pre_paint_rng,
            "{context} pre-paint RNG"
        );
        if depth == 2 {
            let map = actual.map.as_ref().expect("regular floor map");
            let actual_heaps: Vec<_> = map
                .heaps
                .iter()
                .map(|heap| OracleHeap {
                    cell: heap.cell,
                    heap_type: heap.heap_type.clone(),
                    items: heap
                        .items
                        .iter()
                        .map(|item| OracleItem {
                            class_name: item.class_name.clone(),
                            quantity: item.quantity,
                            level: item.level,
                            cursed: item.cursed,
                        })
                        .collect(),
                })
                .collect();
            assert_eq!(
                seeded_heaps(&actual_heaps),
                seeded_heaps(&expected.final_heaps),
                "{context} exact seeded heaps"
            );
            assert_eq!(
                actual.pre_mobs_rng_probe, expected.pre_mobs_rng,
                "{context} pre-mobs RNG"
            );
            assert_eq!(
                actual.pre_items_rng_probe, expected.pre_items_rng,
                "{context} pre-items RNG"
            );
        } else if depth == 3 {
            let map = actual.map.as_ref().expect("regular floor map");
            let actual_exit = map
                .transitions
                .iter()
                .find(|transition| transition.transition_type == "REGULAR_EXIT")
                .expect("AAA floor-3 regular exit");
            let java_exit = expected
                .transitions
                .as_ref()
                .expect("AAA floor-3 Java transitions")
                .iter()
                .find(|transition| transition.transition_type == "REGULAR_EXIT")
                .expect("AAA floor-3 Java regular exit");
            assert_eq!(
                actual_exit.cell, java_exit.cell,
                "{context} exact regular exit"
            );
            assert_eq!(
                expected.pre_doors_rng,
                [
                    -1107632305,
                    1945287652,
                    -1328999995,
                    -1387751388,
                    -1694303545,
                    -1627736540,
                    2119338483,
                    1053549476
                ],
                "{context} pinned post-room-paint boundary"
            );
            assert_eq!(
                expected.post_doors_rng,
                [
                    -700035324,
                    -240926920,
                    -1024974318,
                    -2119231834,
                    1908727426,
                    344618962,
                    -1144966623,
                    1333384002
                ],
                "{context} pinned post-paintDoors boundary"
            );
            assert_eq!(
                actual.pre_mobs_rng_probe, expected.pre_mobs_rng,
                "{context} pre-mobs RNG"
            );
            assert_eq!(
                actual.pre_items_rng_probe, expected.pre_items_rng,
                "{context} pre-items RNG"
            );
        }
    }
}

fn seeded_heaps(heaps: &[OracleHeap]) -> Vec<&OracleHeap> {
    heaps
        .iter()
        .filter(|heap| heap.items.iter().all(|item| item.class_name != "GuidePage"))
        .collect()
}

fn fixture_and_floor(fixture: &OracleFixture, depth: u32) -> &OracleFloor {
    assert_eq!(fixture.schema_version, FINAL_HEAPS_SCHEMA_VERSION);
    assert_eq!(fixture.contract.as_deref(), Some("final_placed_heaps"));
    assert_eq!(fixture.input.depths, [depth]);
    fixture.floors.first().expect("replay oracle floor")
}
