//! `StandardRoom` / entrance / exit creation tables.

use crate::random::Random;

use super::types::RoomSpec;

/// Java `StandardRoom.rooms` static list (35 entries).
const STANDARD_ROOMS: &[&str] = &[
    "SewerPipeRoom",
    "RingRoom",
    "WaterBridgeRoom",
    "RegionDecoPatchRoom",
    "CircleBasinRoom",
    "RegionDecoLineRoom",
    "SegmentedRoom",
    "PillarsRoom",
    "ChasmBridgeRoom",
    "CellBlockRoom",
    "CaveRoom",
    "RegionDecoBridgeRoom",
    "CavesFissureRoom",
    "CirclePitRoom",
    "CircleWallRoom",
    "HallwayRoom",
    "LibraryHallRoom",
    "LibraryRingRoom",
    "StatuesRoom",
    "SegmentedLibraryRoom",
    "RuinsRoom",
    "RegionDecoPatchRoom",
    "ChasmRoom",
    "SkullsRoom",
    "RitualRoom",
    "PlantsRoom",
    "AquariumRoom",
    "PlatformRoom",
    "BurnedRoom",
    "FissureRoom",
    "GrassyGraveRoom",
    "StripedRoom",
    "StudyRoom",
    "SuspiciousChestRoom",
    "MinefieldRoom",
];

fn chances_for_depth(depth: i32) -> &'static [f32] {
    match depth {
        1 => &[
            16., 8., 8., 4., 4., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            0., 0., 0., 0., 1., 0., 1., 0., 1., 0., 1., 1., 0., 0.,
        ],
        2..=4 => &[
            16., 8., 8., 4., 4., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            0., 0., 0., 0., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
        ],
        5 => &[
            16., 8., 8., 4., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        6..=10 => &[
            0., 0., 0., 0., 0., 10., 10., 10., 5., 5., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            0., 0., 0., 0., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
        ],
        11..=15 => &[
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 16., 8., 8., 4., 4., 0., 0., 0., 0., 0., 0.,
            0., 0., 0., 0., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
        ],
        16..=20 => &[
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 10., 10., 10., 5., 5., 0.,
            0., 0., 0., 0., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
        ],
        _ => &[
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 10.,
            10., 10., 5., 5., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
        ],
    }
}

/// Per-class `sizeCatProbs()`; default always normal.
pub fn size_cat_probs(room_name: &str) -> [f32; 3] {
    match room_name {
        "SewerPipeRoom" => [3., 2., 1.],
        "RingRoom" => [9., 3., 1.],
        "CircleBasinRoom" => [0., 3., 1.],
        "RegionDecoBridgeRoom" => [2., 1., 0.],
        "CircleWallRoom" => [0., 3., 1.],
        "PlantsRoom" | "AquariumRoom" => [3., 1., 0.],
        "PlatformRoom" | "FissureRoom" => [6., 3., 1.],
        "BurnedRoom" | "MinefieldRoom" => [4., 1., 0.],
        "StripedRoom" | "StudyRoom" => [2., 1., 0.],
        "SegmentedRoom" | "PillarsRoom" | "CavesFissureRoom" => [9., 3., 1.],
        "CellBlockRoom" | "SkullsRoom" | "SegmentedLibraryRoom" => [0., 3., 1.],
        "CirclePitRoom" | "RuinsRoom" | "LibraryRingRoom" => [4., 2., 1.],
        "LibraryHallRoom" | "LibraryHallEntranceRoom" | "LibraryHallExitRoom" => [2., 1., 0.],
        "StatuesRoom" => [9., 3., 1.],
        "CaveRoom" => [4., 2., 1.],
        "RitualRoom" => [6., 3., 1.],
        // entrances/exits with non-default sizes
        "RingEntranceRoom"
        | "CircleBasinEntranceRoom"
        | "LibraryRingEntranceRoom"
        | "RingExitRoom"
        | "CircleBasinExitRoom"
        | "LibraryRingExitRoom" => [0., 1., 0.],
        "PillarsEntranceRoom"
        | "PillarsExitRoom"
        | "StatuesEntranceRoom"
        | "StatuesExitRoom"
        | "CavesFissureEntranceRoom" => [3., 1., 0.],
        "RegionDecoBridgeEntranceRoom" | "RegionDecoBridgeExitRoom" => [2., 1., 0.],
        "CavesFissureExitRoom" => [3., 1., 0.],
        "CircleWallEntranceRoom" | "CircleWallExitRoom" => [0., 1., 0.],
        "CellBlockEntranceRoom" | "CellBlockExitRoom" => [0., 1., 0.],
        "CaveEntranceRoom" | "RuinsEntranceRoom" | "ChasmEntranceRoom" => [2., 1., 0.],
        _ => [1., 0., 0.],
    }
}

