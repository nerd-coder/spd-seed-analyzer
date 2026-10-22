//! Structural projections for fixed-form dedicated levels.

use crate::level::terrain;
use crate::report::{FloorMap, MapTransition};

mod caves;
mod city_overlays;
mod data;
mod halls;
mod last_level;
mod prison;
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
    let mut floor = FloorMap {
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
        custom_terrain: Vec::new(),
        custom_walls: Vec::new(),
        uncertain_cells: Vec::new(),
        hidden_cells: Vec::new(),
        concealed_cells: Vec::new(),
        per_run_cells: Vec::new(),
        sentry_scans: Vec::new(),
        laser_beams: Vec::new(),
        history_cells: Vec::new(),
        constrained_equipment_cells: Vec::new(),
    };
    if depth == 20 {
        (floor.custom_tiles, floor.custom_terrain, floor.custom_walls) =
            city_overlays::layers(&floor.tiles);
    }
    Some(floor)
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
        26 => Some(last_level::build(
            dungeon,
            expand(T26),
            expand(D26),
            depth_seed,
        )),
        10 => Some(prison::build(depth_seed)),
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
mod entity_tests;
#[cfg(test)]
mod tests;
