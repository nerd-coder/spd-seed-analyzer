//! Platform, fissure, striped, study, and suspicious-chest geometry.

use crate::geom::{Point, Rect};
use crate::level::terrain::{
    TerrainMap, BOOKSHELF, CHASM, EMPTY, EMPTY_SP, HIGH_GRASS, PEDESTAL, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;

use super::super::super::DoorMap;
use super::{center, draw_inside, fill_margin, fill_rect, fill_room, set};

pub(super) fn paint_platform(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, CHASM);

    let mut platforms = Vec::new();
    split_platforms(
        Rect {
            left: room.left + 2,
            top: room.top + 2,
            right: room.right - 2,
            bottom: room.bottom - 2,
        },
        &mut platforms,
    );
    for platform in platforms {
        fill_rect(map, platform, EMPTY_SP);
    }

    for &other in &room.connected {
        if let Some(door) = doors.get(room_index, other) {
            draw_inside(map, room, Point::new(door.x, door.y), 2, EMPTY_SP);
        }
    }
}

fn split_platforms(platform: Rect, all: &mut Vec<Rect>) {
    let area = (platform.raw_width() + 1) * (platform.raw_height() + 1);
    if Random::float() < (area - 25) as f32 / 11.0 {
        if platform.raw_width() > platform.raw_height()
            || (platform.raw_width() == platform.raw_height() && Random::int_max(2) == 0)
        {
            let split_x = Random::int_range_inclusive(platform.left + 2, platform.right - 2);
            split_platforms(
                Rect {
                    left: platform.left,
                    top: platform.top,
                    right: split_x - 1,
                    bottom: platform.bottom,
                },
                all,
            );
            split_platforms(
                Rect {
                    left: split_x + 1,
                    top: platform.top,
                    right: platform.right,
                    bottom: platform.bottom,
                },
                all,
            );
            let bridge_y = Random::normal_int_range(platform.top, platform.bottom);
            all.push(Rect {
                left: split_x - 1,
                top: bridge_y,
                right: split_x + 1,
                bottom: bridge_y,
            });
        } else {
            let split_y = Random::int_range_inclusive(platform.top + 2, platform.bottom - 2);
            split_platforms(
                Rect {
                    left: platform.left,
                    top: platform.top,
                    right: platform.right,
                    bottom: split_y - 1,
                },
                all,
            );
            split_platforms(
                Rect {
                    left: platform.left,
                    top: split_y + 1,
                    right: platform.right,
                    bottom: platform.bottom,
                },
                all,
            );
            let bridge_x = Random::normal_int_range(platform.left, platform.right);
            all.push(Rect {
                left: bridge_x,
                top: split_y - 1,
                right: bridge_x,
                bottom: split_y + 1,
            });
        }
    } else {
        all.push(platform);
    }
}

pub(super) fn paint_fissure(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    if room.square() <= 25 {
        let point = center(room);
        set(map, point.x, point.y, CHASM);
        return;
    }

    let smallest = room.width().min(room.height());
    let sqrt = (smallest as f64).sqrt() as f32;
    let floor_width = sqrt as i32;
    let edge_floor_chance = (sqrt % 1.0 + (floor_width - 1) as f32 * 0.5) / floor_width as f32;
    for y in (room.top + 2)..=(room.bottom - 2) {
        for x in (room.left + 2)..=(room.right - 2) {
            let vertical = (y - room.top).min(room.bottom - y);
            let horizontal = (x - room.left).min(room.right - x);
            let edge = vertical.min(horizontal);
            if edge > floor_width || (edge == floor_width && Random::float() > edge_floor_chance) {
                set(map, x, y, CHASM);
            }
        }
    }
}

pub(super) fn paint_striped(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    if room.size_factor == 1 {
        fill_margin(map, room, 1, EMPTY_SP);
        if room.width() > room.height()
            || (room.width() == room.height() && Random::int_max(2) == 0)
        {
            for x in (room.left + 2..room.right).step_by(2) {
                fill_size(map, x, room.top + 1, 1, room.height() - 2, HIGH_GRASS);
            }
        } else {
            for y in (room.top + 2..room.bottom).step_by(2) {
                fill_size(map, room.left + 1, y, room.width() - 2, 1, HIGH_GRASS);
            }
        }
    } else if room.size_factor == 2 {
        let layers = (room.width().min(room.height()) - 1) / 2;
        for margin in 1..=layers {
            fill_margin(
                map,
                room,
                margin,
                if margin % 2 == 1 {
                    EMPTY_SP
                } else {
                    HIGH_GRASS
                },
            );
        }
    }
}

pub(super) fn paint_study(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) -> Point {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, BOOKSHELF);
    fill_margin(map, room, 2, EMPTY_SP);
    for &other in &room.connected {
        if let Some(door) = doors.get(room_index, other) {
            draw_inside(map, room, Point::new(door.x, door.y), 2, EMPTY_SP);
        }
    }

    if room.size_factor == 2 {
        let pillar_width = (room.width() - 7) / 2;
        let pillar_height = (room.height() - 7) / 2;
        fill_size(map, room.left + 3, room.top + 3, pillar_width, 1, BOOKSHELF);
        fill_size(
            map,
            room.left + 3,
            room.top + 3,
            1,
            pillar_height,
            BOOKSHELF,
        );
        fill_size(
            map,
            room.left + 3,
            room.bottom - 3,
            pillar_width,
            1,
            BOOKSHELF,
        );
        fill_size(
            map,
            room.left + 3,
            room.bottom - 2 - pillar_height,
            1,
            pillar_height,
            BOOKSHELF,
        );
        fill_size(
            map,
            room.right - 2 - pillar_width,
            room.top + 3,
            pillar_width,
            1,
            BOOKSHELF,
        );
        fill_size(
            map,
            room.right - 3,
            room.top + 3,
            1,
            pillar_height,
            BOOKSHELF,
        );
        fill_size(
            map,
            room.right - 2 - pillar_width,
            room.bottom - 3,
            pillar_width,
            1,
            BOOKSHELF,
        );
        fill_size(
            map,
            room.right - 3,
            room.bottom - 2 - pillar_height,
            1,
            pillar_height,
            BOOKSHELF,
        );
    }

    let center = center(room);
    set(map, center.x, center.y, PEDESTAL);
    if let Some(cell) = map.point_to_cell(center.x, center.y) {
        map.item_allowed[cell] = false;
    }
    center
}

pub(super) fn paint_suspicious_chest_base(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);
}

fn fill_size(map: &mut TerrainMap, x: i32, y: i32, width: i32, height: i32, terrain: i32) {
    if width <= 0 || height <= 0 {
        return;
    }
    fill_rect(
        map,
        Rect {
            left: x,
            top: y,
            right: x + width - 1,
            bottom: y + height - 1,
        },
        terrain,
    );
}
