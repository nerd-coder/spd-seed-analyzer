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

#[test]
fn depth_one_final_heaps_characterize_known_analyzer_mismatch() {
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

        assert_eq!(
            signatures(&expected_projection),
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
            "pinned Java report-visible projection in {context}"
        );
        assert_eq!(
            signatures(&actual_projection),
            [
                "Food",
                "LeatherArmor",
                "PotionOfInvisibility",
                "ScrollOfRage",
                "ScrollOfRecharging",
                "StoneOfAggression",
                "StoneOfBlink",
                "StoneOfDeepSleep",
                "ThrowingHammer",
            ],
            "documented current Rust projection in {context}"
        );
        assert_ne!(
            actual_projection, expected_projection,
            "convert this known-mismatch contract to an exact full-fact equality test only after the analyzer retains and matches v3 heap facts in {context}"
        );
        assert_eq!(report.status, "partial", "accuracy status in {context}");
        compared += 1;
    }
    assert!(
        compared > 0,
        "no schema v3 final-heap fixtures were compared"
    );
}
