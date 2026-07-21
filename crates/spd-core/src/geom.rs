//! Minimal geometry helpers (Point / Rect) from watabou utils.

use crate::random::Random;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn empty() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    pub fn is_empty(self) -> bool {
        self.right <= self.left || self.bottom <= self.top
    }

    pub fn set_empty(&mut self) {
        self.left = 0;
        self.right = 0;
        self.top = 0;
        self.bottom = 0;
    }

    /// Raw width (right - left), matching watabou `Rect.width()`.
    pub fn raw_width(self) -> i32 {
        self.right - self.left
    }

    pub fn raw_height(self) -> i32 {
        self.bottom - self.top
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        self.right = self.left + w;
        self.bottom = self.top + h;
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        let w = self.raw_width();
        let h = self.raw_height();
        self.left = x;
        self.top = y;
        self.right = x + w;
        self.bottom = y + h;
    }

    /// Inclusive room width (+1), matching SPD `Room.width()`.
    pub fn room_width(self) -> i32 {
        self.raw_width() + 1
    }

    pub fn room_height(self) -> i32 {
        self.raw_height() + 1
    }

    pub fn center_deterministic(self) -> Point {
        Point::new((self.left + self.right) / 2, (self.top + self.bottom) / 2)
    }

    /// `Room.center()` with optional Random.Int(2) jitter.
    pub fn center_room(self) -> Point {
        Point::new(
            (self.left + self.right) / 2
                + if (self.right - self.left) % 2 == 1 {
                    Random::int_max(2)
                } else {
                    0
                },
            (self.top + self.bottom) / 2
                + if (self.bottom - self.top) % 2 == 1 {
                    Random::int_max(2)
                } else {
                    0
                },
        )
    }
}
