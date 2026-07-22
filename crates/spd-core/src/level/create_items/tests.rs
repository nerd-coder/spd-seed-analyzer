use super::*;
use crate::java_random::JavaRandom;
use crate::level::terrain::{self, EMPTY, WATER};

fn room(name: &str) -> Room {
    let mut room = Room::new(0, name, RoomKind::Standard, 1, 16, 5, 5, 5, 5);
    room.left = 1;
    room.top = 1;
    room.right = 5;
    room.bottom = 5;
    room
}

fn placed_room(
    id: usize,
    name: &str,
    kind: RoomKind,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
) -> Room {
    let mut room = Room::new(id, name, kind, 1, 16, 4, 8, 4, 8);
    room.left = left;
    room.top = top;
    room.right = right;
    room.bottom = bottom;
    room
}

/// Set-empty room (zeroed rect), retained in the floor list exactly like
/// Java's builder leaves unplaced rooms behind (`Rect.setEmpty`).
fn empty_room(id: usize, name: &str, kind: RoomKind) -> Room {
    Room::new(id, name, kind, 1, 16, 4, 8, 4, 8)
}

/// One try of the Java-semantics oracle below.
struct OracleTry {
    /// Room index chosen by the instanceof scan (None = scan found nothing).
    picked: Option<usize>,
    shuffle_draws: u32,
    point_draws: u32,
}

struct OracleOutcome {
    cell: i32,
    tries: Vec<OracleTry>,
    /// Next draw after the call — pins the total draw count.
    next: i32,
}

/// watabou `IntRange(min, max)` on a private generator: `Int(max <= 0)`
/// returns `min` WITHOUT consuming a draw (degenerate/inverted ranges).
fn oracle_int_range(rng: &mut JavaRandom, draws: &mut u32, min: i32, max: i32) -> i32 {
    let bound = max - min + 1;
    if bound <= 0 {
        return min;
    }
    *draws += 1;
    min + rng.next_int_bound(bound)
}

/// Direct transcription of `RegularLevel.randomDropCell(StandardRoom.class)`
/// driven by a private `java.util.Random` twin seeded with the same
/// scrambled seed the level RNG would use.
fn java_oracle(rooms: &[Room], map: &TerrainMap, seed: i64) -> OracleOutcome {
    let mut rng = JavaRandom::new(Random::scramble_seed(seed));
    let exit_cell = map.map.iter().position(|&t| t == EXIT);
    let occupied = vec![false; map.len()];
    let mut order: Vec<usize> = (0..rooms.len()).collect();
    let mut tries = Vec::new();
    let mut cell = -1;
    for _ in 0..100 {
        // JDK Collections.shuffle: for (i = size; i > 1; i--) swap(i-1, nextInt(i)).
        let mut shuffle_draws = 0;
        for i in (2..=order.len()).rev() {
            let j = rng.next_int_bound(i as i32) as usize;
            order.swap(i - 1, j);
            shuffle_draws += 1;
        }
        let picked = order.iter().copied().find(|&ri| {
            matches!(
                rooms[ri].kind,
                RoomKind::Entrance | RoomKind::Exit | RoomKind::Standard
            )
        });
        let Some(ri) = picked else {
            tries.push(OracleTry {
                picked: None,
                shuffle_draws,
                point_draws: 0,
            });
            break; // room == null → return -1 on the FIRST try
        };
        let room = &rooms[ri];
        if room.is_entrance() {
            tries.push(OracleTry {
                picked: Some(ri),
                shuffle_draws,
                point_draws: 0,
            });
            continue;
        }
        let mut point_draws = 0;
        let x = oracle_int_range(&mut rng, &mut point_draws, room.left + 1, room.right - 1);
        let y = oracle_int_range(&mut rng, &mut point_draws, room.top + 1, room.bottom - 1);
        tries.push(OracleTry {
            picked: Some(ri),
            shuffle_draws,
            point_draws,
        });
        let Some(idx) = map.point_to_cell(x, y) else {
            continue;
        };
        if idx >= map.passable.len() || !map.passable[idx] {
            continue;
        }
        if map.is_solid(idx) {
            continue;
        }
        if Some(idx) == exit_cell {
            continue;
        }
        if occupied[idx] {
            continue;
        }
        if !map.item_allowed.get(idx).copied().unwrap_or(false) {
            continue;
        }
        if room.name == "AquariumRoom" && map.map[idx] == WATER {
            continue;
        }
        if map.trap_destroys_items.get(idx).copied().unwrap_or(false) {
            continue;
        }
        cell = idx as i32;
        break;
    }
    OracleOutcome {
        cell,
        tries,
        next: rng.next_int(),
    }
}

