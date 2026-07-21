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

/// Place rooms with one Java builder instance. Failed inner attempts retain
/// FigureEightBuilder's selected landmark, while appended connection rooms are discarded.
pub fn build_rooms(
    rooms: &mut Vec<Room>,
    kind: BuilderKind,
    intensity: f32,
    offset: f32,
    depth: i32,
    max_tries: u32,
) -> bool {
    let params = BuilderParams {
        curve_exponent: 2,
        curve_intensity: intensity % 1.0,
        curve_offset: offset % 0.5,
        ..Default::default()
    };

    let mut figure_state = figure_eight::FigureEightState::default();
    for _ in 0..max_tries {
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
            BuilderKind::Loop => loop_builder::build(rooms, &params, depth).is_some(),
            BuilderKind::FigureEight => {
                figure_eight::build(rooms, &params, depth, &mut figure_state).is_some()
            }
        };
        if ok {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;
    use crate::rooms::room::dims_for_kind;
    use crate::rooms::types::RoomKind;

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
        let built = build_rooms(&mut rooms, BuilderKind::FigureEight, 0.55, 0.0, 12, 1_000);
        let rng_tail = Random::int();
        Random::pop_generator();

        assert!(built);
        assert!(rooms[..9].iter().all(|room| !room.connected.is_empty()));
        let vertices = rooms.iter().filter(|room| !room.is_empty()).count();
        let edges = rooms.iter().map(|room| room.connected.len()).sum::<usize>() / 2;
        assert!(edges + 1 >= vertices + 2, "expected two independent loops");
        assert_eq!(rng_tail, 1_744_841_133);
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
