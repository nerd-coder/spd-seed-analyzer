use super::*;

use std::ffi::OsStr;
use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PaintTrace {
    depth: i32,
    checkpoints: Vec<PaintCheckpoint>,
}

#[derive(Debug, Deserialize)]
struct PaintCheckpoint {
    stage: String,
    room: String,
    rng: Vec<i32>,
}

#[test]
fn aaa_floor_twenty_two_structural_lifecycle_matches_oracle() {
    let name = OsStr::new("aaa-aaa-aaa-final-heaps-floor-22.json");
    let path = fixture_paths()
        .into_iter()
        .find(|path| path.file_name().is_some_and(|file| file == name))
        .expect("missing AAA floor-22 fixture");
    let fixture = read_fixture(&path);
    let expected = fixture.floors.first().expect("floor-22 oracle facts");

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    let mut actual = None;
    for depth in 1_i32..=22 {
        dungeon.depth = depth;
        actual = Some(create_level_partial(&mut dungeon));
    }
    let actual = actual.expect("floor-22 replay");

    assert_eq!(actual.pre_paint_rng_probe, expected.pre_paint_rng);
    let mut rooms = actual.rooms.clone();
    rooms.sort();
    assert_eq!(rooms, expected.rooms, "floor-22 room selection");
    let bounds: Vec<_> = actual
        .room_bounds
        .iter()
        .map(|room| OracleRoomFact {
            class_name: room.class_name.clone(),
            left: room.left,
            top: room.top,
            right: room.right,
            bottom: room.bottom,
        })
        .collect();
    assert_eq!(bounds, expected.room_bounds, "floor-22 room bounds");

    let trace_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/java-oracle/fixtures/traces/aaa-aaa-aaa-floor-22-halls-paint.json");
    let trace: PaintTrace = serde_json::from_str(
        &fs::read_to_string(&trace_path)
            .unwrap_or_else(|error| panic!("read {}: {error}", trace_path.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", trace_path.display()));
    assert_eq!(trace.depth, 22);
    let first_java_callback = trace.checkpoints.first().expect("first room callback");
    let first_rust_callback = actual
        .room_paint_rng_checkpoints
        .first()
        .expect("first Rust room callback");
    assert_eq!(first_java_callback.stage, "room");
    assert_eq!(first_java_callback.room, "StatueRoom");
    assert_eq!(first_rust_callback.room, "RuinsRoom");
    assert_ne!(
        first_rust_callback.room, first_java_callback.room,
        "the pinned trace isolates the remaining divergence before any room callback"
    );
    let doors = trace.checkpoints.last().expect("paintDoors checkpoint");
    assert_eq!(doors.stage, "doors");
    assert_eq!(doors.room, "paintDoors");
    assert_eq!(doors.rng.len(), 8);
}
