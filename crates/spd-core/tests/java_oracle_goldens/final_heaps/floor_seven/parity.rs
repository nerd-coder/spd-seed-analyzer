use super::super::*;
use spd_core::report::FloorMap;

pub(super) fn assert_map(map: &FloorMap, expected: &OracleFloor) {
    let terrain = expected.terrain.as_ref().expect("floor-7 terrain");
    let terrain_mismatches: Vec<_> = map
        .tiles
        .iter()
        .zip(terrain)
        .enumerate()
        .filter_map(|(cell, (&actual, &expected))| {
            (actual != expected).then_some((cell, actual, expected))
        })
        .collect();
    assert!(
        terrain_mismatches.is_empty(),
        "HKT floor-7 terrain mismatches: {terrain_mismatches:?}"
    );
    assert_eq!(
        map.discoverable,
        *expected
            .discoverable
            .as_ref()
            .expect("floor-7 discoverable mask"),
        "HKT floor-7 discoverable mask"
    );
    assert_eq!(
        map.tile_variance,
        *expected
            .tile_variance
            .as_ref()
            .expect("floor-7 tile variance"),
        "HKT floor-7 tile variance"
    );

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
    assert_eq!(actual_heaps, expected.final_heaps, "HKT floor-7 heaps");

    let actual_mobs: Vec<_> = map
        .mobs
        .iter()
        .map(|mob| OracleMob {
            cell: mob.cell,
            class_name: mob.class_name.clone(),
        })
        .collect();
    assert_eq!(actual_mobs, expected.final_mobs, "HKT floor-7 mobs");

    let actual_transitions: Vec<_> = map
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
        .collect();
    assert_eq!(
        actual_transitions,
        *expected.transitions.as_ref().expect("floor-7 transitions"),
        "HKT floor-7 transitions"
    );

    let actual_traps: Vec<_> = map
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
        .collect();
    assert_eq!(
        actual_traps,
        *expected.traps.as_ref().expect("floor-7 traps"),
        "HKT floor-7 traps"
    );

    let actual_plants: Vec<_> = map
        .plants
        .iter()
        .map(|plant| OraclePlant {
            cell: plant.cell,
            class_name: plant.class_name.clone(),
            image: plant.image,
        })
        .collect();
    assert_eq!(
        actual_plants,
        *expected.plants.as_ref().expect("floor-7 plants"),
        "HKT floor-7 plants"
    );

    let actual_blobs: Vec<_> = map
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
        .collect();
    assert_eq!(
        actual_blobs,
        *expected.blobs.as_ref().expect("floor-7 blobs"),
        "HKT floor-7 blobs"
    );
}
