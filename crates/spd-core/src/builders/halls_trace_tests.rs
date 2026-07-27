use super::*;

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::level::create_level_partial;
use crate::run::{dungeon_from_run, init_run};

#[derive(Deserialize)]
struct Trace {
    builder_kind: String,
    build_attempts: Vec<Attempt>,
}

#[derive(Deserialize)]
struct Attempt {
    attempt: u32,
    start_rng: Vec<i32>,
    end_rng: Vec<i32>,
    success: bool,
}

#[test]
fn afu_halls_builder_retries_match_java() {
    for (depth, builder_kind) in [
        (21, "LoopBuilder"),
        (22, "FigureEightBuilder"),
        (23, "FigureEightBuilder"),
        (24, "FigureEightBuilder"),
    ] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tools/java-oracle/fixtures/traces")
            .join(format!("aaa-aaa-afu-floor-{depth}-halls-paint.json"));
        let expected: Trace = serde_json::from_str(
            &fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
        )
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
        assert_eq!(expected.builder_kind, builder_kind);

        let seed = crate::parse_seed("AAA-AAA-AFU").expect("valid AFU seed");
        let mut dungeon = dungeon_from_run(init_run(seed.numeric));
        let mut floor = None;
        for replay_depth in 1..=depth {
            dungeon.depth = replay_depth;
            floor = Some(create_level_partial(&mut dungeon));
        }
        let floor = floor.expect("requested floor");

        match builder_kind {
            "LoopBuilder" => {
                assert_eq!(floor.builder, Some(BuilderKind::Loop));
                let actual = LAST_LOOP_TRACE.with(|trace| trace.borrow().clone());
                assert_eq!(actual.len(), expected.build_attempts.len());
                for (actual, expected) in actual.iter().zip(&expected.build_attempts) {
                    assert_eq!(actual.attempt, expected.attempt);
                    assert_eq!(actual.start_rng_probe, expected.start_rng);
                    assert_eq!(actual.end_rng_probe, expected.end_rng);
                    assert_eq!(actual.success, expected.success);
                }
            }
            "FigureEightBuilder" => {
                assert_eq!(floor.builder, Some(BuilderKind::FigureEight));
                let actual = LAST_FIGURE_EIGHT_TRACE.with(|trace| trace.borrow().clone());
                assert_eq!(actual.len(), expected.build_attempts.len());
                for (actual, expected) in actual.iter().zip(&expected.build_attempts) {
                    assert_eq!(actual.attempt, expected.attempt);
                    assert_eq!(actual.start_rng_probe, expected.start_rng);
                    assert_eq!(actual.end_rng_probe, expected.end_rng);
                    assert_eq!(actual.success, expected.success);
                }
            }
            _ => unreachable!("fixture has a supported builder"),
        }
        assert!(
            expected.build_attempts[..expected.build_attempts.len() - 1]
                .iter()
                .all(|attempt| !attempt.success),
            "all prior Java attempts fail"
        );
        assert!(
            expected
                .build_attempts
                .last()
                .expect("at least one builder attempt")
                .success,
            "last Java attempt succeeds"
        );
    }
}
