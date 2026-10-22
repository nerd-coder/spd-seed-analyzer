use super::*;

use std::ffi::OsStr;

/// Replays FVX-YTN-SLC with all nine challenges through `fixture`'s floor and
/// compares painted terrain, mobs, and seed-determined heaps with the oracle.
fn assert_all_challenge_floor(fixture_name: &str) {
    let name = OsStr::new(fixture_name);
    let path = fixture_paths()
        .into_iter()
        .find(|path| path.file_name().is_some_and(|file| file == name))
        .unwrap_or_else(|| panic!("missing {fixture_name}"));
    let fixture = read_fixture(&path);
    assert_eq!(fixture.input.challenge.as_deref(), Some("all"));
    let expected = fixture.floors.first().expect("oracle floor facts");
    let context = format!("{fixture_name} floor {}", expected.depth);

    let profile = spd_core::MapProfile {
        challenges: spd_core::Challenge::ALL.to_vec(),
        ..spd_core::MapProfile::default()
    };
    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    let mut actual = None;
    for depth in 1..=expected.depth {
        dungeon.depth = depth as i32;
        actual = Some(spd_core::level::create_level_partial_for_profile(
            &mut dungeon,
            &profile,
        ));
    }
    let actual = actual.expect("profiled replay");
    assert_eq!(
        actual.pre_paint_rng_probe, expected.pre_paint_rng,
        "{context}"
    );
    assert_eq!(
        actual.pre_mobs_rng_probe, expected.pre_mobs_rng,
        "{context}"
    );
    assert_eq!(
        actual.pre_items_rng_probe, expected.pre_items_rng,
        "{context}"
    );

    let map = actual.map.as_ref().expect("floor map");
    // The oracle has not read the searching guide page, so floor 2 hides its
    // entrance doors (`RegularPainter.java:268-275`); the public map shows them.
    let terrain: Vec<u16> = expected
        .terrain
        .as_ref()
        .expect("terrain")
        .iter()
        .zip(&map.tiles)
        .map(|(&java, &rust)| if (java, rust) == (16, 5) { 5 } else { java })
        .collect();
    assert_eq!(map.tiles, terrain, "{context} terrain");
    assert!(
        map.plants.is_empty(),
        "{context}: Barren Land couches no plants"
    );

    let actual_mobs: Vec<_> = map
        .mobs
        .iter()
        .map(|mob| OracleMob {
            cell: mob.cell,
            class_name: mob.class_name().to_string(),
        })
        .collect();
    assert_eq!(actual_mobs, expected.final_mobs, "{context} mobs");
    // Guide pages depend on the player's journal, not the seed.
    let not_guide_page =
        |heap: &OracleHeap| heap.items.iter().all(|item| item.class_name != "GuidePage");
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
                    class_name: item.class_name().to_string(),
                    quantity: item.quantity,
                    level: item.level,
                    cursed: item.cursed,
                })
                .collect(),
        })
        .filter(not_guide_page)
        .collect();
    // A Suspicious Chest prize drawn from the spawn queue stays unpublished;
    // its chest sits on the room's center pedestal.
    const PEDESTAL: u16 = 11;
    let unpublished_suspicious_chest = |heap: &OracleHeap| {
        actual
            .rooms
            .iter()
            .any(|room| room == "SuspiciousChestRoom")
            && map.tiles[heap.cell as usize] == PEDESTAL
            && heap.heap_type == "chest"
            && !map.heaps.iter().any(|actual| actual.cell == heap.cell)
    };
    let expected_heaps: Vec<_> = expected
        .final_heaps
        .iter()
        .filter(|heap| not_guide_page(heap) && !unpublished_suspicious_chest(heap))
        .cloned()
        .collect();
    assert_eq!(actual_heaps, expected_heaps, "{context} heaps");
    assert!(
        actual_heaps
            .iter()
            .any(|heap| heap.items.iter().any(|item| item.class_name == "Torch")),
        "{context}: Into Darkness places a torch"
    );
}

/// Forbidden Runes drops floor 2's queued Scroll of Upgrade, shifting every
/// later queued drop (keys included); Into Darkness adds the floor torch.
#[test]
fn fvx_floor_two_all_challenges_matches_oracle() {
    assert_all_challenge_floor("fvx-ytn-slc-final-heaps-floor-2-all-challenges.json");
}

/// Champion Enemies is unmodeled: a Giant champion moves later mob placement
/// once one rolls, so deeper floors are not compared.
#[test]
fn fvx_later_floors_all_challenges_match_oracle() {
    for depth in [4, 8, 12] {
        assert_all_challenge_floor(&format!(
            "fvx-ytn-slc-final-heaps-floor-{depth}-all-challenges.json"
        ));
    }
}
