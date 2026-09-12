//! City `StatuesRoom` and its entrance/exit variants.

use crate::level::carpet::{
    self, CITY_ENTRANCE, CITY_PEDESTAL, CITY_STATUE_BL, CITY_STATUE_BR, CITY_STATUE_TL,
    CITY_STATUE_TR, SKIP,
};
use crate::level::terrain::{
    TerrainMap, CUSTOM_DECO_EMPTY, EMPTY, EMPTY_DECO, ENTRANCE, EXIT, REGION_DECO, STATUE, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::{center, fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, depth: i32) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let rows = (room.width() + 1) / 6;
    let cols = (room.height() + 1) / 6;
    let w = (room.width() - 4 - (rows - 1)) / rows;
    let h = (room.height() - 4 - (cols - 1)) / cols;
    let w_spacing = if rows % 2 == room.width() % 2 { 2 } else { 1 };
    let h_spacing = if cols % 2 == room.height() % 2 { 2 } else { 1 };
    let place_transition =
        (room.kind == RoomKind::Entrance || room.kind == RoomKind::Exit) && rows == 1 && cols == 1;

    for x_index in 0..rows {
        for y_index in 0..cols {
            let left = room.left + 2 + x_index * (w + w_spacing);
            let top = room.top + 2 + y_index * (h + h_spacing);
            fill_rect(map, left, top, left + w - 1, top + h - 1, CUSTOM_DECO_EMPTY);
            set(map, left, top, STATUE);
            set(map, left + w - 1, top, STATUE);
            set(map, left, top + h - 1, STATUE);
            set(map, left + w - 1, top + h - 1, STATUE);

            let mut overrides = vec![
                (0, 0, CITY_STATUE_TL),
                (w - 1, 0, CITY_STATUE_TR),
                (0, h - 1, CITY_STATUE_BL),
                (w - 1, h - 1, CITY_STATUE_BR),
            ];
            if w >= 5 && h >= 5 || place_transition {
                let mut center_x = left + w / 2;
                if w % 2 == 0 && Random::int_max(2) == 0 {
                    center_x -= 1;
                }
                let mut center_y = top + h / 2;
                if h % 2 == 0 && Random::int_max(2) == 0 {
                    center_y -= 1;
                }
                let (terrain, overlay) = if place_transition && room.kind == RoomKind::Entrance {
                    (ENTRANCE, CITY_ENTRANCE)
                } else if place_transition && room.kind == RoomKind::Exit {
                    (EXIT, SKIP)
                } else {
                    (REGION_DECO, CITY_PEDESTAL)
                };
                set(map, center_x, center_y, terrain);
                overrides.push((center_x - left, center_y - top, overlay));
            }
            carpet::add_city(map, left, top, w, h, depth, &overrides);
        }
    }
    paint_transition(map, room, depth);
}

fn paint_transition(map: &mut TerrainMap, room: &Room, depth: i32) {
    let (wanted, overlay) = match room.kind {
        RoomKind::Entrance => (ENTRANCE, CITY_ENTRANCE),
        RoomKind::Exit => (EXIT, SKIP),
        _ => return,
    };
    let cell = if room.width() >= 11 || room.height() >= 11 {
        let point = center(room);
        if let Some(cell) = map.point_to_cell(point.x, point.y) {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if terrain_at(map, point.x + dx, point.y + dy) != Some(STATUE) {
                        set(map, point.x + dx, point.y + dy, EMPTY_DECO);
                    }
                }
            }
            let (left, top, width, height) = large_transition_carpet(room, point);
            carpet::add_city_under(
                map,
                left,
                top,
                width,
                height,
                depth,
                &[(point.x - left, point.y - top, overlay)],
            );
            Some(cell)
        } else {
            None
        }
    } else {
        find_terrain(map, room, wanted).and_then(|(x, y)| map.point_to_cell(x, y))
    };
    let Some(cell) = cell else {
        return;
    };
    map.map[cell] = wanted;
    if room.kind == RoomKind::Exit {
        map.character_allowed[cell] = false;
    }
}

fn large_transition_carpet(room: &Room, point: crate::geom::Point) -> (i32, i32, i32, i32) {
    let mut left = point.x - 1;
    let mut top = point.y - 1;
    let mut right = point.x + 1;
    let mut bottom = point.y + 1;
    if room.width() % 2 == 0 {
        if (point.x as f32) < (room.left + room.right) as f32 / 2.0 {
            right += 1;
        } else {
            left -= 1;
        }
    }
    if room.height() % 2 == 0 {
        if (point.y as f32) < (room.top + room.bottom) as f32 / 2.0 {
            bottom += 1;
        } else {
            top -= 1;
        }
    }
    (left, top, right - left + 1, bottom - top + 1)
}

fn find_terrain(map: &TerrainMap, room: &Room, terrain: i32) -> Option<(i32, i32)> {
    for x in room.left..=room.right {
        for y in room.top..=room.bottom {
            if terrain_at(map, x, y) == Some(terrain) {
                return Some((x, y));
            }
        }
    }
    None
}