/// Runs the real `random_drop_cell` on a seeded level-RNG stack and asserts
/// both the selected cell and the total RNG draw count match the Java oracle.
fn assert_java_semantics(rooms: &[Room], map: &TerrainMap, seed: i64) -> OracleOutcome {
    Random::reset_generators();
    Random::push_generator_seeded(seed);
    let mut occupied = vec![false; map.len()];
    let mut order: Vec<usize> = (0..rooms.len()).collect();
    let actual_cell = random_drop_cell(rooms, &mut order, map, &mut occupied);
    let actual_next = Random::int();
    Random::pop_generator();

    let oracle = java_oracle(rooms, map, seed);
    assert_eq!(
        actual_cell, oracle.cell,
        "seed {seed}: selected cell must match Java semantics"
    );
    assert_eq!(
        actual_next, oracle.next,
        "seed {seed}: RNG draw count must match Java semantics"
    );
    oracle
}

#[test]
fn random_drop_cell_enforces_room_item_mask() {
    Random::reset_generators();
    let room = room("PlantsRoom");
    let mut map = terrain::paint_minimal(std::slice::from_ref(&room)).expect("map");
    map.item_allowed.fill(false);
    let only = map.point_to_cell(3, 3).expect("center");
    map.item_allowed[only] = true;
    let mut occupied = vec![false; map.len()];
    let mut order = vec![0];
    Random::push_generator_seeded(3);
    let selected = random_drop_cell(&[room], &mut order, &map, &mut occupied);
    Random::pop_generator();
    assert_eq!(selected, only as i32);
}

#[test]
fn aquarium_rejects_water_from_later_painter_passes() {
    Random::reset_generators();
    let room = room("AquariumRoom");
    let mut map = terrain::paint_minimal(std::slice::from_ref(&room)).expect("map");
    for y in 2..=4 {
        for x in 2..=4 {
            let cell = map.point_to_cell(x, y).expect("interior");
            map.map[cell] = WATER;
        }
    }
    let only = map.point_to_cell(3, 3).expect("center");
    map.map[only] = EMPTY;
    let mut occupied = vec![false; map.len()];
    let mut order = vec![0];
    Random::push_generator_seeded(7);
    let selected = random_drop_cell(&[room], &mut order, &map, &mut occupied);
    Random::pop_generator();
    assert_eq!(selected, only as i32);
}

#[test]
fn full_room_list_reshuffled_every_try_with_standard_instanceof_scan() {
    let rooms = vec![
        placed_room(0, "EntranceRoom", RoomKind::Entrance, 2, 2, 6, 6),
        placed_room(1, "ExitRoom", RoomKind::Exit, 12, 2, 16, 6),
        placed_room(2, "EmptyRoom", RoomKind::Standard, 22, 2, 26, 6),
        placed_room(3, "SegmentedRoom", RoomKind::Standard, 32, 2, 36, 6),
        placed_room(4, "PoolRoom", RoomKind::Special, 42, 2, 46, 6),
        placed_room(5, "SecretGardenRoom", RoomKind::Secret, 52, 2, 56, 6),
        placed_room(6, "TunnelRoom", RoomKind::Connection, 62, 2, 66, 6),
        empty_room(7, "UnplacedStandard", RoomKind::Standard),
    ];
    let map = terrain::paint_minimal(&rooms).expect("map");
    for seed in [11i64, 12345, 777_777] {
        let oracle = assert_java_semantics(&rooms, &map, seed);
        // Each try burns exactly one FULL-list shuffle: n-1 draws for n = 8.
        // (The old pre-filtered candidate list would burn fewer and desync.)
        assert!(
            oracle.tries.iter().all(|t| t.shuffle_draws == 7),
            "seed {seed}: every try must reshuffle the full room list"
        );
        // The scan only ever selects StandardRoom instances. In Java
        // EntranceRoom/ExitRoom extend StandardRoom, so all three kinds match.
        assert!(oracle.tries.iter().all(|t| {
            t.picked.is_none_or(|ri| {
                matches!(
                    rooms[ri].kind,
                    RoomKind::Entrance | RoomKind::Exit | RoomKind::Standard
                )
            })
        }));
    }
}

