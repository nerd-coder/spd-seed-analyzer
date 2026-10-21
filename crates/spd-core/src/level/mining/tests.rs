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
    traps: Vec<FixtureTrap>,
    custom_tiles: Vec<FixtureLayer>,
    custom_walls: Vec<FixtureLayer>,
    mobs: Vec<FixtureMob>,
    heaps: Vec<FixtureHeap>,
}

#[derive(Deserialize)]
struct FixtureMob {
    cell: u32,
    #[serde(rename = "class")]
    class_name: String,
    state: crate::report::MapMobState,
}

#[derive(Deserialize)]
struct FixtureHeap {
    cell: u32,
    #[serde(rename = "type")]
    heap_type: String,
    items: Vec<FixtureHeapItem>,
}

#[derive(Deserialize)]
struct FixtureHeapItem {
    #[serde(rename = "class")]
    class_name: String,
    quantity: i32,
}

/// Room-painted quest mobs and dark-gold chests match Java at `createMobs`.
fn assert_entities(actual: &crate::report::FloorMap, expected: &Fixture) {
    let mut mobs: Vec<_> = actual
        .mobs
        .iter()
        .map(|mob| (mob.cell, mob.mob.class_name(), mob.state))
        .collect();
    mobs.sort_by_key(|mob| mob.0);
    let expected_mobs: Vec<_> = expected
        .mobs
        .iter()
        .map(|mob| (mob.cell, mob.class_name.as_str(), mob.state))
        .collect();
    assert_eq!(mobs, expected_mobs, "mining mobs");

    let mut heaps: Vec<_> = actual
        .heaps
        .iter()
        .map(|heap| {
            let items: Vec<_> = heap
                .items
                .iter()
                .map(|item| (item.item.class_name(), item.quantity))
                .collect();
            (heap.cell, heap.heap_type.to_lowercase(), items)
        })
        .collect();
    heaps.sort_by_key(|heap| heap.0);
    let expected_heaps: Vec<_> = expected
        .heaps
        .iter()
        .map(|heap| {
            let items: Vec<_> = heap
                .items
                .iter()
                .map(|item| (item.class_name.as_str(), item.quantity))
                .collect();
            (heap.cell, heap.heap_type.clone(), items)
        })
        .collect();
    assert_eq!(heaps, expected_heaps, "mining heaps");
    assert!(actual.uncertain_cells.is_empty());
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
    let expected_traps = expected
        .traps
        .iter()
        .map(|trap| crate::report::MapTrap {
            cell: trap.cell,
            trap: spd_objects::TrapId::from_class_name_lossy(&trap.class_name),
            visible: trap.visible,
            active: trap.active,
            color: trap.color,
            shape: trap.shape,
        })
        .collect::<Vec<_>>();
    assert_eq!(actual.traps, expected_traps);
    assert_layers(&actual.custom_tiles, &expected.custom_tiles);
    assert_layers(&actual.custom_walls, &expected.custom_walls);
    assert_entities(actual, &expected);
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
        include_str!(
            "../../../../../tools/java-oracle/fixtures/mining/aaa-aaa-aaa-floor-12-crystal.json"
        ),
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
    let branch_map = branch.map.as_ref().expect("configured default-profile map");
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
    assert_eq!(
        branch["access"]["required_item"],
        spd_objects::ItemId::Pickaxe.id()
    );
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
