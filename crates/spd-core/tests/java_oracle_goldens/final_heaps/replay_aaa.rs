use super::*;

use std::ffi::OsStr;

#[test]
fn aaa_replay_pins_floors_six_through_nine_and_matches_through_library() {
    let fixtures: Vec<_> = (6..=9)
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
    for depth in 1_u32..=9 {
        dungeon.depth = depth as i32;
        let actual = create_level_partial(&mut dungeon);
        if depth < 6 {
            continue;
        }
        let fixture = &fixtures[(depth - 6) as usize];
        let expected = fixture.floors.first().expect("replay oracle floor");
        let context = format!("AAA floor-{depth}");
        assert_eq!(
            fixture.schema_version, FINAL_HEAPS_SCHEMA_VERSION,
            "{context} schema"
        );
        assert_eq!(
            fixture.contract.as_deref(),
            Some("final_placed_heaps"),
            "{context} contract"
        );
        assert_eq!(fixture.input.depths, [depth], "{context} requested depth");
        assert_eq!(expected.depth, depth, "{context} floor depth");

        let map = actual.map.as_ref().expect("regular replay map");
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
            (map.width, map.height),
            (expected.width, expected.height),
            "{context} map bounds"
        );
        assert_eq!(
            actual.pre_paint_rng_probe, expected.pre_paint_rng,
            "{context} pre-paint RNG"
        );
        assert_eq!(
            expected.pre_mobs_rng.len(),
            8,
            "{context} pinned pre-mobs RNG"
        );
        assert_eq!(
            expected.pre_items_rng.len(),
            8,
            "{context} pinned pre-items RNG"
        );
        assert_eq!(
            expected.terrain.as_ref().map(Vec::len),
            Some(map.tiles.len()),
            "{context} pinned terrain"
        );
        assert_eq!(
            expected.discoverable.as_ref().map(Vec::len),
            Some(map.tiles.len()),
            "{context} pinned discoverability"
        );
        assert_eq!(
            expected.tile_variance.as_ref().map(Vec::len),
            Some(map.tiles.len()),
            "{context} pinned variance"
        );
        assert!(
            !expected.final_heaps.is_empty(),
            "{context} pinned final heaps"
        );
        assert!(
            !expected.final_mobs.is_empty(),
            "{context} pinned final mobs"
        );

        if depth == 6 {
            assert_eq!(
                actual.pre_mobs_rng_probe, expected.pre_mobs_rng,
                "{context} pre-mobs RNG"
            );
            assert_eq!(
                actual.pre_items_rng_probe, expected.pre_items_rng,
                "{context} pre-items RNG"
            );
        }

        if depth == 7 {
            let oracle = expected.terrain.as_ref().unwrap();
            let garden = expected
                .room_bounds
                .iter()
                .find(|room| room.class_name == "GardenRoom")
                .expect("floor-7 garden bounds");
            let mut actual_cells = Vec::new();
            let mut oracle_cells = Vec::new();
            for y in garden.top..=garden.bottom {
                for x in garden.left..=garden.right {
                    let cell = (y * expected.width as i32 + x) as usize;
                    actual_cells.push(map.tiles[cell]);
                    oracle_cells.push(oracle[cell]);
                }
            }
            assert_eq!(
                actual_cells, oracle_cells,
                "{context} full GardenRoom terrain"
            );
            assert_eq!(map.tiles[1285], 2, "{context} planted grass cell");
            assert_armory_boundary(map, expected, &context);
            assert_library(map, expected, &context);
            assert_eq!(
                actual.pre_mobs_rng_probe, expected.pre_mobs_rng,
                "{context} pre-mobs RNG after PitRoom crystal-door paint"
            );
            assert_eq!(
                actual.pre_items_rng_probe, expected.pre_items_rng,
                "{context} pre-items RNG after quest hook and createMobs"
            );
            let actual_mobs: Vec<_> = map
                .mobs
                .iter()
                .map(|mob| OracleMob {
                    cell: mob.cell,
                    class_name: mob.class_name.clone(),
                })
                .collect();
            assert_eq!(actual_mobs, expected.final_mobs, "{context} exact mobs");
            assert!(
                actual_mobs.contains(&OracleMob {
                    cell: 2138,
                    class_name: "Statue".into(),
                }),
                "{context} pinned StatueRoom mob"
            );
        }

        if depth == 6 {
            let vault_heaps: Vec<_> = map
                .heaps
                .iter()
                .filter(|heap| heap.heap_type == "crystal_chest")
                .map(|heap| {
                    (
                        heap.cell,
                        heap.items[0].class_name.as_str(),
                        heap.items[0].level,
                        heap.items[0].cursed,
                    )
                })
                .collect();
            assert_eq!(
                vault_heaps,
                [
                    (736, "WandOfFrost", 1, false),
                    (814, "RingOfTenacity", 1, true)
                ],
                "{context} CrystalVaultRoom chests"
            );
            assert_crystal_vault_terrain(map, expected, &context);
        }
    }
}

