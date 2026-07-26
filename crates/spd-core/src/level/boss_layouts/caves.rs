//! Pinned SPD v3.3.8 `CavesBossLevel` structural generation.

use crate::dungeon::DungeonState;
use crate::level::{map_facts, patch, terrain};
use crate::random::Random;
use crate::report::{FloorMap, MapTransition};

const WIDTH: i32 = 33;
const HEIGHT: i32 = 42;
const UNCHANGED: i32 = -1;

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

pub(super) fn build(dungeon: &mut DungeonState, depth_seed: i64) -> Option<FloorMap> {
    let mut map = blank_map(WIDTH, HEIGHT);
    // Java's `Rect(14, 13, 19, 14)` is right/bottom exclusive.
    fill(&mut map, 14, 13, 5, 1, terrain::CUSTOM_DECO);
    // Java's `Rect(5, 14, 28, 37)` uses exclusive right/bottom bounds.
    fill_ellipse(&mut map, 5, 14, 23, 23, terrain::EMPTY);

    let water = patch::generate(WIDTH, HEIGHT - 14, 0.15, 2, true);
    for cell in (14 * WIDTH) as usize..map.map.len() {
        if map.map[cell] == terrain::EMPTY {
            map.map[cell] = if water[cell - (14 * WIDTH) as usize] {
                terrain::WATER
            } else if Random::int_max(8) == 0 {
                terrain::INACTIVE_TRAP
            } else {
                terrain::EMPTY
            };
        }
    }

    let entrance_variant = Random::int_max(4);
    paint_mirrored(
        &mut map,
        [(9, 18), (23, 18), (23, 32), (9, 32)],
        entrance_variant as usize,
        &ENTRANCES,
        8,
    );
    set(&mut map, 16, 25, terrain::ENTRANCE);
    let corner_variant = Random::int_max(4);
    paint_mirrored(
        &mut map,
        [(2, 11), (30, 11), (30, 39), (2, 39)],
        corner_variant as usize,
        &CORNERS,
        10,
    );

    // `CavesPainter.paint(level, null)` decorates the whole map under an
    // isolated generator. Its results normalize to the same structural tiles,
    // but its raw terrain pass is retained for source parity.
    let decoration_seed = Random::long();
    Random::push_generator_seeded(decoration_seed);
    decorate_caves(&mut map);
    Random::pop_generator();

    fill(&mut map, 0, 3, WIDTH, 4, terrain::CHASM);
    fill(&mut map, 6, 7, 21, 1, terrain::CHASM);
    fill(&mut map, 9, 3, 1, 6, terrain::REGION_DECO_ALT);
    fill(&mut map, 23, 3, 1, 6, terrain::REGION_DECO_ALT);
    fill(&mut map, 10, 8, 13, 1, terrain::CHASM);
    fill(&mut map, 12, 9, 9, 1, terrain::CHASM);
    fill(&mut map, 13, 10, 7, 1, terrain::CHASM);
    fill(&mut map, 14, 3, 5, 10, terrain::EMPTY);
    fill(&mut map, 15, 2, 3, 3, terrain::EMPTY_SP);
    fill(&mut map, 15, 5, 3, 1, terrain::STATUE);
    fill(&mut map, 15, 7, 3, 1, terrain::STATUE);
    fill(&mut map, 15, 9, 3, 1, terrain::STATUE);
    fill(&mut map, 16, 5, 1, 6, terrain::EMPTY_SP);
    fill(&mut map, 15, 0, 3, 3, terrain::EXIT);

    if !pylons_reachable(&map) {
        return None;
    }

    // `Level.create` invokes `createMobs` before `createItems`. Each of the
    // four `Pylon` instances initializes `targetNeighbor = Random.Int(8)`.
    // The layout projection omits mobs, but must retain this outer-stream use.
    for _ in 0..4 {
        Random::int_max(8);
    }

    #[cfg(test)]
    LAST_PRE_ITEMS_RNG.with(|probe| *probe.borrow_mut() = Random::peek_ints(8));

    for tile in &mut map.map {
        *tile = match *tile {
            terrain::EMPTY_DECO | terrain::REGION_DECO | terrain::REGION_DECO_ALT => terrain::EMPTY,
            terrain::WALL_DECO => terrain::WALL,
            tile => tile,
        };
    }

    let mut floor = map_facts::MapFacts::from_room_paint(&map)
        .into_floor_map(&map, 15, dungeon.branch, depth_seed)
        .into_layout_only();
    floor.transitions = vec![
        transition(16 + 2 * WIDTH, "REGULAR_EXIT", (14, 0, 18, 2), 16),
        transition(16 + 25 * WIDTH, "REGULAR_ENTRANCE", (16, 25, 16, 25), 14),
    ];
    Some(floor)
}

