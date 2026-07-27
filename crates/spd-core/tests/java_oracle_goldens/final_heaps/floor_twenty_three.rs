use super::*;

use std::fs;

use serde::Deserialize;
use spd_core::rooms::init_rooms::BuilderKind;

#[derive(Debug, Deserialize)]
struct PaintTrace {
    depth: i32,
    #[serde(default)]
    build_attempts: Vec<BuildAttempt>,
    pre_shuffle_rooms: Vec<TraceRoom>,
    checkpoints: Vec<PaintCheckpoint>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct BuildAttempt {
    attempt: u32,
    start_rng: Vec<i32>,
    end_rng: Vec<i32>,
    success: bool,
}

#[derive(Debug, Deserialize)]
struct TraceRoom {
    #[serde(rename = "class")]
    class_name: String,
    bounds: [i32; 4],
}

#[derive(Debug, Deserialize)]
struct PaintCheckpoint {
    stage: String,
    room: String,
    rng: Vec<i32>,
}

pub(super) fn assert_halls_paint_trace(
    seed_text: &str,
    fixture_name: &str,
    depth: i32,
    expected_attempts: usize,
    expected_rooms: usize,
    expected_builder: Option<BuilderKind>,
) {
    let trace_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/java-oracle/fixtures/traces")
        .join(fixture_name);
    let trace: PaintTrace = serde_json::from_str(
        &fs::read_to_string(&trace_path)
            .unwrap_or_else(|error| panic!("read {}: {error}", trace_path.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", trace_path.display()));
    assert_eq!(trace.depth, depth);
    assert_eq!(
        trace.build_attempts.len(),
        expected_attempts,
        "Java builder retry count"
    );
    assert!(
        trace.build_attempts.iter().all(|attempt| attempt.success),
        "Java builder success"
    );

    let seed = parse_seed(seed_text).expect("valid oracle seed");
    let mut dungeon = dungeon_from_run(init_run(seed.numeric));
    let mut actual = None;
    for depth in 1_i32..=trace.depth {
        dungeon.depth = depth;
        actual = Some(create_level_partial(&mut dungeon));
    }
    let actual = actual.expect("requested floor replay");
    if let Some(expected_builder) = expected_builder {
        assert_eq!(actual.builder, Some(expected_builder), "selected builder");
    }
    let report = actual.to_floor_report();
    let guaranteed_torches = report
        .items
        .iter()
        .filter(|item| {
            item.name == "two Torches"
                && item.class_name.as_deref() == Some("Torch")
                && item.source.as_deref() == Some("guaranteed floor spawn")
        })
        .count();
    assert_eq!(
        guaranteed_torches, 1,
        "Halls replay preserves the guaranteed two-Torch spawn"
    );
    assert!(
        report.items.iter().any(|item| {
            item.category == "food" && item.source.as_deref() == Some("guaranteed floor spawn")
        }),
        "Halls replay preserves a guaranteed food spawn"
    );

    let actual_pre_shuffle = actual
        .pre_shuffle_room_bounds
        .iter()
        .map(|room| {
            (
                room.class_name.clone(),
                [room.left, room.top, room.right, room.bottom],
            )
        })
        .collect::<Vec<_>>();
    let oracle_pre_shuffle = trace
        .pre_shuffle_rooms
        .into_iter()
        .map(|room| (room.class_name, room.bounds))
        .collect::<Vec<_>>();
    let actual_callbacks = actual
        .room_paint_rng_checkpoints
        .iter()
        .map(|checkpoint| (checkpoint.room.clone(), checkpoint.rng.clone()))
        .collect::<Vec<_>>();
    let oracle_callbacks = trace
        .checkpoints
        .iter()
        .filter(|checkpoint| checkpoint.stage == "room")
        .map(|checkpoint| (checkpoint.room.clone(), checkpoint.rng.clone()))
        .collect::<Vec<_>>();
    let doors = trace.checkpoints.last().expect("paintDoors checkpoint");
    assert_eq!(
        (doors.stage.as_str(), doors.room.as_str()),
        ("doors", "paintDoors")
    );
    assert_eq!(doors.rng.len(), 8, "fixed-width Java RNG probe");
    assert_eq!(
        oracle_pre_shuffle.len(),
        expected_rooms,
        "all Java rooms recorded"
    );
    assert_eq!(
        oracle_callbacks.len(),
        expected_rooms,
        "every Java room callback recorded"
    );
    assert_eq!(
        actual_pre_shuffle, oracle_pre_shuffle,
        "pre-shuffle room bounds"
    );
    assert_eq!(
        actual_callbacks, oracle_callbacks,
        "room paint RNG callbacks"
    );
    assert_eq!(
        actual.post_doors_rng_probe, doors.rng,
        "post-door RNG boundary"
    );
}

#[test]
fn aaa_floor_twenty_three_halls_trace_matches_preserved_run() {
    assert_halls_paint_trace(
        "AAA-AAA-AAA",
        "aaa-aaa-aaa-floor-23-halls-paint.json",
        23,
        1,
        21,
        Some(BuilderKind::FigureEight),
    );
}

#[test]
fn abc_floor_twenty_three_halls_paint_trace_matches_preserved_run() {
    assert_halls_paint_trace(
        "ABC-DEF-GHI",
        "abc-def-ghi-floor-23-halls-paint.json",
        23,
        1,
        26,
        Some(BuilderKind::FigureEight),
    );
}

#[test]
fn gfx_floor_twenty_three_halls_paint_trace_matches_loop_builder_history() {
    assert_halls_paint_trace(
        "GFX-PZH-DCH",
        "gfx-pzh-dch-floor-23-halls-paint.json",
        23,
        0,
        21,
        Some(BuilderKind::Loop),
    );
}
