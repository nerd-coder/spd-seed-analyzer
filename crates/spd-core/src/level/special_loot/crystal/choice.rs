//! Exact `CrystalChoiceRoom.paint` geometry and prize placement.

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::{Point, Rect};
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, CRYSTAL_DOOR, EMPTY, EMPTY_SP, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

pub(super) fn paint(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    room_index: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[room_index];
    let entrance = room
        .connected
        .iter()
        .find_map(|&other| doors.get(room_index, other))
        .expect("placed CrystalChoiceRoom has an entrance");

    // The three temporary EmptyRooms each run StandardRoom's constructor.
    for _ in 0..3 {
        let _ = Random::float();
    }

    let center = room_center(room.as_rect());
    let (entry, mut room1, mut room2, crystal_doors) = if entrance.x == room.left {
        let entry = Rect {
            left: room.left + 1,
            top: room.top + 1,
            right: room.left + 2,
            bottom: room.bottom - 1,
        };
        let room1 = Rect {
            left: entry.right + 2,
            top: room.top + 1,
            right: room.right - 1,
            bottom: center.y - 1,
        };
        let room2 = Rect {
            left: entry.right + 2,
            top: room1.bottom + 2,
            right: room.right - 1,
            bottom: room.bottom - 1,
        };
        (
            entry,
            room1,
            room2,
            [
                Point::new(entry.right + 1, (room1.top + room1.bottom + 1) / 2),
                Point::new(entry.right + 1, (room2.top + room2.bottom) / 2),
            ],
        )
    } else if entrance.y == room.top {
        let entry = Rect {
            left: room.left + 1,
            top: room.top + 1,
            right: room.right - 1,
            bottom: room.top + 2,
        };
        let room1 = Rect {
            left: room.left + 1,
            top: entry.bottom + 2,
            right: center.x - 1,
            bottom: room.bottom - 1,
        };
        let room2 = Rect {
            left: room1.right + 2,
            top: entry.bottom + 2,
            right: room.right - 1,
            bottom: room.bottom - 1,
        };
        (
            entry,
            room1,
            room2,
            [
                Point::new((room1.left + room1.right + 1) / 2, entry.bottom + 1),
                Point::new((room2.left + room2.right) / 2, entry.bottom + 1),
            ],
        )
    } else if entrance.x == room.right {
        let entry = Rect {
            left: room.right - 2,
            top: room.top + 1,
            right: room.right - 1,
            bottom: room.bottom - 1,
        };
        let room1 = Rect {
            left: room.left + 1,
            top: room.top + 1,
            right: entry.left - 2,
            bottom: center.y - 1,
        };
        let room2 = Rect {
            left: room.left + 1,
            top: room1.bottom + 2,
            right: entry.left - 2,
            bottom: room.bottom - 1,
        };
        (
            entry,
            room1,
            room2,
            [
                Point::new(entry.left - 1, (room1.top + room1.bottom + 1) / 2),
                Point::new(entry.left - 1, (room2.top + room2.bottom) / 2),
            ],
        )
    } else {
        let entry = Rect {
            left: room.left + 1,
            top: room.bottom - 2,
            right: room.right - 1,
            bottom: room.bottom - 1,
        };
        let room1 = Rect {
            left: room.left + 1,
            top: room.top + 1,
            right: center.x - 1,
            bottom: entry.top - 2,
        };
        let room2 = Rect {
            left: room1.right + 2,
            top: room.top + 1,
            right: room.right - 1,
            bottom: entry.top - 2,
        };
        (
            entry,
            room1,
            room2,
            [
                Point::new((room1.left + room1.right + 1) / 2, entry.top - 1),
                Point::new((room2.left + room2.right) / 2, entry.top - 1),
            ],
        )
    };

    paint_rect(map, room.as_rect(), WALL);
    paint_rect(map, entry, EMPTY);
    paint_rect(map, room1, EMPTY_SP);
    paint_rect(map, room2, EMPTY_SP);
    for door in crystal_doors {
        if let Some(cell) = map.point_to_cell(door.x, door.y) {
            map.map[cell] = CRYSTAL_DOOR;
        }
    }

    if Random::int_max(2) == 0 {
        std::mem::swap(&mut room1, &mut room2);
    }

    let n = Random::normal_int_range(3, 4);
    let mut out = Vec::new();
    for _ in 0..n {
        let cat = *Random::one_of(&[Category::Potion, Category::Scroll]);
        let mut item = dungeon.generator.random_category(cat, dungeon.depth);
        item.source = Some("CrystalChoiceRoom".into());
        let margin = if rect_square(room1) >= 16 { 1 } else { 0 };
        let cell = random_unoccupied_cell(room1, margin, map);
        map.record_heap(cell, "heap", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }

    let hidden_cat = *Random::one_of(&[Category::Wand, Category::Ring, Category::Artifact]);
    let mut hidden = dungeon.generator.random_category(hidden_cat, dungeon.depth);
    hidden.source = Some("CrystalChoiceRoom".into());
    let hidden_center = room_center(room2);
    let hidden_cell = map
        .point_to_cell(hidden_center.x, hidden_center.y)
        .expect("CrystalChoiceRoom center is on-map");
    map.record_heap(hidden_cell, "chest", hidden.clone());
    out.push(PlacedLoot {
        item: hidden,
        heap_type: "chest",
    });

    items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    out
}

fn room_center(rect: Rect) -> Point {
    Point::new(
        (rect.left + rect.right) / 2
            + if (rect.right - rect.left) % 2 == 1 {
                Random::int_max(2)
            } else {
                0
            },
        (rect.top + rect.bottom) / 2
            + if (rect.bottom - rect.top) % 2 == 1 {
                Random::int_max(2)
            } else {
                0
            },
    )
}

fn rect_square(rect: Rect) -> i32 {
    (rect.raw_width() + 1) * (rect.raw_height() + 1)
}

fn paint_rect(map: &mut TerrainMap, rect: Rect, terrain: i32) {
    for y in rect.top..=rect.bottom {
        for x in rect.left..=rect.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = terrain;
            }
        }
    }
}

fn random_unoccupied_cell(rect: Rect, margin: i32, map: &TerrainMap) -> usize {
    loop {
        let x = Random::int_range_inclusive(rect.left + margin, rect.right - margin);
        let y = Random::int_range_inclusive(rect.top + margin, rect.bottom - margin);
        let Some(cell) = map.point_to_cell(x, y) else {
            continue;
        };
        if !map.heap_occupied[cell] {
            return cell;
        }
    }
}
