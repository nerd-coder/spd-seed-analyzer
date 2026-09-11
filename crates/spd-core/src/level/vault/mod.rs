//! Pinned Imp `VaultLevel` side-branch generation (GridBuilder + painter-complete layout).

#![allow(dead_code)] // nested BranchFloorReport is PR 4; tests call generate now

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
use crate::generator::Category;
use crate::level::map_facts::MapFacts;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, WALL};
use crate::random::Random;
use crate::report::FloorMap;
use crate::rooms::room::{clear_all_connections, Room};

use super::state::LevelRoomFact;

const BUILD_RETRY_LIMIT: u32 = 10_000;
const PAINT_PADDING: i32 = 1;

#[derive(Debug, Clone)]
pub(crate) struct VaultLayout {
    pub rooms: Vec<LevelRoomFact>,
    pub dropped_before: [i32; 3],
    pub dropped_after: [i32; 3],
    pub map: FloorMap,
}

/// Forced branch-1 vault at `dungeon.depth`. Painter-complete layout only;
/// nested `BranchFloorReport` is PR 4. Occupancy RNG still runs for stream
/// parity; mobs/heaps are stripped from the public map.
pub(crate) fn generate(dungeon: &mut DungeonState) -> Option<VaultLayout> {
    let depth = dungeon.depth;
    if !(17..=19).contains(&depth) {
        return None;
    }
    let depth_seed = crate::dungeon::seed_for_depth(dungeon.seed, depth, 1);
    Random::push_generator_seeded(depth_seed);
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
        Random::pop_generator();
        return None;
    }

    shift_rooms(&mut room_list, PAINT_PADDING);
    let mut map = blank_map(&room_list, PAINT_PADDING)?;
    let mut doors = DoorMap::new();
    let order = paint::paint_rooms(&mut map, &room_list, &mut doors, dungeon, &mut gen);
    doors::paint(&mut map, &room_list, &order, &mut doors, depth);
    environment::paint(&mut map, &room_list, &order, &doors, depth);
    map.recompute_passable();

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
    let floor_map = MapFacts::from_room_paint(&map)
        .into_floor_map(&map, depth, 1, depth_seed)
        .into_layout_only();
    Random::pop_generator();

    Some(VaultLayout {
        rooms,
        dropped_before,
        dropped_after,
        map: floor_map,
    })
}

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
mod tests {
    use serde::Deserialize;

    use super::*;
    use crate::run::{dungeon_from_run, init_run};

    #[derive(Deserialize)]
    struct Fixture {
        schema_version: u32,
        contract: String,
        width: u32,
        height: u32,
        dropped: Dropped,
        rooms: Vec<FixtureRoom>,
        terrain: Vec<u16>,
        discoverable: Vec<bool>,
        traps: Vec<FixtureTrap>,
        blobs: Vec<FixtureBlob>,
        custom_tiles: Vec<FixtureLayer>,
        custom_terrain: Vec<FixtureLayer>,
    }

    #[derive(Deserialize)]
    struct FixtureTrap {
        cell: u32,
        #[serde(rename = "class")]
        class_name: String,
        visible: bool,
        active: bool,
        color: u8,
        shape: u8,
    }

    #[derive(Deserialize)]
    struct FixtureBlob {
        #[serde(rename = "class")]
        class_name: String,
        volume: u32,
        always_visible: bool,
        cells: Vec<FixtureBlobCell>,
    }

    #[derive(Deserialize)]
    struct FixtureBlobCell {
        cell: u32,
        value: u32,
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
        static_data: Vec<i16>,
    }

    #[derive(Deserialize)]
    struct Dropped {
        #[serde(rename = "RING")]
        ring: Counter,
        #[serde(rename = "WAND")]
        wand: Counter,
        #[serde(rename = "ARTIFACT")]
        artifact: Counter,
    }

    #[derive(Deserialize)]
    struct Counter {
        before: i32,
        after: i32,
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

    fn sort_key(class_name: &str, left: i32, top: i32, right: i32, bottom: i32) -> String {
        format!("{class_name}:{left},{top},{right},{bottom}")
    }

