//! Pinned SPD connection-room painters.

mod common;
mod maze_connection;
mod perimeter;
mod tunnel;

use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_SP};
use crate::rooms::room::Room;

use super::DoorMap;

pub(crate) fn paint(
    map: &mut TerrainMap,
    rooms: &[Room],
    room_index: usize,
    doors: &DoorMap,
    chasm_feeling: bool,
) {
    let room = &rooms[room_index];
    let floor = if chasm_feeling { EMPTY_SP } else { EMPTY };
    match room.name.as_str() {
        "TunnelRoom" => tunnel::paint(map, room, room_index, doors, floor, false),
        "BridgeRoom" => {
            common::paint_chasm_interior(map, room, false);
            tunnel::paint(map, room, room_index, doors, floor, false);
            common::paint_bridge_neighbours(map, rooms, room);
        }
        "PerimeterRoom" => perimeter::paint(map, room, room_index, doors, floor),
        "WalkwayRoom" => {
            common::paint_chasm_interior(map, room, false);
            perimeter::paint(map, room, room_index, doors, floor);
            common::paint_bridge_neighbours(map, rooms, room);
        }
        "RingTunnelRoom" => tunnel::paint(map, room, room_index, doors, floor, true),
        "RingBridgeRoom" => {
            common::paint_chasm_interior(map, room, true);
            tunnel::paint(map, room, room_index, doors, floor, true);
            common::paint_bridge_neighbours(map, rooms, room);
        }
        "MazeConnectionRoom" => maze_connection::paint(map, room, room_index, doors),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point;
    use crate::level::terrain::{paint_minimal, CHASM};
    use crate::random::Random;
    use crate::rooms::types::RoomKind;

    fn connection(name: &str) -> Room {
        let mut room = Room::new(0, name, RoomKind::Connection, 1, 16, 5, 5, 5, 5);
        room.left = 0;
        room.top = 0;
        room.right = 4;
        room.bottom = 4;
        room
    }

    #[test]
    fn bridge_large_interior_starts_as_chasm() {
        let room = connection("BridgeRoom");
        let mut map = paint_minimal(std::slice::from_ref(&room)).unwrap();
        common::paint_chasm_interior(&mut map, &room, false);
        assert_eq!(common::terrain_at(&map, 2, 2), Some(CHASM));
    }

    #[test]
    fn ring_connection_space_is_three_by_three() {
        Random::reset_generators();
        Random::push_generator_seeded(123);
        let room = connection("RingTunnelRoom");
        let space = tunnel::connection_space(&room, &[Point::new(0, 2), Point::new(4, 2)], true);
        Random::pop_generator();
        assert_eq!((space.raw_width(), space.raw_height()), (2, 2));
        assert_eq!((space.left, space.top), (1, 1));
    }

    fn paint_tail(name: &str, seed: i64) -> i32 {
        let mut room = connection(name);
        let mut map = paint_minimal(std::slice::from_ref(&room)).unwrap();
        room.connected = vec![1, 2];
        let mut doors = DoorMap::new();
        doors.insert_test_point(0, 1, Point::new(0, 2));
        doors.insert_test_point(0, 2, Point::new(4, 2));
        Random::reset_generators();
        Random::push_generator_seeded(seed);
        match name {
            "TunnelRoom" => tunnel::paint(&mut map, &room, 0, &doors, EMPTY, false),
            "RingTunnelRoom" => tunnel::paint(&mut map, &room, 0, &doors, EMPTY, true),
            "MazeConnectionRoom" => maze_connection::paint(&mut map, &room, 0, &doors),
            _ => unreachable!(),
        }
        let tail = Random::int();
        Random::pop_generator();
        tail
    }

    #[test]
    fn connection_painters_keep_pinned_rng_tails() {
        assert_eq!(paint_tail("TunnelRoom", 0x7100), 1_748_705_672);
        assert_eq!(paint_tail("RingTunnelRoom", 0x7100), 1_748_705_672);
        assert_eq!(paint_tail("MazeConnectionRoom", 0x7100), 913_966_729);
    }
}
