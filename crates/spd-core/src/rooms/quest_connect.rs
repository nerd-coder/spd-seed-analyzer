//! Quest-room connection predicates (pinned SPD v4).

use super::room::{can_connect_rooms, Room, DIR_ALL, DIR_LEFT, DIR_TOP};
use super::types::RoomKind;
use crate::geom::Point;

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

fn blacksmith_8x8() -> Room {
    let mut room = Room::new(0, "BlacksmithRoom", RoomKind::Standard, 1, 16, 8, 10, 8, 10);
    room.left = 0;
    room.top = 0;
    room.right = 7;
    room.bottom = 7;
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

#[test]
fn blacksmith_top_edge_connects_only_at_corners() {
    let smith = blacksmith_8x8();
    assert_eq!(smith.max_connections(DIR_TOP), 1);
    assert_eq!(smith.max_connections(DIR_LEFT), 4);
    assert_eq!(smith.max_connections(DIR_ALL), 16);

    assert!(smith.can_connect_point(Point::new(1, 0)));
    assert!(smith.can_connect_point(Point::new(6, 0)));
    assert!(!smith.can_connect_point(Point::new(0, 0)));
    assert!(!smith.can_connect_point(Point::new(3, 0)));
    assert!(!smith.can_connect_point(Point::new(7, 0)));
    assert!(!smith.can_connect_point(Point::new(0, 1)));
    assert!(!smith.can_connect_point(Point::new(7, 1)));
    assert!(smith.can_connect_point(Point::new(0, 3)));
    assert!(smith.can_connect_point(Point::new(3, 7)));
}

#[test]
fn blacksmith_does_not_connect_to_exit() {
    let smith = blacksmith_8x8();
    let mut neighbor = Room::new(1, "CaveRoom", RoomKind::Standard, 1, 16, 5, 10, 5, 10);
    neighbor.left = 0;
    neighbor.top = 7;
    neighbor.right = 7;
    neighbor.bottom = 14;
    let rooms = [smith.clone(), neighbor.clone()];
    assert!(can_connect_rooms(&rooms[0], &rooms[1], &rooms));
    assert!(can_connect_rooms(&rooms[1], &rooms[0], &rooms));

    let mut exit = Room::new(1, "CaveExitRoom", RoomKind::Exit, 1, 16, 5, 10, 5, 10);
    exit.left = 0;
    exit.top = -6;
    exit.right = 7;
    exit.bottom = 0;
    let blocked = [smith, exit];
    assert!(!can_connect_rooms(&blocked[0], &blocked[1], &blocked));
    assert!(!can_connect_rooms(&blocked[1], &blocked[0], &blocked));
}
