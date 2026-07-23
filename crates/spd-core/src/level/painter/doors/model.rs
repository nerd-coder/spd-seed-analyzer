//! Door types, map, placeDoors, and room-paint type upgrades.

use std::collections::HashMap;

use crate::geom::Point;
use crate::random::Random;
use crate::rooms::room::{intersect, Room};
use crate::rooms::types::RoomKind;

/// SPD `Room.Door.Type` ordinal order (used by `Door.set` upgrade rule).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)] // full enum kept for Door.set ordinal parity with SPD
pub enum DoorType {
    Empty = 0,
    Tunnel,
    Water,
    Regular,
    Unlocked,
    Hidden,
    Barricade,
    Locked,
    Crystal,
    Wall,
}

#[derive(Debug, Clone)]
pub struct Door {
    pub x: i32,
    pub y: i32,
    pub door_type: DoorType,
}

impl Door {
    pub(super) fn new(p: Point) -> Self {
        Self {
            x: p.x,
            y: p.y,
            door_type: DoorType::Empty,
        }
    }

    /// SPD `Door.set` — only upgrades type (higher ordinal wins).
    pub fn set(&mut self, t: DoorType) {
        if t > self.door_type {
            self.door_type = t;
        }
    }
}

/// Undirected door map keyed by `(min_idx, max_idx)`.
#[derive(Debug, Default)]
pub struct DoorMap {
    pub(super) doors: HashMap<(usize, usize), Door>,
}

impl DoorMap {
    pub fn new() -> Self {
        Self::default()
    }

    fn key(a: usize, b: usize) -> (usize, usize) {
        (a.min(b), a.max(b))
    }

    pub fn get(&self, a: usize, b: usize) -> Option<&Door> {
        self.doors.get(&Self::key(a, b))
    }

    pub fn get_mut(&mut self, a: usize, b: usize) -> Option<&mut Door> {
        self.doors.get_mut(&Self::key(a, b))
    }

    pub fn insert(&mut self, a: usize, b: usize, door: Door) {
        self.doors.insert(Self::key(a, b), door);
    }

    #[cfg(test)]
    pub(crate) fn insert_test_point(&mut self, a: usize, b: usize, point: Point) {
        self.insert(a, b, Door::new(point));
    }

    pub fn contains(&self, a: usize, b: usize) -> bool {
        self.doors.contains_key(&Self::key(a, b))
    }
}

/// Candidate door cells on the shared edge (`placeDoors` doorSpots).
pub fn door_spots(a: &Room, b: &Room) -> Vec<Point> {
    let i = intersect(a, b);
    let mut spots = Vec::new();
    for x in i.left..=i.right {
        for y in i.top..=i.bottom {
            let p = Point::new(x, y);
            if a.can_connect_point(p) && b.can_connect_point(p) {
                spots.push(p);
            }
        }
    }
    spots
}

/// `placeDoors(r)` for one room — pick `Random.element` on null connections.
pub fn place_doors_for_room(rooms: &[Room], ri: usize, doors: &mut DoorMap) {
    let room = &rooms[ri];
    if room.is_empty() {
        return;
    }
    for &ni in &room.connected {
        if doors.contains(ri, ni) {
            continue;
        }
        let other = &rooms[ni];
        if other.is_empty() {
            continue;
        }
        let spots = door_spots(room, other);
        if spots.is_empty() {
            continue;
        }
        let p = *Random::element(&spots).expect("non-empty spots");
        doors.insert(ri, ni, Door::new(p));
    }
}

/// After room "paint": upgrade door types the way each room's `paint` does.
pub fn apply_room_door_types(rooms: &[Room], ri: usize, doors: &mut DoorMap) {
    let Some(room) = rooms.get(ri) else {
        return;
    };
    if room.is_empty() {
        return;
    }
    for &ni in &room.connected {
        if let Some(d) = doors.get_mut(ri, ni) {
            let upgrade = if room.name == "SewerPipeRoom"
                && rooms
                    .get(ni)
                    .is_some_and(|other| other.name == "SewerPipeRoom")
            {
                DoorType::Water
            } else {
                door_type_for_room(room)
            };
            d.set(upgrade);
        }
    }
}

fn door_type_for_room(room: &Room) -> DoorType {
    match room.kind {
        RoomKind::Secret => DoorType::Hidden,
        RoomKind::Connection if room.name == "MazeConnectionRoom" => DoorType::Hidden,
        RoomKind::Connection => DoorType::Tunnel,
        RoomKind::Shop | RoomKind::Entrance | RoomKind::Exit | RoomKind::Standard => {
            DoorType::Regular
        }
        RoomKind::Special => special_door_type(&room.name),
    }
}

fn special_door_type(name: &str) -> DoorType {
    match name {
        "DemonSpawnerRoom" => DoorType::Unlocked,
        "SacrificeRoom" => DoorType::Empty,
        "StorageRoom" => DoorType::Barricade,
        "CryptRoom" | "ArmoryRoom" | "LibraryRoom" | "TreasuryRoom" | "RunestoneRoom"
        | "LaboratoryRoom" | "StatueRoom" | "GardenRoom" | "MagicWellRoom" | "CrystalVaultRoom"
        | "CrystalChoiceRoom" | "RotGardenRoom" => DoorType::Locked,
        "PoolRoom" | "TrapsRoom" | "SentryRoom" | "ToxicGasRoom" | "MagicalFireRoom"
        | "WeakFloorRoom" | "CrystalPathRoom" | "PitRoom" => DoorType::Regular,
        _ => DoorType::Regular,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_is_the_barricaded_special_room() {
        assert_eq!(special_door_type("StorageRoom"), DoorType::Barricade);
        assert_eq!(special_door_type("MagicalFireRoom"), DoorType::Regular);
    }
}
