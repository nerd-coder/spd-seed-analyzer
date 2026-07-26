//! Structural projections for fixed-form dedicated levels.

use crate::level::terrain;
use crate::report::{FloorMap, MapTransition};

mod caves;
mod data;
mod halls;
mod sewer;

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
        custom_tiles: Vec::new(),
        custom_walls: Vec::new(),
        runtime_sensitive_loot_cells: Vec::new(),
        constrained_equipment_cells: Vec::new(),
    })
}

pub(super) fn generated_layout(
    dungeon: &mut crate::dungeon::DungeonState,
    depth_seed: i64,
) -> Option<FloorMap> {
    match dungeon.depth {
        5 => sewer::build(dungeon, depth_seed),
        // `Level.create` reruns the entire CavesBossLevel build until all four
        // pylons are reachable. Each failed attempt intentionally advances the
        // same level RNG stream.
        15 => (0..10_000).find_map(|_| caves::build(dungeon, depth_seed)),
        25 => (0..10_000).find_map(|_| halls::build(dungeon, depth_seed)),
        _ => fixed_layout(dungeon.depth),
    }
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
        tile_variance: Vec<u8>,
        #[serde(default)]
        custom_tiles: Vec<crate::report::MapCustomTile>,
        #[serde(default)]
        custom_walls: Vec<crate::report::MapCustomTile>,
        transitions: Vec<MapTransition>,
        #[serde(default)]
        pre_items_rng: Vec<i32>,
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

    #[test]
    fn sewer_boss_structures_match_normalized_java_oracles() {
        let profile = crate::MapProfile {
            trinket: crate::MapTrinketProfile::NoMapAffectingTrinkets,
            meta: crate::MapMetaProfile::Fresh,
            trinket_start_depth: 1,
        };
        for (seed, json) in [
            (
                "AAA-AAA-AAA",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-5.json"
                ),
            ),
            (
                "ABC-DEF-GHI",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-5.json"
                ),
            ),
            (
                "GFX-PZH-DCH",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/gfx-pzh-dch-final-heaps-floor-5.json"
                ),
            ),
            (
                "HKT-JZN-XQQ",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/hkt-jzn-xqq-final-heaps-floor-5.json"
                ),
            ),
            (
                "ZZZ-ZZZ-ZZZ",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/zzz-zzz-zzz-final-heaps-floor-5.json"
                ),
            ),
        ] {
            let fixture: Fixture = serde_json::from_str(json).unwrap();
            let oracle = &fixture.floors[0];
            let report = crate::analyze_seed_with_profile(seed, 5, Some(profile.clone())).unwrap();
            let actual = report.floors[4].map.as_ref().expect("depth-5 layout");
            assert_eq!(
                (actual.width, actual.height),
                (oracle.width, oracle.height),
                "{seed}"
            );
            let normalized = oracle
                .terrain
                .iter()
                .map(|&tile| match i32::from(tile) {
                    terrain::EMPTY_DECO | terrain::REGION_DECO | terrain::REGION_DECO_ALT => {
                        terrain::EMPTY as u16
                    }
                    terrain::WALL_DECO => terrain::WALL as u16,
                    tile => tile as u16,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                sewer::last_pre_items_rng(),
                oracle.pre_items_rng,
                "{seed} pre-items RNG"
            );
            if actual.tiles != normalized {
                let diffs = actual
                    .tiles
                    .iter()
                    .zip(&normalized)
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .take(20)
                    .map(|(cell, (a, b))| {
                        (
                            cell,
                            cell % actual.width as usize,
                            cell / actual.width as usize,
                            *a,
                            *b,
                        )
                    })
                    .collect::<Vec<_>>();
                panic!("{seed} terrain first diffs: {diffs:?}");
            }
            assert_eq!(
                actual.discoverable, oracle.discoverable,
                "{seed} discoverable"
            );
            assert_eq!(actual.transitions, oracle.transitions, "{seed} transitions");
            assert!(actual.markers.is_empty() && actual.heaps.is_empty() && actual.mobs.is_empty());
        }
    }

    #[test]
    fn caves_boss_structures_match_normalized_java_oracles() {
        let profile = crate::MapProfile {
            trinket: crate::MapTrinketProfile::NoMapAffectingTrinkets,
            meta: crate::MapMetaProfile::Fresh,
            trinket_start_depth: 1,
        };
        for (seed, json) in [
            (
                "AAA-AAA-AAA",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-15.json"
                ),
            ),
            (
                "ABC-DEF-GHI",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-15.json"
                ),
            ),
            (
                "GFX-PZH-DCH",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/gfx-pzh-dch-final-heaps-floor-15.json"
                ),
            ),
            (
                "ZZZ-ZZZ-ZZZ",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/zzz-zzz-zzz-final-heaps-floor-15.json"
                ),
            ),
        ] {
            let fixture: Fixture = serde_json::from_str(json).unwrap();
            let oracle = &fixture.floors[0];
            let report = crate::analyze_seed_with_profile(seed, 15, Some(profile.clone())).unwrap();
            let actual = report.floors[14].map.as_ref().expect("depth-15 layout");
            assert_eq!(
                (actual.width, actual.height),
                (oracle.width, oracle.height),
                "{seed}"
            );
            let normalized = oracle
                .terrain
                .iter()
                .map(|&tile| match i32::from(tile) {
                    terrain::EMPTY_DECO | terrain::REGION_DECO | terrain::REGION_DECO_ALT => {
                        terrain::EMPTY as u16
                    }
                    terrain::WALL_DECO => terrain::WALL as u16,
                    tile => tile as u16,
                })
                .collect::<Vec<_>>();
            if actual.tiles != normalized {
                let width = actual.width as usize;
                let diffs = actual
                    .tiles
                    .iter()
                    .zip(&normalized)
                    .enumerate()
                    .filter(|(_, (actual, oracle))| actual != oracle)
                    .take(20)
                    .map(|(cell, (actual, oracle))| {
                        (cell, cell % width, cell / width, *actual, *oracle)
                    })
                    .collect::<Vec<_>>();
                panic!("{seed} terrain first diffs: {diffs:?}");
            }
            assert_eq!(
                caves::last_pre_items_rng(),
                oracle.pre_items_rng,
                "{seed} pre-items RNG"
            );
            assert_eq!(
                actual.discoverable, oracle.discoverable,
                "{seed} discoverable"
            );
            assert_eq!(actual.transitions, oracle.transitions, "{seed} transitions");
            assert!(actual.markers.is_empty() && actual.heaps.is_empty() && actual.mobs.is_empty());
        }
    }

    #[test]
    fn halls_boss_structures_match_exact_java_oracles() {
        let profile = crate::MapProfile {
            trinket: crate::MapTrinketProfile::NoMapAffectingTrinkets,
            meta: crate::MapMetaProfile::Fresh,
            trinket_start_depth: 1,
        };
        for (seed, json) in [
            (
                "AAA-AAA-AAA",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-25.json"
                ),
            ),
            (
                "ABC-DEF-GHI",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-25.json"
                ),
            ),
            (
                "GFX-PZH-DCH",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/gfx-pzh-dch-final-heaps-floor-25.json"
                ),
            ),
            (
                "ZZZ-ZZZ-ZZZ",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/zzz-zzz-zzz-final-heaps-floor-25.json"
                ),
            ),
        ] {
            let fixture: Fixture = serde_json::from_str(json).unwrap();
            let oracle = &fixture.floors[0];
            let report = crate::analyze_seed_with_profile(seed, 25, Some(profile.clone())).unwrap();
            let actual = report.floors[24].map.as_ref().expect("depth-25 layout");
            assert_eq!(
                (actual.width, actual.height),
                (oracle.width, oracle.height),
                "{seed}"
            );
            assert_eq!(actual.tiles, oracle.terrain, "{seed} raw terrain");
            assert_eq!(
                actual.tile_variance, oracle.tile_variance,
                "{seed} tile variance"
            );
            assert_eq!(
                halls::last_pre_items_rng(),
                oracle.pre_items_rng,
                "{seed} pre-items RNG"
            );
            assert_eq!(
                actual.discoverable, oracle.discoverable,
                "{seed} discoverable"
            );
            assert_eq!(actual.transitions, oracle.transitions, "{seed} transitions");
            assert!(actual.markers.is_empty() && actual.heaps.is_empty() && actual.mobs.is_empty());
        }
    }

    #[test]
    fn halls_boss_custom_layers_match_java_for_preserved_runs() {
        let profile = crate::MapProfile {
            trinket: crate::MapTrinketProfile::NoMapAffectingTrinkets,
            meta: crate::MapMetaProfile::Fresh,
            trinket_start_depth: 1,
        };
        for (seed, json) in [
            (
                "AAA-AAA-AAA",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-25.json"
                ),
            ),
            (
                "ABC-DEF-GHI",
                include_str!(
                    "../../../../tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-25.json"
                ),
            ),
        ] {
            let fixture: Fixture = serde_json::from_str(json).unwrap();
            let oracle = &fixture.floors[0];
            let report = crate::analyze_seed_with_profile(seed, 25, Some(profile.clone())).unwrap();
            let actual = report.floors[24].map.as_ref().expect("depth-25 layout");
            assert_eq!(
                actual.custom_tiles, oracle.custom_tiles,
                "{seed} custom tiles"
            );
            assert_eq!(
                actual.custom_walls, oracle.custom_walls,
                "{seed} custom walls"
            );
        }
    }
}
