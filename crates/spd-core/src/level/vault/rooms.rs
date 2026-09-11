//! `VaultLevel.initRooms`, `VaultRoom.createRoom`, treasure round-robin.

#![allow(dead_code)] // generate is test-only until PR 4

use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

const CHANCE_ROOMS: [&str; 10] = [
    "VaultRingRoom",
    "VaultCrossRoom",
    "VaultQuadrantsRoom",
    "VaultRingsRoom",
    "VaultEnemyCenterRoom",
    "VaultHallwayRoom",
    "VaultLongRingsRoom",
    "VaultCircleRoom",
    "VaultAlternatingFireRoom",
    "VaultLasersRoom",
];
const CHANCE_WEIGHTS: [f32; 10] = [2.0, 2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0];

const T1_TREASURE: [&str; 3] = [
    "VaultFlamePathRoom",
    "VaultLaserTreasureRoom",
    "VaultCircleScanTreasureRoom",
];
const T2_TREASURE: [&str; 3] = [
    "VaultSingleEnemyTreasureRoom",
    "VaultBookcaseTreasureRoom",
    "VaultFlamesTreasureRoom",
];
const T3_TREASURE: [&str; 3] = [
    "VaultManyScansRoom",
    "VaultMultipleEnemyTreasureRoom",
    "VaultHardLaserTreasureRoom",
];

pub(super) fn init() -> Vec<Room> {
    let mut rooms = vec![vault_room(
        "VaultEntranceRoom",
        RoomKind::Entrance,
        1,
        16,
        11,
        11,
        false,
    )];
    // Java `VaultRoom.setupChances()` assigns the weights, then createRoom
    // mutates that array. This local copy is that array.
    let mut chances = CHANCE_WEIGHTS;
    let mut size = 0;
    while size < 9 {
        let room = create_chance_room(&mut chances);
        size += room.size_factor;
        rooms.push(room);
    }
    rooms.push(vault_room(
        "VaultTokensRoom",
        RoomKind::Standard,
        2,
        16,
        11,
        21,
        true,
    ));
    rooms.push(vault_room(
        "VaultSimpleEnemyTreasureRoom",
        RoomKind::Standard,
        1,
        16,
        11,
        11,
        false,
    ));

    let treasures = generate_treasure_list();
    for name in treasures.into_iter().take(7) {
        rooms.push(vault_room(name, RoomKind::Standard, 1, 1, 11, 11, false));
    }
    rooms.push(Room::new(
        0,
        "VaultFinalRoom",
        RoomKind::Exit,
        1,
        1,
        21,
        21,
        21,
        21,
    ));
    for (id, room) in rooms.iter_mut().enumerate() {
        room.id = id;
    }
    rooms
}

fn create_chance_room(chances: &mut [f32; 10]) -> Room {
    let mut idx = Random::chances(chances);
    if idx < 0 {
        *chances = CHANCE_WEIGHTS;
        idx = Random::chances(chances);
    }
    chances[idx as usize] -= 1.0;
    let name = CHANCE_ROOMS[idx as usize];
    let long = matches!(name, "VaultHallwayRoom" | "VaultLongRingsRoom");
    vault_room(
        name,
        RoomKind::Standard,
        if long { 2 } else { 1 },
        16,
        11,
        if long { 21 } else { 11 },
        long,
    )
}

fn generate_treasure_list() -> Vec<&'static str> {
    let mut t1 = T1_TREASURE.to_vec();
    Random::shuffle_list(&mut t1);
    let mut t2 = T2_TREASURE.to_vec();
    Random::shuffle_list(&mut t2);
    let mut t3 = T3_TREASURE.to_vec();
    Random::shuffle_list(&mut t3);
    let mut full = vec![t1, t2, t3];
    let mut spawned = Vec::new();
    while !full.is_empty() {
        let mut current = full.remove(0);
        spawned.push(current.remove(0));
        if !current.is_empty() {
            full.push(current);
        }
    }
    spawned
}

#[allow(clippy::too_many_arguments)] // mirrors VaultRoom constructor size + orientation
fn vault_room(
    name: &str,
    kind: RoomKind,
    size_factor: i32,
    max_connections: i32,
    short_side: i32,
    long_side: i32,
    long: bool,
) -> Room {
    // VaultRoom.sizeCatProbs is always LARGE; the roll is still consumed.
    let _ = Random::chances(&[0.0, 1.0, 0.0]);
    let (min_w, max_w, min_h, max_h) = if long {
        if Random::int_max(2) == 0 {
            (long_side, long_side, short_side, short_side)
        } else {
            (short_side, short_side, long_side, long_side)
        }
    } else {
        (short_side, short_side, short_side, short_side)
    };
    Room::new(
        0,
        name,
        kind,
        size_factor,
        max_connections,
        min_w,
        max_w,
        min_h,
        max_h,
    )
}
