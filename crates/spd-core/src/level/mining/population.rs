//! `MiningLevel` population after painting: ambient `createMobs` and
//! `createItems` (food rations, Darkness torch).

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_mobs;
use crate::level::terrain::{TerrainMap, FURROWED_GRASS, GRASS, HIGH_GRASS};
use crate::random::Random;
use crate::rooms::room::Room;

const DARKNESS: i32 = 32;

pub(super) fn populate(
    map: &mut TerrainMap,
    rooms: &[Room],
    order: &[usize],
    dungeon: &DungeonState,
    depth: i32,
    crystal: bool,
) {
    let entrance_cell = map.branch_entrances[0];
    create_mobs::create_mining(depth, rooms, order, map, entrance_cell, crystal);
    create_items(map, rooms, order, dungeon, depth, !crystal, entrance_cell);
}

fn create_items(
    map: &mut TerrainMap,
    rooms: &[Room],
    order: &[usize],
    dungeon: &DungeonState,
    depth: i32,
    gnoll: bool,
    entrance_cell: usize,
) {
    let mut order = order.to_vec();
    // Bones get their own stream; a fresh run has none to place.
    let bones_seed = Random::long();
    Random::push_generator_seeded(bones_seed);
    Random::pop_generator();

    let rations = if gnoll { 2 } else { 1 };
    for _ in 0..rations {
        let cell = random_drop_cell(map, rooms, &mut order, entrance_cell);
        let food = dungeon
            .generator
            .clone()
            .random_using_defaults(Category::Food, depth);
        drop(map, cell, food);
    }
    if dungeon.challenges & DARKNESS != 0 {
        let cell = random_drop_cell(map, rooms, &mut order, entrance_cell);
        drop(map, cell, GeneratedItem::new("Torch", ItemCategory::Other));
    }
}

/// `RegularLevel.randomDropCell(MineSmallRoom.class)`; `exit()` falls back to
/// the branch-entrance cell.
fn random_drop_cell(
    map: &TerrainMap,
    rooms: &[Room],
    order: &mut [usize],
    entrance_cell: usize,
) -> Option<usize> {
    for _ in 0..100 {
        Random::shuffle_list(order);
        let ri = *order
            .iter()
            .find(|&&ri| rooms[ri].name == "MineSmallRoom")?;
        let room = &rooms[ri];
        if room.is_entrance() {
            continue;
        }
        let p = room.random();
        let Some(cell) = map.point_to_cell(p.x, p.y) else {
            continue;
        };
        if map.passable[cell]
            && !map.is_solid(cell)
            && cell != entrance_cell
            && !map.heap_occupied[cell]
            && !map.mob_occupied[cell]
            && !map.trap_destroys_items[cell]
        {
            return Some(cell);
        }
    }
    None
}

fn drop(map: &mut TerrainMap, cell: Option<usize>, item: GeneratedItem) {
    let Some(cell) = cell else {
        return;
    };
    if matches!(map.map[cell], HIGH_GRASS | FURROWED_GRASS) {
        map.map[cell] = GRASS;
    }
    map.record_heap(cell, "heap", item);
}
