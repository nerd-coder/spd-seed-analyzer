use super::*;

#[test]
fn hkt_floor_eight_fixture_and_partial_structure_are_pinned() {
    let path = fixture_paths()
        .into_iter()
        .find(|path| {
            path.file_name()
                .is_some_and(|name| name == "hkt-jzn-xqq-final-heaps-floor-8.json")
        })
        .expect("HKT floor-8 schema-v3 fixture");
    let fixture = read_fixture(&path);
    let expected = fixture.floors.first().expect("floor-8 oracle facts");
    assert_eq!(fixture.schema_version, FINAL_HEAPS_SCHEMA_VERSION);
    assert_eq!(fixture.contract.as_deref(), Some("final_placed_heaps"));
    assert_eq!(fixture.input.depths, [8]);
    assert_eq!(fixture.floors.len(), 1);
    assert_eq!(expected.depth, 8);
    assert_eq!((expected.width, expected.height), (30, 42));
    assert_eq!(
        expected.rooms,
        [
            "LaboratoryRoom",
            "PerimeterRoom",
            "PerimeterRoom",
            "PerimeterRoom",
            "PerimeterRoom",
            "PerimeterRoom",
            "PillarsRoom",
            "RegionDecoLineEntranceRoom",
            "RegionDecoLineExitRoom",
            "RegionDecoLineRoom",
            "SecretSummoningRoom",
            "SegmentedRoom",
            "SegmentedRoom",
            "SegmentedRoom",
            "WalkwayRoom",
        ]
    );
    assert_eq!(
        expected.pre_paint_rng,
        [
            1_459_946_731,
            372_555_040,
            1_670_478_137,
            1_388_842_747,
            -939_261_660,
            347_828_306,
            -2_091_316_954,
            1_081_201_683,
        ]
    );
    assert_eq!(
        expected.pre_mobs_rng,
        [
            645_369_548,
            1_479_742_776,
            -924_382_995,
            597_987_058,
            -995_259_156,
            -725_190_451,
            -1_975_660_786,
            397_556_436,
        ]
    );
    assert_eq!(
        expected.pre_items_rng,
        [
            827_105_069,
            338_948_646,
            -947_330_788,
            2_046_911_624,
            1_476_440_778,
            -1_424_626_728,
            2_004_907_284,
            -1_895_014_390,
        ]
    );
    assert_eq!(expected.terrain.as_ref().map(Vec::len), Some(30 * 42));
    assert_eq!(expected.discoverable.as_ref().map(Vec::len), Some(30 * 42));
    assert_eq!(expected.tile_variance.as_ref().map(Vec::len), Some(30 * 42));
    assert!(expected.forced_items.is_empty());
    assert_eq!(expected.final_heaps.len(), 11);
    assert!(expected
        .final_heaps
        .windows(2)
        .all(|pair| pair[0].cell < pair[1].cell));
    assert!(expected
        .final_heaps
        .iter()
        .all(|heap| heap.items.len() == 1));
    let heap_facts: Vec<_> = expected
        .final_heaps
        .iter()
        .map(|heap| {
            let item = &heap.items[0];
            (
                heap.cell,
                heap.heap_type.as_str(),
                item.class_name.as_str(),
                item.quantity,
                item.level,
                item.cursed,
            )
        })
        .collect();
    assert_eq!(
        heap_facts,
        [
            (69, "heap", "EnergyCrystal", 5, 0, false),
            (155, "heap", "PotionOfStrength", 1, 0, false),
            (160, "heap", "AlchemyPage", 1, 0, false),
            (161, "heap", "AlchemyPage", 1, 0, false),
            (164, "heap", "ScrollOfIdentify", 1, 0, false),
            (198, "chest", "ScrollOfLullaby", 1, 0, false),
            (431, "heap", "Crossbow", 1, 0, false),
            (700, "skeleton", "PotionOfExperience", 1, 0, false),
            (802, "heap", "IronKey", 1, 0, false),
            (872, "heap", "GuidePage", 1, 0, false),
            (936, "heap", "Food", 1, 0, false),
        ]
    );
    assert_eq!(expected.final_mobs.len(), 7);
    assert!(expected
        .final_mobs
        .windows(2)
        .all(|pair| pair[0].cell < pair[1].cell));
    let mob_facts: Vec<_> = expected
        .final_mobs
        .iter()
        .map(|mob| (mob.cell, mob.class_name.as_str()))
        .collect();
    assert_eq!(
        mob_facts,
        [
            (226, "Guard"),
            (376, "Skeleton"),
            (460, "Skeleton"),
            (464, "DM100"),
            (578, "SpectralNecromancer"),
            (754, "Thief"),
            (847, "Guard"),
        ]
    );
    assert_eq!(expected.transitions.as_ref().map(Vec::len), Some(2));
    assert_eq!(expected.traps.as_ref().map(Vec::len), Some(14));
    assert_eq!(expected.plants.as_ref().map(Vec::len), Some(0));
    assert_eq!(expected.blobs.as_ref().map(Vec::len), Some(1));
    for cells in [
        expected
            .transitions
            .as_ref()
            .expect("floor-8 transitions")
            .iter()
            .map(|transition| transition.cell)
            .collect::<Vec<_>>(),
        expected
            .traps
            .as_ref()
            .expect("floor-8 traps")
            .iter()
            .map(|trap| trap.cell)
            .collect(),
    ] {
        assert!(cells.windows(2).all(|pair| pair[0] < pair[1]));
    }

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    let mut actual = None;
    for depth in 1..=8 {
        dungeon.depth = depth;
        actual = Some(create_level_partial(&mut dungeon));
    }
    let actual = actual.expect("floor-8 Rust facts");
    let map = actual.map.as_ref().expect("floor-8 regular map");
    let mut actual_rooms = actual.rooms.clone();
    actual_rooms.sort();

    assert_eq!(actual.feeling.as_str(), "none", "HKT floor-8 feeling");
    assert_eq!(
        actual.pre_paint_rng_probe, expected.pre_paint_rng,
        "HKT floor-8 pre-paint RNG boundary"
    );
    assert_eq!(actual.pre_mobs_rng_probe.len(), 8);
    assert_eq!(actual.pre_items_rng_probe.len(), 8);
    // Bounds also remain exact, but RNG divergence begins during painting and
    // is visible at the pre-mobs boundary. Do not treat these narrow matches
    // as terrain, entity-cell, or downstream lifecycle parity.
    assert_eq!(
        (map.width, map.height),
        (expected.width, expected.height),
        "HKT floor-8 map bounds"
    );
    assert_eq!(actual_rooms, expected.rooms, "HKT floor-8 room classes");
}
