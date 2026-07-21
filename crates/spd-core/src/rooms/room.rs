//! Geometric room model with connections (SPD `Room`).

use crate::geom::{Point, Rect};
use crate::random::Random;
use crate::rooms::types::RoomKind;

pub const DIR_ALL: i32 = 0;
pub const DIR_LEFT: i32 = 1;
pub const DIR_TOP: i32 = 2;
pub const DIR_RIGHT: i32 = 3;
pub const DIR_BOTTOM: i32 = 4;

#[derive(Debug, Clone)]
pub struct Room {
    pub id: usize,
    pub name: String,
    pub kind: RoomKind,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub size_factor: i32,
    pub min_w: i32,
    pub max_w: i32,
    pub min_h: i32,
    pub max_h: i32,
    pub max_connections_all: i32,
    pub neighbours: Vec<usize>,
    pub connected: Vec<usize>,
}

impl Room {
    pub fn new(
        id: usize,
        name: impl Into<String>,
        kind: RoomKind,
        size_factor: i32,
        max_connections_all: i32,
        min_w: i32,
        max_w: i32,
        min_h: i32,
        max_h: i32,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            kind,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            size_factor,
            min_w,
            max_w,
            min_h,
            max_h,
            max_connections_all,
            neighbours: Vec::new(),
            connected: Vec::new(),
        }
    }

    pub fn is_entrance(&self) -> bool {
        self.kind == RoomKind::Entrance
    }

    pub fn is_exit(&self) -> bool {
        self.kind == RoomKind::Exit
    }

    pub fn is_empty(&self) -> bool {
        self.right <= self.left || self.bottom <= self.top
    }

    pub fn set_empty(&mut self) {
        self.left = 0;
        self.top = 0;
        self.right = 0;
        self.bottom = 0;
    }

    /// Inclusive room width (SPD `Room.width()`).
    pub fn width(&self) -> i32 {
        self.right - self.left + 1
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top + 1
    }

    pub fn as_rect(&self) -> Rect {
        Rect {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        self.left = x;
        self.top = y;
        self.right = x + w;
        self.bottom = y + h;
    }

    pub fn shift(&mut self, dx: i32, dy: i32) {
        self.left += dx;
        self.right += dx;
        self.top += dy;
        self.bottom += dy;
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        self.right = self.left + w;
        self.bottom = self.top + h;
    }

    pub fn min_width(&self) -> i32 {
        self.min_w
    }
    pub fn max_width(&self) -> i32 {
        self.max_w
    }
    pub fn min_height(&self) -> i32 {
        self.min_h
    }
    pub fn max_height(&self) -> i32 {
        self.max_h
    }

    pub fn set_size(&mut self) -> bool {
        self.set_size_range(self.min_w, self.max_w, self.min_h, self.max_h)
    }

    pub fn set_size_range(&mut self, min_w: i32, max_w: i32, min_h: i32, max_h: i32) -> bool {
        if min_w < self.min_w
            || max_w > self.max_w
            || min_h < self.min_h
            || max_h > self.max_h
            || min_w > max_w
            || min_h > max_h
        {
            return false;
        }
        // subtract one because rooms are inclusive to right/bottom
        self.resize(
            Random::normal_int_range(min_w, max_w) - 1,
            Random::normal_int_range(min_h, max_h) - 1,
        );
        true
    }

    pub fn set_size_with_limit(&mut self, w: i32, h: i32) -> bool {
        if w < self.min_w || h < self.min_h {
            return false;
        }
        self.set_size();
        if self.width() > w || self.height() > h {
            self.resize(
                self.width().min(w) - 1,
                self.height().min(h) - 1,
            );
        }
        true
    }

    pub fn clear_connections(&mut self) {
        self.neighbours.clear();
        self.connected.clear();
    }

    pub fn connection_weight(&self) -> i32 {
        self.size_factor * self.size_factor
    }

    pub fn max_connections(&self, direction: i32) -> i32 {
        if direction == DIR_ALL {
            self.max_connections_all
        } else {
            4
        }
    }

    pub fn cur_connections(&self, direction: i32, rooms: &[Room]) -> i32 {
        if direction == DIR_ALL {
            return self.connected.len() as i32;
        }
        let mut total = 0;
        for &oid in &self.connected {
            let other = &rooms[oid];
            let i = intersect(self, other);
            if direction == DIR_LEFT && i.raw_width() == 0 && i.left == self.left {
                total += 1;
            } else if direction == DIR_TOP && i.raw_height() == 0 && i.top == self.top {
                total += 1;
            } else if direction == DIR_RIGHT && i.raw_width() == 0 && i.right == self.right {
                total += 1;
            } else if direction == DIR_BOTTOM && i.raw_height() == 0 && i.bottom == self.bottom {
                total += 1;
            }
        }
        total
    }

    pub fn rem_connections(&self, direction: i32, rooms: &[Room]) -> i32 {
        if self.cur_connections(DIR_ALL, rooms) >= self.max_connections(DIR_ALL) {
            0
        } else {
            self.max_connections(direction) - self.cur_connections(direction, rooms)
        }
    }

    pub fn can_connect_point(&self, p: Point) -> bool {
        (p.x == self.left || p.x == self.right) != (p.y == self.top || p.y == self.bottom)
    }

    pub fn can_connect_dir(&self, direction: i32, rooms: &[Room]) -> bool {
        self.rem_connections(direction, rooms) > 0
    }
}

