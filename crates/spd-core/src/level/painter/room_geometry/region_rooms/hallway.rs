//! City `HallwayRoom` and its entrance/exit variants.

use crate::geom::Point;
use crate::level::carpet::{self, CITY_ENTRANCE, CITY_PEDESTAL, CITY_STATUE, SKIP};
use crate::level::terrain::{
    TerrainMap, EMPTY, EMPTY_SP, ENTRANCE_SP, EXIT, REGION_DECO_ALT, STATUE_SP, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::{center, door_points, draw_line, fill_margin, fill_rect, fill_room, set};

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
    depth: i32,
) {
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
    // Entrance/exit skip the statue-vs-pedestal roll so the city stream stays aligned.
    let (detail, overlay) = if room.kind == RoomKind::Entrance {
        (ENTRANCE_SP, CITY_ENTRANCE)
    } else if room.kind == RoomKind::Exit {
        (EXIT, SKIP)
    } else if Random::int_max(2) == 0 {
        (STATUE_SP, CITY_STATUE)
    } else {
        (REGION_DECO_ALT, CITY_PEDESTAL)
    };
    set(map, connection_center.x, connection_center.y, detail);
    carpet::add_city(
        map,
        connection_center.x - 1,
        connection_center.y - 1,
        3,
        3,
        depth,
        &[(1, 1, overlay)],
    );
    if room.kind == RoomKind::Exit {
        if let Some(cell) = map.point_to_cell(connection_center.x, connection_center.y) {
            map.character_allowed[cell] = false;
        }
    }
}