    #[test]
    fn aaa_floor_seventeen_room_multiset_matches_java() {
        let expected: Fixture = serde_json::from_str(include_str!(
            "../../../../../tools/java-oracle/fixtures/vault/aaa-aaa-aaa-floor-17.json"
        ))
        .expect("vault fixture");
        assert_eq!(expected.schema_version, 1);
        assert_eq!(expected.contract, "vault_level_using_defaults");

        let mut dungeon = dungeon_from_run(init_run(0));
        dungeon.depth = 17;
        dungeon.branch = 1;
        dungeon.imp.spawned = true;
        let generated = generate(&mut dungeon).expect("vault rooms");

        assert_eq!(generated.dropped_before, [0, 0, 0]);
        assert_eq!(generated.dropped_after, [0, 0, 0]);
        assert_eq!(expected.dropped.ring.before, 0);
        assert_eq!(expected.dropped.ring.after, 0);
        assert_eq!(expected.dropped.wand.before, 0);
        assert_eq!(expected.dropped.wand.after, 0);
        assert_eq!(expected.dropped.artifact.before, 0);
        assert_eq!(expected.dropped.artifact.after, 0);

        let mut actual: Vec<_> = generated
            .rooms
            .iter()
            .map(|room| {
                sort_key(
                    &room.class_name,
                    room.left,
                    room.top,
                    room.right,
                    room.bottom,
                )
            })
            .collect();
        let mut expected_rooms: Vec<_> = expected
            .rooms
            .iter()
            .map(|room| {
                sort_key(
                    &room.class_name,
                    room.left,
                    room.top,
                    room.right,
                    room.bottom,
                )
            })
            .collect();
        actual.sort();
        expected_rooms.sort();
        assert_eq!(actual, expected_rooms, "vault room-name multiset + bounds");

        assert_eq!(generated.map.width, expected.width);
        assert_eq!(generated.map.height, expected.height);
        assert_eq!(
            generated.map.mobs,
            Vec::new(),
            "public vault map has no mobs"
        );
        assert_eq!(
            generated.map.heaps,
            Vec::new(),
            "public vault map has no heaps"
        );
        assert_terrain(&generated.map.tiles, &expected.terrain);
        assert_eq!(generated.map.discoverable, expected.discoverable);
        assert_eq!(
            generated
                .map
                .traps
                .iter()
                .map(|trap| (
                    trap.cell,
                    trap.class_name.as_str(),
                    trap.visible,
                    trap.active,
                    trap.color,
                    trap.shape
                ))
                .collect::<Vec<_>>(),
            expected
                .traps
                .iter()
                .map(|trap| (
                    trap.cell,
                    trap.class_name.as_str(),
                    trap.visible,
                    trap.active,
                    trap.color,
                    trap.shape
                ))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            generated
                .map
                .blobs
                .iter()
                .map(|blob| (
                    blob.class_name.as_str(),
                    blob.volume,
                    blob.always_visible,
                    blob.cells
                        .iter()
                        .map(|cell| (cell.cell, cell.value))
                        .collect::<Vec<_>>()
                ))
                .collect::<Vec<_>>(),
            expected
                .blobs
                .iter()
                .map(|blob| (
                    blob.class_name.as_str(),
                    blob.volume,
                    blob.always_visible,
                    blob.cells
                        .iter()
                        .map(|cell| (cell.cell, cell.value))
                        .collect::<Vec<_>>()
                ))
                .collect::<Vec<_>>()
        );
        assert_layers(&generated.map.custom_tiles, &expected.custom_tiles);
        assert_layers(&generated.map.custom_terrain, &expected.custom_terrain);
    }

    fn assert_terrain(actual: &[u16], expected: &[u16]) {
        if actual != expected {
            let first = actual
                .iter()
                .zip(expected)
                .position(|(a, e)| a != e)
                .unwrap_or(actual.len().min(expected.len()));
            panic!(
                "terrain mismatch at {first}: actual {:?}, expected {:?}; lengths {} vs {}",
                actual.get(first),
                expected.get(first),
                actual.len(),
                expected.len()
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
                    layer.static_data.as_slice(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
