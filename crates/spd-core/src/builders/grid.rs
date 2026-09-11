//! Pinned `GridBuilder` (SPD v4 rewrite).
//!
//! Vault rooms sit on a rigid 11-tile grid. Neighbour picking walks a libGDX
//! `IntMap` key array, so that order is load-bearing.

#![allow(dead_code)] // analyze wiring is PR 4; vault tests call this now

use crate::geom::{Point, Rect};
use crate::random::Random;
use crate::rooms::room::{connect, Room};

use super::int_map::IntMap;
use super::place::find_neighbours;

pub(super) const ROOM_SIZE: i32 = 11;
const EXTRA_CONNECTION_CHANCE: f32 = 0.55;

pub(super) fn build(rooms: &mut [Room]) -> bool {
    for room in rooms.iter_mut() {
        room.set_empty();
    }

    let entrance = rooms.iter().position(Room::is_entrance);
    let exit = rooms.iter().position(Room::is_exit);
    let Some(entrance) = entrance else {
        return false;
    };

    if !rooms[entrance].force_size(ROOM_SIZE, ROOM_SIZE) {
        panic!("rigid room sizes for now!");
    }
    rooms[entrance].set_pos(0, 0);

    let mut multis = Vec::new();
    let mut singles = Vec::new();
    for (index, room) in rooms.iter().enumerate() {
        if room.max_connections(crate::rooms::room::DIR_ALL) == 1 {
            singles.push(index);
        } else {
            multis.push(index);
        }
    }

    // Assumes 1×1 cells; 2×2 rooms may poke the bound (Java comment).
    let mut max_width = 0;
    let mut max_height = 0;
    while ((max_width * max_height) as f32) < rooms.len() as f32 * 1.25 {
        if max_width < max_height || (max_width == max_height && Random::int_max(2) == 0) {
            max_width += 1;
        } else {
            max_height += 1;
        }
    }

    let mut to_place = Vec::new();
    if !multis.is_empty() {
        to_place.push(multis.remove(0));
    }
    if !multis.is_empty() {
        to_place.push(multis.remove(0));
    }
    while !multis.is_empty() || !singles.is_empty() {
        if !multis.is_empty() {
            to_place.push(multis.remove(0));
        }
        if !multis.is_empty() {
            to_place.push(multis.remove(0));
        }
        if !multis.is_empty() {
            to_place.push(multis.remove(0));
        }
        if !singles.is_empty() {
            to_place.push(singles.remove(0));
        }
    }

    to_place.retain(|&index| index != entrance);
    let (entry_x, entry_y) = match Random::int_max(4) {
        0 => (0, Random::int_range(0, max_height)),
        1 => (Random::int_range(0, max_width), 0),
        2 => (max_width - 1, Random::int_range(0, max_height)),
        _ => (Random::int_range(0, max_width), max_height - 1),
    };
    rooms[entrance].set_pos(entry_x * (ROOM_SIZE - 1), entry_y * (ROOM_SIZE - 1));
    let mut placed = vec![entrance];

    if let Some(exit) = exit {
        to_place.retain(|&index| index != exit);
        to_place.push(exit);
    }

    let aim_center_x = max_width as f32 / 2.0;
    let aim_center_y = max_height as f32 / 2.0;
    let mut grid_cells: IntMap<usize> = IntMap::new();
    grid_cells.put(grid_idx(entry_x, entry_y), entrance);

    let mut room_placement_failures = 0;
    while !to_place.is_empty() {
        let r = to_place.remove(0);
        let (cell_width, cell_height) = force_grid_size(&mut rooms[r]);
        let mut tries = 0;
        loop {
            rooms[r].neighbours.clear();
            tries += 1;
            if tries > 100 {
                to_place.insert((2).min(to_place.len()), r);
                room_placement_failures += 1;
                if room_placement_failures > 100 {
                    return false;
                }
                break;
            }

            let (n, n_idx) = if placed.len() < 3 {
                (entrance, grid_idx(entry_x, entry_y))
            } else {
                let keys = grid_cells.key_array();
                let n_idx = keys[Random::int_max(keys.len() as i32) as usize];
                let n = *grid_cells.get(n_idx).expect("grid cell occupied");
                (n, n_idx)
            };

            let mut r_idx = n_idx;
            let x_diff = aim_center_x - ((r_idx % 1000) - 100) as f32;
            let y_diff = aim_center_y - ((r_idx / 1000) - 100) as f32;
            let dist = ((f64::from(x_diff) * f64::from(x_diff)
                + f64::from(y_diff) * f64::from(y_diff))
            .sqrt()) as f32;
            // Farther from centre = more likely to pull in. Always pull on the
            // first extra room (placed == entrance only).
            if Random::float_max(12.0) < 8.0 + dist || placed.len() == 1 {
                if x_diff.abs() >= y_diff.abs() {
                    if x_diff > 0.0 {
                        r_idx += 1;
                    } else {
                        r_idx -= 1;
                    }
                } else if y_diff > 0.0 {
                    r_idx += 1000;
                } else {
                    r_idx -= 1000;
                }
            } else {
                match Random::int_max(4) {
                    0 => r_idx += 1,
                    1 => r_idx -= 1000,
                    2 => r_idx -= 1,
                    _ => r_idx += 1000,
                }
            }

            let mut x = (r_idx % 1000) - 100;
            let mut y = (r_idx / 1000) - 100;
            // Java uses maxHeight for both axes; keep that bound.
            let mut valid = !(x < 0 || x >= max_height || y < 0 || y >= max_height);

            if valid && grid_cells.contains_key(r_idx) {
                valid = false;
            } else if valid && (cell_width > 1 || cell_height > 1) {
                let mut space =
                    find_free_grid_space(Point::new(x, y), &grid_cells, cell_width, cell_height);
                if cell_width * cell_height <= 2 {
                    space.left = space.left.max(0);
                    space.top = space.top.max(0);
                    space.right = space.right.min(max_width - 1);
                    space.bottom = space.bottom.min(max_height - 1);
                }
                let excess_width = (space.raw_width() + 1) - cell_width;
                let excess_height = (space.raw_height() + 1) - cell_height;
                valid = excess_width >= 0 && excess_height >= 0;
                if valid {
                    x = space.left + Random::int_max(excess_width + 1);
                    y = space.top + Random::int_max(excess_height + 1);
                    r_idx = grid_idx(x, y);
                }
            }

            if valid {
                rooms[r].set_pos(x * (ROOM_SIZE - 1), y * (ROOM_SIZE - 1));
                for &other in &placed {
                    if rooms[other].name == rooms[r].name {
                        let i = crate::rooms::room::intersect(&rooms[r], &rooms[other]);
                        if i.raw_width() > 0 || i.raw_height() > 0 {
                            valid = false;
                            break;
                        }
                    }
                }
            }

            if valid && connect(rooms, r, n) {
                placed.push(r);
                for i in 0..cell_width {
                    for j in 0..cell_height {
                        grid_cells.put(r_idx + i + j * 1000, r);
                    }
                }
            }

            if placed.contains(&r) {
                break;
            }
        }
    }

    find_neighbours(rooms);
    extra_connections(rooms);
    true
}

