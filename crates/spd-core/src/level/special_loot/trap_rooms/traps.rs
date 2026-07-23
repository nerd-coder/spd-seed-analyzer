//! Exact seeded paint path for `TrapsRoom`.

use super::super::placement::find_prize_item;
use super::super::special_rooms::is_curse_enchant;
use crate::dungeon::DungeonState;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, CHASM, EMPTY, PEDESTAL, TRAP, WALL};
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

    // Int(4)==0 uses chasm; otherwise Java selects and instantiates one trap
    // class for every trap tile in the room.
    let trap_class = if Random::int_max(4) == 0 {
        None
    } else {
        let region = (dungeon.depth / 5).clamp(0, 4) as usize;
        let classes = TRAPS_ROOM_TRAPS[region];
        Some(classes[Random::int_max(classes.len() as i32) as usize])
    };
    let interior = if trap_class.is_some() { TRAP } else { CHASM };
    fill_margin(map, room, 1, interior);

    let door = room
        .connected
        .first()
        .and_then(|&other| doors.get(ri, other))
        .map(|door| Point::new(door.x, door.y))
        .expect("placed TrapsRoom has an entrance door");
    let last_row = if interior == CHASM { CHASM } else { EMPTY };
    let prize_pos = if door.x == room.left {
        let x = room.right - 1;
        fill_vertical(map, x, room.top + 1, room.height() - 2, last_row);
        Point::new(x, room.top + room.height() / 2)
    } else if door.x == room.right {
        let x = room.left + 1;
        fill_vertical(map, x, room.top + 1, room.height() - 2, last_row);
        Point::new(x, room.top + room.height() / 2)
    } else if door.y == room.top {
        let y = room.bottom - 1;
        fill_horizontal(map, room.left + 1, y, room.width() - 2, last_row);
        Point::new(room.left + room.width() / 2, y)
    } else {
        let y = room.top + 1;
        fill_horizontal(map, room.left + 1, y, room.width() - 2, last_row);
        Point::new(room.left + room.width() / 2, y)
    };

    // Java instantiates traps after painting the safe row, so only cells that
    // remain Terrain.TRAP carry a trap object.
    if let Some(class_name) = trap_class {
        for y in (room.top + 1)..room.bottom {
            for x in (room.left + 1)..room.right {
                if let Some(cell) = map.point_to_cell(x, y) {
                    if map.map[cell] == TRAP {
                        map.trap_names[cell] = Some(class_name);
                        map.trap_destroys_items[cell] =
                            matches!(class_name, "ExplosiveTrap" | "DisintegrationTrap");
                    }
                }
            }
        }
    }

    if Random::int_max(3) == 0 {
        if last_row == CHASM {
            set(map, prize_pos.x, prize_pos.y, EMPTY);
        }
    } else {
        set(map, prize_pos.x, prize_pos.y, PEDESTAL);
    }
    let prize_cell = map
        .point_to_cell(prize_pos.x, prize_pos.y)
        .expect("TrapsRoom prize cell is on map");
    map.heap_occupied[prize_cell] = true;

    let mut prize = generate_prize(dungeon, items_to_spawn);
    let consumed_forced = prize.source.as_deref() == Some("forced");
    prize.source = Some(if consumed_forced {
        "TrapsRoom:forced".into()
    } else {
        "TrapsRoom".into()
    });
    // Java chooses the far-row cell before generating the prize, then drops
    // the generated item into a chest at that exact cell.
    map.record_heap(prize_cell, "chest", prize.clone());
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLevitation",
        ItemCategory::Potion,
    ));
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

const TRAPS_ROOM_TRAPS: [&[&str]; 5] = [
    &["GrippingTrap", "TeleportationTrap", "FlockTrap"],
    &["PoisonDartTrap", "GrippingTrap", "ExplosiveTrap"],
    &["PoisonDartTrap", "FlashingTrap", "ExplosiveTrap"],
    &["WarpingTrap", "FlashingTrap", "DisintegrationTrap"],
    &["GrimTrap"],
];

fn generate_prize(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> GeneratedItem {
    // Java returns a queued prize immediately, without uncursing/upgrading or
    // consuming the trailing Int(3) from the generated-equipment branch.
    if Random::int_max(3) != 0 {
        if let Some(prize) = find_prize_item(items_to_spawn, None) {
            return prize;
        }
    }

    let mut prize = random_equip(dungeon);
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize
}

fn random_equip(dungeon: &mut DungeonState) -> GeneratedItem {
    let floor = (dungeon.depth / 5) + 1;
    if Random::int_max(2) == 0 {
        dungeon.generator.random_weapon(floor, false, dungeon.depth)
    } else {
        dungeon.generator.random_armor(floor, dungeon.depth)
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    for y in room.top..=room.bottom {
        fill_horizontal(map, room.left, y, room.width(), terrain);
    }
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    for y in (room.top + margin)..=(room.bottom - margin) {
        fill_horizontal(
            map,
            room.left + margin,
            y,
            room.width() - 2 * margin,
            terrain,
        );
    }
}

fn fill_horizontal(map: &mut TerrainMap, x: i32, y: i32, width: i32, terrain: i32) {
    for dx in 0..width {
        set(map, x + dx, y, terrain);
    }
}

fn fill_vertical(map: &mut TerrainMap, x: i32, y: i32, height: i32, terrain: i32) {
    for dy in 0..height {
        set(map, x, y + dy, terrain);
    }
}

fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}