fn blank_map(width: i32, height: i32) -> terrain::TerrainMap {
    let len = (width * height) as usize;
    terrain::TerrainMap {
        width,
        height,
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
    }
}

fn paint_mirrored(
    map: &mut terrain::TerrainMap,
    corners: [(i32, i32); 4],
    variant: usize,
    variants: &[&[i32]],
    side: i32,
) {
    let [northwest, northeast, southeast, southwest] = corners;
    let tiles = variants[variant];
    for (index, &tile) in tiles.iter().enumerate() {
        if tile == UNCHANGED {
            continue;
        }
        let x = index as i32 % side;
        let y = index as i32 / side;
        set(map, northwest.0 + x, northwest.1 + y, tile);
        set(map, northeast.0 - x, northeast.1 + y, tile);
        set(map, southeast.0 - x, southeast.1 - y, tile);
        set(map, southwest.0 + x, southwest.1 - y, tile);
    }
}

fn fill(map: &mut terrain::TerrainMap, x: i32, y: i32, width: i32, height: i32, tile: i32) {
    for yy in y..y + height {
        for xx in x..x + width {
            set(map, xx, yy, tile);
        }
    }
}

fn fill_ellipse(map: &mut terrain::TerrainMap, x: i32, y: i32, width: i32, height: i32, tile: i32) {
    let radius_h = height as f64 / 2.0;
    let radius_w = width as f64 / 2.0;
    for row in 0..height {
        let row_y = -radius_h + 0.5 + row as f64;
        let row_width =
            2.0 * ((radius_w * radius_w) * (1.0 - row_y * row_y / (radius_h * radius_h))).sqrt();
        let row_width = if width % 2 == 0 {
            (row_width / 2.0).round() as i32 * 2
        } else {
            (row_width / 2.0).floor() as i32 * 2 + 1
        };
        fill(
            map,
            x + (width - row_width) / 2,
            y + row,
            row_width,
            1,
            tile,
        );
    }
}

fn set(map: &mut terrain::TerrainMap, x: i32, y: i32, tile: i32) {
    map.map[(x + y * map.width) as usize] = tile;
}

/// `CavesPainter.decorate(level, emptyRooms)` at the pinned commit.
fn decorate_caves(map: &mut terrain::TerrainMap) {
    let width = map.width as usize;
    let length = map.map.len();
    for cell in width + 1..length - width {
        if map.map[cell] != terrain::EMPTY {
            continue;
        }
        let mut walls = 0;
        for neighbour in [cell + 1, cell - 1, cell + width, cell - width] {
            if map.map[neighbour] == terrain::WALL {
                walls += 1;
            }
        }
        if Random::int_max(6) <= walls {
            map.map[cell] = terrain::EMPTY_DECO;
        }
    }

    for cell in 0..length - width {
        if map.map[cell] == terrain::WALL
            && floor_tile(map.map[cell + width])
            && Random::int_max(4) == 0
        {
            map.map[cell] = terrain::WALL_DECO;
        }
    }
}

fn floor_tile(tile: i32) -> bool {
    matches!(
        tile,
        terrain::WATER
            | terrain::EMPTY
            | terrain::GRASS
            | terrain::EMPTY_WELL
            | terrain::EMPTY_SP
            | terrain::ENTRANCE
            | terrain::EXIT
            | terrain::EMBERS
            | terrain::PEDESTAL
            | terrain::HIGH_GRASS
            | terrain::TRAP
            | terrain::INACTIVE_TRAP
            | terrain::EMPTY_DECO
            | terrain::WELL
            | terrain::STATUE
            | terrain::STATUE_SP
            | terrain::ALCHEMY
            | terrain::FURROWED_GRASS
            | terrain::CUSTOM_DECO_EMPTY
            | terrain::REGION_DECO
            | terrain::REGION_DECO_ALT
            | terrain::ENTRANCE_SP
    )
}

fn pylons_reachable(map: &terrain::TerrainMap) -> bool {
    let start = (16 + 25 * WIDTH) as usize;
    let mut reached = vec![false; map.map.len()];
    let mut frontier = std::collections::VecDeque::from([start]);
    reached[start] = true;
    while let Some(cell) = frontier.pop_front() {
        let x = cell as i32 % WIDTH;
        let y = cell as i32 / WIDTH;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if !(0..WIDTH).contains(&nx) || !(0..HEIGHT).contains(&ny) {
                    continue;
                }
                let next = (nx + ny * WIDTH) as usize;
                if !reached[next]
                    && matches!(
                        map.map[next],
                        terrain::EMPTY | terrain::EMPTY_SP | terrain::EMPTY_DECO
                    )
                {
                    reached[next] = true;
                    frontier.push_back(next);
                }
            }
        }
    }
    [
        4 + 13 * WIDTH,
        28 + 13 * WIDTH,
        4 + 37 * WIDTH,
        28 + 37 * WIDTH,
    ]
    .into_iter()
    .all(|cell| reached[cell as usize])
}

