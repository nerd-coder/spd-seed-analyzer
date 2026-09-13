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
fn aaa_floor_twenty_one_records_first_trap_candidate_boundary() {
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
    assert_eq!(
        (oracle.trap_candidates.len(), actual.valid.len()),
        (379, 389)
    );
    assert_eq!(
        (oracle.trap_non_hall_cells.len(), actual.non_hall.len()),
        (302, 316)
    );
    let first_valid = actual
        .valid
        .iter()
        .zip(&oracle.trap_candidates)
        .position(|(rust, java)| {
            rust.cell != java.cell || rust.room != java.room || rust.terrain != java.terrain
        });
    let first_non_hall = actual
        .non_hall
        .iter()
        .zip(&oracle.trap_non_hall_cells)
        .position(|(rust, java)| rust != java);
    assert_eq!(first_valid, Some(229));
    assert_eq!(actual.valid[229].cell, 891);
    assert_eq!(actual.valid[229].room, "ChasmRoom");
    assert_eq!(actual.valid[229].terrain, crate::level::terrain::EMPTY);
    assert_eq!(oracle.trap_candidates[229].cell, 1131);
    assert_eq!(oracle.trap_candidates[229].room, "ChasmRoom");
    assert_eq!(
        oracle.trap_candidates[229].terrain,
        crate::level::terrain::EMPTY
    );
    assert!(!oracle.trap_candidates.iter().any(|entry| entry.cell == 891));
    assert_eq!(first_non_hall, Some(189));
    assert_eq!(actual.non_hall[189], 1034);
    assert_eq!(oracle.trap_non_hall_cells[189], 1130);
    assert_eq!(
        actual.valid.iter().position(|entry| entry.cell == 1160),
        Some(24)
    );
    assert_eq!(
        oracle
            .trap_candidates
            .iter()
            .position(|entry| entry.cell == 1160),
        Some(24)
    );
}
