//! City `HallwayRoom` and its entrance/exit variants.

use crate::geom::Point;
use crate::level::terrain::{
    TerrainMap, EMPTY, EMPTY_SP, ENTRANCE_SP, EXIT, REGION_DECO_ALT, STATUE_SP, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::{center, door_points, draw_line, fill_margin, fill_rect, fill_room, set};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let mut connection_center = center(room);
    connection_center.x = connection_center.x.clamp(room.left + 2, room.right - 2);
    connection_center.y = connection_center.y.clamp(room.top + 2, room.bottom - 2);

    for door in door_points(room, room_index, doors) {
        let mut start = door;
        if start.x == room.left {
            start.x += 1;
        } else if start.y == room.top {
            start.y += 1;
        } else if start.x == room.right {
            start.x -= 1;
        } else if start.y == room.bottom {
            start.y -= 1;
        }

        let right_shift = if start.x < connection_center.x - 1 {
            connection_center.x - 1 - start.x
        } else if start.x > connection_center.x + 1 {
            connection_center.x + 1 - start.x
        } else {
            0
        };
        let down_shift = if start.y < connection_center.y - 1 {
            connection_center.y - 1 - start.y
        } else if start.y > connection_center.y + 1 {
            connection_center.y + 1 - start.y
        } else {
            0
        };

        let (mid, end) = if door.x == room.left || door.x == room.right {
            let mid = Point::new(start.x + right_shift, start.y);
            (mid, Point::new(mid.x, mid.y + down_shift))
        } else {
            let mid = Point::new(start.x, start.y + down_shift);
            (mid, Point::new(mid.x + right_shift, mid.y))
        };
        draw_line(map, start, mid, EMPTY_SP);
        draw_line(map, mid, end, EMPTY_SP);
    }

    fill_rect(
        map,
        connection_center.x - 1,
        connection_center.y - 1,
        connection_center.x + 1,
        connection_center.y + 1,
        EMPTY_SP,
    );
    let detail = if Random::int_max(2) == 0 {
        STATUE_SP
    } else {
        REGION_DECO_ALT
    };
    set(map, connection_center.x, connection_center.y, detail);
    paint_transition(map, room);
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let terrain = match room.kind {
        RoomKind::Entrance => ENTRANCE_SP,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    // `getPoints()` is x-major, then y-major. There is exactly one detail tile.
    for x in room.left..=room.right {
        for y in room.top..=room.bottom {
            if matches!(
                super::terrain_at(map, x, y),
                Some(STATUE_SP) | Some(REGION_DECO_ALT)
            ) {
                set(map, x, y, terrain);
                if room.kind == RoomKind::Exit {
                    if let Some(cell) = map.point_to_cell(x, y) {
                        map.character_allowed[cell] = false;
                    }
                }
                return;
            }
        }
    }
}
