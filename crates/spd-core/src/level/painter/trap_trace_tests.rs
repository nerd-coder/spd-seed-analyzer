use serde::Deserialize;

use super::take_trap_candidate_snapshot;

#[derive(Debug, Deserialize)]
struct OracleTrace {
    pre_trap_rng: Vec<i32>,
    post_trap_capture_rng: Vec<i32>,
    trap_candidates: Vec<OracleCandidate>,
    trap_non_hall_cells: Vec<usize>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct OracleCandidate {
    cell: usize,
    room: String,
    terrain: i32,
}

#[test]
fn aaa_floor_twenty_one_trap_candidate_order_matches_java() {
    let oracle: OracleTrace = serde_json::from_str(include_str!(
        "../../../../../tools/java-oracle/fixtures/traces/aaa-aaa-aaa-floor-21-halls-paint.json"
    ))
    .expect("parse AAA floor-21 Halls paint trace");

    let seed = crate::parse_seed("AAA-AAA-AAA").expect("valid seed");
    let mut dungeon = crate::dungeon_from_run(crate::init_run(seed.numeric));
    for depth in 1..=21 {
        dungeon.depth = depth;
        crate::level::create_level_partial(&mut dungeon);
    }
    let actual = take_trap_candidate_snapshot().expect("floor-21 trap candidate snapshot");

    assert_eq!(oracle.pre_trap_rng, oracle.post_trap_capture_rng);
    assert_eq!(actual.pre_capture_rng, actual.post_capture_rng);
    assert_eq!(actual.pre_capture_rng, oracle.pre_trap_rng);
    assert_eq!(oracle.trap_candidates.len(), 379, "Java valid count");
    assert_eq!(actual.valid.len(), 379, "Rust valid count");
    assert_eq!(oracle.trap_non_hall_cells.len(), 302, "Java non-hall count");
    assert_eq!(actual.non_hall.len(), 302, "Rust non-hall count");
    let actual_valid = actual
        .valid
        .iter()
        .map(|entry| (entry.cell, entry.room.as_str(), entry.terrain))
        .collect::<Vec<_>>();
    let oracle_valid = oracle
        .trap_candidates
        .iter()
        .map(|entry| (entry.cell, entry.room.as_str(), entry.terrain))
        .collect::<Vec<_>>();
    assert_eq!(
        actual_valid, oracle_valid,
        "full ordered trap candidate list"
    );
    assert_eq!(
        actual.non_hall, oracle.trap_non_hall_cells,
        "full ordered non-hall candidate list"
    );
}
