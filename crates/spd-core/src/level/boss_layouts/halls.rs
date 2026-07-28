//! Pinned SPD v3.3.8 `HallsBossLevel` structural generation.

use std::collections::VecDeque;

use crate::dungeon::DungeonState;
use crate::level::{map_facts, patch, terrain};
use crate::random::Random;
use crate::report::{FloorMap, MapCustomTile, MapTransition};

const WIDTH: i32 = 32;
const HEIGHT: i32 = 32;
const ROOM_LEFT: i32 = WIDTH / 2 - 4;
const ROOM_RIGHT: i32 = WIDTH / 2 + 4;
const ROOM_TOP: i32 = 8;
const ROOM_BOTTOM: i32 = ROOM_TOP + 8;

#[cfg(test)]
thread_local! {
    static LAST_PRE_ITEMS_RNG: std::cell::RefCell<Vec<i32>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

#[cfg(test)]
pub(super) fn last_pre_items_rng() -> Vec<i32> {
    LAST_PRE_ITEMS_RNG.with(|probe| probe.borrow().clone())
}

/// One `Level.build()` attempt. A failed path check deliberately leaves the
/// active level RNG advanced; the caller retries against that same stream.
pub(super) fn build(dungeon: &mut DungeonState, depth_seed: i64) -> Option<FloorMap> {
    let mut map = blank_map();
    let mut entrance = 0;
    for i in 0..5 {
        let (top, bottom) = match i {
            0 | 4 => (
                Random::int_range_inclusive(ROOM_TOP - 1, ROOM_TOP + 3),
                Random::int_range_inclusive(ROOM_BOTTOM + 2, ROOM_BOTTOM + 6),
            ),
            1 | 3 => (
                Random::int_range_inclusive(ROOM_TOP - 5, ROOM_TOP - 1),
                Random::int_range_inclusive(ROOM_BOTTOM + 6, ROOM_BOTTOM + 10),
            ),
            _ => (
                Random::int_range_inclusive(ROOM_TOP - 6, ROOM_TOP - 3),
                Random::int_range_inclusive(ROOM_BOTTOM + 8, ROOM_BOTTOM + 12),
            ),
        };
        fill(
            &mut map,
            4 + i * 5,
            top,
            5,
            bottom - top + 1,
            terrain::EMPTY,
        );
        if i == 2 {
            entrance = 6 + i * 5 + (bottom - 1) * WIDTH;
        }
    }

    let exit = WIDTH / 2 + (ROOM_TOP + 1) * WIDTH;
    let boss_pos = exit + WIDTH * 3;
    for (cell, patched) in patch::generate(WIDTH, HEIGHT, 0.20, 0, true)
        .into_iter()
        .enumerate()
    {
        if map.map[cell] == terrain::EMPTY && patched {
            map.map[cell] = if distance(cell as i32, boss_pos) + Random::int_max(5) >= 10 {
                terrain::REGION_DECO
            } else {
                terrain::STATUE
            };
        }
    }
    map.map[entrance as usize] = terrain::ENTRANCE;

    fill(
        &mut map,
        ROOM_LEFT - 1,
        ROOM_TOP - 1,
        11,
        11,
        terrain::EMPTY,
    );
    for (cell, patched) in patch::generate(WIDTH, HEIGHT, 0.30, 3, true)
        .into_iter()
        .enumerate()
    {
        if patched
            && matches!(
                map.map[cell],
                terrain::EMPTY | terrain::STATUE | terrain::REGION_DECO
            )
        {
            map.map[cell] = terrain::WATER;
        }
    }
    for tile in &mut map.map {
        if *tile == terrain::EMPTY && Random::int_max(4) == 0 {
            *tile = terrain::EMPTY_DECO;
        }
    }

    fill(&mut map, ROOM_LEFT, ROOM_TOP, 9, 9, terrain::EMPTY_SP);
    fill(&mut map, ROOM_LEFT, ROOM_TOP, 9, 2, terrain::WALL_DECO);
    fill(
        &mut map,
        ROOM_LEFT,
        ROOM_BOTTOM - 1,
        2,
        2,
        terrain::WALL_DECO,
    );
    fill(
        &mut map,
        ROOM_RIGHT - 1,
        ROOM_BOTTOM - 1,
        2,
        2,
        terrain::WALL_DECO,
    );
    fill(&mut map, ROOM_LEFT + 3, ROOM_TOP + 2, 3, 4, terrain::EMPTY);

    for tile in &mut map.map {
        if *tile == terrain::REGION_DECO && Random::int_max(2) == 0 {
            *tile = terrain::REGION_DECO_ALT;
        }
    }
    if !has_path(&map, entrance as usize, exit as usize) {
        return None;
    }

    #[cfg(test)]
    LAST_PRE_ITEMS_RNG.with(|probe| *probe.borrow_mut() = Random::peek_ints(8));

    let mut floor = map_facts::MapFacts::from_room_paint(&map)
        .into_floor_map(&map, 25, dungeon.branch, depth_seed)
        .into_layout_only();
    (floor.custom_tiles, floor.custom_walls) = center_piece_layers();
    let entrance_xy = (entrance % WIDTH, entrance / WIDTH);
    let exit_xy = (exit % WIDTH, exit / WIDTH);
    floor.transitions = vec![
        transition(entrance as u32, "REGULAR_ENTRANCE", entrance_xy, 24),
        transition(exit as u32, "REGULAR_EXIT", exit_xy, 26),
    ];
    // Java grows the exit rectangle one cell from its point: top--, left--,
    // right++. `LevelTransition` starts with all bounds at `exit`.
    floor.transitions[1].top -= 1;
    floor.transitions[1].left -= 1;
    floor.transitions[1].right += 1;
    floor.transitions.sort_by_key(|transition| transition.cell);
    Some(floor)
}

fn center_piece_layers() -> (Vec<MapCustomTile>, Vec<MapCustomTile>) {
    let visual = MapCustomTile {
        class_name: "CenterPieceVisuals".into(),
        texture: "halls_special".into(),
        x: ROOM_LEFT as u32,
        y: (ROOM_TOP + 1) as u32,
        width: 9,
        height: 8,
        static_data: vec![
            8, 9, 10, 11, 11, 11, 12, 13, 14, 16, 17, 18, 27, 19, 27, 20, 21, 22, 24, 25, 26, 19,
            19, 19, 28, 29, 30, 24, 25, 26, 19, 19, 19, 28, 29, 30, 24, 25, 26, 19, 19, 19, 28, 29,
            30, 24, 25, 34, 35, 35, 35, 34, 29, 30, 40, 41, 36, 36, 36, 36, 36, 40, 41, 48, 49, 36,
            36, 36, 36, 36, 48, 49,
        ],
    };
    // `CenterPieceWalls` deliberately declares a 9×9 rect but its source map
    // has eight rows, exactly as in pinned Java.
    let walls = MapCustomTile {
        class_name: "CenterPieceWalls".into(),
        texture: "halls_special".into(),
        x: ROOM_LEFT as u32,
        y: ROOM_TOP as u32,
        width: 9,
        height: 9,
        static_data: vec![
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 32, 33, -1, -1, -1, -1, -1, 32, 33, 40, 41, -1,
            -1, -1, -1, -1, 40, 41,
        ],
    };
    (vec![visual], vec![walls])
}

fn blank_map() -> terrain::TerrainMap {
    let len = (WIDTH * HEIGHT) as usize;
    terrain::TerrainMap {
        width: WIDTH,
        height: HEIGHT,
        origin_x: 0,
        origin_y: 0,
        map: vec![terrain::WALL; len],
        passable: vec![false; len],
        water_allowed: vec![false; len],
        grass_allowed: vec![false; len],
        trap_allowed: vec![false; len],
        item_allowed: vec![false; len],
        character_allowed: vec![false; len],
        mob_occupied: vec![false; len],
        plant_occupied: vec![false; len],
        known_plants: vec![None; len],
        known_mobs: vec![None; len],
        heap_occupied: vec![false; len],
        known_heaps: vec![None; len],
        known_blobs: Vec::new(),
        trap_destroys_items: vec![false; len],
        trap_names: vec![None; len],
        branch_exits: Vec::new(),
        custom_tiles: Vec::new(),
        custom_walls: Vec::new(),
    }
}

fn fill(map: &mut terrain::TerrainMap, x: i32, y: i32, width: i32, height: i32, tile: i32) {
    for y in y..y + height {
        for x in x..x + width {
            map.map[(x + y * WIDTH) as usize] = tile;
        }
    }
}

fn distance(a: i32, b: i32) -> i32 {
    let (ax, ay) = (a % WIDTH, a / WIDTH);
    let (bx, by) = (b % WIDTH, b / WIDTH);
    (ax - bx).abs().max((ay - by).abs())
}

fn has_path(map: &terrain::TerrainMap, start: usize, goal: usize) -> bool {
    let mut seen = vec![false; map.map.len()];
    let mut queue = VecDeque::from([start]);
    seen[start] = true;
    while let Some(cell) = queue.pop_front() {
        if cell == goal {
            return true;
        }
        let x = cell as i32 % WIDTH;
        let y = cell as i32 / WIDTH;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= WIDTH || ny >= HEIGHT {
                    continue;
                }
                let next = (nx + ny * WIDTH) as usize;
                // PathFinder.getStep accepts the requested destination even when
                // its terrain is not passable (the boss exit is WALL_DECO here).
                if !seen[next] && (next == goal || terrain::is_passable_tile(map.map[next])) {
                    seen[next] = true;
                    queue.push_back(next);
                }
            }
        }
    }
    false
}

fn transition(cell: u32, kind: &str, bounds: (i32, i32), dest: i32) -> MapTransition {
    MapTransition {
        cell,
        transition_type: kind.into(),
        left: bounds.0 as u32,
        top: bounds.1 as u32,
        right: bounds.0 as u32,
        bottom: bounds.1 as u32,
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
