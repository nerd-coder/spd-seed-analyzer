use super::*;

#[test]
fn pool_room_lifecycle_matches_oracle() {
    let fixture = read_fixture(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-1.json"),
    );
    let expected = &fixture.floors[0];
    assert!(expected.rooms.iter().any(|room| room == "PoolRoom"));
    assert_eq!((expected.width, expected.height), (40, 30));
    assert_eq!(expected.pre_paint_rng.len(), 8);
    assert_eq!(expected.pre_mobs_rng.len(), 8);
    assert_eq!(expected.pre_items_rng.len(), 8);

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    dungeon.depth = 1;
    let level = create_level_partial(&mut dungeon);
    assert_eq!(level.pre_paint_rng_probe, expected.pre_paint_rng);
    assert_eq!(level.pre_mobs_rng_probe, expected.pre_mobs_rng);
    assert_eq!(level.pre_items_rng_probe, expected.pre_items_rng);

    let report = analyze_seed(&fixture.input.seed, 1).expect("analyze PoolRoom seed");
    let actual = report.floors[0].map.as_ref().expect("depth-one map");
    let mut actual_rooms = report.floors[0].rooms.clone();
    actual_rooms.sort();
    assert_eq!(actual_rooms, expected.rooms);
    assert_eq!(
        (actual.width, actual.height),
        (expected.width, expected.height)
    );
    let pool_heap = actual.heaps.iter().find(|heap| heap.cell == 315);
    assert_eq!(
        pool_heap.map(|heap| {
            (
                heap.heap_type.as_str(),
                heap.items[0].class_name.as_str(),
                heap.items[0].level,
                heap.items[0].cursed,
            )
        }),
        Some(("chest", "ScaleArmor", 0, false))
    );
    let actual_heaps: Vec<_> = actual
        .heaps
        .iter()
        .map(|heap| OracleHeap {
            cell: heap.cell,
            heap_type: heap.heap_type.clone(),
            items: heap
                .items
                .iter()
                .map(|item| OracleItem {
                    class_name: if item.class_name.ends_with("Seed") {
                        "Seed".into()
                    } else {
                        item.class_name.clone()
                    },
                    quantity: item.quantity,
                    level: item.level,
                    cursed: item.cursed,
                })
                .collect(),
        })
        .collect();
    assert_eq!(actual_heaps, expected.final_heaps);
    let actual_mobs: Vec<_> = actual
        .mobs
        .iter()
        .map(|mob| OracleMob {
            cell: mob.cell,
            class_name: mob.class_name.clone(),
        })
        .collect();
    assert_eq!(actual_mobs, expected.final_mobs);
    assert!(!actual.markers.iter().any(|marker| {
        marker.cell == 315
            && marker.kind == spd_core::report::MapMarkerKind::Item
            && marker.label == "Room loot"
    }));
}
