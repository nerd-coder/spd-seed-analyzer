//! Room kind tags used during initRooms / reporting.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomKind {
    Entrance,
    Exit,
    Standard,
    Special,
    Secret,
    Shop,
    Connection,
}

#[derive(Debug, Clone)]
pub struct RoomSpec {
    pub name: String,
    pub kind: RoomKind,
    /// Size category value for standard rooms (1=normal, 2=large, 3=giant).
    pub size_factor: i32,
    /// Max connections (ALL direction). Specials often 1.
    pub max_connections: i32,
}

impl RoomSpec {
    pub fn entrance(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: RoomKind::Entrance,
            size_factor: 1,
            max_connections: 16,
        }
    }

    pub fn exit(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: RoomKind::Exit,
            size_factor: 1,
            max_connections: 16,
        }
    }

    pub fn standard(name: impl Into<String>, size_factor: i32) -> Self {
        Self {
            name: name.into(),
            kind: RoomKind::Standard,
            size_factor,
            max_connections: 16,
        }
    }

    pub fn special(name: impl Into<String>) -> Self {
        // Most specials maxConnections = 1 (SpecialRoom default)
        Self {
            name: name.into(),
            kind: RoomKind::Special,
            size_factor: 1,
            max_connections: 1,
        }
    }

    pub fn secret(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: RoomKind::Secret,
            size_factor: 1,
            max_connections: 1,
        }
    }

    pub fn shop() -> Self {
        Self {
            name: "ShopRoom".into(),
            kind: RoomKind::Shop,
            size_factor: 1,
            max_connections: 1,
        }
    }
}