fn transition(cell: i32, kind: &str, bounds: (i32, i32, i32, i32), dest: i32) -> MapTransition {
    MapTransition {
        cell: cell as u32,
        transition_type: kind.into(),
        left: bounds.0 as u32,
        top: bounds.1 as u32,
        right: bounds.2 as u32,
        bottom: bounds.3 as u32,
        dest_depth: dest,
        dest_branch: 0,
        dest_type: Some(
            if kind == "REGULAR_EXIT" {
                "REGULAR_ENTRANCE"
            } else {
                "REGULAR_EXIT"
            }
            .into(),
        ),
    }
}

const ENTRANCES: [&[i32]; 4] = [
    &[
        N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, W, E, W, W, N, N, N, W, W, E,
        W, W, N, N, W, W, E, E, E, E, N, N, E, E, E, W, W, E, N, N, W, W, E, W, E, E, N, N, W, W,
        E, E, E, E,
    ],
    &[
        N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, E, E, E, N, N, N, W, E, W,
        W, E, N, N, N, E, E, E, E, E, N, N, E, W, E, W, W, E, N, N, E, W, E, W, E, E, N, N, E, E,
        E, E, E, E,
    ],
    &[
        N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, W, W, E,
        W, W, N, N, N, W, W, E, W, W, N, N, N, E, E, E, E, E, N, N, N, W, W, E, W, E, N, N, N, W,
        W, E, E, E,
    ],
    &[
        N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, E, N, N, N, N, N, N, W, E, N, N, N, N, N, W,
        W, E, N, N, N, N, W, W, W, E, N, N, N, W, W, W, W, E, N, N, W, W, W, W, E, E, N, E, E, E,
        E, E, E, E,
    ],
];

const CORNERS: [&[i32]; 4] = [
    &[
        W, W, W, W, W, W, W, W, W, W, W, S, S, S, E, E, E, W, W, W, W, S, S, S, W, W, E, E, W, W,
        W, S, S, S, W, W, W, E, E, W, W, E, W, W, W, W, W, W, E, N, W, E, W, W, W, W, W, N, N, N,
        W, E, E, W, W, W, N, N, N, N, W, W, E, E, W, N, N, N, N, N, W, W, W, E, E, N, N, N, N, N,
        W, W, W, W, N, N, N, N, N, N,
    ],
    &[
        W, W, W, W, W, W, W, W, W, W, W, S, S, S, W, W, W, W, W, W, W, S, S, S, E, E, E, E, E, W,
        W, S, S, S, W, W, W, W, E, E, W, W, E, W, W, W, W, W, W, E, W, W, E, W, W, W, W, N, N, N,
        W, W, E, W, W, W, N, N, N, N, W, W, E, W, W, N, N, N, N, N, W, W, E, E, W, N, N, N, N, N,
        W, W, W, E, E, N, N, N, N, N,
    ],
    &[
        W, W, W, W, W, W, W, W, W, W, W, S, S, S, W, W, W, W, W, W, W, S, S, S, E, E, E, E, W, W,
        W, S, S, S, W, W, W, E, W, W, W, W, E, W, W, W, W, E, W, N, W, W, E, W, W, W, W, E, E, N,
        W, W, E, W, W, W, N, N, N, N, W, W, E, E, E, E, N, N, N, N, W, W, W, W, W, E, N, N, N, N,
        W, W, W, W, N, N, N, N, N, N,
    ],
    &[
        W, W, W, W, W, W, W, W, W, W, W, S, S, S, W, W, W, W, W, W, W, S, S, S, E, E, E, W, W, W,
        W, S, S, S, W, W, E, W, W, W, W, W, E, W, W, W, E, W, W, N, W, W, E, W, W, W, E, E, N, N,
        W, W, E, E, E, E, E, N, N, N, W, W, W, W, W, E, N, N, N, N, W, W, W, W, W, N, N, N, N, N,
        W, W, W, W, N, N, N, N, N, N,
    ],
];

const N: i32 = UNCHANGED;
const W: i32 = terrain::WALL;
const E: i32 = terrain::EMPTY;
const S: i32 = terrain::EMPTY_SP;
