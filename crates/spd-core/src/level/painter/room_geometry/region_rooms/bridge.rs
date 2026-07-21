//! `RegionDecoBridgeRoom` and its cave entrance/exit variants.

use crate::geom::{Point, Rect};
use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_SP, ENTRANCE, EXIT, REGION_DECO_ALT, WALL};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::{door_points, fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let door_points = door_points(room, room_index, doors);
    let mut doors_xy = 0;
    for door in &door_points {
        if door.x == room.left || door.x == room.right {
            doors_xy += 1;
        } else {
            doors_xy -= 1;
        }
    }
    doors_xy += (room.width() - room.height()) / 2;

    let horizontal_space = doors_xy > 0 || (doors_xy == 0 && Random::int_max(2) == 0);
    let (space, bridge) = if horizontal_space {
        horizontal_rects(room, &door_points)
    } else {
        vertical_rects(room, &door_points)
    };

    fill_rect(
        map,
        space.left,
        space.top,
        space.right,
        space.bottom,
        REGION_DECO_ALT,
    );
    fill_rect(
        map,
        bridge.left,
        bridge.top,
        bridge.right,
        bridge.bottom,
        EMPTY_SP,
    );
    for y in space.top..=space.bottom {
        for x in space.left..=space.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.item_allowed[cell] = false;
            }
        }
    }

    if matches!(room.kind, RoomKind::Entrance | RoomKind::Exit) {
        paint_transition(map, room, space);
    }
}

fn horizontal_rects(room: &Room, doors: &[Point]) -> (Rect, Rect) {
    let mut points: Vec<i32> = doors
        .iter()
        .filter(|door| door.y == room.top || door.y == room.bottom)
        .map(|door| door.x)
        .collect();
    points.extend([room.left + 1, room.right - 1]);
    points.sort_unstable();
    let (mut start, mut end) = widest_gap(&points);
    while end - start > 2 {
        if Random::int_max(2) == 0 {
            start += 1;
        } else {
            end -= 1;
        }
    }
    let space = Rect {
        left: start + 1,
        top: room.top + 1,
        right: end - 1,
        bottom: room.bottom - 1,
    };
    let bridge_y = Random::normal_int_range(space.top + 1, space.bottom - 1);
    let bridge = Rect {
        left: space.left,
        top: bridge_y,
        right: space.right,
        bottom: bridge_y,
    };
    (space, bridge)
}

fn vertical_rects(room: &Room, doors: &[Point]) -> (Rect, Rect) {
    let mut points: Vec<i32> = doors
        .iter()
        .filter(|door| door.x == room.left || door.x == room.right)
        .map(|door| door.y)
        .collect();
    points.extend([room.top + 1, room.bottom - 1]);
    points.sort_unstable();
    let (mut start, mut end) = widest_gap(&points);
    while end - start > 2 {
        if Random::int_max(2) == 0 {
            start += 1;
        } else {
            end -= 1;
        }
    }
    let space = Rect {
        left: room.left + 1,
        top: start + 1,
        right: room.right - 1,
        bottom: end - 1,
    };
    let bridge_x = Random::normal_int_range(space.left + 1, space.right - 1);
    let bridge = Rect {
        left: bridge_x,
        top: space.top,
        right: bridge_x,
        bottom: space.bottom,
    };
    (space, bridge)
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

fn paint_transition(map: &mut TerrainMap, room: &Room, space: Rect) {
    let terrain = if room.kind == RoomKind::Entrance {
        ENTRANCE
    } else {
        EXIT
    };
    for _ in 0..10_000 {
        let p = room.random_margin(2);
        if p.x >= space.left && p.x <= space.right && p.y >= space.top && p.y <= space.bottom {
            continue;
        }
        let touches_deco = (-1..=1).any(|dy| {
            (-1..=1).any(|dx| {
                (dx != 0 || dy != 0) && terrain_at(map, p.x + dx, p.y + dy) == Some(REGION_DECO_ALT)
            })
        });
        if !touches_deco {
            set(map, p.x, p.y, terrain);
            return;
        }
    }
}
