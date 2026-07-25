use super::*;

use std::ffi::OsStr;

#[test]
fn aaa_floor_nineteen_matches_through_tile_variance() {
    let name = OsStr::new("aaa-aaa-aaa-final-heaps-floor-19.json");
    let path = fixture_paths()
        .into_iter()
        .find(|path| path.file_name().is_some_and(|file| file == name))
        .expect("missing AAA floor-19 fixture");
    let fixture = read_fixture(&path);
    let expected = fixture.floors.first().expect("floor-19 oracle facts");

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    let mut actual = None;
    for depth in 1_i32..=19 {
        dungeon.depth = depth;
        actual = Some(create_level_partial(&mut dungeon));
    }
    let actual = actual.expect("floor-19 replay");
    let quest_rewards = actual
        .placed_items
        .iter()
        .filter(|item| item.source.as_deref() == Some("Wandmaker.Quest"))
        .map(|item| OracleItem {
            class_name: item.class_name.clone(),
            quantity: item.quantity,
            level: item.level,
            cursed: item.cursed,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        quest_rewards, expected.quest_rewards,
        "floor-19 persistent Wandmaker rewards"
    );
    assert_eq!(
        quest_rewards,
        vec![
            OracleItem {
                class_name: "WandOfTransfusion".into(),
                quantity: 1,
                level: 1,
                cursed: false,
            },
            OracleItem {
                class_name: "WandOfFrost".into(),
                quantity: 1,
                level: 2,
                cursed: false,
            },
        ],
        "floor-19 exact persistent quest reward identities and state"
    );
    let mut rooms = actual.rooms.clone();
    rooms.sort();
    assert_eq!(rooms, expected.rooms, "floor-19 room classes");

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
    assert_eq!(bounds, expected.room_bounds, "floor-19 normalized bounds");
    assert_eq!(
        actual.pre_paint_rng_probe, expected.pre_paint_rng,
        "floor-19 pre-paint RNG boundary"
    );
    assert_eq!(
        actual.pre_mobs_rng_probe, expected.pre_mobs_rng,
        "floor-19 pre-mobs RNG boundary"
    );
    assert_eq!(
        actual.pre_items_rng_probe, expected.pre_items_rng,
        "floor-19 pre-items RNG boundary"
    );
    let mobs: Vec<_> = actual
        .map
        .as_ref()
        .expect("floor-19 map")
        .mobs
        .iter()
        .map(|mob| OracleMob {
            cell: mob.cell,
            class_name: mob.class_name.clone(),
        })
        .collect();
    assert_eq!(mobs, expected.final_mobs, "floor-19 final mobs");
    let heaps: Vec<_> = actual
        .map
        .as_ref()
        .expect("floor-19 map facts")
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
    assert_eq!(heaps, expected.final_heaps, "floor-19 final heaps");
    let map = actual.map.as_ref().expect("floor-19 map facts");
    let expected_terrain = expected.terrain.as_ref().expect("floor-19 terrain");
    assert_eq!(
        map.tiles.len(),
        expected_terrain.len(),
        "floor-19 terrain length"
    );
    let terrain_mismatches = map
        .tiles
        .iter()
        .zip(expected_terrain)
        .enumerate()
        .filter_map(|(cell, (&actual, &expected))| {
            (actual != expected).then_some((cell, actual, expected))
        })
        .collect::<Vec<_>>();
    assert!(
        terrain_mismatches.is_empty(),
        "floor-19 terrain mismatches: {terrain_mismatches:?}"
    );
    assert_eq!(
        &map.discoverable,
        expected
            .discoverable
            .as_ref()
            .expect("floor-19 discoverable mask"),
        "floor-19 discoverable mask"
    );
    assert_eq!(
        &map.tile_variance,
        expected
            .tile_variance
            .as_ref()
            .expect("floor-19 tile variance"),
        "floor-19 tile variance"
    );
    let transitions = map
        .transitions
        .iter()
        .map(|transition| OracleTransition {
            cell: transition.cell,
            transition_type: transition.transition_type.clone(),
            left: transition.left,
            top: transition.top,
            right: transition.right,
            bottom: transition.bottom,
            dest_depth: transition.dest_depth,
            dest_branch: transition.dest_branch,
            dest_type: transition.dest_type.clone(),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        &transitions,
        expected.transitions.as_ref().expect("floor-19 transitions"),
        "floor-19 transitions"
    );
    let traps = map
        .traps
        .iter()
        .map(|trap| OracleTrap {
            cell: trap.cell,
            class_name: trap.class_name.clone(),
            visible: trap.visible,
            active: trap.active,
            color: trap.color,
            shape: trap.shape,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        &traps,
        expected.traps.as_ref().expect("floor-19 traps"),
        "floor-19 traps"
    );
    let plants = map
        .plants
        .iter()
        .map(|plant| OraclePlant {
            cell: plant.cell,
            class_name: plant.class_name.clone(),
            image: plant.image,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        &plants,
        expected.plants.as_ref().expect("floor-19 plants"),
        "floor-19 plants"
    );
    let blobs = map
        .blobs
        .iter()
        .map(|blob| OracleBlob {
            class_name: blob.class_name.clone(),
            volume: blob.volume,
            always_visible: blob.always_visible,
            cells: blob
                .cells
                .iter()
                .map(|cell| OracleBlobCell {
                    cell: cell.cell,
                    value: cell.value,
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        &blobs,
        expected.blobs.as_ref().expect("floor-19 blobs"),
        "floor-19 blobs"
    );
    assert_eq!(
        blobs,
        vec![OracleBlob {
            class_name: "Alchemy".into(),
            volume: 1,
            always_visible: false,
            cells: vec![OracleBlobCell {
                cell: 1223,
                value: 1,
            }],
        }],
        "floor-19 LaboratoryRoom retains the exact Alchemy seed"
    );
}
