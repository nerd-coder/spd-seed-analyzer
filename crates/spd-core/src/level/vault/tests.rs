use serde::Deserialize;

use super::*;
use crate::report::{BranchFloorId, BranchFloorKind, BranchFloorReport};
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
    transitions: Vec<crate::report::MapTransition>,
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
    let generated = generate(&mut dungeon, true).expect("vault rooms");
    let map = generated.report.map.as_ref().expect("public vault map");

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

    assert_eq!(map.width, expected.width);
    assert_eq!(map.height, expected.height);
    assert_eq!(map.mobs, Vec::new(), "public vault map has no mobs");
    assert_eq!(map.heaps, Vec::new(), "public vault map has no heaps");
    assert_terrain(&map.tiles, &expected.terrain);
    assert_eq!(map.discoverable, expected.discoverable);
    assert_eq!(map.transitions, expected.transitions);
    assert_eq!(
        map.traps
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
        map.blobs
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
    assert_layers(&map.custom_tiles, &expected.custom_tiles);
    assert_layers(&map.custom_terrain, &expected.custom_terrain);
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

fn imp_vault(report: &crate::report::SeedReport) -> &BranchFloorReport {
    report
        .floors
        .iter()
        .find_map(|floor| {
            floor
                .branches
                .iter()
                .find(|branch| branch.kind == BranchFloorKind::ImpVault)
        })
        .expect("fresh run nests the Imp vault under the spawn floor")
}

#[test]
fn report_serializes_branch_schema_and_reciprocal_transition() {
    let report = crate::analyze_seed("AAA-AAA-AAA", 19).unwrap();
    let origin = report
        .floors
        .iter()
        .find(|floor| {
            floor
                .branches
                .iter()
                .any(|branch| branch.kind == BranchFloorKind::ImpVault)
        })
        .expect("Imp vault nested under spawn floor");
    assert!((17..=19).contains(&origin.depth));
    let branch = origin
        .branches
        .iter()
        .find(|branch| branch.kind == BranchFloorKind::ImpVault)
        .expect("imp_vault branch");
    assert_eq!(
        branch.id,
        BranchFloorId {
            depth: origin.depth,
            branch: 1,
        }
    );
    assert_eq!(
        branch.origin,
        BranchFloorId {
            depth: origin.depth,
            branch: 0,
        }
    );
    assert_eq!(branch.access.quest_id, "ambitious_imp");
    assert!(branch.access.requires_acceptance);
    assert_eq!(branch.access.required_item, None);
    assert_eq!(branch.objective, "Vault");
    let branch_map = branch.map.as_ref().expect("configured baseline map");
    assert!(branch_map.mobs.is_empty());
    assert!(branch_map.heaps.is_empty());
    let return_transition = branch_map
        .transitions
        .iter()
        .find(|transition| transition.transition_type == "BRANCH_ENTRANCE")
        .expect("vault return transition");
    assert_eq!(return_transition.dest_branch, 0);
    assert_eq!(return_transition.dest_depth, origin.depth as i32);
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
    let branches = json["floors"][origin.depth as usize - 1]["branches"]
        .as_array()
        .expect("origin branches");
    let branch = branches
        .iter()
        .find(|branch| branch["kind"] == "imp_vault")
        .expect("imp_vault json");
    assert_eq!(branch["kind"], "imp_vault");
    assert_eq!(branch["id"]["branch"], 1);
    assert_eq!(branch["access"]["requires_acceptance"], true);
    assert!(branch["access"].get("required_item").is_none());
    assert!(origin
        .items
        .iter()
        .any(|item| item.source.as_deref() == Some("Imp.Quest")));
}

#[test]
fn nested_vault_does_not_mutate_later_main_path_replay() {
    let through_origin = crate::analyze_seed("AAA-AAA-AAA", 19).unwrap();
    let through_next = crate::analyze_seed("AAA-AAA-AAA", 20).unwrap();
    let origin_depth = imp_vault(&through_origin).id.depth;
    assert!((17..=19).contains(&origin_depth));
    assert_eq!(through_origin.floors, through_next.floors[..19]);
}
