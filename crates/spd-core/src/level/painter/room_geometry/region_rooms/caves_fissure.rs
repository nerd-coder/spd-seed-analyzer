//! `CavesFissureRoom` and its entrance/exit variants.

use std::f64::consts::PI;

use crate::level::terrain::{TerrainMap, CHASM, EMPTY, EMPTY_SP, ENTRANCE, EXIT, WALL};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::pathing::{all_floor_reachable, room_cell_from_door};
use super::{center, door_points, draw_inside, fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    let doors = door_points(room, room_index, doors);
    for _ in 0..10_000 {
        fill_room(map, room, WALL);
        fill_margin(map, room, 1, EMPTY);

        let center_point = center(room);
        let center = (center_point.x as f32 + 0.5, center_point.y as f32 + 0.5);
        let door_angles: Vec<f32> = doors
            .iter()
            .map(|door| angle_between(center, (door.x as f32 + 0.5, door.y as f32 + 0.5)))
            .collect();
        let line_angles = generate_line_angles(room, &door_angles);
        if line_angles.len() < 2 {
            paint_transition(map, room);
            return;
        }

        for &angle in &line_angles {
            paint_fissure_line(map, angle, center, room.size_factor == 3);
        }
        if line_angles.len() >= 3 {
            let radius = if room.size_factor == 3 { 2 } else { 1 };
            fill_rect(
                map,
                center.0.floor() as i32 - radius,
                center.1.floor() as i32 - radius,
                center.0.floor() as i32 + radius,
                center.1.floor() as i32 + radius,
                CHASM,
            );
        }

        if line_angles.len() == 2 {
            let angle = line_angles[Random::int_max(line_angles.len() as i32) as usize];
            build_bridge(map, room, angle, center, 1);
        } else {
            for &angle in &line_angles {
                build_bridge(map, room, angle, center, room.size_factor);
            }
        }

        let mut door_point = 0;
        for &door in &doors {
            draw_inside(map, room, door, 1, EMPTY);
            door_point = room_cell_from_door(room, door);
        }
        if all_floor_reachable(map, room, door_point) {
            paint_transition(map, room);
            apply_place_masks(map, room);
            return;
        }
    }
}

fn apply_place_masks(map: &mut TerrainMap, room: &Room) {
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            if terrain_at(map, x, y) == Some(EMPTY_SP) {
                if let Some(cell) = map.point_to_cell(x, y) {
                    map.item_allowed[cell] = false;
                    map.character_allowed[cell] = false;
                }
            }
        }
    }
}

fn generate_line_angles(room: &Room, door_angles: &[f32]) -> Vec<f32> {
    let mut line_angles = Vec::new();
    let num_lines = 1 + room.size_factor;
    for _ in 0..num_lines {
        let mut tries = 100;
        loop {
            let line_angle = Random::float_range(0.0, 360.0);
            let far_from_doors = door_angles.iter().all(|&door_angle| {
                angle_difference(line_angle, door_angle)
                    > if room.size_factor == 1 { 30.0 } else { 15.0 }
            });
            let far_from_lines = line_angles.iter().all(|&existing| {
                angle_difference(line_angle, existing) > if num_lines == 2 { 120.0 } else { 60.0 }
            });
            if far_from_doors && far_from_lines {
                line_angles.push(line_angle);
                break;
            }
            tries -= 1;
            if tries == 0 {
                break;
            }
        }
    }
    line_angles
}

fn angle_difference(a: f32, b: f32) -> f32 {
    let difference = (a - b).abs();
    if difference > 180.0 {
        360.0 - difference
    } else {
        difference
    }
}

fn angle_between(from: (f32, f32), to: (f32, f32)) -> f32 {
    let slope = (to.1 - from.1) / (to.0 - from.0);
    let mut angle = ((180.0 / PI) * ((slope as f64).atan() + PI / 2.0)) as f32;
    if from.0 > to.0 {
        angle -= 180.0;
    }
    if angle < 0.0 {
        angle += 360.0;
    }
    angle
}

fn fissure_vector(angle: f32) -> (f32, f32, bool) {
    let angle_scale = 180.0 / PI;
    let radians = angle as f64 / angle_scale - PI / 2.0;
    let mut dx = radians.cos() as f32;
    let mut dy = radians.sin() as f32;
    let horizontal = dx.abs() >= dy.abs();
    if horizontal {
        dy /= dx.abs();
        dx /= dx.abs();
    } else {
        dx /= dy.abs();
        dy /= dy.abs();
    }
    (dx, dy, horizontal)
}

