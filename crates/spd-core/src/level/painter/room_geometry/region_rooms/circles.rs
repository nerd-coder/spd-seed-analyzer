//! Cave circular pit/wall rooms and circle-wall transitions.

use crate::geom::Point;
use crate::level::terrain::{
    TerrainMap, CHASM, EMPTY, EMPTY_SP, ENTRANCE, EXIT, REGION_DECO_ALT, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::{center, door_points, draw_inside, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    match room.name.as_str() {
        "CirclePitRoom" => paint_pit(map, room, room_index, doors),
        _ => paint_wall(map, room, room_index, doors),
    }
}

fn paint_pit(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    paint_circle_base(map, room, room_index, doors);
    fill_ellipse(map, room, 3, CHASM);

    if room.size_factor != 1 && Random::int_max(4 - room.size_factor) == 0 {
        let mut center = center(room);
        center.x += Random::int_range_inclusive(-1, 1);
        center.y += Random::int_range_inclusive(-1, 1);
        let mut edge = center;
        match Random::int_max(4) {
            0 => edge.x = room.left,
            1 => edge.y = room.top,
            2 => edge.x = room.right,
            _ => edge.y = room.bottom,
        }
        if !door_points(room, room_index, doors).contains(&edge) {
            draw_line(map, edge, center, REGION_DECO_ALT);
            draw_inside(map, room, edge, 1, EMPTY_SP);
            set(map, edge.x, edge.y, WALL);
        }
    }
}

fn paint_wall(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    paint_circle_base(map, room, room_index, doors);
    fill_ellipse(map, room, 3, WALL);

    if matches!(room.kind, RoomKind::Entrance | RoomKind::Exit) {
        paint_center_transition(map, room);
    }
}

fn paint_circle_base(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_ellipse(map, room, 1, EMPTY);
    for door in door_points(room, room_index, doors) {
        let distance = if door.x == room.left || door.x == room.right {
            room.width() / 2
        } else {
            room.height() / 2
        };
        draw_inside(map, room, door, distance, EMPTY);
    }
}

pub(super) fn fill_ellipse(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    let x = room.left + margin;
    let y = room.top + margin;
    let width = room.width() - margin * 2;
    let height = room.height() - margin * 2;
    if width <= 0 || height <= 0 {
        return;
    }
    let radius_h = height as f64 / 2.0;
    let radius_w = width as f64 / 2.0;
    for row in 0..height {
        let row_y = -radius_h + 0.5 + row as f64;
        let mut row_width = 2.0
            * ((radius_w * radius_w) * (1.0 - row_y * row_y / (radius_h * radius_h)))
                .max(0.0)
                .sqrt();
        if width % 2 == 0 {
            row_width = (row_width / 2.0).round() * 2.0;
        } else {
            row_width = (row_width / 2.0).floor() * 2.0 + 1.0;
        }
        let row_width = row_width as i32;
        let start = x + (width - row_width) / 2;
        for column in start..(start + row_width) {
            set(map, column, y + row, terrain);
        }
    }
}

pub(super) fn draw_line(map: &mut TerrainMap, from: Point, to: Point, terrain: i32) {
    let mut x = from.x as f32;
    let mut y = from.y as f32;
    let mut dx = (to.x - from.x) as f32;
    let mut dy = (to.y - from.y) as f32;
    let moving_by_x = dx.abs() >= dy.abs();
    if moving_by_x {
        dy /= dx.abs();
        dx /= dx.abs();
    } else {
        dx /= dy.abs();
        dy /= dy.abs();
    }
    set(map, java_round(x), java_round(y), terrain);
    while (moving_by_x && to.x as f32 != x) || (!moving_by_x && to.y as f32 != y) {
        x += dx;
        y += dy;
        set(map, java_round(x), java_round(y), terrain);
    }
}

fn java_round(value: f32) -> i32 {
    (value + 0.5).floor() as i32
}

fn paint_center_transition(map: &mut TerrainMap, room: &Room) {
    let mut p = center(room);
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if terrain_at(map, p.x + 2 * dx, p.y + 2 * dy) == Some(WALL) {
                set(map, p.x + dx, p.y + dy, EMPTY);
            }
        }
    }
    set(
        map,
        p.x,
        p.y,
        if room.kind == RoomKind::Entrance {
            ENTRANCE
        } else {
            EXIT
        },
    );

    let (dx, dy) = if Random::int_max(2) == 0 {
        (if Random::int_max(2) == 0 { 1 } else { -1 }, 0)
    } else {
        (0, if Random::int_max(2) == 0 { 1 } else { -1 })
    };
    p.x += 2 * dx;
    p.y += 2 * dy;
    while terrain_at(map, p.x, p.y) == Some(WALL) {
        set(map, p.x, p.y, EMPTY);
        p.x += dx;
        p.y += dy;
    }
}