#[test]
fn exit_room_hosts_drops_but_never_on_the_exit_cell() {
    // The exit room is the ONLY StandardRoom instance here, so every try's
    // scan selects it: proves the exit room can host drops.
    let rooms = vec![
        placed_room(0, "ExitRoom", RoomKind::Exit, 12, 2, 16, 6),
        placed_room(1, "PoolRoom", RoomKind::Special, 22, 2, 26, 6),
        placed_room(2, "SecretGardenRoom", RoomKind::Secret, 32, 2, 36, 6),
    ];
    let map = terrain::paint_minimal(&rooms).expect("map");
    let exit_cell = map.map.iter().position(|&t| t == EXIT).expect("exit tile");
    let mut hosted = 0;
    for seed in 1..=20i64 {
        let oracle = assert_java_semantics(&rooms, &map, seed);
        assert!(oracle.tries.iter().all(|t| t.picked == Some(0)));
        if oracle.cell >= 0 {
            hosted += 1;
            assert_ne!(
                oracle.cell as usize, exit_cell,
                "seed {seed}: drop must never land on the exit cell"
            );
            // ...but must land inside the exit room interior.
            let x = oracle.cell as i32 % map.width + map.origin_x;
            let y = oracle.cell as i32 / map.width + map.origin_y;
            assert!((13..=15).contains(&x) && (3..=5).contains(&y));
        }
    }
    assert!(hosted > 0, "exit room must be able to host drops");
}

#[test]
fn entrance_pick_wastes_the_try_and_never_hosts() {
    // The entrance is the ONLY StandardRoom instance: every try picks it,
    // burns just the shuffle (`room != roomEntrance`), and hosts nothing.
    let rooms = vec![
        placed_room(0, "EntranceRoom", RoomKind::Entrance, 2, 2, 6, 6),
        placed_room(1, "PoolRoom", RoomKind::Special, 12, 2, 16, 6),
    ];
    let map = terrain::paint_minimal(&rooms).expect("map");
    let oracle = assert_java_semantics(&rooms, &map, 42);
    assert_eq!(oracle.cell, -1);
    assert_eq!(
        oracle.tries.len(),
        100,
        "every try picks the entrance and is wasted"
    );
    assert!(oracle
        .tries
        .iter()
        .all(|t| t.picked == Some(0) && t.shuffle_draws == 1 && t.point_draws == 0));
}

#[test]
fn degenerate_empty_room_pick_burns_zero_point_draws() {
    // The set-empty standard room is the ONLY StandardRoom instance. Java
    // still shuffles it into contention; when picked, `Room.random()` runs
    // IntRange(1, -1) on the zeroed rect — watabou `Int(max <= 0)` returns
    // 0 WITHOUT consuming a draw, so the point pick burns ZERO draws (not
    // one per coordinate) and yields (1, 1), wall padding on a real map.
    let rooms = vec![
        placed_room(0, "PoolRoom", RoomKind::Special, 2, 2, 6, 6),
        placed_room(1, "SecretGardenRoom", RoomKind::Secret, 12, 2, 16, 6),
        placed_room(2, "TunnelRoom", RoomKind::Connection, 22, 2, 26, 6),
        empty_room(3, "UnplacedStandard", RoomKind::Standard),
    ];
    let map = terrain::paint_minimal(&rooms).expect("map");
    let oracle = assert_java_semantics(&rooms, &map, 7);
    assert_eq!(
        oracle.cell, -1,
        "degenerate (1, 1) pick fails the terrain checks; try always wasted"
    );
    assert_eq!(oracle.tries.len(), 100);
    assert!(oracle
        .tries
        .iter()
        .all(|t| t.picked == Some(3) && t.shuffle_draws == 3 && t.point_draws == 0));
}

#[test]
fn no_standard_instance_returns_minus_one_after_one_shuffle() {
    // No Entrance/Exit/Standard rooms at all: Java's scan finds nothing and
    // returns -1 on the FIRST try — one full-list shuffle burned, no fallback.
    let rooms = vec![
        placed_room(0, "PoolRoom", RoomKind::Special, 2, 2, 6, 6),
        placed_room(1, "SecretGardenRoom", RoomKind::Secret, 12, 2, 16, 6),
        placed_room(2, "TunnelRoom", RoomKind::Connection, 22, 2, 26, 6),
        placed_room(3, "ShopRoom", RoomKind::Shop, 32, 2, 36, 6),
    ];
    let map = terrain::paint_minimal(&rooms).expect("map");
    let oracle = assert_java_semantics(&rooms, &map, 99);
    assert_eq!(oracle.cell, -1);
    assert_eq!(oracle.tries.len(), 1, "Java gives up on the first try");
    assert_eq!(oracle.tries[0].shuffle_draws, 3);
    assert_eq!(oracle.tries[0].picked, None);
}
