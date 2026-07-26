//! Structural projections for fixed-form dedicated levels.

use crate::level::terrain;
use crate::report::{FloorMap, MapTransition};

mod data;

use data::{D10, D20, D26, T10, T20, T26};

pub(super) fn fixed_layout(depth: i32) -> Option<FloorMap> {
    let (width, height, terrain, discoverable) = match depth {
        10 => (32, 32, T10, D10),
        20 => (15, 48, T20, D20),
        26 => (16, 64, T26, D26),
        _ => return None,
    };
    let tiles = expand(terrain);
    let discoverable = expand(discoverable);
    debug_assert_eq!(tiles.len(), width * height);
    debug_assert_eq!(discoverable.len(), width * height);
    Some(FloorMap {
        width: width as u32,
        height: height as u32,
        tileset: terrain::tileset_for_depth(depth).to_string(),
        tiles,
        tile_variance: vec![0; width * height],
        discoverable,
        markers: Vec::new(),
        heaps: Vec::new(),
        mobs: Vec::new(),
        transitions: transitions(depth),
        traps: Vec::new(),
        plants: Vec::new(),
        blobs: Vec::new(),
        runtime_sensitive_loot_cells: Vec::new(),
        constrained_equipment_cells: Vec::new(),
    })
}

fn expand<T: Copy>(runs: &[(T, usize)]) -> Vec<T> {
    runs.iter()
        .flat_map(|&(value, count)| std::iter::repeat_n(value, count))
        .collect()
}

fn transition(cell: u32, kind: &str, bounds: (u32, u32, u32, u32), dest: i32) -> MapTransition {
    MapTransition {
        cell,
        transition_type: kind.into(),
        left: bounds.0,
        top: bounds.1,
        right: bounds.2,
        bottom: bounds.3,
        dest_depth: dest,
        dest_branch: 0,
        dest_type: Some(
            if kind == "REGULAR_ENTRANCE" {
                "REGULAR_EXIT"
            } else {
                "REGULAR_ENTRANCE"
            }
            .into(),
        ),
    }
}

fn transitions(depth: i32) -> Vec<MapTransition> {
    match depth {
        10 => vec![
            transition(138, "REGULAR_ENTRANCE", (10, 4, 10, 4), 9),
            transition(502, "REGULAR_EXIT", (22, 15, 24, 18), 11),
        ],
        20 => vec![
            transition(127, "REGULAR_EXIT", (4, 4, 10, 8), 21),
            transition(667, "REGULAR_ENTRANCE", (7, 44, 7, 44), 19),
        ],
        26 => vec![transition(872, "REGULAR_ENTRANCE", (7, 54, 9, 56), 25)],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Fixture {
        floors: Vec<OracleFloor>,
    }
    #[derive(Deserialize)]
    struct OracleFloor {
        depth: i32,
        width: u32,
        height: u32,
        terrain: Vec<u16>,
        discoverable: Vec<bool>,
        transitions: Vec<MapTransition>,
    }

    #[test]
    fn fixed_structures_match_normalized_java_oracles() {
        for fixture in [
            include_str!(
                "../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-10.json"
            ),
            include_str!(
                "../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-20.json"
            ),
            include_str!(
                "../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-26.json"
            ),
        ] {
            let mut oracle: OracleFloor = serde_json::from_str::<Fixture>(fixture)
                .unwrap()
                .floors
                .remove(0);
            for tile in &mut oracle.terrain {
                *tile = match i32::from(*tile) {
                    terrain::EMPTY_DECO | terrain::REGION_DECO | terrain::REGION_DECO_ALT => {
                        terrain::EMPTY as u16
                    }
                    terrain::WALL_DECO => terrain::WALL as u16,
                    tile => tile as u16,
                };
            }
            let actual = fixed_layout(oracle.depth).unwrap();
            assert_eq!((actual.width, actual.height), (oracle.width, oracle.height));
            assert_eq!(actual.tiles, oracle.terrain);
            assert_eq!(actual.discoverable, oracle.discoverable);
            assert_eq!(actual.transitions, oracle.transitions);
            assert!(actual.markers.is_empty() && actual.heaps.is_empty() && actual.mobs.is_empty());
        }
    }
}
