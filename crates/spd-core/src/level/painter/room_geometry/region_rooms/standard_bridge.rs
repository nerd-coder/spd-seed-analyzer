//! Shared `StandardBridgeRoom.paint` layout.

use crate::geom::{Point, Rect};
use crate::random::Random;
use crate::rooms::room::Room;

#[derive(Clone, Copy)]
pub(super) struct BridgeLayout {
    /// Cells actually painted by `Painter.fill(spaceRect, ...)`.
    pub space: Rect,
    /// Cells actually painted by `Painter.fill(bridgeRect, ...)`.
    pub bridge: Rect,
    /// Java `Rect` coordinates, whose right/bottom bounds are exclusive to fill
    /// but used by `Rect.inside` for placement policy.
    pub logical_space: Rect,
}

pub(super) fn layout(
    room: &Room,
    doors: &[Point],
    horizontal_max_bridge_width: i32,
    vertical_max_bridge_width: i32,
) -> BridgeLayout {
    let mut doors_xy = 0;
    for door in doors {
        if door.x == room.left || door.x == room.right {
            doors_xy += 1;
        } else {
            doors_xy -= 1;
        }
    }
    doors_xy += (room.width() - room.height()) / 2;

    if doors_xy > 0 || (doors_xy == 0 && Random::int_max(2) == 0) {
        horizontal(room, doors, horizontal_max_bridge_width)
    } else {
        vertical(room, doors, vertical_max_bridge_width)
    }
}

fn horizontal(room: &Room, doors: &[Point], max_bridge_width: i32) -> BridgeLayout {
    let mut points: Vec<i32> = doors
        .iter()
        .filter(|door| door.y == room.top || door.y == room.bottom)
        .map(|door| door.x)
        .collect();
    points.extend([room.left + 1, room.right - 1]);
    points.sort_unstable();
    let (mut start, mut end) = widest_gap(&points);
    while end - start > max_bridge_width + 1 {
        if Random::int_max(2) == 0 {
            start += 1;
        } else {
            end -= 1;
        }
    }

    let logical_space = Rect {
        left: start + 1,
        top: room.top + 1,
        right: end,
        bottom: room.bottom,
    };
    let space = exclusive_rect(logical_space);
    let bridge_y = Random::normal_int_range(logical_space.top + 1, logical_space.bottom - 2);
    BridgeLayout {
        space,
        bridge: Rect {
            left: space.left,
            top: bridge_y,
            right: space.right,
            bottom: bridge_y,
        },
        logical_space,
    }
}

fn vertical(room: &Room, doors: &[Point], max_bridge_width: i32) -> BridgeLayout {
    let mut points: Vec<i32> = doors
        .iter()
        .filter(|door| door.x == room.left || door.x == room.right)
        .map(|door| door.y)
        .collect();
    points.extend([room.top + 1, room.bottom - 1]);
    points.sort_unstable();
    let (mut start, mut end) = widest_gap(&points);
    while end - start > max_bridge_width + 1 {
        if Random::int_max(2) == 0 {
            start += 1;
        } else {
            end -= 1;
        }
    }

    let logical_space = Rect {
        left: room.left + 1,
        top: start + 1,
        right: room.right,
        bottom: end,
    };
    let space = exclusive_rect(logical_space);
    let bridge_x = Random::normal_int_range(logical_space.left + 1, logical_space.right - 2);
    BridgeLayout {
        space,
        bridge: Rect {
            left: bridge_x,
            top: space.top,
            right: bridge_x,
            bottom: space.bottom,
        },
        logical_space,
    }
}

fn widest_gap(points: &[i32]) -> (i32, i32) {
    let mut widest = (0, 0);
    for pair in points.windows(2) {
        if widest.1 - widest.0 < pair[1] - pair[0] {
            widest = (pair[0], pair[1]);
        }
    }
    widest
}

fn exclusive_rect(rect: Rect) -> Rect {
    Rect {
        right: rect.right - 1,
        bottom: rect.bottom - 1,
        ..rect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rooms::types::RoomKind;

    fn room(width: i32, height: i32) -> Room {
        let mut room = Room::new(
            0,
            "WaterBridgeRoom",
            RoomKind::Standard,
            1,
            16,
            5,
            14,
            5,
            14,
        );
        room.resize(width - 1, height - 1);
        room
    }

    #[test]
    fn asymmetric_bridge_uses_width_for_horizontal_limit() {
        let room = room(7, 12);
        let doors = [
            Point::new(room.left, 2),
            Point::new(room.left, 4),
            Point::new(room.right, 6),
            Point::new(room.right, 8),
            Point::new(room.right, 10),
        ];
        Random::push_generator_seeded(0xB41D63);
        let layout = layout(&room, &doors, 2, 3);
        Random::pop_generator();
        assert!(layout.space.raw_width() < 2);
        assert_eq!(layout.space.raw_height() + 1, room.height() - 2);
    }

    #[test]
    fn asymmetric_bridge_uses_height_for_vertical_limit() {
        let room = room(12, 7);
        let doors = [
            Point::new(2, room.top),
            Point::new(4, room.top),
            Point::new(6, room.bottom),
            Point::new(8, room.bottom),
            Point::new(10, room.bottom),
        ];
        Random::push_generator_seeded(0xB41D63);
        let layout = layout(&room, &doors, 3, 2);
        Random::pop_generator();
        assert!(layout.space.raw_height() < 2);
        assert_eq!(layout.space.raw_width() + 1, room.width() - 2);
    }
}
