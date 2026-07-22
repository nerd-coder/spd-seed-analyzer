//! Room dimension policy, including standard-room subclass overrides.

use super::types::RoomKind;

/// Dimensions from size category (standard rooms).
pub fn dims_for_size_factor(size_factor: i32) -> (i32, i32, i32, i32) {
    match size_factor {
        2 => (10, 14, 10, 14),
        3 => (14, 18, 14, 18),
        _ => (4, 10, 4, 10),
    }
}

pub fn dims_for_kind(kind: RoomKind, size_factor: i32, name: &str) -> (i32, i32, i32, i32) {
    match name {
        "MassGraveRoom" => return (7, 10, 7, 10),
        "RotGardenRoom" => return (10, 10, 10, 10),
        "AmbitiousImpRoom" => return (9, 9, 9, 9),
        "BlacksmithRoom" => {
            let (mw, xw, mh, xh) = dims_for_size_factor(size_factor);
            return (mw.max(6), xw, mh.max(6), xh);
        }
        "CrystalVaultRoom" => return (7, 7, 7, 7),
        "CrystalChoiceRoom" | "CrystalPathRoom" => return (7, 10, 7, 10),
        "SecretMazeRoom" => return (14, 18, 14, 18),
        "SecretChestChasmRoom" => return (8, 9, 8, 9),
        "SecretSummoningRoom" => return (5, 8, 5, 8),
        "PitRoom" => return (6, 9, 6, 9),
        "PoolRoom" | "RunestoneRoom" => return (6, 10, 6, 10),
        "MagicalFireRoom" => return (7, 10, 7, 10),
        "TrapsRoom" => return (6, 8, 6, 8),
        "RitualSiteRoom" => {
            let (mw, xw, mh, xh) = dims_for_size_factor(size_factor);
            return (mw.max(9), xw, mh.max(9), xh);
        }
        _ => {}
    }

    let (min_w, max_w, min_h, max_h) = dims_for_size_factor(size_factor);
    match name {
        "PlantsRoom" | "FissureRoom" | "SuspiciousChestRoom" => {
            return (min_w.max(5), max_w, min_h.max(5), max_h);
        }
        "PlatformRoom" => return (min_w.max(6), max_w, min_h.max(6), max_h),
        "AquariumRoom" | "StudyRoom" => {
            return (min_w.max(7), max_w, min_h.max(7), max_h);
        }
        "RegionDecoPatchEntranceRoom"
        | "RegionDecoPatchExitRoom"
        | "CaveEntranceRoom"
        | "CaveExitRoom"
        | "RuinsEntranceRoom"
        | "RuinsExitRoom"
        | "ChasmEntranceRoom"
        | "ChasmExitRoom" => return (min_w.max(7), max_w, min_h.max(7), max_h),
        "RegionDecoPatchRoom" | "CaveRoom" | "ChasmRoom" | "RegionDecoBridgeRoom" => {
            return (min_w.max(5), max_w, min_h.max(5), max_h);
        }
        "SewerPipeRoom" | "RingRoom" => {
            return (min_w.max(7), max_w, min_h.max(7), max_h);
        }
        "WaterBridgeRoom" => return (min_w.max(5), max_w, min_h.max(5), max_h),
        "WaterBridgeEntranceRoom" | "WaterBridgeExitRoom" => {
            return (min_w.max(7), max_w, min_h.max(7), max_h);
        }
        "CircleBasinRoom" | "CircleBasinEntranceRoom" | "CircleBasinExitRoom" => {
            return (min_w + 1, max_w, min_h + 1, max_h);
        }
        "RegionDecoBridgeEntranceRoom" | "RegionDecoBridgeExitRoom" => {
            return (min_w.max(8), max_w, min_h.max(8), max_h);
        }
        "CavesFissureRoom" | "CavesFissureEntranceRoom" | "CavesFissureExitRoom" => {
            return (min_w.max(7), max_w, min_h.max(7), max_h);
        }
        "CirclePitRoom" => return (min_w.max(8), max_w, min_h.max(8), max_h),
        "CircleWallEntranceRoom" | "CircleWallExitRoom" => {
            return (min_w.max(11), max_w, min_h.max(11), max_h);
        }
        "RegionDecoLineRoom" | "ChasmBridgeRoom" => {
            return (min_w.max(5), max_w, min_h.max(5), max_h);
        }
        "RegionDecoLineEntranceRoom"
        | "RegionDecoLineExitRoom"
        | "ChasmBridgeEntranceRoom"
        | "ChasmBridgeExitRoom" => {
            return (min_w.max(7), max_w, min_h.max(7), max_h);
        }
        "SegmentedRoom" | "PillarsRoom" | "PillarsEntranceRoom" | "PillarsExitRoom" => {
            return (min_w.max(7), max_w, min_h.max(7), max_h);
        }
        "HallwayRoom" | "HallwayEntranceRoom" | "HallwayExitRoom" => {
            return (min_w.max(5), max_w, min_h.max(5), max_h);
        }
        "LibraryHallRoom"
        | "LibraryHallEntranceRoom"
        | "LibraryHallExitRoom"
        | "StatuesRoom"
        | "StatuesEntranceRoom"
        | "StatuesExitRoom" => return (min_w.max(7), max_w, min_h.max(7), max_h),
        "LibraryRingRoom" => return (min_w.max(9), max_w, min_h.max(9), max_h),
        "LibraryRingEntranceRoom" | "LibraryRingExitRoom" => {
            return (min_w.max(13), max_w, min_h.max(13), max_h);
        }
        "SkullsRoom" => return (min_w.max(7), max_w, min_h.max(7), max_h),
        "RitualRoom" | "RitualEntranceRoom" | "RitualExitRoom" => {
            return (min_w.max(9), max_w, min_h.max(9), max_h);
        }
        _ => {}
    }

    match kind {
        RoomKind::Connection => (3, 10, 3, 10),
        RoomKind::Special | RoomKind::Secret | RoomKind::Shop => (5, 10, 5, 10),
        RoomKind::Entrance | RoomKind::Exit => (min_w.max(5), max_w, min_h.max(5), max_h),
        RoomKind::Standard => (min_w, max_w, min_h, max_h),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_room_dimension_overrides_match_subclasses() {
        let cases = [
            (RoomKind::Standard, 1, "CaveRoom", (5, 10, 5, 10)),
            (RoomKind::Entrance, 1, "CaveEntranceRoom", (7, 10, 7, 10)),
            (RoomKind::Exit, 2, "ChasmExitRoom", (10, 14, 10, 14)),
            (RoomKind::Standard, 1, "PlantsRoom", (5, 10, 5, 10)),
            (RoomKind::Standard, 1, "PlatformRoom", (6, 10, 6, 10)),
            (RoomKind::Standard, 1, "AquariumRoom", (7, 10, 7, 10)),
            (RoomKind::Standard, 2, "StudyRoom", (10, 14, 10, 14)),
            (
                RoomKind::Entrance,
                2,
                "RegionDecoBridgeEntranceRoom",
                (10, 14, 10, 14),
            ),
            (RoomKind::Standard, 1, "CavesFissureRoom", (7, 10, 7, 10)),
            (
                RoomKind::Entrance,
                2,
                "CircleWallEntranceRoom",
                (11, 14, 11, 14),
            ),
            (RoomKind::Standard, 1, "SewerPipeRoom", (7, 10, 7, 10)),
            (
                RoomKind::Entrance,
                1,
                "WaterBridgeEntranceRoom",
                (7, 10, 7, 10),
            ),
            (RoomKind::Standard, 2, "CircleBasinRoom", (11, 14, 11, 14)),
            (RoomKind::Standard, 1, "RegionDecoLineRoom", (5, 10, 5, 10)),
            (
                RoomKind::Entrance,
                1,
                "RegionDecoLineEntranceRoom",
                (7, 10, 7, 10),
            ),
            (RoomKind::Standard, 1, "SegmentedRoom", (7, 10, 7, 10)),
            (RoomKind::Standard, 1, "PillarsRoom", (7, 10, 7, 10)),
            (RoomKind::Exit, 1, "ChasmBridgeExitRoom", (7, 10, 7, 10)),
            (
                RoomKind::Entrance,
                2,
                "CellBlockEntranceRoom",
                (10, 14, 10, 14),
            ),
            (RoomKind::Standard, 1, "HallwayRoom", (5, 10, 5, 10)),
            (RoomKind::Standard, 1, "LibraryHallRoom", (7, 10, 7, 10)),
            (RoomKind::Standard, 1, "LibraryRingRoom", (9, 10, 9, 10)),
            (RoomKind::Exit, 2, "LibraryRingExitRoom", (13, 14, 13, 14)),
            (RoomKind::Entrance, 1, "StatuesEntranceRoom", (7, 10, 7, 10)),
            (RoomKind::Standard, 1, "SkullsRoom", (7, 10, 7, 10)),
            (RoomKind::Standard, 1, "RitualRoom", (9, 10, 9, 10)),
            (RoomKind::Special, 1, "PoolRoom", (6, 10, 6, 10)),
            (RoomKind::Special, 1, "RunestoneRoom", (6, 10, 6, 10)),
            (RoomKind::Special, 1, "MagicalFireRoom", (7, 10, 7, 10)),
            (RoomKind::Special, 1, "TrapsRoom", (6, 8, 6, 8)),
            (
                RoomKind::Entrance,
                2,
                "RitualEntranceRoom",
                (10, 14, 10, 14),
            ),
        ];
        for (kind, size, name, expected) in cases {
            assert_eq!(dims_for_kind(kind, size, name), expected, "{name}");
        }
    }
}
