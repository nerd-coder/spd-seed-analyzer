//! Pinned SPD v3.3.8 `SewerBossLevel` structural generation.

use crate::builders;
use crate::dungeon::DungeonState;
use crate::items::model::GeneratedItem;
use crate::level::{map_facts, painter, special_loot, terrain, Feeling};
use crate::random::Random;
use crate::report::{FloorMap, MapTransition};
use crate::rooms::room::{dims_for_kind, Room};
use crate::rooms::standard;
use crate::rooms::types::RoomKind;

const RETRY_LIMIT: u32 = 10_000;

#[cfg(test)]
thread_local! {
    static LAST_PRE_ITEMS_RNG: std::cell::RefCell<Vec<i32>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

#[cfg(test)]
pub(super) fn last_pre_items_rng() -> Vec<i32> {
    LAST_PRE_ITEMS_RNG.with(|probe| probe.borrow().clone())
}

pub(super) fn build(dungeon: &mut DungeonState, depth_seed: i64) -> Option<FloorMap> {
    // SewerBossLevel.builder() runs before initRooms on every outer build.
    let intensity = Random::float_range(0.3, 0.8);
    let mut rooms = init_rooms();
    Random::shuffle_list(&mut rooms);
    for (id, room) in rooms.iter_mut().enumerate() {
        room.id = id;
    }
    if !builders::build_sewer_boss_rooms(&mut rooms, intensity, RETRY_LIMIT) {
        return None;
    }

    terrain::shift_rooms_for_painter(&mut rooms, false);
    let mut map = terrain::paint_minimal(&rooms)?;
    let mut empty_items = Vec::<GeneratedItem>::new();
    let special = special_loot::special_room_loot(
        dungeon,
        &rooms,
        &mut map,
        &mut empty_items,
        &[],
        Feeling::None,
    );
    let mut doors = special.doors;
    painter::paint_doors(
        &mut map,
        &rooms,
        &special.paint_order,
        5,
        Feeling::None,
        &mut doors,
    );
    painter::paint_sewer_boss_environment(&mut map, &rooms, &special.paint_order, &doors);

    #[cfg(test)]
    LAST_PRE_ITEMS_RNG.with(|probe| *probe.borrow_mut() = Random::peek_ints(8));

    for tile in &mut map.map {
        *tile = match *tile {
            terrain::EMPTY_DECO | terrain::REGION_DECO | terrain::REGION_DECO_ALT => terrain::EMPTY,
            terrain::WALL_DECO => terrain::WALL,
            tile => tile,
        };
    }

    let mut floor = map_facts::MapFacts::from_room_paint(&map)
        .into_floor_map(&map, 5, dungeon.branch, depth_seed)
        .into_layout_only();
    floor.transitions = sewer_transitions(&map);
    Some(floor)
}

fn init_rooms() -> Vec<Room> {
    let mut rooms = Vec::new();
    let _ = Random::chances(&[1.0, 0.0, 0.0]);
    push(&mut rooms, "SewerBossEntranceRoom", RoomKind::Entrance, 1, 16);
    let _ = Random::chances(&[1.0, 0.0, 0.0]);
    push(&mut rooms, "SewerBossExitRoom", RoomKind::Exit, 1, 16);
    for _ in 0..3 {
        let (name, _created_size) = standard::create_standard_room(5);
        let _ = Random::chances(&[1.0, 0.0, 0.0]);
        push(&mut rooms, &name, RoomKind::Standard, 1, 16);
    }
    let goo = match Random::int_max(4) {
        1 => "WalledGooRoom",
        2 => "ThinPillarsGooRoom",
        3 => "ThickPillarsGooRoom",
        _ => "DiamondGooRoom",
    };
    let _ = Random::chances(&[0.0, 1.0, 0.0]);
    push(&mut rooms, goo, RoomKind::Standard, 2, 16);
    push(&mut rooms, "RatKingRoom", RoomKind::Secret, 1, 1);
    rooms
}

fn push(rooms: &mut Vec<Room>, name: &str, kind: RoomKind, size: i32, max_connections: i32) {
    let (min_w, max_w, min_h, max_h) = dims_for_kind(kind, size, name);
    rooms.push(Room::new(
        rooms.len(), name, kind, size, max_connections, min_w, max_w, min_h, max_h,
    ));
}

fn sewer_transitions(map: &terrain::TerrainMap) -> Vec<MapTransition> {
    let mut out = Vec::new();
    for (cell, &tile) in map.map.iter().enumerate() {
        let (kind, dest, dest_type, bounds) = if tile == terrain::ENTRANCE {
            ("REGULAR_ENTRANCE", 4, "REGULAR_EXIT", None)
        } else if tile == terrain::LOCKED_EXIT {
            let x = cell as u32 % map.width as u32;
            let y = cell as u32 / map.width as u32;
            ("REGULAR_EXIT", 6, "REGULAR_ENTRANCE", Some((x - 1, y - 1, x + 1, y)))
        } else {
            continue;
        };
        let x = cell as u32 % map.width as u32;
        let y = cell as u32 / map.width as u32;
        let (left, top, right, bottom) = bounds.unwrap_or((x, y, x, y));
        out.push(MapTransition {
            cell: cell as u32,
            transition_type: kind.into(),
            left,
            top,
            right,
            bottom,
            dest_depth: dest,
            dest_branch: 0,
            dest_type: Some(dest_type.into()),
        });
    }
    out.sort_by_key(|transition| transition.cell);
    out
}
