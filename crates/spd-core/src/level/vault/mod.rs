//! Pinned Imp `VaultLevel` side-branch generation (GridBuilder + painter-complete layout).

mod combat;
mod doors;
mod entrance;
mod environment;
mod final_room;
mod geometry;
mod hallway;
mod hazards;
mod loot;
mod paint;
mod rooms;
mod tokens;
mod treasure;

use crate::dungeon::DungeonState;
#[cfg(test)]
use crate::generator::Category;
use crate::level::map_facts::MapFacts;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, WALL};
use crate::random::Random;
use crate::report::{BranchAccessReport, BranchFloorId, BranchFloorKind, BranchFloorReport};
use crate::rooms::room::{clear_all_connections, Room};

use super::state::LevelRoomFact;

const BUILD_RETRY_LIMIT: u32 = 10_000;
const PAINT_PADDING: i32 = 1;

#[derive(Debug, Clone)]
pub(crate) struct VaultLayout {
    pub report: BranchFloorReport,
    #[cfg(test)]
    pub dropped_before: [i32; 3],
    #[cfg(test)]
    pub dropped_after: [i32; 3],
    #[cfg(test)]
    pub rooms: Vec<LevelRoomFact>,
}

/// Forced branch-1 vault at `dungeon.depth`. Painter-complete layout only.
/// Occupancy RNG still runs for stream parity; mobs/heaps are stripped from
/// the public map. Callers that share a run `DungeonState` must clone first:
/// usingDefaults draws must not leak into later main-path floors.
pub(crate) fn generate(dungeon: &mut DungeonState, expose_map: bool) -> Option<VaultLayout> {
    let depth = dungeon.depth;
    if !dungeon.imp.spawned || !(17..=19).contains(&depth) {
        return None;
    }
    dungeon.branch = 1;
    let depth_seed = crate::dungeon::seed_for_depth(dungeon.seed, depth, 1);
    Random::push_generator_seeded(depth_seed);
    let layout = generate_on_stream(dungeon, depth, depth_seed, expose_map);
    Random::pop_generator();
    layout
}

fn generate_on_stream(
    dungeon: &mut DungeonState,
    depth: i32,
    depth_seed: i64,
    expose_map: bool,
) -> Option<VaultLayout> {
    #[cfg(test)]
    let dropped_before = dropped_triple(&dungeon.generator);

    let mut gen = loot::queue_floor_loot(dungeon);
    let mut room_list = rooms::init();
    Random::shuffle_list(&mut room_list);
    for (id, room) in room_list.iter_mut().enumerate() {
        room.id = id;
    }

    let mut built = false;
    for _ in 0..BUILD_RETRY_LIMIT {
        clear_all_connections(&mut room_list);
        if crate::builders::build_grid_rooms(&mut room_list) {
            built = true;
            break;
        }
    }
    if !built {
        return None;
    }

    shift_rooms(&mut room_list, PAINT_PADDING);
    let mut map = blank_map(&room_list, PAINT_PADDING)?;
    let mut doors = DoorMap::new();
    let order = paint::paint_rooms(&mut map, &room_list, &mut doors, dungeon, &mut gen);
    doors::paint(&mut map, &room_list, &order, &mut doors, depth);
    environment::paint(&mut map, &room_list, &order, &doors, depth);
    map.recompute_passable();

    #[cfg(test)]
    let dropped_after = dropped_triple(&dungeon.generator);
    let rooms: Vec<_> = room_list
        .iter()
        .map(|room| LevelRoomFact {
            class_name: room.name.clone(),
            left: room.left,
            top: room.top,
            right: room.right,
            bottom: room.bottom,
        })
        .collect();
    let room_names = rooms.iter().map(|room| room.class_name.clone()).collect();
    let floor_map = MapFacts::from_room_paint(&map)
        .into_floor_map(&map, depth, 1, depth_seed)
        .into_layout_only();

    Some(VaultLayout {
        report: BranchFloorReport {
            id: BranchFloorId {
                depth: depth as u32,
                branch: 1,
            },
            origin: BranchFloorId {
                depth: depth as u32,
                branch: 0,
            },
            kind: BranchFloorKind::ImpVault,
            objective: "Vault".into(),
            access: BranchAccessReport {
                quest_id: "ambitious_imp".into(),
                requires_acceptance: true,
                required_item: None,
            },
            rooms: room_names,
            map: expose_map.then_some(floor_map),
            assumed_map: None,
        },
        #[cfg(test)]
        dropped_before,
        #[cfg(test)]
        dropped_after,
        #[cfg(test)]
        rooms,
    })
}

#[cfg(test)]
fn dropped_triple(generator: &crate::generator::GeneratorState) -> [i32; 3] {
    [
        generator.deck_dropped(Category::Ring),
        generator.deck_dropped(Category::Wand),
        generator.deck_dropped(Category::Artifact),
    ]
}

fn shift_rooms(rooms: &mut [Room], padding: i32) {
    let left = rooms.iter().map(|room| room.left).min().unwrap_or_default();
    let top = rooms.iter().map(|room| room.top).min().unwrap_or_default();
    for room in rooms {
        room.shift(padding - left, padding - top);
    }
}

fn blank_map(rooms: &[Room], padding: i32) -> Option<TerrainMap> {
    let right = rooms.iter().map(|room| room.right).max()? + padding;
    let bottom = rooms.iter().map(|room| room.bottom).max()? + padding;
    let width = right + 1;
    let height = bottom + 1;
    let len = (width * height) as usize;
    Some(TerrainMap {
        width,
        height,
        origin_x: 0,
        origin_y: 0,
        map: vec![WALL; len],
        passable: vec![false; len],
        water_allowed: vec![true; len],
        grass_allowed: vec![true; len],
        trap_allowed: vec![true; len],
        item_allowed: vec![true; len],
        character_allowed: vec![true; len],
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
        branch_entrances: Vec::new(),
        custom_tiles: Vec::new(),
        custom_terrain: Vec::new(),
        custom_walls: Vec::new(),
    })
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
