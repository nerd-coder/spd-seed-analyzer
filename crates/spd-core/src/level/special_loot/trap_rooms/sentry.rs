//! Exact seeded paint path for `SentryRoom`.

use super::super::placement::find_prize_item;
use super::{is_curse_enchant, sentry_equip};
use crate::dungeon::DungeonState;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_SP, PEDESTAL, STATUE, STATUE_SP, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

pub(crate) fn paint(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    let room = &rooms[ri];
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY_SP);
    let entrance = room
        .connected
        .first()
        .and_then(|&other| doors.get(ri, other))
        .map(|door| Point::new(door.x, door.y))
        .expect("placed SentryRoom has an entrance door");

    let center = select_center(room, entrance);
    let (sentry, treasure) = positions_and_path(map, room, entrance, center);
    set(map, sentry, PEDESTAL);
    let sentry_cell = cell(map, sentry);
    map.mob_occupied[sentry_cell] = true;
    map.known_mobs[sentry_cell] = Some("Sentry");

    // `new EmptyRoom()` constructs a StandardRoom for the sentry's bounds.
    // Its initializer calls `setSizeCat()` before the treasure prize is made.
    let _ = Random::chances(&[1.0, 0.0, 0.0]);

    set(map, treasure, PEDESTAL);
    let treasure_cell = cell(map, treasure);
    let mut prize = generate_prize(dungeon, items_to_spawn);
    let forced = prize.source.as_deref() == Some("forced");
    prize.source = Some(
        if forced {
            "SentryRoom:forced"
        } else {
            "SentryRoom"
        }
        .into(),
    );
    map.record_heap(treasure_cell, "chest", prize.clone());
    items_to_spawn.push(GeneratedItem::new("PotionOfHaste", ItemCategory::Potion));
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

fn select_center(room: &Room, entrance: Point) -> Point {
    let base = room.as_rect().center_deterministic();
    let x_can_differ =
        base.x != entrance.x || ((room.right - room.left) % 2 == 1 && base.x + 1 != entrance.x);
    let y_can_differ =
        base.y != entrance.y || ((room.bottom - room.top) % 2 == 1 && base.y + 1 != entrance.y);

    if x_can_differ && y_can_differ {
        loop {
            let center = room.as_rect().center_room();
            if center.x != entrance.x && center.y != entrance.y {
                return center;
            }
        }
    }

    // Pinned Java layouts make the retry above satisfiable through exact door
    // placement. Approximate layouts may violate that invariant; use a safe
    // interior fallback rather than spinning forever in WASM.
    Point::new(
        if base.x == entrance.x {
            (base.x + 1).min(room.right - 1)
        } else {
            base.x
        },
        if base.y == entrance.y {
            (base.y + 1).min(room.bottom - 1)
        } else {
            base.y
        },
    )
}

fn generate_prize(dungeon: &mut DungeonState, spawn: &mut Vec<GeneratedItem>) -> GeneratedItem {
    if Random::int_max(2) == 0 {
        if let Some(prize) = find_prize_item(spawn, None) {
            return prize;
        }
    }
    let mut prize = sentry_equip(dungeon);
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize
}

fn positions_and_path(map: &mut TerrainMap, r: &Room, e: Point, c: Point) -> (Point, Point) {
    if e.x == r.left {
        fill_rect(map, r.left + 1, r.top + 1, 1, r.height() - 2, EMPTY);
        let t = if e.y > c.y {
            fill_rect(map, r.left + 1, r.top + 1, 2, c.y - r.top - 1, EMPTY);
            Point::new(r.left + 1, (r.top + 1 + c.y) / 2)
        } else {
            fill_rect(map, r.left + 1, c.y + 1, 2, r.bottom - c.y - 1, EMPTY);
            Point::new(r.left + 1, (r.bottom + c.y) / 2)
        };
        for x in ((r.left + 1)..=(r.right - 3)).rev() {
            statue(map, Point::new(x, c.y));
        }
        (Point::new(r.right - 1, c.y), t)
    } else if e.x == r.right {
        fill_rect(map, r.right - 1, r.top + 1, 1, r.height() - 2, EMPTY);
        let t = if e.y > c.y {
            fill_rect(map, r.right - 2, r.top + 1, 2, c.y - r.top - 1, EMPTY);
            Point::new(r.right - 1, (r.top + 1 + c.y) / 2)
        } else {
            fill_rect(map, r.right - 2, c.y + 1, 2, r.bottom - c.y - 1, EMPTY);
            Point::new(r.right - 1, (r.bottom + 1 + c.y) / 2)
        };
        for x in (r.left + 3)..r.right {
            statue(map, Point::new(x, c.y));
        }
        (Point::new(r.left + 1, c.y), t)
    } else if e.y == r.top {
        fill_rect(map, r.left + 1, r.top + 1, r.width() - 2, 1, EMPTY);
        let t = if e.x > c.x {
            fill_rect(map, r.left + 1, r.top + 1, c.x - r.left - 1, 2, EMPTY);
            Point::new((r.left + 1 + c.x) / 2, r.top + 1)
        } else {
            fill_rect(map, c.x + 1, r.top + 1, r.right - c.x - 1, 2, EMPTY);
            Point::new((r.right + c.x) / 2, r.top + 1)
        };
        for y in ((r.top + 1)..=(r.bottom - 3)).rev() {
            statue(map, Point::new(c.x, y));
        }
        (Point::new(c.x, r.bottom - 1), t)
    } else {
        fill_rect(map, r.left + 1, r.bottom - 1, r.width() - 2, 1, EMPTY);
        let t = if e.x > c.x {
            fill_rect(map, r.left + 1, r.bottom - 2, c.x - r.left - 1, 2, EMPTY);
            Point::new((r.left + 1 + c.x) / 2, r.bottom - 1)
        } else {
            fill_rect(map, c.x + 1, r.bottom - 2, r.right - c.x - 1, 2, EMPTY);
            Point::new((r.right + c.x) / 2, r.bottom - 1)
        };
        for y in (r.top + 3)..r.bottom {
            statue(map, Point::new(c.x, y));
        }
        (Point::new(c.x, r.top + 1), t)
    }
}

fn statue(map: &mut TerrainMap, p: Point) {
    let tile = if map.map[cell(map, p)] == EMPTY_SP {
        STATUE_SP
    } else {
        STATUE
    };
    set(map, p, tile);
}
fn fill_room(map: &mut TerrainMap, r: &Room, tile: i32) {
    fill_rect(map, r.left, r.top, r.width(), r.height(), tile);
}
fn fill_margin(map: &mut TerrainMap, r: &Room, m: i32, tile: i32) {
    fill_rect(
        map,
        r.left + m,
        r.top + m,
        r.width() - 2 * m,
        r.height() - 2 * m,
        tile,
    );
}
fn fill_rect(map: &mut TerrainMap, x: i32, y: i32, w: i32, h: i32, tile: i32) {
    for dy in 0..h {
        for dx in 0..w {
            set(map, Point::new(x + dx, y + dy), tile);
        }
    }
}
fn cell(map: &TerrainMap, p: Point) -> usize {
    map.point_to_cell(p.x, p.y)
        .expect("SentryRoom point on map")
}
fn set(map: &mut TerrainMap, p: Point, tile: i32) {
    let cell = cell(map, p);
    map.map[cell] = tile;
    map.grass_allowed[cell] = false;
}