fn extra_connections(rooms: &mut [Room]) {
    let order: Vec<usize> = (0..rooms.len()).collect();
    for index in order {
        let neighbours = rooms[index].neighbours.clone();
        for n in neighbours {
            if rooms[n].connected.contains(&index) {
                continue;
            }
            if Random::float() < EXTRA_CONNECTION_CHANCE {
                let _ = connect(rooms, index, n);
            }
        }
    }
}

fn force_grid_size(room: &mut Room) -> (i32, i32) {
    if room.force_size(ROOM_SIZE, ROOM_SIZE) {
        return (1, 1);
    }
    let double = 2 * ROOM_SIZE - 1;
    if room.force_size(double, double) {
        return (2, 2);
    }
    if room.force_size(ROOM_SIZE, double) {
        return (1, 2);
    }
    if room.force_size(double, ROOM_SIZE) {
        return (2, 1);
    }
    panic!("rigid room sizes for now!");
}

fn find_free_grid_space(
    start: Point,
    collision: &IntMap<usize>,
    max_width: i32,
    max_height: i32,
) -> Rect {
    let mut space = Rect {
        left: start.x,
        top: start.y,
        right: start.x,
        bottom: start.y,
    };
    let mut expanded = true;
    while expanded {
        expanded = false;
        if space.left > start.x - (max_width - 1) {
            let mut valid = true;
            for y in space.top..=space.bottom {
                if collision.contains_key(grid_idx(space.left - 1, y)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                space.left -= 1;
                expanded = true;
            }
        }
        if space.top > start.y - (max_height - 1) {
            let mut valid = true;
            for x in space.left..=space.right {
                if collision.contains_key(grid_idx(x, space.top - 1)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                space.top -= 1;
                expanded = true;
            }
        }
        if space.right < start.x + (max_width - 1) {
            let mut valid = true;
            for y in space.top..=space.bottom {
                if collision.contains_key(grid_idx(space.right + 1, y)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                space.right += 1;
                expanded = true;
            }
        }
        if space.bottom < start.y + (max_height - 1) {
            let mut valid = true;
            for x in space.left..=space.right {
                if collision.contains_key(grid_idx(x, space.bottom + 1)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                space.bottom += 1;
                expanded = true;
            }
        }
    }
    space
}

fn grid_idx(x: i32, y: i32) -> i32 {
    (x + 100) + 1000 * (y + 100)
}
