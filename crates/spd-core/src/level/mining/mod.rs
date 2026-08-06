//! Pinned Blacksmith `MiningLevel` side-branch generation.

mod doors;
mod environment;
mod geometry;
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
        .map(|room| room.class_name.clone())
        .collect();
    let mut floor_map = MapFacts::from_room_paint(&map)
        .into_floor_map(&map, depth, 1, depth_seed)
        .into_layout_only();
    floor_map.tileset = match objective {
        BlacksmithQuestType::Crystal => "caves_crystal",
        BlacksmithQuestType::Gnoll => "caves_gnoll",
        BlacksmithQuestType::Fungi => unreachable!(),
    }
    .into();
    #[cfg(test)]
    let floor_rng_tail = Random::peek_ints(8);
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
                required_item: Some("Pickaxe".into()),
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
        heap_occupied: vec![false; len],
        known_heaps: vec![None; len],
        known_blobs: Vec::new(),
        trap_destroys_items: vec![false; len],
        trap_names: vec![None; len],
        branch_exits: Vec::new(),
        branch_entrances: Vec::new(),
        custom_tiles: Vec::new(),
        custom_walls: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct Fixture {
        width: u32,
        height: u32,
        rooms: Vec<FixtureRoom>,
        post_doors_rng: Vec<i32>,
        pre_decoration_terrain: Vec<u16>,
        floor_rng_tail: Vec<i32>,
        room_rng: Vec<RoomRng>,
        terrain: Vec<u16>,
        discoverable: Vec<bool>,
        transitions: Vec<crate::report::MapTransition>,
        traps: Vec<crate::report::MapTrap>,
        custom_tiles: Vec<FixtureLayer>,
        custom_walls: Vec<FixtureLayer>,
    }

    #[derive(Deserialize)]
    struct FixtureRoom {
        #[serde(rename = "class")]
        class_name: String,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[derive(Deserialize)]
    struct FixtureLayer {
        #[serde(rename = "class")]
        class_name: String,
        texture: String,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        map: Vec<i16>,
    }

    #[derive(Deserialize)]
    struct RoomRng {
        #[serde(rename = "class")]
        class_name: String,
        left: i32,
        top: i32,
        rng: Vec<i32>,
        doors: Vec<String>,
    }

    fn assert_fixture(json: &str, objective: i32) {
        let expected: Fixture = serde_json::from_str(json).unwrap();
        let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
        dungeon.blacksmith.spawned = true;
        dungeon.blacksmith.depth = 12;
        dungeon.blacksmith.quest_type = objective;
        let generated = generate(&dungeon, true).expect("mining layout");
        let actual = generated.report.map.as_ref().unwrap();
        assert_eq!(
            actual.tileset,
            if objective == 1 {
                "caves_crystal"
            } else {
                "caves_gnoll"
            }
        );

        for (index, ((room, actual_rng, actual_doors), expected_rng)) in generated
            .room_rng
            .iter()
            .zip(&expected.room_rng)
            .enumerate()
        {
            assert_eq!(
                room.class_name, expected_rng.class_name,
                "room checkpoint {index}"
            );
            assert_eq!((room.left, room.top), (expected_rng.left, expected_rng.top));
            assert_eq!(actual_doors, &expected_rng.doors, "door checkpoint {index}");
            if !actual_rng.starts_with(&expected_rng.rng) {
                let expected_in_actual = actual_rng
                    .windows(expected_rng.rng.len())
                    .position(|window| window == expected_rng.rng);
                panic!(
                    "room checkpoint {index} {} ({}, {}) RNG mismatch; expected at actual offset {expected_in_actual:?}; actual={actual_rng:?}; expected={:?}",
                    room.class_name, room.left, room.top, expected_rng.rng
                );
            }
        }
        assert_eq!(generated.room_rng.len(), expected.room_rng.len());
        assert_eq!(generated.post_doors_rng, expected.post_doors_rng);

        assert_terrain(
            "pre-decoration",
            &generated.pre_decoration_terrain,
            &expected.pre_decoration_terrain,
        );
        assert_eq!(generated.floor_rng_tail, expected.floor_rng_tail);

        assert_eq!(
            (actual.width, actual.height),
            (expected.width, expected.height)
        );
        let actual_rooms = generated
            .room_bounds
            .iter()
            .map(|room| {
                (
                    room.class_name.as_str(),
                    room.left,
                    room.top,
                    room.right,
                    room.bottom,
                )
            })
            .collect::<Vec<_>>();
        let expected_rooms = expected
            .rooms
            .iter()
            .map(|room| {
                (
                    room.class_name.as_str(),
                    room.left,
                    room.top,
                    room.right,
                    room.bottom,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(actual_rooms, expected_rooms);
        assert_eq!(actual.tiles, expected.terrain);
        assert_eq!(actual.discoverable, expected.discoverable);
        assert_eq!(actual.transitions, expected.transitions);
        assert_eq!(actual.traps, expected.traps);
        assert_layers(&actual.custom_tiles, &expected.custom_tiles);
        assert_layers(&actual.custom_walls, &expected.custom_walls);
    }

    fn assert_terrain(label: &str, actual: &[u16], expected: &[u16]) {
        if actual != expected {
            let first = actual
                .iter()
                .zip(expected)
                .position(|(actual, expected)| actual != expected)
                .unwrap();
            panic!(
                "{label} terrain mismatch at {first}: actual {}, expected {}",
                actual[first], expected[first]
            );
        }
    }

    fn assert_layers(actual: &[crate::report::MapCustomTile], expected: &[FixtureLayer]) {
        let actual = actual
            .iter()
            .map(|layer| {
                (
                    layer.class_name.as_str(),
                    layer.texture.as_str(),
                    layer.x,
                    layer.y,
                    layer.width,
                    layer.height,
                    layer.static_data.as_slice(),
                )
            })
            .collect::<Vec<_>>();
        let expected = expected
            .iter()
            .map(|layer| {
                (
                    layer.class_name.as_str(),
                    layer.texture.as_str(),
                    layer.x,
                    layer.y,
                    layer.width,
                    layer.height,
                    layer.map.as_slice(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn crystal_fixture_is_exact() {
        assert_fixture(
            include_str!("../../../../../tools/java-oracle/fixtures/mining/aaa-aaa-aaa-floor-12-crystal.json"),
            1,
        );
    }

    #[test]
    fn gnoll_fixture_is_exact() {
        assert_fixture(
            include_str!(
                "../../../../../tools/java-oracle/fixtures/mining/aaa-aaa-aaa-floor-12-gnoll.json"
            ),
            2,
        );
    }

    #[test]
    fn report_serializes_branch_schema_and_reciprocal_transition() {
        let report = crate::analyze_seed("AAA-AAA-AAA", 14).unwrap();
        let origin = report
            .floors
            .iter()
            .find(|floor| !floor.branches.is_empty())
            .expect("fresh run spawns the Blacksmith by floor 12");
        let branch = &origin.branches[0];
        assert_eq!(branch.id.branch, 1);
        assert_eq!(branch.origin.depth, origin.depth);
        assert!(matches!(branch.objective.as_str(), "Crystal" | "Gnoll"));
        let branch_map = branch.map.as_ref().expect("configured baseline map");
        let return_transition = branch_map
            .transitions
            .iter()
            .find(|transition| transition.transition_type == "BRANCH_ENTRANCE")
            .expect("mine return transition");
        assert_eq!(return_transition.dest_branch, 0);
        let origin_map = origin
            .map
            .as_ref()
            .or(origin.assumed_map.as_ref())
            .expect("origin map");
        assert!(origin_map.transitions.iter().any(|transition| {
            transition.transition_type == "BRANCH_EXIT"
                && transition.dest_branch == 1
                && transition.dest_depth == origin.depth as i32
        }));

        let json = serde_json::to_value(&report).unwrap();
        let branch = &json["floors"][origin.depth as usize - 1]["branches"][0];
        assert_eq!(branch["kind"], "blacksmith_mine");
        assert_eq!(branch["access"]["required_item"], "Pickaxe");
    }

    #[test]
    fn branch_generation_does_not_mutate_later_main_path_replay() {
        let through_origin = crate::analyze_seed("AAA-AAA-AAA", 14).unwrap();
        let through_next = crate::analyze_seed("AAA-AAA-AAA", 15).unwrap();
        assert!(through_origin
            .floors
            .iter()
            .any(|floor| !floor.branches.is_empty()));
        assert_eq!(through_origin.floors, through_next.floors[..14]);
    }
}
