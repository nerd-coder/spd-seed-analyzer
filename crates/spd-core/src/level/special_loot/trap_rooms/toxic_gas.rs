//! Exact paint-time heap association for `ToxicGasRoom`.

use super::super::placement::find_prize_item;
use crate::dungeon::DungeonState;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, EMPTY, INACTIVE_TRAP, STATUE, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

pub(crate) fn paint(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    fill_room(map, room);
    let center = room.as_rect().center_room();
    set(map, center, STATUE);

    // Java accepts vents only on EMPTY, and changes each accepted cell to an
    // inactive trap before choosing the next one.
    let traps = (room.width() - 2).min(room.height() - 2).max(0);
    for _ in 0..traps {
        let point = random_point(room, 2);
        let point = retry_until(room, 2, point, |cell| map.map[cell] == EMPTY, map);
        set(map, point, INACTIVE_TRAP);
    }

    // Gold positions may overlap vents. They exclude only the statue and
    // earlier gold positions, exactly as the pinned ArrayList.contains loop.
    let mut positions = Vec::with_capacity(8);
    for _ in 0..10_000 {
        if positions.len() == 8 {
            break;
        }
        let point = random_point(room, 2);
        let cell = cell(map, point);
        if map.map[cell] != STATUE && !positions.contains(&point) {
            positions.push(point);
        }
    }
    assert_eq!(
        positions.len(),
        8,
        "ToxicGasRoom must fit eight gold positions: {room:?}"
    );

    let entrance = room
        .connected
        .iter()
        .find_map(|&other| doors.get(ri, other))
        .map(|door| Point::new(door.x, door.y))
        .expect("placed ToxicGasRoom has an entrance");
    // Java replaces the winner only for a strictly greater distance, so an
    // equal-distance tie retains the earliest ArrayList entry.
    let mut furthest_index = 0;
    for index in 1..positions.len() {
        if squared_distance(entrance, positions[index])
            > squared_distance(entrance, positions[furthest_index])
        {
            furthest_index = index;
        }
    }
    let skeleton = positions.remove(furthest_index);

    let mut main = GeneratedItem::new("Gold", ItemCategory::Gold);
    randomize_item(&mut main, dungeon.depth);
    main.quantity = main.quantity.saturating_mul(2);
    main.source = Some("ToxicGasRoom".into());
    map.record_heap(cell(map, skeleton), "skeleton", main.clone());
    let mut out = vec![PlacedLoot {
        item: main,
        heap_type: "skeleton",
    }];

    for point in positions.drain(..2) {
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                let mut gold = GeneratedItem::new("Gold", ItemCategory::Gold);
                randomize_item(&mut gold, dungeon.depth);
                gold
            });
        item.source = Some("ToxicGasRoom".into());
        map.record_heap(cell(map, point), "chest", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "chest",
        });
    }

    items_to_spawn.push(GeneratedItem::new("PotionOfPurity", ItemCategory::Potion));
    out
}

fn fill_room(map: &mut TerrainMap, room: &Room) {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let terrain = if x == room.left || x == room.right || y == room.top || y == room.bottom
            {
                WALL
            } else {
                EMPTY
            };
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = terrain;
            }
        }
    }
}

fn retry_until(
    room: &Room,
    margin: i32,
    first: Point,
    accepts: impl Fn(usize) -> bool,
    map: &TerrainMap,
) -> Point {
    let mut point = first;
    for _ in 0..10_000 {
        if accepts(cell(map, point)) {
            return point;
        }
        point = random_point(room, margin);
    }
    panic!("ToxicGasRoom could not place a vent: {room:?}")
}

fn random_point(room: &Room, margin: i32) -> Point {
    Point::new(
        Random::int_range_inclusive(room.left + margin, room.right - margin),
        Random::int_range_inclusive(room.top + margin, room.bottom - margin),
    )
}

fn squared_distance(a: Point, b: Point) -> i64 {
    let dx = i64::from(a.x - b.x);
    let dy = i64::from(a.y - b.y);
    dx * dx + dy * dy
}

fn set(map: &mut TerrainMap, point: Point, terrain: i32) {
    let cell = cell(map, point);
    map.map[cell] = terrain;
}

fn cell(map: &TerrainMap, point: Point) -> usize {
    map.point_to_cell(point.x, point.y)
        .expect("ToxicGasRoom point lies inside map")
}
