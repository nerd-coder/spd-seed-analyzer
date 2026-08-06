//! Mining room constructors and objective-specific paint callbacks.

use std::collections::VecDeque;

use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::painter::{self, clean_diagonal_edges, setup_patch, DoorMap, DoorType};
use crate::level::terrain::{
    TerrainMap, BARRICADE, EMPTY, EMPTY_DECO, EMPTY_SP, ENTRANCE, MINE_BOULDER, MINE_CRYSTAL, TRAP,
    WALL, WALL_DECO,
};
use crate::quests::BlacksmithQuestType;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::geometry::{
    fill_ellipse, fill_ellipse_raw_rect, fill_raw_rect, fill_room, neighbour8, occupy_mob,
    point_distance, set,
};

pub(super) fn init() -> Vec<Room> {
    let mut rooms = Vec::new();
    rooms.push(new_room("MineEntrance", RoomKind::Entrance, &[1., 0., 0.]));
    let mut giant = new_room("MineGiantRoom", RoomKind::Standard, &[0., 0., 1.]);
    giant.size_factor = Random::chances(&[0., 0., 1.]) + 1;
    set_dimensions(&mut giant);
    rooms.push(giant);
    for _ in 0..3 {
        let mut room = new_room("MineLargeRoom", RoomKind::Standard, &[0., 1., 0.]);
        room.size_factor = Random::chances(&[0., 1., 0.]) + 1;
        set_dimensions(&mut room);
        rooms.push(room);
    }
    let small_count = Random::normal_int_range(6, 8);
    for _ in 0..small_count {
        let mut room = new_room("MineSmallRoom", RoomKind::Standard, &[1., 0., 0.]);
        room.size_factor = Random::chances(&[1., 0., 0.]) + 1;
        set_dimensions(&mut room);
        rooms.push(room);
    }
    for _ in 0..2 {
        let id = rooms.len();
        rooms.push(Room::new(
            id,
            "MineSecretRoom",
            RoomKind::Secret,
            1,
            1,
            5,
            7,
            5,
            7,
        ));
    }
    for (id, room) in rooms.iter_mut().enumerate() {
        room.id = id;
    }
    Random::shuffle_list(&mut rooms);
    for (id, room) in rooms.iter_mut().enumerate() {
        room.id = id;
    }
    rooms
}

fn new_room(name: &str, kind: RoomKind, chances: &[f32]) -> Room {
    let size_factor = Random::chances(chances) + 1;
    let mut room = Room::new(0, name, kind, size_factor, 16, 4, 10, 4, 10);
    set_dimensions(&mut room);
    room
}

