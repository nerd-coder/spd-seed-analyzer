//! Pinned `PrisonBossLevel` start state: cell cages, torches, and the IronKey.

use super::fixed_layout;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::map_facts::heap_item;
use crate::level::terrain::{self, REGION_DECO, WALL, WALL_DECO};
use crate::random::Random;
use crate::report::{FloorMap, MapHeap};

const WIDTH: i32 = 32;
/// `PrisonBossLevel.startCells` as `(left, top, right, bottom)`.
const START_CELLS: [(i32, i32, i32, i32); 4] = [
    (5, 9, 10, 16),
    (11, 9, 16, 16),
    (5, 15, 10, 22),
    (11, 15, 16, 22),
];
const START_TORCHES: [(i32, i32); 7] = [
    (10, 2),
    (7, 9),
    (13, 9),
    (7, 15),
    (13, 15),
    (8, 23),
    (12, 23),
];

pub(super) fn build(depth_seed: i64) -> FloorMap {
    let mut floor = fixed_layout(10).expect("depth 10 has a fixed layout");
    for (x, y) in START_TORCHES {
        floor.tiles[(x + WIDTH * y) as usize] = WALL_DECO as u16;
    }

    // `addCagesToCells` reseeds a child generator with `seedCurDepth`.
    Random::push_generator_seeded(depth_seed);
    for _ in 0..5 {
        let cell = random_prison_cell_pos();
        let beside_wall = [-WIDTH, -1, 1, WIDTH]
            .iter()
            .any(|offset| i32::from(floor.tiles[(cell + offset) as usize]) == WALL);
        if beside_wall {
            floor.tiles[cell as usize] = REGION_DECO as u16;
        }
    }
    Random::pop_generator();

    // `createItems`: the Bones child seed, then the key on a non-solid cell.
    let _bones_seed = Random::long();
    let key = loop {
        let cell = random_prison_cell_pos();
        if !terrain::is_solid_tile(i32::from(floor.tiles[cell as usize])) {
            break cell;
        }
    };
    let item = GeneratedItem::new("IronKey", ItemCategory::Other);
    floor.heaps.push(MapHeap {
        cell: key as u32,
        heap_type: "heap".into(),
        items: vec![heap_item(&item)],
    });
    floor
}

fn random_prison_cell_pos() -> i32 {
    let (left, top, right, bottom) =
        START_CELLS[Random::int_max(START_CELLS.len() as i32) as usize];
    Random::int_range_inclusive(left + 1, right - 2)
        + WIDTH * Random::int_range_inclusive(top + 1, bottom - 2)
}
