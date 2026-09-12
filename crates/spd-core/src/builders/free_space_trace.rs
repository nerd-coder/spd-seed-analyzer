//! Test-only recorder for v4 `Builder.findFreeSpace` (Sentry placement + axis ties).

use std::cell::RefCell;

use crate::geom::Point;
use crate::rooms::room::Room;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Candidate {
    pub class_name: String,
    pub bounds: [i32; 4],
    pub cur_diff: f32,
    pub inside: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Step {
    pub room: String,
    pub bounds: [i32; 4],
    pub closest_diff: f32,
    pub candidates: Vec<Candidate>,
    pub w_diff: i32,
    pub h_diff: i32,
    pub tie_draw: Option<i32>,
    pub axis: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct TargetFreeSpace {
    pub start: [i32; 2],
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RoomRef {
    pub class_name: String,
    pub bounds: [i32; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct EqualAxisTie {
    pub placement: u32,
    pub prev: RoomRef,
    pub next_class: String,
    pub start: [i32; 2],
    pub closest: RoomRef,
    pub w_diff: i32,
    pub h_diff: i32,
    pub draw: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct AttemptTrace {
    pub start_rng: Vec<i32>,
    pub end_rng: Vec<i32>,
    pub success: bool,
    pub target_free_space: Option<TargetFreeSpace>,
    pub equal_axis_ties: Vec<EqualAxisTie>,
}

#[derive(Clone)]
struct Placement {
    ordinal: u32,
    prev: RoomRef,
    next_class: String,
}

thread_local! {
    static PLACEMENT_ORDINAL: RefCell<u32> = const { RefCell::new(0) };
    static CURRENT: RefCell<Option<Placement>> = const { RefCell::new(None) };
    static STEPS: RefCell<Vec<Step>> = const { RefCell::new(Vec::new()) };
    static TARGET: RefCell<Option<TargetFreeSpace>> = const { RefCell::new(None) };
    static TIES: RefCell<Vec<EqualAxisTie>> = const { RefCell::new(Vec::new()) };
    static LAST_ATTEMPTS: RefCell<Vec<AttemptTrace>> = const { RefCell::new(Vec::new()) };
}

pub(in crate::builders) fn clear_build() {
    LAST_ATTEMPTS.with(|attempts| attempts.borrow_mut().clear());
}

pub(in crate::builders) fn begin_attempt() {
    PLACEMENT_ORDINAL.with(|ordinal| *ordinal.borrow_mut() = 0);
    CURRENT.with(|current| *current.borrow_mut() = None);
    STEPS.with(|steps| steps.borrow_mut().clear());
    TARGET.with(|target| *target.borrow_mut() = None);
    TIES.with(|ties| ties.borrow_mut().clear());
}

pub(in crate::builders) fn push_attempt(start_rng: Vec<i32>, end_rng: Vec<i32>, success: bool) {
    let mut attempt = take_attempt();
    attempt.start_rng = start_rng;
    attempt.end_rng = end_rng;
    attempt.success = success;
    LAST_ATTEMPTS.with(|attempts| attempts.borrow_mut().push(attempt));
}

fn last_attempts() -> Vec<AttemptTrace> {
    LAST_ATTEMPTS.with(|attempts| attempts.borrow().clone())
}

fn room_ref(room: &Room) -> RoomRef {
    RoomRef {
        class_name: room.name.clone(),
        bounds: [room.left, room.top, room.right, room.bottom],
    }
}

fn take_attempt() -> AttemptTrace {
    CURRENT.with(|current| *current.borrow_mut() = None);
    STEPS.with(|steps| steps.borrow_mut().clear());
    AttemptTrace {
        start_rng: Vec::new(),
        end_rng: Vec::new(),
        success: false,
        target_free_space: TARGET.with(|target| target.borrow_mut().take()),
        equal_axis_ties: TIES.with(|ties| std::mem::take(&mut *ties.borrow_mut())),
    }
}

pub(super) fn before_find(prev: &Room, next: &Room) {
    let ordinal = PLACEMENT_ORDINAL.with(|value| {
        let mut value = value.borrow_mut();
        let ordinal = *value;
        *value += 1;
        ordinal
    });
    STEPS.with(|steps| steps.borrow_mut().clear());
    CURRENT.with(|current| {
        *current.borrow_mut() = Some(Placement {
            ordinal,
            prev: room_ref(prev),
            next_class: next.name.clone(),
        });
    });
}

pub(super) fn candidate(room: &Room, length: f32, inside: bool) -> Candidate {
    Candidate {
        class_name: room.name.clone(),
        bounds: [room.left, room.top, room.right, room.bottom],
        cur_diff: length,
        inside,
    }
}

pub(super) fn record_step(
    closest: &Room,
    closest_diff: f32,
    candidates: Vec<Candidate>,
    w_diff: i32,
    h_diff: i32,
    tie_draw: Option<i32>,
    width_axis: bool,
) {
    STEPS.with(|steps| {
        steps.borrow_mut().push(Step {
            room: closest.name.clone(),
            bounds: [closest.left, closest.top, closest.right, closest.bottom],
            closest_diff,
            candidates,
            w_diff,
            h_diff,
            tie_draw,
            axis: if width_axis { "width" } else { "height" },
        });
    });
}

pub(super) fn record_tie(start: Point, closest: &Room, w_diff: i32, h_diff: i32, draw: i32) {
    let Some(placement) = CURRENT.with(|current| current.borrow().clone()) else {
        return;
    };
    TIES.with(|ties| {
        ties.borrow_mut().push(EqualAxisTie {
            placement: placement.ordinal,
            prev: placement.prev,
            next_class: placement.next_class,
            start: [start.x, start.y],
            closest: room_ref(closest),
            w_diff,
            h_diff,
            draw,
        });
    });
}

pub(super) fn finish_call(start: Point) {
    let Some(placement) = CURRENT.with(|current| current.borrow_mut().take()) else {
        return;
    };
    if placement.next_class != "SentryRoom" {
        STEPS.with(|steps| steps.borrow_mut().clear());
        return;
    }
    let steps = STEPS.with(|steps| std::mem::take(&mut *steps.borrow_mut()));
    TARGET.with(|target| {
        *target.borrow_mut() = Some(TargetFreeSpace {
            start: [start.x, start.y],
            steps,
        });
    });
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use serde::Deserialize;

    use super::last_attempts;
    use crate::builders::place::find_free_space;
    use crate::geom::{Point, Rect};
    use crate::level::create_level_partial;
    use crate::rooms::room::Room;
    use crate::rooms::types::RoomKind;
    use crate::run::{dungeon_from_run, init_run};

    fn room(id: usize, name: &str) -> Room {
        Room::new(id, name, RoomKind::Standard, 1, 16, 3, 10, 3, 10)
    }

    fn bounds(room: &mut Room, left: i32, top: i32, right: i32, bottom: i32) {
        room.left = left;
        room.top = top;
        room.right = right;
        room.bottom = bottom;
    }

    #[test]
    fn find_free_space_uses_euclidean_length_not_manhattan() {
        let start = Point::new(0, 0);
        // (5, 0): Manhattan 5, Euclidean 5. (4, 2): Manhattan 6, Euclidean ~4.47.
        let mut manhattan_closer = room(0, "ManhattanCloser");
        bounds(&mut manhattan_closer, 5, -10, 15, 10);
        let mut euclidean_closer = room(1, "EuclideanCloser");
        bounds(&mut euclidean_closer, 4, 2, 15, 12);

        let space = find_free_space(start, &[manhattan_closer, euclidean_closer], 20);
        assert_eq!(
            space,
            Rect {
                left: -20,
                top: -20,
                right: 4,
                bottom: 20,
            },
            "v4 picks the Euclidean-closer room even when Manhattan prefers the other"
        );
    }

    #[test]
    fn find_free_space_resets_inside_per_colliding_room() {
        let start = Point::new(5, 5);
        let mut outside = room(0, "Outside");
        bounds(&mut outside, 10, 0, 20, 20);
        let mut container = room(1, "ContainsStart");
        bounds(&mut container, 0, 0, 10, 10);

        let space = find_free_space(start, &[outside, container], 20);
        assert_eq!(
            space,
            Rect {
                left: 5,
                top: 5,
                right: 5,
                bottom: 5,
            },
            "v4 re-evaluates inside per room instead of leaving it sticky"
        );
    }

    #[derive(Debug, Deserialize)]
    struct OracleTrace {
        depth: i32,
        attempts: Vec<OracleAttempt>,
    }

    #[derive(Debug, Deserialize)]
    struct OracleAttempt {
        attempt: u32,
        start_rng: Vec<i32>,
        end_rng: Vec<i32>,
        target_free_space: Option<OracleTarget>,
        equal_axis_ties: Vec<OracleTie>,
        success: bool,
    }

    #[derive(Debug, Deserialize)]
    struct OracleTarget {
        start: [i32; 2],
        steps: Vec<OracleStep>,
    }

    #[derive(Debug, Deserialize)]
    struct OracleStep {
        room: String,
        bounds: [i32; 4],
        closest_diff: f32,
        candidates: Vec<OracleCandidate>,
        w_diff: i32,
        h_diff: i32,
        tie_draw: Option<i32>,
        axis: String,
    }

    #[derive(Debug, Deserialize)]
    struct OracleCandidate {
        #[serde(rename = "class")]
        class_name: String,
        bounds: [i32; 4],
        cur_diff: f32,
        inside: bool,
    }

    #[derive(Debug, Deserialize)]
    struct OracleTie {
        placement: u32,
        prev: OracleRoomRef,
        next_class: String,
        start: [i32; 2],
        closest: OracleRoomRef,
        w_diff: i32,
        h_diff: i32,
        draw: i32,
    }

    #[derive(Debug, Deserialize)]
    struct OracleRoomRef {
        #[serde(rename = "class")]
        class_name: String,
        bounds: [i32; 4],
    }

    fn assert_close(actual: f32, expected: f32, context: &str) {
        assert!(
            (actual - expected).abs() <= 1e-5,
            "{context}: {actual} vs {expected}"
        );
    }

    #[test]
    fn abc_floor_twenty_three_sentry_find_free_space_matches_java() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tools/java-oracle/fixtures/traces/abc-def-ghi-floor-23-free-space.json");
        let expected: OracleTrace = serde_json::from_str(
            &fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
        )
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
        assert_eq!(expected.depth, 23);

        let seed = crate::parse_seed("ABC-DEF-GHI").expect("valid seed");
        let mut dungeon = dungeon_from_run(init_run(seed.numeric));
        for depth in 1..=23 {
            dungeon.depth = depth;
            let _ = create_level_partial(&mut dungeon);
        }
        let actual = last_attempts();
        assert_eq!(actual.len(), expected.attempts.len());

        for (index, (actual, expected)) in actual.iter().zip(&expected.attempts).enumerate() {
            assert_eq!(expected.attempt, index as u32);
            assert_eq!(actual.start_rng, expected.start_rng);
            assert_eq!(actual.end_rng, expected.end_rng);
            assert_eq!(actual.success, expected.success);
            assert_eq!(actual.equal_axis_ties.len(), expected.equal_axis_ties.len());
            for (actual_tie, expected_tie) in
                actual.equal_axis_ties.iter().zip(&expected.equal_axis_ties)
            {
                assert_eq!(actual_tie.placement, expected_tie.placement);
                assert_eq!(actual_tie.prev.class_name, expected_tie.prev.class_name);
                assert_eq!(actual_tie.prev.bounds, expected_tie.prev.bounds);
                assert_eq!(actual_tie.next_class, expected_tie.next_class);
                assert_eq!(actual_tie.start, expected_tie.start);
                assert_eq!(
                    actual_tie.closest.class_name,
                    expected_tie.closest.class_name
                );
                assert_eq!(actual_tie.closest.bounds, expected_tie.closest.bounds);
                assert_eq!(actual_tie.w_diff, expected_tie.w_diff);
                assert_eq!(actual_tie.h_diff, expected_tie.h_diff);
                assert_eq!(actual_tie.draw, expected_tie.draw);
            }

            match (&actual.target_free_space, &expected.target_free_space) {
                (None, None) => {}
                (Some(actual_target), Some(expected_target)) => {
                    assert_eq!(actual_target.start, expected_target.start);
                    assert_eq!(actual_target.steps.len(), expected_target.steps.len());
                    for (actual_step, expected_step) in
                        actual_target.steps.iter().zip(&expected_target.steps)
                    {
                        assert_eq!(actual_step.room, expected_step.room);
                        assert_eq!(actual_step.bounds, expected_step.bounds);
                        assert_close(
                            actual_step.closest_diff,
                            expected_step.closest_diff,
                            "closest_diff",
                        );
                        assert_eq!(actual_step.w_diff, expected_step.w_diff);
                        assert_eq!(actual_step.h_diff, expected_step.h_diff);
                        assert_eq!(actual_step.tie_draw, expected_step.tie_draw);
                        assert_eq!(actual_step.axis, expected_step.axis);
                        assert_eq!(actual_step.candidates.len(), expected_step.candidates.len());
                        for (actual_candidate, expected_candidate) in
                            actual_step.candidates.iter().zip(&expected_step.candidates)
                        {
                            assert_eq!(actual_candidate.class_name, expected_candidate.class_name);
                            assert_eq!(actual_candidate.bounds, expected_candidate.bounds);
                            assert_close(
                                actual_candidate.cur_diff,
                                expected_candidate.cur_diff,
                                "cur_diff",
                            );
                            assert_eq!(actual_candidate.inside, expected_candidate.inside);
                        }
                    }
                }
                (actual_target, expected_target) => panic!(
                    "target_free_space mismatch: rust {actual_target:?} java {expected_target:?}"
                ),
            }
        }
        assert!(
            expected
                .attempts
                .iter()
                .any(|attempt| attempt.target_free_space.is_some()),
            "Java fixture records the Sentry findFreeSpace walk"
        );
    }
}