fn paint_fissure_line(map: &mut TerrainMap, angle: f32, center: (f32, f32), giant: bool) {
    let (dx, dy, horizontal) = fissure_vector(angle);
    let mut current = center;
    set(
        map,
        current.0.floor() as i32,
        current.1.floor() as i32,
        CHASM,
    );
    loop {
        let x = current.0.floor() as i32;
        let y = current.1.floor() as i32;
        if !horizontal {
            if terrain_at(map, x - 1, y) == Some(EMPTY)
                && (current.0.rem_euclid(1.0) <= 0.5 || giant)
            {
                set(map, x - 1, y, CHASM);
            }
            if terrain_at(map, x, y) == Some(EMPTY) {
                set(map, x, y, CHASM);
            }
            if terrain_at(map, x + 1, y) == Some(EMPTY)
                && (current.0.rem_euclid(1.0) > 0.5 || giant)
            {
                set(map, x + 1, y, CHASM);
            }
        } else {
            if terrain_at(map, x, y - 1) == Some(EMPTY)
                && (current.1.rem_euclid(1.0) <= 0.5 || giant)
            {
                set(map, x, y - 1, CHASM);
            }
            if terrain_at(map, x, y) == Some(EMPTY) {
                set(map, x, y, CHASM);
            }
            if terrain_at(map, x, y + 1) == Some(EMPTY)
                && (current.1.rem_euclid(1.0) > 0.5 || giant)
            {
                set(map, x, y + 1, CHASM);
            }
        }
        current.0 += dx;
        current.1 += dy;
        if !matches!(
            terrain_at(map, current.0.floor() as i32, current.1.floor() as i32),
            Some(EMPTY) | Some(CHASM)
        ) {
            break;
        }
    }
}

fn build_bridge(
    map: &mut TerrainMap,
    room: &Room,
    angle: f32,
    center: (f32, f32),
    center_margin: i32,
) {
    let (dx, dy, _) = fissure_vector(angle);
    let edge_margin = 2;
    if dy.abs() >= dx.abs() {
        let y = if dy > 0.0 {
            Random::int_range_inclusive(
                center.1.floor() as i32 + center_margin,
                room.bottom - edge_margin,
            )
        } else {
            Random::int_range_inclusive(
                room.top + edge_margin,
                center.1.floor() as i32 - center_margin,
            )
        };
        if dx <= 0.0 {
            bridge_row(map, room.left + 1..=room.right - 1, y);
        } else {
            bridge_row(map, (room.left + 1..=room.right - 1).rev(), y);
        }
    } else {
        let x = if dx > 0.0 {
            Random::int_range_inclusive(
                center.0.floor() as i32 + center_margin,
                room.right - edge_margin,
            )
        } else {
            Random::int_range_inclusive(
                room.left + edge_margin,
                center.0.floor() as i32 - center_margin,
            )
        };
        if dy <= 0.0 {
            bridge_column(map, x, room.top + 1..=room.bottom - 1);
        } else {
            bridge_column(map, x, (room.top + 1..=room.bottom - 1).rev());
        }
    }
}

fn bridge_row<I: Iterator<Item = i32>>(map: &mut TerrainMap, xs: I, y: i32) {
    let mut found_chasm = false;
    for x in xs {
        if terrain_at(map, x, y) == Some(CHASM) {
            found_chasm = true;
            set(map, x, y, EMPTY_SP);
        } else if found_chasm {
            break;
        }
    }
}

fn bridge_column<I: Iterator<Item = i32>>(map: &mut TerrainMap, x: i32, ys: I) {
    let mut found_chasm = false;
    for y in ys {
        if terrain_at(map, x, y) == Some(CHASM) {
            found_chasm = true;
            set(map, x, y, EMPTY_SP);
        } else if found_chasm {
            break;
        }
    }
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let terrain = match room.kind {
        RoomKind::Entrance => ENTRANCE,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    for _ in 0..10_000 {
        let p = room.random_margin(2);
        if matches!(terrain_at(map, p.x, p.y), Some(CHASM) | Some(EMPTY_SP)) {
            continue;
        }
        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            if terrain_at(map, p.x + dx, p.y + dy) == Some(CHASM) {
                set(map, p.x + dx, p.y + dy, EMPTY);
            }
        }
        set(map, p.x, p.y, terrain);
        return;
    }
}
