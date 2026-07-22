//! `Builder.findFreeSpace` / `placeRoom` / angle helpers.

use crate::geom::{Point, Rect};
use crate::random::Random;
use crate::rooms::room::{
    add_neighbour, connect, intersect, Room, DIR_BOTTOM, DIR_LEFT, DIR_RIGHT, DIR_TOP,
};

const A: f64 = 180.0 / std::f64::consts::PI;

pub fn gate(min: f32, value: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn angle_between_points(from: (f32, f32), to: (f32, f32)) -> f32 {
    // Java evaluates the PointF division as `float` before widening the
    // result to the local `double` slope.
    let m = ((to.1 - from.1) / (to.0 - from.0)) as f64;
    let mut angle = (A * (m.atan() + std::f64::consts::PI / 2.0)) as f32;
    if from.0 > to.0 {
        angle -= 180.0;
    }
    angle
}

pub fn angle_between_rooms(from: &Room, to: &Room) -> f32 {
    let from_c = (
        (from.left + from.right) as f32 / 2.0,
        (from.top + from.bottom) as f32 / 2.0,
    );
    let to_c = (
        (to.left + to.right) as f32 / 2.0,
        (to.top + to.bottom) as f32 / 2.0,
    );
    angle_between_points(from_c, to_c)
}

/// Maximum free space from a start point among colliding rooms.
pub fn find_free_space(start: Point, collision: &[Room], max_size: i32) -> Rect {
    let mut space = Rect {
        left: start.x - max_size,
        top: start.y - max_size,
        right: start.x + max_size,
        bottom: start.y + max_size,
    };

    let mut colliding: Vec<usize> = (0..collision.len()).collect();
    loop {
        colliding.retain(|&idx| {
            let room = &collision[idx];
            if room.is_empty() {
                return false;
            }
            // overlapping?
            !(space.left.max(room.left) >= space.right.min(room.right)
                || space.top.max(room.top) >= space.bottom.min(room.bottom))
        });

        if colliding.is_empty() {
            break;
        }

        let mut closest_room: Option<usize> = None;
        let mut closest_diff = i32::MAX;

        // Pinned Builder.findFreeSpace declares these outside the room loop.
        // `inside` therefore stays false after the first room that does not
        // contain the start point, and `curDiff` accumulates across rooms.
        let mut inside = true;
        let mut cur_diff = 0;
        for &idx in &colliding {
            let cur = &collision[idx];
            if start.x <= cur.left {
                inside = false;
                cur_diff += cur.left - start.x;
            } else if start.x >= cur.right {
                inside = false;
                cur_diff += start.x - cur.right;
            }
            if start.y <= cur.top {
                inside = false;
                cur_diff += cur.top - start.y;
            } else if start.y >= cur.bottom {
                inside = false;
                cur_diff += start.y - cur.bottom;
            }
            if inside {
                return Rect {
                    left: start.x,
                    top: start.y,
                    right: start.x,
                    bottom: start.y,
                };
            }
            if cur_diff < closest_diff {
                closest_diff = cur_diff;
                closest_room = Some(idx);
            }
        }

        if let Some(idx) = closest_room {
            let closest = &collision[idx];
            let mut w_diff = i32::MAX;
            if closest.left >= start.x {
                w_diff = (space.right - closest.left) * (space.raw_height() + 1);
            } else if closest.right <= start.x {
                w_diff = (closest.right - space.left) * (space.raw_height() + 1);
            }
            let mut h_diff = i32::MAX;
            if closest.top >= start.y {
                h_diff = (space.bottom - closest.top) * (space.raw_width() + 1);
            } else if closest.bottom <= start.y {
                h_diff = (closest.bottom - space.top) * (space.raw_width() + 1);
            }

            if w_diff < h_diff || (w_diff == h_diff && Random::int_max(2) == 0) {
                if closest.left >= start.x && closest.left < space.right {
                    space.right = closest.left;
                }
                if closest.right <= start.x && closest.right > space.left {
                    space.left = closest.right;
                }
            } else {
                if closest.top >= start.y && closest.top < space.bottom {
                    space.bottom = closest.top;
                }
                if closest.bottom <= start.y && closest.bottom > space.top {
                    space.top = closest.bottom;
                }
            }
            colliding.retain(|&i| i != idx);
        } else {
            colliding.clear();
        }
    }
    space
}

/// Place `next` next to `prev` at approximately `angle` degrees. Returns angle or -1.
/// `next` is identified by index into `rooms` which must include all collision rooms + next.
pub fn place_room(rooms: &mut [Room], prev: usize, next: usize, angle: f32) -> f32 {
    place_room_with_prepare(rooms, prev, next, angle, &mut |_| {})
}

/// ShopRoom computes its minimum dimensions lazily from generated stock.
/// Java reaches that hook inside `setSizeWithLimit`, after the angle draw and
/// `findFreeSpace`, but before the room-size draws.
pub(super) fn place_room_with_prepare(
    rooms: &mut [Room],
    prev: usize,
    next: usize,
    mut angle: f32,
    prepare: &mut impl FnMut(&mut Room),
) -> f32 {
    angle %= 360.0;
    if angle < 0.0 {
        angle += 360.0;
    }

    let prev_center = (
        (rooms[prev].left + rooms[prev].right) as f32 / 2.0,
        (rooms[prev].top + rooms[prev].bottom) as f32 / 2.0,
    );

    let m = (angle as f64 / A + std::f64::consts::PI / 2.0).tan();
    let b = prev_center.1 as f64 - m * prev_center.0 as f64;

    let (direction, mut start) = if m.abs() >= 1.0 {
        if !(90.0..=270.0).contains(&angle) {
            let x = ((rooms[prev].top as f64 - b) / m).round() as i32;
            (DIR_TOP, Point::new(x, rooms[prev].top))
        } else {
            let x = ((rooms[prev].bottom as f64 - b) / m).round() as i32;
            (DIR_BOTTOM, Point::new(x, rooms[prev].bottom))
        }
    } else if angle < 180.0 {
        let y = (m * rooms[prev].right as f64 + b).round() as i32;
        (DIR_RIGHT, Point::new(rooms[prev].right, y))
    } else {
        let y = (m * rooms[prev].left as f64 + b).round() as i32;
        (DIR_LEFT, Point::new(rooms[prev].left, y))
    };

    if direction == DIR_TOP || direction == DIR_BOTTOM {
        start.x = gate(
            (rooms[prev].left + 1) as f32,
            start.x as f32,
            (rooms[prev].right - 1) as f32,
        ) as i32;
    } else {
        start.y = gate(
            (rooms[prev].top + 1) as f32,
            start.y as f32,
            (rooms[prev].bottom - 1) as f32,
        ) as i32;
    }

    let max_dim = rooms[next].max_width().max(rooms[next].max_height());
    // collision list: all rooms except empty next? Java uses full list including unplaced empties
    let space = find_free_space(start, rooms, max_dim);
    prepare(&mut rooms[next]);
    if !rooms[next].set_size_with_limit(space.raw_width() + 1, space.raw_height() + 1) {
        return -1.0;
    }

    let (nw, nh) = (rooms[next].width(), rooms[next].height());
    let (pl, pt, pr, pb) = (
        rooms[prev].left,
        rooms[prev].top,
        rooms[prev].right,
        rooms[prev].bottom,
    );
    if direction == DIR_TOP {
        let ty = pt as f32 - (nh - 1) as f32 / 2.0;
        let tx = if m.abs() < 1e-9 {
            prev_center.0
        } else {
            ((ty as f64 - b) / m) as f32
        };
        let nx = (tx - (nw - 1) as f32 / 2.0).round() as i32;
        let ny = pt - (nh - 1);
        rooms[next].set_pos(nx, ny);
    } else if direction == DIR_BOTTOM {
        let ty = pb as f32 + (nh - 1) as f32 / 2.0;
        let tx = ((ty as f64 - b) / m) as f32;
        let nx = (tx - (nw - 1) as f32 / 2.0).round() as i32;
        rooms[next].set_pos(nx, pb);
    } else if direction == DIR_RIGHT {
        let tx = pr as f32 + (nw - 1) as f32 / 2.0;
        let ty = (m * tx as f64 + b) as f32;
        let ny = (ty - (nh - 1) as f32 / 2.0).round() as i32;
        rooms[next].set_pos(pr, ny);
    } else {
        let tx = pl as f32 - (nw - 1) as f32 / 2.0;
        let ty = (m * tx as f64 + b) as f32;
        let ny = (ty - (nh - 1) as f32 / 2.0).round() as i32;
        rooms[next].set_pos(pl - (nw - 1), ny);
    }

    if direction == DIR_TOP || direction == DIR_BOTTOM {
        let (nl, nr) = (rooms[next].left, rooms[next].right);
        if nr < pl + 2 {
            rooms[next].shift(pl + 2 - nr, 0);
        } else if nl > pr - 2 {
            rooms[next].shift(pr - 2 - nl, 0);
        }
        let (nl, nr) = (rooms[next].left, rooms[next].right);
        if nr > space.right {
            rooms[next].shift(space.right - nr, 0);
        } else if nl < space.left {
            rooms[next].shift(space.left - nl, 0);
        }
    } else {
        let (nt, nb) = (rooms[next].top, rooms[next].bottom);
        if nb < pt + 2 {
            rooms[next].shift(0, pt + 2 - nb);
        } else if nt > pb - 2 {
            rooms[next].shift(0, pb - 2 - nt);
        }
        let (nt, nb) = (rooms[next].top, rooms[next].bottom);
        if nb > space.bottom {
            rooms[next].shift(0, space.bottom - nb);
        } else if nt < space.top {
            rooms[next].shift(0, space.top - nt);
        }
    }

    if connect(rooms, next, prev) {
        angle_between_rooms(&rooms[prev], &rooms[next])
    } else {
        -1.0
    }
}

/// Find neighbours among placed rooms (non-empty).
pub fn find_neighbours(rooms: &mut [Room]) {
    let n = rooms.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if rooms[i].is_empty() || rooms[j].is_empty() {
                continue;
            }
            let _ = add_neighbour(rooms, i, j);
        }
    }
}

// silence unused
#[allow(dead_code)]
fn _intersect_unused(a: &Room, b: &Room) -> Rect {
    intersect(a, b)
}