fn set_dimensions(room: &mut Room) {
    let (min, max) = match room.size_factor {
        2 => (10, 14),
        3 => (14, 18),
        _ => (4, 10),
    };
    room.min_w = min;
    room.max_w = max;
    room.min_h = min;
    room.max_h = max;
    match room.name.as_str() {
        "MineEntrance" => {
            room.min_w = room.min_w.max(7);
            room.min_h = room.min_h.max(7);
        }
        "MineLargeRoom" => {
            room.min_w = 11;
            room.min_h = 11;
        }
        "MineSmallRoom" => {
            room.min_w = room.min_w.max(6);
            room.min_h = room.min_h.max(6);
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn paint(
    map: &mut TerrainMap,
    rooms: &[Room],
    room_index: usize,
    doors: &mut DoorMap,
    objective: BlacksmithQuestType,
    depth: i32,
    dark_gold: &mut i32,
) {
    let room = &rooms[room_index];
    if room.kind == RoomKind::Connection {
        painter::apply_room_door_types(rooms, room_index, doors);
        painter::paint_connection_room(map, rooms, room_index, doors, false);
        return;
    }
    if room.kind == RoomKind::Secret {
        paint_secret(map, room, room_index, doors, objective, dark_gold);
        return;
    }

    paint_cave_base(map, room, room_index, doors, fill_for(room));
    if room.name == "MineEntrance" {
        paint_entrance(map, room, depth);
    }
    match objective {
        BlacksmithQuestType::Crystal => paint_crystal(map, room, room_index),
        BlacksmithQuestType::Gnoll => paint_gnoll(map, rooms, room_index, doors),
        BlacksmithQuestType::Fungi => unreachable!(),
    }
    if room.name == "MineEntrance" && objective == BlacksmithQuestType::Crystal {
        scatter_entrance_crystal(map, room);
    }
}

fn fill_for(room: &Room) -> f32 {
    match room.name.as_str() {
        "MineGiantRoom" => 0.70,
        "MineLargeRoom" => 0.55,
        "MineSmallRoom" => 0.40,
        _ => 0.30 + (room.width() * room.height()).min(18 * 18) as f32 / 1024.0,
    }
}

fn paint_cave_base(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &mut DoorMap,
    fill: f32,
) {
    fill_room(map, room, 0, WALL);
    fill_room(map, room, 1, EMPTY);
    for &other in &room.connected {
        if let Some(door) = doors.get_mut(room_index, other) {
            door.set(DoorType::Regular);
        }
    }
    let mut patch = setup_patch(room, room_index, doors, fill, 3, !room.connected.is_empty());
    clean_diagonal_edges(&mut patch, room.width() - 2);
    let width = room.width() - 2;
    for y in room.top + 1..room.bottom {
        for x in room.left + 1..room.right {
            if patch[((x - room.left - 1) + (y - room.top - 1) * width) as usize] {
                set(map, x, y, WALL);
            }
        }
    }
}

fn paint_crystal(map: &mut TerrainMap, room: &Room, room_index: usize) {
    match room.name.as_str() {
        "MineGiantRoom" => {
            fill_ellipse(map, room, 3, EMPTY);
            scatter(map, room, room.width() * room.height() / 2, MINE_CRYSTAL);
            let center = room.as_rect().center_room();
            // CrystalSpire chooses one of three sprite colors in its constructor.
            let _color = Random::int_max(3);
            occupy_mob(map, center, "CrystalSpire");
            set(map, center.x, center.y, EMPTY);
        }
        "MineLargeRoom" => {
            fill_ellipse(map, room, 3, MINE_CRYSTAL);
            fill_ellipse(map, room, 4, EMPTY);
            let point = room.random_margin(5);
            seal_internal_crystal(map, map.point_to_cell(point.x, point.y).unwrap());
            scatter(map, room, room.width() * room.height() / 4, MINE_CRYSTAL);
            // CrystalGuardian's constructor rolls its sprite color.
            let _color = Random::int_max(3);
            occupy_mob(map, point, "CrystalGuardian");
            set(map, point.x, point.y, EMPTY);
        }
        "MineEntrance" => {
            // Applied after the exit is selected, unlike the other room callbacks.
        }
        "MineSmallRoom" => scatter(map, room, room.width() * room.height() / 3, MINE_CRYSTAL),
        _ => {}
    }
    let _ = room_index;
}

fn paint_entrance(map: &mut TerrainMap, room: &Room, depth: i32) {
    let entrance = loop {
        let point = room.random_margin(3);
        let cell = map.point_to_cell(point.x, point.y).unwrap();
        let w = map.width as isize;
        let valid = [-w - 1, -w, -w + 1, -1, 0, 1, w - 1, w, w + 1]
            .iter()
            .any(|offset| map.map[(cell as isize + offset) as usize] != WALL);
        if valid || (room.height() == 7 && room.width() == 7) {
            break point;
        }
    };
    let cell = map.point_to_cell(entrance.x, entrance.y).unwrap();
    map.map[cell] = ENTRANCE;
    let w = map.width as isize;
    for offset in [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1] {
        map.map[(cell as isize + offset) as usize] = EMPTY;
    }
    map.branch_entrances.push(cell);
    map.record_custom_tile(
        "QuestExit",
        "environment/custom_tiles/caves_quest.png",
        (entrance.x - 1, entrance.y - 1, 3, 3),
        vec![8, 9, 10, 16, 17, 18, 24, 25, 26],
    );
    let _ = depth;
}

fn scatter_entrance_crystal(map: &mut TerrainMap, room: &Room) {
    let entrance = map.branch_entrances[0];
    let ex = entrance as i32 % map.width;
    let ey = entrance as i32 / map.width;
    for _ in 0..room.width() * room.height() / 2 {
        let point = room.random_margin(1);
        let cell = map.point_to_cell(point.x, point.y).unwrap();
        if (point.x - ex).abs().max((point.y - ey).abs()) > 1 && map.map[cell] != WALL {
            map.map[cell] = MINE_CRYSTAL;
        }
    }
}

fn paint_gnoll(map: &mut TerrainMap, rooms: &[Room], room_index: usize, doors: &mut DoorMap) {
    let room = &rooms[room_index];
    if matches!(room.name.as_str(), "MineGiantRoom" | "MineLargeRoom") {
        fill_ellipse(map, room, 3, EMPTY);
    }
    lock_mining_doors(rooms, room_index, doors);
    let wall_doors = room
        .connected
        .iter()
        .filter_map(|&other| doors.get(room_index, other))
        .filter(|door| door.door_type == DoorType::Wall)
        .map(|door| Point::new(door.x, door.y))
        .collect::<Vec<_>>();

    if room.name == "MineLargeRoom" {
        let sapper = room.random_margin(5);
        // GnollSapper initializes abilityCooldown before it is added.
        let _cooldown = Random::normal_int_range(4, 6);
        occupy_mob(map, sapper, "GnollSapper");
        let neighbours = neighbour8(map.width);
        let guard_cell = loop {
            let cell = (map.point_to_cell(sapper.x, sapper.y).unwrap() as isize
                + neighbours[Random::int_max(8) as usize]) as usize;
            if map.map[cell] == EMPTY {
                break cell;
            }
        };
        map.mob_occupied[guard_cell] = true;
        map.known_mobs[guard_cell] = Some("GnollGuard");
        let barricades = if Random::int_max(2) == 0 { 2 } else { 1 };
        let sapper_cell = map.point_to_cell(sapper.x, sapper.y).unwrap();
        for _ in 0..barricades {
            loop {
                let cell =
                    (sapper_cell as isize + neighbours[Random::int_max(8) as usize]) as usize;
                if map.map[cell] == EMPTY && cell != guard_cell {
                    map.map[cell] = BARRICADE;
                    break;
                }
            }
        }
        let trap_count = if room.square() > 150 { 3 } else { 2 };
        for _ in 0..trap_count {
            loop {
                let point = room.random_margin(2);
                let cell = map.point_to_cell(point.x, point.y).unwrap();
                if map.map[cell] == EMPTY && cell != sapper_cell && cell != guard_cell {
                    map.map[cell] = TRAP;
                    map.trap_names[cell] = Some("GnollRockfallTrap");
                    break;
                }
            }
        }
    }

    let max_dist = match room.name.as_str() {
        "MineGiantRoom" => 3.1,
        "MineLargeRoom" => 4.0,
        _ => 5.0,
    };
    let deco_dist = if room.name == "MineSmallRoom" {
        2.0
    } else {
        3.0
    };
    let subtract = if matches!(
        room.name.as_str(),
        "MineEntrance" | "MineGiantRoom" | "MineLargeRoom"
    ) {
        0.5
    } else {
        0.0
    };
    for x in room.left..=room.right {
        for y in room.top..=room.bottom {
            let cell = map.point_to_cell(x, y).unwrap();
            if map.map[cell] != EMPTY || map.mob_occupied[cell] {
                continue;
            }
            if room.name == "MineEntrance"
                && map.branch_entrances.first().is_some_and(|&entrance| {
                    let ex = entrance as i32 % map.width;
                    let ey = entrance as i32 / map.width;
                    (x - ex).abs().max((y - ey).abs()) <= 1
                })
            {
                continue;
            }
            let mut distance = 1000.0f32;
            for door in &wall_doors {
                distance = distance.min(point_distance(Point::new(x, y), *door));
            }
            distance = (distance - subtract).clamp(1.0, max_dist);
            let value = Random::float_max(f64::from(distance).powi(2) as f32);
            if value <= 0.75 || distance <= 1.0 {
                map.map[cell] = MINE_BOULDER;
            } else if value <= 5.0 && distance <= deco_dist {
                map.map[cell] = EMPTY_DECO;
            }
        }
    }
    if room.name == "MineGiantRoom" {
        let center = room.as_rect().center_room();
        fill_ellipse_raw_rect(
            map,
            center.x - 2,
            center.y - 2,
            center.x + 3,
            center.y + 3,
            MINE_BOULDER,
        );
        fill_raw_rect(
            map,
            center.x - 2,
            center.y - 2,
            center.x + 3,
            center.y + 3,
            2,
            EMPTY_DECO,
        );
        occupy_mob(map, center, "GnollGeomancer");
        // GnollGeomancer initializes abilityCooldown during construction.
        let _cooldown = Random::normal_int_range(3, 5);
    }
}

fn lock_mining_doors(rooms: &[Room], room_index: usize, doors: &mut DoorMap) {
    for &other in &rooms[room_index].connected {
        if rooms[other].kind != RoomKind::Secret
            && doors
                .get(room_index, other)
                .is_some_and(|door| door.door_type == DoorType::Regular)
        {
            let kind = if Random::int_max(10) == 0 {
                DoorType::Empty
            } else {
                DoorType::Wall
            };
            doors.get_mut(room_index, other).unwrap().set_and_lock(kind);
        }
    }
}

fn paint_secret(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &mut DoorMap,
    objective: BlacksmithQuestType,
    dark_gold: &mut i32,
) {
    fill_room(map, room, 0, WALL);
    for &other in &room.connected {
        if let Some(door) = doors.get_mut(room_index, other) {
            door.set(DoorType::Hidden);
        }
    }
    match objective {
        BlacksmithQuestType::Crystal => {
            fill_room(map, room, 1, MINE_CRYSTAL);
            let amount = Random::normal_int_range(4, 5);
            for _ in 0..amount {
                loop {
                    let checked = room.random_margin(1);
                    let checked_cell = map.point_to_cell(checked.x, checked.y).unwrap();
                    if map.map[checked_cell] != WALL_DECO {
                        let point = room.random_margin(1);
                        set(map, point.x, point.y, WALL_DECO);
                        break;
                    }
                }
            }
        }
        BlacksmithQuestType::Gnoll => {
            fill_room(map, room, 1, EMPTY_SP);
            let amount = Random::normal_int_range(4, 5);
            *dark_gold += amount;
            let center = room.as_rect().center_room();
            let cell = map.point_to_cell(center.x, center.y).unwrap();
            let mut gold = GeneratedItem::new("DarkGold", ItemCategory::Other);
            gold.quantity = amount;
            map.record_heap(cell, "chest", gold);
        }
        BlacksmithQuestType::Fungi => unreachable!(),
    }
}

fn scatter(map: &mut TerrainMap, room: &Room, count: i32, terrain: i32) {
    for _ in 0..count {
        let point = room.random_margin(1);
        let cell = map.point_to_cell(point.x, point.y).unwrap();
        if map.map[cell] != WALL {
            map.map[cell] = terrain;
        }
    }
}

fn seal_internal_crystal(map: &mut TerrainMap, start: usize) {
    let w = map.width as isize;
    let mut internal = vec![false; map.len()];
    let mut queue = VecDeque::from([start]);
    internal[start] = true;
    while let Some(cell) = queue.pop_front() {
        for offset in [-1, 1, -w, w] {
            let next = (cell as isize + offset) as usize;
            if !internal[next] && map.map[next] != MINE_CRYSTAL {
                internal[next] = true;
                queue.push_back(next);
            }
        }
    }
    let offsets = neighbour8(map.width);
    for cell in 0..internal.len() {
        if !internal[cell] {
            continue;
        }
        if offsets.iter().any(|offset| {
            let next = (cell as isize + offset) as usize;
            !internal[next] && map.map[next] != MINE_CRYSTAL
        }) {
            map.map[cell] = MINE_CRYSTAL;
        }
    }
}