/// `setSizeCat(minOrdinal=0, maxOrdinal)`.
/// Returns size_factor (1/2/3) or None if impossible.
pub fn set_size_cat(room_name: &str, max_room_value: i32) -> Option<i32> {
    let max_ordinal = max_room_value - 1;
    if max_ordinal < 0 {
        return None;
    }
    let mut probs = size_cat_probs(room_name);
    for p in probs.iter_mut().skip(max_ordinal as usize + 1) {
        *p = 0.0;
    }
    let ordinal = Random::chances(&probs);
    if ordinal < 0 {
        None
    } else {
        Some(ordinal + 1)
    }
}

/// Unconstrained `setSizeCat()` (ctor).
pub fn set_size_cat_default(room_name: &str) -> i32 {
    set_size_cat(room_name, 3).unwrap_or(1)
}

/// `StandardRoom.createRoom()` + ctor size cat.
pub fn create_standard_room(depth: i32) -> (String, i32) {
    let chances = chances_for_depth(depth);
    let idx = Random::chances(chances) as usize;
    let name = STANDARD_ROOMS[idx].to_string();
    let size = set_size_cat_default(&name);
    (name, size)
}

// --- Entrance ---

const ENTRANCE_ROOMS: &[&str] = &[
    "WaterBridgeEntranceRoom",
    "RegionDecoPatchEntranceRoom",
    "RingEntranceRoom",
    "CircleBasinEntranceRoom",
    "RegionDecoLineEntranceRoom",
    "ChasmBridgeEntranceRoom",
    "PillarsEntranceRoom",
    "CellBlockEntranceRoom",
    "CaveEntranceRoom",
    "RegionDecoBridgeEntranceRoom",
    "CavesFissureEntranceRoom",
    "CircleWallEntranceRoom",
    "HallwayEntranceRoom",
    "StatuesEntranceRoom",
    "LibraryHallEntranceRoom",
    "LibraryRingEntranceRoom",
    "RegionDecoPatchEntranceRoom",
    "RuinsEntranceRoom",
    "ChasmEntranceRoom",
    "RitualEntranceRoom",
];

