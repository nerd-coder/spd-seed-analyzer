use super::*;

fn report_blacklists(class_name: &str) -> bool {
    matches!(
        class_name,
        "Gold"
            | "Dewdrop"
            | "IronKey"
            | "GoldenKey"
            | "CrystalKey"
            | "EnergyCrystal"
            | "CorpseDust"
            | "Embers"
            | "CeremonialCandle"
            | "Pickaxe"
    )
}

fn signatures(items: &[ComparableItem]) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            if item.cursed {
                format!("cursed {}", item.class_name)
            } else {
                item.class_name.clone()
            }
        })
        .collect()
}

fn assert_aaa_regression_facts(
    fixture: &OracleFixture,
    floor: &OracleFloor,
    projection: &[ComparableItem],
    context: &impl std::fmt::Display,
) {
    if fixture.input.seed != "AAA-AAA-AAA" {
        return;
    }

    assert_eq!(
        (floor.width, floor.height),
        (40, 30),
        "pinned AAA Java map bounds in {context}"
    );
    assert_eq!(
        floor.final_mobs,
        [
            OracleMob {
                cell: 175,
                class_name: "Rat".into()
            },
            OracleMob {
                cell: 314,
                class_name: "Piranha".into()
            },
            OracleMob {
                cell: 404,
                class_name: "Snake".into()
            },
            OracleMob {
                cell: 432,
                class_name: "Piranha".into()
            },
            OracleMob {
                cell: 436,
                class_name: "Piranha".into()
            },
            OracleMob {
                cell: 497,
                class_name: "Rat".into()
            },
            OracleMob {
                cell: 524,
                class_name: "Rat".into()
            },
            OracleMob {
                cell: 738,
                class_name: "Rat".into()
            },
            OracleMob {
                cell: 752,
                class_name: "Rat".into()
            },
            OracleMob {
                cell: 778,
                class_name: "Snake".into()
            },
            OracleMob {
                cell: 902,
                class_name: "Rat".into()
            },
        ],
        "pinned AAA Java final mobs in {context}"
    );
    assert_eq!(
        signatures(projection),
        [
            "Food",
            "PotionOfHealing",
            "PotionOfInvisibility",
            "ScaleArmor",
            "ScrollOfRage",
            "ScrollOfRecharging",
            "StoneOfAggression",
            "StoneOfBlink",
        ],
        "pinned AAA Java report-visible projection in {context}"
    );
}

