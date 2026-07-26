//! Level builders (Loop / Figure-Eight).

mod connection;
mod figure_eight;
mod loop_builder;
mod place;
mod regular;

pub use place::{angle_between_rooms, find_free_space, place_room};
pub use regular::BuilderParams;

use crate::rooms::init_rooms::BuilderKind;
use crate::rooms::room::{clear_all_connections, Room};

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct FigureEightAttemptTrace {
    attempt: u32,
    start_rng_probe: Vec<i32>,
    end_rng_probe: Vec<i32>,
    failure_stage: Option<figure_eight::FailureStage>,
    rooms: Vec<(String, i32, i32, i32, i32)>,
    success: bool,
}

#[cfg(test)]
thread_local! {
    static LAST_FIGURE_EIGHT_TRACE: std::cell::RefCell<Vec<FigureEightAttemptTrace>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

/// Place rooms with one Java builder instance. Failed inner attempts retain
/// FigureEightBuilder's selected landmark, while appended connection rooms are discarded.
pub fn build_rooms(
    rooms: &mut Vec<Room>,
    kind: BuilderKind,
    intensity: f32,
    offset: f32,
    depth: i32,
    max_tries: u32,
    prepare_shop: &mut impl FnMut(&mut Room),
) -> bool {
    let params = BuilderParams {
        curve_exponent: 2,
        curve_intensity: intensity % 1.0,
        curve_offset: offset % 0.5,
        ..Default::default()
    };

    let mut figure_state = figure_eight::FigureEightState::default();
    #[cfg(test)]
    LAST_FIGURE_EIGHT_TRACE.with(|trace| trace.borrow_mut().clear());
    for _attempt in 0..max_tries {
        clear_all_connections(rooms);
        for r in rooms.iter_mut() {
            r.set_empty();
        }
        // Re-assign ids after possible connection room appends from failed tries
        rooms.retain(|r| r.kind != crate::rooms::types::RoomKind::Connection);
        for (i, r) in rooms.iter_mut().enumerate() {
            r.id = i;
        }

        let ok = match kind {
            BuilderKind::Loop => loop_builder::build(rooms, &params, depth, prepare_shop).is_some(),
            BuilderKind::FigureEight => {
                #[cfg(test)]
                let start_rng_probe = crate::random::Random::peek_ints(8);
                let result =
                    figure_eight::build(rooms, &params, depth, &mut figure_state, prepare_shop);
                #[cfg(test)]
                LAST_FIGURE_EIGHT_TRACE.with(|trace| {
                    trace.borrow_mut().push(FigureEightAttemptTrace {
                        attempt: _attempt,
                        start_rng_probe,
                        end_rng_probe: crate::random::Random::peek_ints(8),
                        failure_stage: result.as_ref().err().copied(),
                        rooms: rooms
                            .iter()
                            .map(|room| {
                                (
                                    room.name.clone(),
                                    room.left,
                                    room.top,
                                    room.right,
                                    room.bottom,
                                )
                            })
                            .collect(),
                        success: result.is_ok(),
                    });
                });
                result.is_ok()
            }
        };
        if ok {
            return true;
        }
    }
    false
}

/// Pinned `SewerBossLevel.builder()` retry loop.
pub(crate) fn build_sewer_boss_rooms(
    rooms: &mut Vec<Room>,
    intensity: f32,
    max_tries: u32,
) -> bool {
    let params = BuilderParams {
        path_length: 1.0,
        path_len_jitter: [1.0, 0.0, 0.0, 0.0],
        path_tunnel_chances: [1.0, 2.0, 0.0],
        branch_tunnel_chances: [1.0, 0.0, 0.0],
        // SewerBossLevel leaves RegularBuilder's default extra-connection
        // chance untouched (setLoopShape only changes curve parameters).
        extra_connection_chance: 0.30,
        curve_exponent: 2,
        curve_intensity: intensity,
        curve_offset: 0.0,
    };
    let landmark = rooms
        .iter()
        .position(|room| room.name.ends_with("GooRoom"))
        .expect("SewerBossLevel has a Goo landmark room");
    let mut state = figure_eight::FigureEightState::with_landmark(landmark);
    for _ in 0..max_tries {
        clear_all_connections(rooms);
        for room in rooms.iter_mut() {
            room.set_empty();
        }
        rooms.retain(|room| room.kind != crate::rooms::types::RoomKind::Connection);
        for (id, room) in rooms.iter_mut().enumerate() {
            room.id = id;
        }
        if figure_eight::build(rooms, &params, 5, &mut state, &mut |_| {}).is_ok() {
            return true;
        }
    }
    false
}

#[cfg(test)]
fn build_figure_eight_traced(
    rooms: &mut Vec<Room>,
    intensity: f32,
    offset: f32,
    depth: i32,
    max_tries: u32,
) -> Vec<FigureEightAttemptTrace> {
    let params = BuilderParams {
        curve_exponent: 2,
        curve_intensity: intensity % 1.0,
        curve_offset: offset % 0.5,
        ..Default::default()
    };
    let mut state = figure_eight::FigureEightState::default();
    let mut trace = Vec::new();
    for attempt in 0..max_tries {
        clear_all_connections(rooms);
        for room in rooms.iter_mut() {
            room.set_empty();
        }
        rooms.retain(|room| room.kind != crate::rooms::types::RoomKind::Connection);
        for (id, room) in rooms.iter_mut().enumerate() {
            room.id = id;
        }
        let start_rng_probe = crate::random::Random::peek_ints(8);
        let result = figure_eight::build(rooms, &params, depth, &mut state, &mut |_| {});
        trace.push(FigureEightAttemptTrace {
            attempt,
            start_rng_probe,
            end_rng_probe: crate::random::Random::peek_ints(8),
            failure_stage: result.as_ref().err().copied(),
            rooms: rooms
                .iter()
                .map(|room| {
                    (
                        room.name.clone(),
                        room.left,
                        room.top,
                        room.right,
                        room.bottom,
                    )
                })
                .collect(),
            success: result.is_ok(),
        });
        if result.is_ok() {
            break;
        }
    }
    trace
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;
    use crate::rooms::room::dims_for_kind;
    use crate::rooms::types::RoomKind;

    #[test]
    fn aaa_floor_twenty_one_matches_java_attempt_boundaries() {
        use crate::level::create_level_partial;
        use crate::run::{dungeon_from_run, init_run};

        let mut dungeon = dungeon_from_run(init_run(0));
        let mut floor = None;
        for depth in 1..=21 {
            dungeon.depth = depth;
            floor = Some(create_level_partial(&mut dungeon));
        }
        let trace = LAST_FIGURE_EIGHT_TRACE.with(|trace| trace.borrow().clone());
        assert_eq!(trace.len(), 3);
        assert_eq!(
            trace
                .iter()
                .map(|attempt| (attempt.failure_stage, attempt.success))
                .collect::<Vec<_>>(),
            [
                (Some(figure_eight::FailureStage::FirstLoop), false),
                (Some(figure_eight::FailureStage::SecondLoop), false),
                (None, true),
            ]
        );
        assert_eq!(
            trace
                .iter()
                .map(|attempt| attempt.start_rng_probe.clone())
                .collect::<Vec<_>>(),
            [
                vec![
                    755658038,
                    2119033571,
                    -291089650,
                    -193444257,
                    -598165080,
                    1384792528,
                    -1540666710,
                    1084686982
                ],
                vec![
                    2147435541,
                    1330567029,
                    -897766932,
                    -2036202939,
                    864136891,
                    -1643441563,
                    -98025121,
                    908687397
                ],
                vec![
                    1271526477,
                    -1171829326,
                    592264908,
                    48167381,
                    -335136343,
                    -1591169427,
                    -1718249634,
                    137460704
                ],
            ]
        );
        assert!(trace
            .windows(2)
            .all(|pair| pair[0].end_rng_probe == pair[1].start_rng_probe));
        let floor = floor.unwrap();
        assert_eq!(
            floor
                .rooms
                .iter()
                .filter(|name| name.as_str() == "TunnelRoom")
                .count(),
            6
        );
        assert_eq!(
            floor.pre_paint_rng_probe,
            [
                1830028298,
                1789541391,
                49840001,
                -704551720,
                -499241945,
                1437454582,
                780588159,
                -1009167912
            ]
        );
    }

    fn room(id: usize, name: &str, kind: RoomKind, size: i32, connections: i32) -> Room {
        let (min_w, max_w, min_h, max_h) = dims_for_kind(kind, size, name);
        Room::new(
            id,
            name,
            kind,
            size,
            connections,
            min_w,
            max_w,
            min_h,
            max_h,
        )
    }

    #[test]
    fn figure_eight_builds_two_cycles_and_connects_every_base_room() {
        let mut rooms = vec![
            room(0, "EntranceRoom", RoomKind::Entrance, 1, 16),
            room(1, "ExitRoom", RoomKind::Exit, 1, 16),
            room(2, "CaveRoom", RoomKind::Standard, 1, 16),
            room(3, "PlantsRoom", RoomKind::Standard, 1, 16),
            room(4, "RegionDecoBridgeRoom", RoomKind::Standard, 2, 16),
            room(5, "CirclePitRoom", RoomKind::Standard, 2, 16),
            room(6, "LibraryRingRoom", RoomKind::Standard, 3, 16),
            room(7, "CryptRoom", RoomKind::Special, 1, 1),
            room(8, "SecretLarderRoom", RoomKind::Secret, 1, 1),
        ];
        Random::reset_generators();
        Random::push_generator_seeded(0xF168_0008);
        let built = build_rooms(
            &mut rooms,
            BuilderKind::FigureEight,
            0.55,
            0.0,
            12,
            1_000,
            &mut |_| {},
        );
        let rng_tail = Random::int();
        Random::pop_generator();

        assert!(built);
        assert!(rooms[..9].iter().all(|room| !room.connected.is_empty()));
        let vertices = rooms.iter().filter(|room| !room.is_empty()).count();
        let edges = rooms.iter().map(|room| room.connected.len()).sum::<usize>() / 2;
        assert!(edges + 1 >= vertices + 2, "expected two independent loops");
        assert_eq!(rng_tail, 1_953_852_014);
    }

    #[test]
    fn figure_eight_attempt_trace_is_non_advancing_and_in_insertion_order() {
        let make_rooms = || {
            vec![
                room(0, "EntranceRoom", RoomKind::Entrance, 1, 16),
                room(1, "ExitRoom", RoomKind::Exit, 1, 16),
                room(2, "CaveRoom", RoomKind::Standard, 1, 16),
                room(3, "PlantsRoom", RoomKind::Standard, 1, 16),
                room(4, "RegionDecoBridgeRoom", RoomKind::Standard, 2, 16),
                room(5, "CirclePitRoom", RoomKind::Standard, 2, 16),
                room(6, "LibraryRingRoom", RoomKind::Standard, 3, 16),
                room(7, "CryptRoom", RoomKind::Special, 1, 1),
                room(8, "SecretLarderRoom", RoomKind::Secret, 1, 1),
            ]
        };

        Random::reset_generators();
        Random::push_generator_seeded(0xF168_0008);
        let mut traced_rooms = make_rooms();
        let trace = build_figure_eight_traced(&mut traced_rooms, 0.55, 0.0, 12, 1_000);
        let traced_tail = Random::int();
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(0xF168_0008);
        let mut ordinary_rooms = make_rooms();
        assert!(build_rooms(
            &mut ordinary_rooms,
            BuilderKind::FigureEight,
            0.55,
            0.0,
            12,
            1_000,
            &mut |_| {},
        ));
        let ordinary_tail = Random::int();
        Random::pop_generator();

        assert_eq!(traced_tail, ordinary_tail, "trace probes must not draw RNG");
        assert_eq!(trace.last().map(|attempt| attempt.success), Some(true));
        assert!(trace
            .iter()
            .enumerate()
            .all(|(index, attempt)| attempt.attempt == index as u32));
        assert!(trace
            .iter()
            .all(|attempt| attempt.start_rng_probe.len() == 8 && attempt.end_rng_probe.len() == 8));
        assert!(trace
            .iter()
            .all(|attempt| attempt.success == attempt.failure_stage.is_none()));
        assert_eq!(
            trace.last().unwrap().rooms,
            traced_rooms
                .iter()
                .map(|room| (
                    room.name.clone(),
                    room.left,
                    room.top,
                    room.right,
                    room.bottom,
                ))
                .collect::<Vec<_>>(),
            "room facts retain builder insertion order"
        );
    }

    #[test]
    fn connection_factory_applies_pinned_subclass_policies() {
        Random::reset_generators();
        Random::push_generator_seeded(0x0C01_1EC7);
        let tunnel = connection::create(0, 5);
        assert_eq!(tunnel.name, "TunnelRoom");
        assert_eq!((tunnel.min_width(), tunnel.min_height()), (3, 3));

        let mut saw_ring = false;
        for id in 1..=100 {
            let connection = connection::create(id, 12);
            if matches!(
                connection.name.as_str(),
                "RingTunnelRoom" | "RingBridgeRoom"
            ) {
                assert_eq!((connection.min_width(), connection.min_height()), (5, 5));
                saw_ring = true;
                break;
            }
        }
        Random::pop_generator();
        assert!(saw_ring);
    }
}
