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

#[test]
fn depth_one_traps_drop_open_neighbors_from_the_open_list_only() {
    use crate::level::terrain::{paint_minimal, SECRET_TRAP, TRAP};
    use crate::level::Feeling;
    use crate::random::Random;
    use crate::rooms::room::Room;
    use crate::rooms::types::RoomKind;

    fn open_room() -> Room {
        let mut room = Room::new(0, "EmptyRoom", RoomKind::Standard, 1, 4, 5, 9, 5, 9);
        room.left = 1;
        room.top = 1;
        room.right = 12;
        room.bottom = 12;
        room
    }

    fn placed_traps(seed: i64) -> (Vec<usize>, Vec<i32>, super::TrapCandidateSnapshot) {
        Random::reset_generators();
        Random::push_generator_seeded(seed);
        let rooms = vec![open_room()];
        let mut map = paint_minimal(&rooms).expect("map");
        super::paint_traps(&mut map, &rooms, &[0], 1, Feeling::None, 2);
        let snapshot = super::take_trap_candidate_snapshot().expect("trap candidates");
        let mut traps = map
            .map
            .iter()
            .enumerate()
            .filter_map(|(cell, tile)| (*tile == TRAP || *tile == SECRET_TRAP).then_some(cell))
            .collect::<Vec<_>>();
        traps.sort_unstable();
        let tail = Random::peek_ints(4);
        Random::pop_generator();
        (traps, tail, snapshot)
    }

    fn simulate(
        snapshot: &super::TrapCandidateSnapshot,
        width: i32,
        remove_neighbors: bool,
    ) -> (Vec<usize>, Vec<i32>) {
        let (classes, chances) = super::params::trap_table(1);
        let mut valid = snapshot
            .valid
            .iter()
            .map(|candidate| candidate.cell)
            .collect::<Vec<_>>();
        let mut non_hall = snapshot.non_hall.clone();
        let mut placed = Vec::new();
        let n_traps = 2.min(valid.len() as i32 / 5);
        let circle4 = [-width, 1, width, -1];
        for _ in 0..n_traps {
            if valid.is_empty() {
                break;
            }
            let class_index = Random::chances(chances);
            if class_index < 0 {
                break;
            }
            let trap = &classes[class_index as usize];
            let pos = if trap.avoids_hallways && !non_hall.is_empty() {
                non_hall[Random::int_max(non_hall.len() as i32) as usize]
            } else {
                valid[Random::int_max(valid.len() as i32) as usize]
            };
            super::remove_first(&mut valid, pos);
            super::remove_first(&mut non_hall, pos);
            if remove_neighbors {
                for delta in circle4 {
                    let neighbor = pos as i32 + delta;
                    if neighbor >= 0 {
                        super::remove_first(&mut non_hall, neighbor as usize);
                    }
                }
            }
            placed.push(pos);
        }
        placed.sort_unstable();
        (placed, Random::peek_ints(4))
    }

    let mut saw_neighbor_effect = false;
    for seed in 1..=5 {
        let rooms = vec![open_room()];
        let map = paint_minimal(&rooms).expect("map");
        let (actual, actual_tail, snapshot) = placed_traps(seed);

        Random::reset_generators();
        Random::push_generator_seeded(seed);
        let (spaced, spaced_tail) = simulate(&snapshot, map.width, true);
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(seed);
        let (packed, packed_tail) = simulate(&snapshot, map.width, false);
        Random::pop_generator();

        assert_eq!(actual, spaced, "seed {seed}");
        assert_eq!(actual_tail, spaced_tail, "seed {seed}");
        assert_eq!(actual.len(), 2, "seed {seed}");
        if spaced != packed || spaced_tail != packed_tail {
            saw_neighbor_effect = true;
        }
        let width = map.width as usize;
        let orthogonal = (actual[0] % width).abs_diff(actual[1] % width)
            + (actual[0] / width).abs_diff(actual[1] / width)
            == 1;
        assert!(
            !orthogonal,
            "seed {seed} placed orthogonally adjacent open-space traps"
        );
    }
    assert!(
        saw_neighbor_effect,
        "fixture seeds never distinguished neighbor removal"
    );
}