pub fn intersect(a: &Room, b: &Room) -> Rect {
    Rect {
        left: a.left.max(b.left),
        right: a.right.min(b.right),
        top: a.top.max(b.top),
        bottom: a.bottom.min(b.bottom),
    }
}

pub fn can_connect_rooms(a: &Room, b: &Room, rooms: &[Room]) -> bool {
    if (a.is_exit() && b.is_entrance()) || (a.is_entrance() && b.is_exit()) {
        return false;
    }
    let i = intersect(a, b);
    let mut found_point = false;
    // points along intersection
    for x in i.left..=i.right {
        for y in i.top..=i.bottom {
            let p = Point::new(x, y);
            if a.can_connect_point(p) && b.can_connect_point(p) {
                found_point = true;
                break;
            }
        }
        if found_point {
            break;
        }
    }
    if !found_point {
        return false;
    }

    if i.raw_width() == 0 && i.left == a.left {
        a.can_connect_dir(DIR_LEFT, rooms) && b.can_connect_dir(DIR_RIGHT, rooms)
    } else if i.raw_height() == 0 && i.top == a.top {
        a.can_connect_dir(DIR_TOP, rooms) && b.can_connect_dir(DIR_BOTTOM, rooms)
    } else if i.raw_width() == 0 && i.right == a.right {
        a.can_connect_dir(DIR_RIGHT, rooms) && b.can_connect_dir(DIR_LEFT, rooms)
    } else if i.raw_height() == 0 && i.bottom == a.bottom {
        a.can_connect_dir(DIR_BOTTOM, rooms) && b.can_connect_dir(DIR_TOP, rooms)
    } else {
        false
    }
}

pub fn add_neighbour(rooms: &mut [Room], a: usize, b: usize) -> bool {
    if rooms[a].neighbours.contains(&b) {
        return true;
    }
    let i = {
        let (ra, rb) = (&rooms[a], &rooms[b]);
        intersect(ra, rb)
    };
    if (i.raw_width() == 0 && i.raw_height() >= 2) || (i.raw_height() == 0 && i.raw_width() >= 2) {
        rooms[a].neighbours.push(b);
        rooms[b].neighbours.push(a);
        true
    } else {
        false
    }
}

pub fn connect(rooms: &mut [Room], a: usize, b: usize) -> bool {
    let has_n = rooms[a].neighbours.contains(&b);
    let can = {
        let ok_n = has_n || {
            // need add_neighbour check without borrow issues
            true
        };
        ok_n
    };
    let _ = can;
    if !has_n && !add_neighbour(rooms, a, b) {
        return false;
    }
    if rooms[a].connected.contains(&b) {
        return false;
    }
    // can_connect_rooms needs immutable view
    let allowed = {
        // Temporarily clone needed fields is hard; use indices carefully
        can_connect_pair(rooms, a, b)
    };
    if !allowed {
        return false;
    }
    rooms[a].connected.push(b);
    rooms[b].connected.push(a);
    true
}

fn can_connect_pair(rooms: &[Room], a: usize, b: usize) -> bool {
    can_connect_rooms(&rooms[a], &rooms[b], rooms)
}

pub fn clear_all_connections(rooms: &mut [Room]) {
    for r in rooms.iter_mut() {
        r.clear_connections();
    }
}

/// Dimensions from size category (standard rooms).
pub fn dims_for_size_factor(size_factor: i32) -> (i32, i32, i32, i32) {
    match size_factor {
        2 => (10, 14, 10, 14), // LARGE
        3 => (14, 18, 14, 18), // GIANT
        _ => (4, 10, 4, 10),   // NORMAL
    }
}

pub fn dims_for_kind(kind: RoomKind, size_factor: i32, name: &str) -> (i32, i32, i32, i32) {
    match kind {
        RoomKind::Connection => (3, 10, 3, 10),
        RoomKind::Special | RoomKind::Secret | RoomKind::Shop => (5, 10, 5, 10),
        RoomKind::Entrance | RoomKind::Exit => {
            let (mw, xw, mh, xh) = dims_for_size_factor(size_factor);
            (mw.max(5), xw, mh.max(5), xh)
        }
        RoomKind::Standard => {
            let (mw, xw, mh, xh) = dims_for_size_factor(size_factor);
            // CircleBasin etc. may force LARGE min — keep size_factor dims
            let _ = name;
            (mw, xw, mh, xh)
        }
    }
}