fn entrance_chances(depth: i32) -> &'static [f32] {
    match depth {
        1 | 2 => &[
            4., 3., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        3..=5 => &[
            4., 3., 2., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        6..=10 => &[
            0., 0., 0., 0., 4., 3., 2., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        11..=15 => &[
            0., 0., 0., 0., 0., 0., 0., 0., 4., 3., 2., 1., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        16..=20 => &[
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 4., 3., 2., 1., 0., 0., 0., 0.,
        ],
        _ => &[
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 4., 3., 2., 1.,
        ],
    }
}

pub fn create_entrance(depth: i32) -> RoomSpec {
    let idx = Random::chances(entrance_chances(depth)) as usize;
    let name = ENTRANCE_ROOMS[idx];
    let size = set_size_cat_default(name);
    let mut r = RoomSpec::entrance(name);
    r.size_factor = size;
    r
}

// --- Exit ---

const EXIT_ROOMS: &[&str] = &[
    "WaterBridgeExitRoom",
    "RegionDecoPatchExitRoom",
    "RingExitRoom",
    "CircleBasinExitRoom",
    "RegionDecoLineExitRoom",
    "ChasmBridgeExitRoom",
    "PillarsExitRoom",
    "CellBlockExitRoom",
    "CaveExitRoom",
    "RegionDecoBridgeExitRoom",
    "CavesFissureExitRoom",
    "CircleWallExitRoom",
    "HallwayExitRoom",
    "StatuesExitRoom",
    "LibraryHallExitRoom",
    "LibraryRingExitRoom",
    "RegionDecoPatchExitRoom",
    "RuinsExitRoom",
    "ChasmExitRoom",
    "RitualExitRoom",
];

fn exit_chances(depth: i32) -> &'static [f32] {
    // floor 1 simpler; floor 2+ includes more
    match depth {
        1 => &[
            4., 3., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        2..=5 => &[
            4., 3., 2., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        6..=10 => &[
            0., 0., 0., 0., 4., 3., 2., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        11..=15 => &[
            0., 0., 0., 0., 0., 0., 0., 0., 4., 3., 2., 1., 0., 0., 0., 0., 0., 0., 0., 0.,
        ],
        16..=20 => &[
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 4., 3., 2., 1., 0., 0., 0., 0.,
        ],
        _ => &[
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 4., 3., 2., 1.,
        ],
    }
}

pub fn create_exit(depth: i32) -> RoomSpec {
    let idx = Random::chances(exit_chances(depth)) as usize;
    let name = EXIT_ROOMS[idx];
    let size = set_size_cat_default(name);
    let mut r = RoomSpec::exit(name);
    r.size_factor = size;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cave_room_keeps_pinned_standard_size_weights() {
        assert_eq!(size_cat_probs("CaveRoom"), [4., 2., 1.]);
        assert_eq!(size_cat_probs("CaveEntranceRoom"), [2., 1., 0.]);
        assert_eq!(size_cat_probs("RegionDecoBridgeRoom"), [2., 1., 0.]);
        assert_eq!(size_cat_probs("CavesFissureExitRoom"), [3., 1., 0.]);
        assert_eq!(size_cat_probs("CircleWallEntranceRoom"), [0., 1., 0.]);
    }

    #[test]
    fn prison_rooms_keep_pinned_size_weights() {
        assert_eq!(size_cat_probs("RegionDecoLineRoom"), [1., 0., 0.]);
        assert_eq!(size_cat_probs("SegmentedRoom"), [9., 3., 1.]);
        assert_eq!(size_cat_probs("PillarsRoom"), [9., 3., 1.]);
        assert_eq!(size_cat_probs("PillarsEntranceRoom"), [3., 1., 0.]);
        assert_eq!(size_cat_probs("PillarsExitRoom"), [3., 1., 0.]);
        assert_eq!(size_cat_probs("CellBlockRoom"), [0., 3., 1.]);
        assert_eq!(size_cat_probs("CellBlockEntranceRoom"), [0., 1., 0.]);
        assert_eq!(size_cat_probs("CellBlockExitRoom"), [0., 1., 0.]);
    }

    #[test]
    fn city_rooms_keep_pinned_size_weights() {
        assert_eq!(size_cat_probs("HallwayRoom"), [1., 0., 0.]);
        assert_eq!(size_cat_probs("LibraryHallRoom"), [2., 1., 0.]);
        assert_eq!(size_cat_probs("LibraryHallExitRoom"), [2., 1., 0.]);
        assert_eq!(size_cat_probs("LibraryRingRoom"), [4., 2., 1.]);
        assert_eq!(size_cat_probs("LibraryRingExitRoom"), [0., 1., 0.]);
        assert_eq!(size_cat_probs("StatuesRoom"), [9., 3., 1.]);
        assert_eq!(size_cat_probs("StatuesExitRoom"), [3., 1., 0.]);
        assert_eq!(size_cat_probs("SegmentedLibraryRoom"), [0., 3., 1.]);
    }
}