fn assert_library(map: &spd_core::report::FloorMap, expected: &OracleFloor, context: &str) {
    let room = expected
        .room_bounds
        .iter()
        .find(|room| room.class_name == "LibraryRoom")
        .unwrap_or_else(|| panic!("{context} LibraryRoom bounds"));
    let oracle = expected.terrain.as_ref().expect("pinned terrain");

    // EMPTY_SP floors may be decorated later, while the wall/bookshelf shell
    // and entrance passage persist through the completed painter.
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let cell = (y * expected.width as i32 + x) as usize;
            if matches!(oracle[cell], 0 | 10 | 27) {
                assert_eq!(
                    map.tiles[cell], oracle[cell],
                    "{context} Library cell {cell}"
                );
            }
        }
    }

    let heaps: Vec<_> = map
        .heaps
        .iter()
        .filter(|heap| {
            let x = heap.cell as i32 % expected.width as i32;
            let y = heap.cell as i32 / expected.width as i32;
            x > room.left && x < room.right && y > room.top && y < room.bottom
        })
        .map(|heap| {
            (
                heap.cell,
                heap.heap_type.as_str(),
                heap.items[0].class_name.as_str(),
            )
        })
        .collect();
    let oracle_heaps: Vec<_> = expected
        .final_heaps
        .iter()
        .filter(|heap| {
            let x = heap.cell as i32 % expected.width as i32;
            let y = heap.cell as i32 / expected.width as i32;
            x > room.left && x < room.right && y > room.top && y < room.bottom
        })
        .map(|heap| {
            (
                heap.cell,
                heap.heap_type.as_str(),
                heap.items[0].class_name.as_str(),
            )
        })
        .collect();
    assert_eq!(
        oracle_heaps,
        [
            (2071, "heap", "ScrollOfIdentify"),
            (2121, "heap", "ScrollOfUpgrade"),
        ],
        "{context} pinned Java Library prizes"
    );
    assert_eq!(heaps, oracle_heaps, "{context} exact Library prizes");
}

fn assert_armory_boundary(map: &spd_core::report::FloorMap, expected: &OracleFloor, context: &str) {
    let room = expected
        .room_bounds
        .iter()
        .find(|room| room.class_name == "ArmoryRoom")
        .unwrap_or_else(|| panic!("{context} ArmoryRoom bounds"));
    let oracle = expected.terrain.as_ref().expect("pinned terrain");

    // Later water/grass/trap/decorate passes may replace EMPTY tiles. Pin the
    // Armory painter's persistent boundary, locked entrance, and statue.
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let cell = (y * expected.width as i32 + x) as usize;
            if x == room.left
                || x == room.right
                || y == room.top
                || y == room.bottom
                || oracle[cell] == 25
            {
                assert_eq!(
                    map.tiles[cell], oracle[cell],
                    "{context} Armory cell {cell}"
                );
            }
        }
    }

    let heaps: Vec<_> = map
        .heaps
        .iter()
        .filter(|heap| {
            let x = heap.cell as i32 % expected.width as i32;
            let y = heap.cell as i32 / expected.width as i32;
            x > room.left && x < room.right && y > room.top && y < room.bottom
        })
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
        heaps,
        [
            (2076, "heap", "DoubleBomb", 1, 0, false),
            (2268, "heap", "ThrowingSpear", 3, 1, true),
        ],
        "{context} Armory prize placement"
    );
}

fn assert_crystal_vault_terrain(
    map: &spd_core::report::FloorMap,
    expected: &OracleFloor,
    context: &str,
) {
    let vault = expected
        .room_bounds
        .iter()
        .find(|room| room.class_name == "CrystalVaultRoom")
        .expect("floor-6 vault bounds");
    assert_eq!(
        (vault.right - vault.left, vault.bottom - vault.top),
        (6, 6),
        "{context} fixed 7x7 vault bounds"
    );
    let oracle = expected.terrain.as_ref().expect("floor-6 terrain");
    let mut actual_cells = Vec::new();
    let mut oracle_cells = Vec::new();
    let mut locked = Vec::new();
    let mut pedestals = Vec::new();
    for y in vault.top..=vault.bottom {
        for x in vault.left..=vault.right {
            let cell = (y * expected.width as i32 + x) as usize;
            let tile = oracle[cell];
            actual_cells.push(map.tiles[cell]);
            oracle_cells.push(tile);
            if x == vault.left || x == vault.right || y == vault.top || y == vault.bottom {
                assert!(
                    matches!(tile, 4 | 10 | 12),
                    "{context} vault boundary cell {cell}"
                );
            } else if x == vault.left + 1
                || x == vault.right - 1
                || y == vault.top + 1
                || y == vault.bottom - 1
            {
                assert_eq!(tile, 14, "{context} EMPTY_SP inset at cell {cell}");
            }
            if tile == 10 {
                locked.push(cell);
            } else if tile == 11 {
                pedestals.push(cell);
            }
        }
    }
    assert_eq!(actual_cells, oracle_cells, "{context} full vault terrain");
    assert_eq!(locked.len(), 1, "{context} locked vault entrance");
    assert_eq!(pedestals, [736, 814], "{context} vault pedestals");
}
