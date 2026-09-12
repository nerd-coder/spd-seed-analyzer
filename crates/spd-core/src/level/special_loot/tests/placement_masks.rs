//! Source-pinned RitualSiteRoom placement and cage-paint coverage.

use super::super::quest_rooms::ritual_site_setup;
use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{
    paint_minimal, CUSTOM_DECO, CUSTOM_DECO_EMPTY, EMPTY, REGION_DECO, WALL,
};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

fn ritual_site_10x10() -> Room {
    let mut room = Room::new(
        0,
        "RitualSiteRoom",
        RoomKind::Standard,
        1,
        16,
        10,
        10,
        10,
        10,
    );
    room.left = 0;
    room.top = 0;
    room.right = 9;
    room.bottom = 9;
    room
}

fn burn_ritual_site_paint_rng(room: &Room, doors: &[Point]) {
    let mut valid = [true; 10];
    valid[0] = false;
    valid[3] = false;
    valid[6] = false;
    valid[9] = false;
    for door in doors {
        if door.y == room.top {
            valid[(door.x - room.left) as usize] = false;
        } else if door.y <= room.top + 2 {
            if door.x == room.left {
                valid[1] = false;
            } else {
                valid[8] = false;
            }
        }
    }
    let _ = Random::int_max(2);
    for i in (room.left..room.right).step_by(3) {
        let plus_one = (i + 1 - room.left) as usize;
        let plus_two = (i + 2 - room.left) as usize;
        if !valid[plus_one] && !valid[plus_two] {
            continue;
        }
        let chosen = if !valid[plus_two] {
            i + 1
        } else if !valid[plus_one] {
            i + 2
        } else {
            i + Random::int_range_inclusive(1, 2)
        };
        valid[(chosen - room.left) as usize] = false;
    }
    let mut tries = 100;
    loop {
        let i = Random::int_range_inclusive(1, 9);
        if valid[i as usize] {
            tries = 0;
        }
        let keep_going = tries > 0;
        tries -= 1;
        if !keep_going {
            break;
        }
    }
}

#[test]
fn ritual_site_blocks_exact_chebyshev_radius_around_shifted_center() {
    let room = ritual_site_10x10();
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("map");
    let mut items = Vec::new();
    let doors = DoorMap::new();

    Random::push_generator_seeded(0xC4AD1E);
    ritual_site_setup(std::slice::from_ref(&room), 0, &mut map, &doors, &mut items);
    let actual_next = Random::peek_ints(4);
    Random::pop_generator();

    Random::push_generator_seeded(0xC4AD1E);
    burn_ritual_site_paint_rng(&room, &[]);
    let mut ritual = room.as_rect().center_room();
    ritual.y += 1;
    let expected_next = Random::peek_ints(4);
    Random::pop_generator();

    assert_eq!(actual_next, expected_next);
    assert_eq!(items.len(), 4);
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let cell = map.point_to_cell(x, y).expect("room cell");
            let inside = x > room.left && y > room.top && x < room.right && y < room.bottom;
            let far = (x - ritual.x).abs().max((y - ritual.y).abs()) >= 2;
            let allowed = inside && far;
            assert_eq!(map.item_allowed[cell], allowed, "item {x},{y}");
            assert_eq!(map.character_allowed[cell], allowed, "char {x},{y}");
            if (x - ritual.x).abs().max((y - ritual.y).abs()) < 2 {
                assert_eq!(map.map[cell], CUSTOM_DECO_EMPTY);
            }
        }
    }
    assert_eq!(
        map.custom_tiles[0].x as i32,
        ritual.x - 2,
        "marker origin after center.y++"
    );
    assert_eq!(map.custom_tiles[0].y as i32, ritual.y - 2);
}

#[test]
fn cages_consume_int_then_int_range_before_center() {
    let room = ritual_site_10x10();
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("map");
    let doors = DoorMap::new();
    let mut items = Vec::new();
    let seed = 0xC4AD1E;

    Random::push_generator_seeded(seed);
    ritual_site_setup(std::slice::from_ref(&room), 0, &mut map, &doors, &mut items);
    let after_paint = Random::peek_ints(4);
    Random::pop_generator();

    Random::push_generator_seeded(seed);
    burn_ritual_site_paint_rng(&room, &[]);
    let _ = room.as_rect().center_room();
    let after_replay = Random::peek_ints(4);
    Random::pop_generator();

    assert_eq!(after_paint, after_replay);
    assert_eq!(items.len(), 4);
    let cell = |x, y| map.point_to_cell(x, y).expect("cell");
    assert_eq!(map.map[cell(room.left + 3, room.top + 1)], WALL);
    assert_eq!(map.map[cell(room.left + 6, room.top + 1)], WALL);
    assert_eq!(map.custom_tiles.len(), 1);
    assert_eq!(map.custom_tiles[0].class_name, "RitualMarker");
    assert_eq!(
        (map.custom_tiles[0].width, map.custom_tiles[0].height),
        (5, 5)
    );
    assert!(!map.custom_terrain.is_empty());
    assert!(map
        .custom_terrain
        .iter()
        .all(|layer| layer.class_name == "Table" && layer.width == 1 && layer.height == 2));
}

#[test]
fn top_door_forces_the_open_column_in_that_group() {
    let mut room = ritual_site_10x10();
    room.connected.push(1);
    let mut neighbor = Room::new(1, "EmptyRoom", RoomKind::Standard, 1, 16, 5, 5, 5, 5);
    neighbor.left = 3;
    neighbor.top = -4;
    neighbor.right = 7;
    neighbor.bottom = 0;
    neighbor.connected.push(0);
    let door = Point::new(room.left + 4, room.top);
    let mut doors = DoorMap::new();
    doors.insert_test_point(0, 1, door);
    let rooms = [room.clone(), neighbor];
    let mut map = paint_minimal(&rooms).expect("map");
    let mut items = Vec::new();

    Random::push_generator_seeded(91);
    ritual_site_setup(&rooms, 0, &mut map, &doors, &mut items);
    let after_paint = Random::peek_ints(4);
    Random::pop_generator();

    Random::push_generator_seeded(91);
    burn_ritual_site_paint_rng(&rooms[0], &[door]);
    let _ = rooms[0].as_rect().center_room();
    let after_replay = Random::peek_ints(4);
    Random::pop_generator();

    assert_eq!(after_paint, after_replay);
    let cell = |x, y| map.point_to_cell(x, y).expect("cell");
    let cage = cell(room.left + 5, room.top + 1);
    assert!(
        map.map[cage] == REGION_DECO || map.map[cage] == CUSTOM_DECO,
        "forced cage column"
    );
    assert_eq!(map.map[cell(room.left + 4, room.top + 1)], EMPTY);
}