#[test]
fn depth_one_final_heaps_match_report_projection() {
    let mut compared = 0;
    for path in fixture_paths() {
        let fixture = read_fixture(&path);
        if fixture.schema_version != FINAL_HEAPS_SCHEMA_VERSION {
            continue;
        }
        let context = path.display();
        assert_eq!(
            fixture.contract.as_deref(),
            Some("final_placed_heaps"),
            "schema v3 contract in {context}"
        );
        assert_eq!(fixture.input.depths, [1], "requested depths in {context}");
        assert_eq!(fixture.floors.len(), 1, "floor count in {context}");
        let expected_floor = &fixture.floors[0];
        assert_eq!(expected_floor.depth, 1, "floor depth in {context}");
        assert!(
            expected_floor.width > 0 && expected_floor.height > 0,
            "pinned Java map has positive bounds in {context}"
        );
        assert!(
            !expected_floor.rooms.is_empty()
                && expected_floor
                    .rooms
                    .windows(2)
                    .all(|pair| pair[0] <= pair[1]),
            "pinned Java rooms are non-empty and sorted in {context}"
        );
        assert_eq!(
            expected_floor.pre_paint_rng.len(),
            8,
            "pre-paint RNG probe length in {context}"
        );
        assert_eq!(
            expected_floor.pre_mobs_rng.len(),
            8,
            "pre-mobs RNG probe length in {context}"
        );
        assert_eq!(
            expected_floor.pre_items_rng.len(),
            8,
            "RNG probe length in {context}"
        );
        assert!(
            expected_floor.forced_items.is_empty(),
            "schema v3 does not reuse the schema v2 forced_items field in {context}"
        );
        assert!(
            !expected_floor.final_heaps.is_empty(),
            "final heap snapshot must not be empty in {context}"
        );
        assert!(
            expected_floor
                .final_heaps
                .windows(2)
                .all(|pair| pair[0].cell < pair[1].cell),
            "final heaps must be strictly ordered by cell in {context}"
        );
        for heap in &expected_floor.final_heaps {
            assert!(
                matches!(
                    heap.heap_type.as_str(),
                    "heap"
                        | "for_sale"
                        | "chest"
                        | "locked_chest"
                        | "crystal_chest"
                        | "tomb"
                        | "skeleton"
                        | "remains"
                ),
                "pinned Heap.Type in {context}: {}",
                heap.heap_type
            );
            assert!(!heap.items.is_empty(), "empty final heap in {context}");
            assert!(
                heap.items
                    .iter()
                    .all(|item| !item.class_name.is_empty() && item.quantity > 0),
                "valid final heap item facts in {context}"
            );
        }

        // The current public analyzer report does not retain quantity, level,
        // heap type, or an item-to-cell relation. Compare its strongest honest
        // projection (report-visible class + curse), while the fixture above
        // still validates and preserves every v3 placement fact.
        let mut expected_projection: Vec<_> = expected_floor
            .final_heaps
            .iter()
            .flat_map(|heap| &heap.items)
            .filter(|item| !report_blacklists(&item.class_name))
            .map(|item| ComparableItem {
                class_name: item.class_name.clone(),
                cursed: item.cursed,
            })
            .collect();
        expected_projection.sort();

        let report = analyze_seed(&fixture.input.seed, 1)
            .unwrap_or_else(|error| panic!("failed to analyze seed in {context}: {error}"));
        let mut actual_rooms = report.floors[0].rooms.clone();
        actual_rooms.sort();
        assert_eq!(
            actual_rooms, expected_floor.rooms,
            "depth-one room classes in {context}"
        );
        let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
        dungeon.depth = 1;
        let level = create_level_partial(&mut dungeon);
        assert_eq!(
            level.pre_paint_rng_probe, expected_floor.pre_paint_rng,
            "pre-painter RNG boundary in {context}"
        );
        assert_eq!(
            level.pre_mobs_rng_probe, expected_floor.pre_mobs_rng,
            "pre-createMobs RNG boundary in {context}"
        );
        assert_eq!(
            level.pre_items_rng_probe, expected_floor.pre_items_rng,
            "pre-createItems RNG boundary in {context}"
        );
        let map = report.floors[0].map.as_ref().expect("depth-one map");
        assert_eq!(
            (map.width, map.height),
            (expected_floor.width, expected_floor.height),
            "Rust map bounds in {context}"
        );
        let actual_item_cells: Vec<_> = map
            .markers
            .iter()
            .filter(|marker| marker.kind == spd_core::report::MapMarkerKind::Item)
            .map(|marker| marker.cell)
            .collect();
        let expected_item_cells: Vec<_> = expected_floor
            .final_heaps
            .iter()
            .map(|heap| heap.cell)
            .collect();
        assert_eq!(
            actual_item_cells, expected_item_cells,
            "depth-one final heap cells in {context}"
        );
        let mut actual_mobs: Vec<_> = report.floors[0]
            .map
            .as_ref()
            .expect("depth-one regular floor has a map")
            .markers
            .iter()
            .filter(|marker| marker.kind == spd_core::report::MapMarkerKind::Mob)
            .map(|marker| OracleMob {
                cell: marker.cell,
                class_name: marker.label.clone(),
            })
            .collect();
        actual_mobs.sort();
        assert_eq!(
            actual_mobs, expected_floor.final_mobs,
            "depth-one final mobs in {context}"
        );
        let mut actual_projection: Vec<_> = report.floors[0]
            .items
            .iter()
            .map(|item| ComparableItem {
                class_name: item
                    .class_name
                    .clone()
                    .expect("all analyzed items have a Java class name"),
                cursed: item.cursed,
            })
            .collect();
        actual_projection.sort();

        assert_aaa_regression_facts(&fixture, expected_floor, &expected_projection, &context);
        assert_eq!(
            actual_projection, expected_projection,
            "report-visible item projection in {context}"
        );
        assert_eq!(report.status, "partial", "accuracy status in {context}");
        compared += 1;
    }
    assert!(
        compared >= 4,
        "expected at least four schema v3 final-heap fixtures, compared {compared}"
    );
}
