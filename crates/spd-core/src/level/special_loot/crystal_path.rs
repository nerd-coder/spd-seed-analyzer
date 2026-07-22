//! Exact seeded paint path for `CrystalPathRoom`.

mod layout;

use super::DoorMap;
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::{Point, Rect};
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, CRYSTAL_DOOR, EMPTY_SP, PEDESTAL, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

pub(super) fn paint(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    fill_room(map, room, WALL);
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.water_allowed[cell] = false;
                map.grass_allowed[cell] = false;
                map.trap_allowed[cell] = false;
            }
        }
    }

    // Java constructs six temporary EmptyRooms. Each StandardRoom initializer
    // performs setSizeCat() even though the rectangles are immediately reset.
    for _ in 0..6 {
        let _ = Random::chances(&[1.0, 0.0, 0.0]);
    }
    let entry = room
        .connected
        .first()
        .and_then(|&other| doors.get(ri, other))
        .map(|door| Point::new(door.x, door.y))
        .expect("placed CrystalPathRoom has an entrance door");
    let (mini_rooms, prize1, prize2) = layout::build(room, entry);
    layout::draw_inside(
        map,
        room,
        entry,
        if entry.x == room.left || entry.x == room.right {
            if room.width() > 8 {
                5
            } else {
                3
            }
        } else if room.height() > 8 {
            5
        } else {
            3
        },
    );
    for mini in mini_rooms {
        fill_rect(map, mini, EMPTY_SP);
    }
    for door in layout::internal_doors(room, entry, &mini_rooms) {
        set(map, door, CRYSTAL_DOOR);
    }
    set(map, prize1, PEDESTAL);
    set(map, prize2, PEDESTAL);

    let mut potions = Vec::new();
    let mut scrolls = Vec::new();
    let mut duplicates = Vec::new();

    if Random::int_max(2) == 0 {
        add_reward_item(dungeon, Category::Potion, &mut potions, &mut duplicates);
        // ExoticCrystals chance is zero, but Java still evaluates Float().
        let _ = Random::float();
        scrolls.push(GeneratedItem::new(
            "ScrollOfTransmutation",
            ItemCategory::Scroll,
        ));
    } else {
        let _ = Random::float();
        potions.push(GeneratedItem::new(
            "PotionOfExperience",
            ItemCategory::Potion,
        ));
        add_reward_item(dungeon, Category::Scroll, &mut scrolls, &mut duplicates);
    }
    add_reward_item(dungeon, Category::Potion, &mut potions, &mut duplicates);
    add_reward_item(dungeon, Category::Scroll, &mut scrolls, &mut duplicates);
    add_reward_item(dungeon, Category::Potion, &mut potions, &mut duplicates);
    add_reward_item(dungeon, Category::Scroll, &mut scrolls, &mut duplicates);

    for duplicate in &duplicates {
        dungeon.generator.undo_drop(&duplicate.class_name);
    }
    sort_by_default_value(dungeon, Category::Potion, &mut potions);
    sort_by_default_value(dungeon, Category::Scroll, &mut scrolls);

    let shuffle = Random::int_max(2) as usize;
    let cells = [
        mini_rooms[if shuffle == 1 { 2 } else { 3 }].center_room(),
        mini_rooms[if shuffle == 1 { 3 } else { 2 }].center_room(),
        mini_rooms[if shuffle == 1 { 0 } else { 1 }].center_room(),
        mini_rooms[if shuffle == 1 { 1 } else { 0 }].center_room(),
        if shuffle == 1 { prize1 } else { prize2 },
        if shuffle == 1 { prize2 } else { prize1 },
    ];
    for point in cells {
        if let Some(cell) = map.point_to_cell(point.x, point.y) {
            map.heap_occupied[cell] = true;
        }
    }

    let mut out = Vec::new();
    for mut item in potions.into_iter().chain(scrolls) {
        item.source = Some("CrystalPathRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    for _ in 0..3 {
        items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    }
    out
}

fn add_reward_item(
    dungeon: &mut DungeonState,
    category: Category,
    items: &mut Vec<GeneratedItem>,
    duplicates: &mut Vec<GeneratedItem>,
) {
    loop {
        let reward = dungeon.generator.random_category(category, dungeon.depth);
        if items
            .iter()
            .any(|item| item.class_name == reward.class_name)
        {
            duplicates.push(reward);
        } else {
            items.push(reward);
            return;
        }
    }
}

fn sort_by_default_value(dungeon: &DungeonState, category: Category, items: &mut [GeneratedItem]) {
    items.sort_by(|a, b| {
        let av = dungeon
            .generator
            .default_prob_total(category, &a.class_name);
        let bv = dungeon
            .generator
            .default_prob_total(category, &b.class_name);
        bv.partial_cmp(&av).unwrap_or(std::cmp::Ordering::Equal)
    });
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    fill_rect(map, room.as_rect(), terrain);
}

fn fill_rect(map: &mut TerrainMap, rect: Rect, terrain: i32) {
    for y in rect.top..=rect.bottom {
        for x in rect.left..=rect.right {
            set(map, Point::new(x, y), terrain);
        }
    }
}

fn set(map: &mut TerrainMap, point: Point, terrain: i32) {
    if let Some(cell) = map.point_to_cell(point.x, point.y) {
        map.map[cell] = terrain;
    }
}
