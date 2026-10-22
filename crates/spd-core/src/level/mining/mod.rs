//! Pinned Blacksmith `MiningLevel` side-branch generation.

mod doors;
mod entrance;
mod environment;
mod geometry;
mod population;
mod rooms;

use crate::builders;
use crate::dungeon::DungeonState;
use crate::level::map_facts::MapFacts;
use crate::level::painter::{self, DoorMap};
use crate::level::terrain::{TerrainMap, WALL};
use crate::quests::BlacksmithQuestType;
use crate::random::Random;
use crate::report::{BranchAccessReport, BranchFloorId, BranchFloorKind, BranchFloorReport};
use crate::rooms::room::Room;

const BUILD_RETRY_LIMIT: u32 = 10_000;

#[derive(Debug, Clone)]
pub(crate) struct MiningLayout {
    pub report: BranchFloorReport,
    #[cfg(test)]
    pub room_bounds: Vec<super::state::LevelRoomFact>,
    #[cfg(test)]
    pub floor_rng_tail: Vec<i32>,
    #[cfg(test)]
    pub post_doors_rng: Vec<i32>,
    #[cfg(test)]
    pub pre_decoration_terrain: Vec<u16>,
    #[cfg(test)]
    pub room_rng: Vec<(super::state::LevelRoomFact, Vec<i32>, Vec<String>)>,
}

pub(crate) fn generate(dungeon: &DungeonState, expose_map: bool) -> Option<MiningLayout> {
    let objective = match dungeon.blacksmith.quest_type {
        1 => BlacksmithQuestType::Crystal,
        2 => BlacksmithQuestType::Gnoll,
        _ => return None,
    };
    let depth = dungeon.blacksmith.depth;
    if !(12..=14).contains(&depth) {
        return None;
    }

    let depth_seed = crate::dungeon::seed_for_depth(dungeon.seed, depth, 1);
    Random::push_generator_seeded(depth_seed);
    let mut built = None;
    for _ in 0..BUILD_RETRY_LIMIT {
        let mut rooms = rooms::init();
        if builders::build_mining_rooms(&mut rooms, depth, BUILD_RETRY_LIMIT) {
            built = Some(rooms);
            break;
        }
    }
    let mut room_list = built?;

    // MiningLevel.painter() constructs and configures the painter before paint().
    let gold = Random::normal_int_range(45, 47);
    shift_rooms(&mut room_list, 3);
    let mut map = blank_map(&room_list, 3)?;
    let mut order: Vec<usize> = (0..room_list.len()).collect();
    Random::shuffle_list(&mut order);
    let mut doors = DoorMap::new();
    let mut dark_gold = 0;
    #[cfg(test)]
    let mut room_rng = Vec::new();
    for &room_index in &order {
        painter::place_doors_for_room(&room_list, room_index, &mut doors);
        rooms::paint(
            &mut map,
            &room_list,
            room_index,
            &mut doors,
            objective,
            depth,
            &mut dark_gold,
        );
        #[cfg(test)]
        if room_list[room_index].kind != crate::rooms::types::RoomKind::Connection {
            let room = &room_list[room_index];
            room_rng.push((
                super::state::LevelRoomFact {
                    class_name: room.name.clone(),
                    left: room.left,
                    top: room.top,
                    right: room.right,
                    bottom: room.bottom,
                },
                Random::peek_ints(32),
                room.connected
                    .iter()
                    .filter_map(|&other| doors.get(room_index, other))
                    .map(|door| {
                        format!("{},{},{:?}", door.x, door.y, door.door_type).to_uppercase()
                    })
                    .collect(),
            ));
        }
    }
    doors::paint(&mut map, &room_list, &order, &mut doors, depth);
    #[cfg(test)]
    let post_doors_rng = Random::peek_ints(8);
    #[cfg(test)]
    let pre_decoration_terrain = map.map.iter().map(|&tile| tile as u16).collect();
    environment::paint(&mut map, &room_list, &mut order, &doors, gold - dark_gold);
    doors::add_border_overlays(&mut map);
    map.recompute_passable();

    let room_bounds = order
        .iter()
        .map(|&index| {
            let room = &room_list[index];
            super::state::LevelRoomFact {
                class_name: room.name.clone(),
                left: room.left,
                top: room.top,
                right: room.right,
                bottom: room.bottom,
            }
        })
        .collect::<Vec<_>>();
    let room_names = room_bounds
        .iter()
        .map(|room| spd_objects::RoomId::from_class_name_lossy(&room.class_name))
        .collect();
    #[cfg(test)]
    let floor_rng_tail = Random::peek_ints(8);
    population::populate(
        &mut map,
        &room_list,
        &order,
        dungeon,
        depth,
        objective == BlacksmithQuestType::Crystal,
    );
    let mut floor_map = MapFacts::from_room_paint(&map).into_floor_map(&map, depth, 1, depth_seed);
    floor_map.tileset = match objective {
        BlacksmithQuestType::Crystal => "caves_crystal",
        BlacksmithQuestType::Gnoll => "caves_gnoll",
        BlacksmithQuestType::Fungi => unreachable!(),
    }
    .into();
    Random::pop_generator();

    Some(MiningLayout {
        report: BranchFloorReport {
            id: BranchFloorId {
                depth: depth as u32,
                branch: 1,
            },
            origin: BranchFloorId {
                depth: depth as u32,
                branch: 0,
            },
            kind: BranchFloorKind::BlacksmithMine,
            objective: objective.as_str().into(),
            access: BranchAccessReport {
                quest_id: "troll_blacksmith".into(),
                requires_acceptance: true,
                required_item: Some(spd_objects::ItemId::Pickaxe),
            },
            rooms: room_names,
            map: expose_map.then_some(floor_map),
            assumed_map: None,
        },
        #[cfg(test)]
        room_bounds,
        #[cfg(test)]
        floor_rng_tail,
        #[cfg(test)]
        post_doors_rng,
        #[cfg(test)]
        pre_decoration_terrain,
        #[cfg(test)]
        room_rng,
    })
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
        mob_loot: vec![Vec::new(); len],
        heap_occupied: vec![false; len],
        heap_order: Vec::new(),
        known_heaps: vec![None; len],
        known_blobs: Vec::new(),
        trap_destroys_items: vec![false; len],
        trap_names: vec![None; len],
        branch_exits: Vec::new(),
        branch_entrances: Vec::new(),
        custom_tiles: Vec::new(),
        custom_terrain: Vec::new(),
        custom_walls: Vec::new(),
        may_differ: Vec::new(),
    })
}

#[cfg(test)]
mod tests;
