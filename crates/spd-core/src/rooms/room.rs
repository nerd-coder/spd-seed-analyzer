//! Geometric room model with connections (SPD `Room`).

use crate::geom::{Point, Rect};
use crate::random::Random;
use crate::rooms::types::RoomKind;

pub use super::dimensions::{dims_for_kind, dims_for_size_factor};

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
    #[allow(clippy::too_many_arguments)] // mirrors SPD Room geometry fields at construction
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

    /// SPD `Rect.square()` = width × height (room area in cells).
    pub fn square(&self) -> i32 {
        self.width() * self.height()
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
        if matches!(
            self.name.as_str(),
            "CircleBasinRoom" | "CircleBasinEntranceRoom" | "CircleBasinExitRoom"
        ) {
            if self.width() % 2 == 0 {
                self.right -= 1;
            }
            if self.height() % 2 == 0 {
                self.bottom -= 1;
            }
        }
        if self.size_factor == 3
            && matches!(
                self.name.as_str(),
                "LibraryRingRoom" | "LibraryRingEntranceRoom" | "LibraryRingExitRoom"
            )
        {
            if self.width() % 2 == 1 {
                self.right -= 1;
            }
            if self.height() % 2 == 1 {
                self.bottom -= 1;
            }
        }
    }

    /// SPD `Room.random()` — point inset by 1 tile from walls.
    pub fn random(&self) -> Point {
        self.random_margin(1)
    }

    /// SPD `Room.random(m)`.
    pub fn random_margin(&self, m: i32) -> Point {
        Point::new(
            Random::int_range_inclusive(self.left + m, self.right - m),
            Random::int_range_inclusive(self.top + m, self.bottom - m),
        )
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
            self.resize(self.width().min(w) - 1, self.height().min(h) - 1);
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
            let matches = match direction {
                DIR_LEFT => i.raw_width() == 0 && i.left == self.left,
                DIR_TOP => i.raw_height() == 0 && i.top == self.top,
                DIR_RIGHT => i.raw_width() == 0 && i.right == self.right,
                DIR_BOTTOM => i.raw_height() == 0 && i.bottom == self.bottom,
                _ => false,
            };
            if matches {
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
        let on_one_edge =
            (p.x == self.left || p.x == self.right) != (p.y == self.top || p.y == self.bottom);
        if self.name == "SewerPipeRoom" {
            on_one_edge
                && ((p.x > self.left + 1 && p.x < self.right - 1)
                    || (p.y > self.top + 1 && p.y < self.bottom - 1))
        } else {
            on_one_edge
        }
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
    // DemonSpawnerRoom.connect rejects the floor exit in pinned SPD.
    if (a.name == "DemonSpawnerRoom" && b.is_exit())
        || (b.name == "DemonSpawnerRoom" && a.is_exit())
    {
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
    if !has_n && !add_neighbour(rooms, a, b) {
        return false;
    }
    if rooms[a].connected.contains(&b) {
        return false;
    }
    if !can_connect_pair(rooms, a, b) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sewer_resize_and_connection_overrides_match_subclasses() {
        let mut basin = Room::new(
            0,
            "CircleBasinRoom",
            RoomKind::Standard,
            2,
            16,
            11,
            14,
            11,
            14,
        );
        basin.resize(11, 13); // requested inclusive size 12×14
        assert_eq!((basin.width(), basin.height()), (11, 13));

        let mut pipe = Room::new(0, "SewerPipeRoom", RoomKind::Standard, 1, 16, 7, 10, 7, 10);
        pipe.resize(8, 8);
        assert!(!pipe.can_connect_point(Point::new(pipe.left, pipe.top + 1)));
        assert!(pipe.can_connect_point(Point::new(pipe.left, pipe.top + 2)));
    }

    #[test]
    fn giant_library_ring_resize_forces_even_inclusive_dimensions() {
        let mut giant = Room::new(
            0,
            "LibraryRingRoom",
            RoomKind::Standard,
            3,
            16,
            14,
            18,
            14,
            18,
        );
        giant.resize(16, 14);
        assert_eq!((giant.width(), giant.height()), (16, 14));

        let mut large = giant.clone();
        large.size_factor = 2;
        large.resize(16, 14);
        assert_eq!((large.width(), large.height()), (17, 15));
    }

    #[test]
    fn demon_spawner_cannot_connect_to_exit() {
        let mut spawner = Room::new(0, "DemonSpawnerRoom", RoomKind::Special, 1, 1, 6, 6, 6, 6);
        spawner.left = 1;
        spawner.top = 1;
        spawner.right = 6;
        spawner.bottom = 6;
        let mut exit = Room::new(1, "ExitRoom", RoomKind::Exit, 1, 16, 5, 5, 6, 6);
        exit.left = 6;
        exit.top = 1;
        exit.right = 10;
        exit.bottom = 6;
        assert!(!can_connect_rooms(
            &spawner,
            &exit,
            &[spawner.clone(), exit.clone()]
        ));
    }
}
