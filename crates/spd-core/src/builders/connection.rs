//! Connection-room selection and subclass construction.

use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

pub(super) fn create(id: usize, depth: i32) -> Room {
    let names = [
        "TunnelRoom",
        "BridgeRoom",
        "PerimeterRoom",
        "WalkwayRoom",
        "RingTunnelRoom",
        "RingBridgeRoom",
    ];
    let chances: &[f32] = match depth {
        1..=4 => &[20., 1., 0., 2., 2., 1.],
        5 | 21 => &[20., 0., 0., 0., 0., 0.],
        6..=10 => &[0., 0., 22., 3., 0., 0.],
        11..=15 => &[12., 0., 0., 5., 5., 3.],
        16..=20 => &[0., 0., 18., 3., 3., 1.],
        _ => &[15., 4., 0., 2., 3., 2.],
    };
    let name = names[Random::chances(chances) as usize];
    let min = if matches!(name, "RingTunnelRoom" | "RingBridgeRoom") {
        5
    } else {
        3
    };
    Room::new(id, name, RoomKind::Connection, 1, 16, min, 10, min, 10)
}

pub(super) fn maze(id: usize) -> Room {
    Room::new(
        id,
        "MazeConnectionRoom",
        RoomKind::Connection,
        1,
        2,
        3,
        10,
        3,
        10,
    )
}
