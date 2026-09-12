//! Connection overrides that would otherwise bloat `room.rs` tests.

use super::room::Room;
use crate::geom::Point;
use crate::rooms::types::RoomKind;

fn ritual_site_10x10() -> Room {
    let mut room = Room::new(
        0,
        "RitualSiteRoom",
        RoomKind::Standard,
        1,
        16,
        10,
        10,
        10,
        10,
    );
    room.left = 2;
    room.top = 3;
    room.right = 11;
    room.bottom = 12;
    room
}

#[test]
fn ritual_site_rejects_top_wall_columns() {
    let room = ritual_site_10x10();
    assert!(!room.can_connect_point(Point::new(room.left + 3, room.top)));
    assert!(!room.can_connect_point(Point::new(room.left + 6, room.top)));
    for x in [1, 2, 4, 5, 7, 8] {
        assert!(
            room.can_connect_point(Point::new(room.left + x, room.top)),
            "top x offset {x}"
        );
    }
    assert!(!room.can_connect_point(Point::new(room.left, room.top)));
    assert!(!room.can_connect_point(Point::new(room.right, room.top)));
    assert!(room.can_connect_point(Point::new(room.left, room.top + 4)));
    assert!(room.can_connect_point(Point::new(room.right, room.bottom - 3)));
    assert!(room.can_connect_point(Point::new(room.left + 4, room.bottom)));
}
