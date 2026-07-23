//! Sentry, traps, fire, sacrifice, toxic gas, and honeypot room prizes.

mod traps;

pub(super) use traps::paint as traps_prize;

use super::crystal::vault_entrance_cell;
use super::placement::{burn_drop_pos, burn_drop_pos_margin, find_prize_item};
use super::special_rooms::{bomb_random, is_curse_enchant, storage_prize};
use crate::dungeon::DungeonState;
use crate::geom::{Point, Rect};
use crate::items::enchants;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_SP, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

/// `SentryRoom.paint` prize — chest equip or findPrizeItem + PotionOfHaste.
pub(super) fn sentry_prize(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    // Layout (center/sentry/treasure) is geometric from entrance — no RNG before prize.
    let mut prize = if Random::int_max(2) == 0 {
        find_prize_item(items_to_spawn, None).unwrap_or_else(|| sentry_equip(dungeon))
    } else {
        sentry_equip(dungeon)
    };
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("SentryRoom".into());
    items_to_spawn.push(GeneratedItem::new("PotionOfHaste", ItemCategory::Potion));
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

fn sentry_equip(dungeon: &mut DungeonState) -> GeneratedItem {
    let floor = (dungeon.depth / 5) + 1;
    // Random.Int(5): 0,1 weapon; 2 missile; 3,4 armor
    match Random::int_max(5) {
        0 | 1 => dungeon.generator.random_weapon(floor, false, dungeon.depth),
        2 => dungeon
            .generator
            .random_missile(floor, false, dungeon.depth),
        _ => dungeon.generator.random_armor(floor, dungeon.depth),
    }
}

/// `MagicalFireRoom.paint` — 3–4 honeypot/consumable drops + PotionOfFrost.
pub(super) fn magical_fire_prizes(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    let center = room.as_rect().center_room();
    let door = room
        .connected
        .first()
        .and_then(|&other| doors.get(ri, other))
        .map(|door| Point::new(door.x, door.y))
        .expect("placed MagicalFireRoom has an entrance door");
    let behind_fire = magical_fire_geometry(map, room, center, door);

    // `new EmptyRoom()` constructs a StandardRoom solely to hold behindFire.
    // Its instance initializer calls setSizeCat() and consumes this chance roll.
    let _ = Random::chances(&[1.0, 0.0, 0.0]);

    let mut honey = Random::int_max(2) == 0;
    let n = Random::int_range_inclusive(3, 4);
    let mut out = Vec::new();
    for _ in 0..n {
        let mut prize_cell = None;
        for _ in 0..10_000 {
            let point = Point::new(
                Random::int_range_inclusive(behind_fire.left, behind_fire.right),
                Random::int_range_inclusive(behind_fire.top, behind_fire.bottom),
            );
            let Some(cell) = map.point_to_cell(point.x, point.y) else {
                continue;
            };
            if map.map[cell] == EMPTY_SP && !map.heap_occupied[cell] {
                map.heap_occupied[cell] = true;
                prize_cell = Some(cell);
                break;
            }
        }
        let Some(prize_cell) = prize_cell else {
            continue;
        };
        let mut item = if honey {
            honey = false;
            GeneratedItem::new("Honeypot", ItemCategory::Other)
        } else {
            // Same prize table as StorageRoom / MagicalFireRoom.prize
            storage_prize(dungeon, items_to_spawn)
        };
        let consumed_forced = item.source.as_deref() == Some("forced");
        item.source = Some(if consumed_forced {
            "MagicalFireRoom:forced".into()
        } else {
            "MagicalFireRoom".into()
        });
        // Java chooses the cell before `prize(level)`, then `Level.drop`
        // records the generated item at that exact cell.
        map.record_heap(prize_cell, "heap", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    items_to_spawn.push(GeneratedItem::new("PotionOfFrost", ItemCategory::Potion));
    out
}

fn magical_fire_geometry(map: &mut TerrainMap, room: &Room, fire_pos: Point, door: Point) -> Rect {
    let mut fire_cells = Vec::new();
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                let inside = x > room.left && x < room.right && y > room.top && y < room.bottom;
                map.map[cell] = if inside { EMPTY } else { WALL };
                map.grass_allowed[cell] = false;
            }
        }
    }

    let behind = if door.x == room.left || door.x == room.right {
        for y in (room.top + 1)..room.bottom {
            set_magical_tile(map, fire_pos.x, y, EMPTY_SP);
            fire_cells.push(Point::new(fire_pos.x, y));
        }
        if door.x == room.left {
            Rect {
                left: fire_pos.x + 1,
                top: room.top + 1,
                right: room.right - 1,
                bottom: room.bottom - 1,
            }
        } else {
            Rect {
                left: room.left + 1,
                top: room.top + 1,
                right: fire_pos.x - 1,
                bottom: room.bottom - 1,
            }
        }
    } else {
        for x in (room.left + 1)..room.right {
            set_magical_tile(map, x, fire_pos.y, EMPTY_SP);
            fire_cells.push(Point::new(x, fire_pos.y));
        }
        if door.y == room.top {
            Rect {
                left: room.left + 1,
                top: fire_pos.y + 1,
                right: room.right - 1,
                bottom: room.bottom - 1,
            }
        } else {
            Rect {
                left: room.left + 1,
                top: room.top + 1,
                right: room.right - 1,
                bottom: fire_pos.y - 1,
            }
        }
    };

    for y in behind.top..=behind.bottom {
        for x in behind.left..=behind.right {
            set_magical_tile(map, x, y, EMPTY_SP);
        }
    }
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let special = map
                .point_to_cell(x, y)
                .is_some_and(|cell| map.map[cell] == EMPTY_SP);
            let next_to_fire = fire_cells
                .iter()
                .any(|fire| (fire.x - x).abs() + (fire.y - y).abs() == 1);
            if special || next_to_fire {
                if let Some(cell) = map.point_to_cell(x, y) {
                    map.character_allowed[cell] = false;
                }
            }
        }
    }
    behind
}

fn set_magical_tile(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}

/// `SacrificeRoom.paint` — cursed upgraded weapon on sacrificial fire.
pub(super) fn sacrifice_prize(dungeon: &mut DungeonState, rooms: &[Room], ri: usize) -> PlacedLoot {
    // Center offset when door is mid-wall aligned with room center.
    burn_sacrifice_center_offset(rooms, ri);

    // 1 floor set higher than normal
    let mut prize = dungeon
        .generator
        .random_weapon((dungeon.depth / 5) + 1, false, dungeon.depth);

    // Always generate curse (parchment scrap isolation), matching CryptRoom pattern.
    let curse = enchants::random_weapon_curse(None).to_string();
    if !prize.cursed {
        prize.level += 1;
        if !is_good_weapon_enchant(&prize) {
            prize.enchantment = Some(curse);
        }
    }
    prize.cursed = true;
    prize.source = Some("SacrificeRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "sacrificial",
    }
}

fn is_good_weapon_enchant(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => !matches!(
            e,
            "Annoying"
                | "Displacing"
                | "Dazzling"
                | "Explosive"
                | "Sacrificial"
                | "Wayward"
                | "Polarized"
                | "Friendly"
        ),
        None => false,
    }
}

/// Burn `Random.Int(2)` center nudge when entrance is mid-edge (SacrificeRoom).
fn burn_sacrifice_center_offset(rooms: &[Room], ri: usize) {
    let room = &rooms[ri];
    if room.is_empty() {
        return;
    }
    let c = Point::new((room.left + room.right) / 2, (room.top + room.bottom) / 2);
    let Some(door) = vault_entrance_cell(rooms, ri) else {
        return;
    };
    let side_door = (door.x == room.left || door.x == room.right) && door.y == c.y;
    let end_door = (door.y == room.top || door.y == room.bottom) && door.x == c.x;
    if side_door || end_door {
        let _ = Random::int_max(2);
    }
}

/// `ToxicGasRoom.paint` — skeleton 2×gold + 2 chests (cata/gold) + trap placement RNG.
pub(super) fn toxic_gas_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Vent traps: min(w-2, h-2) placements at random(2) on EMPTY (approx unique).
    let traps = (room.width() - 2).min(room.height() - 2).max(0);
    let mut occupied = Vec::new();
    for _ in 0..traps {
        burn_drop_pos_margin(room, 2, &mut occupied);
    }

    // 8 candidate gold positions at random(2); furthest becomes skeleton (trueDistance
    // pick is pure geometry — no extra RNG).
    for _ in 0..8 {
        burn_drop_pos_margin(room, 2, &mut occupied);
    }

    let mut out = Vec::new();

    // Main gold ×2 on skeleton heap (blacklisted from report; still for RNG parity).
    let mut main = GeneratedItem::new("Gold", ItemCategory::Gold);
    randomize_item(&mut main, dungeon.depth);
    main.quantity = main.quantity.saturating_mul(2);
    main.source = Some("ToxicGasRoom".into());
    out.push(PlacedLoot {
        item: main,
        heap_type: "skeleton",
    });

    // Two chests: TrinketCatalyst prize item or random gold
    for _ in 0..2 {
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
                randomize_item(&mut g, dungeon.depth);
                g
            });
        item.source = Some("ToxicGasRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "chest",
        });
    }

    items_to_spawn.push(GeneratedItem::new("PotionOfPurity", ItemCategory::Potion));
    out
}

/// `SecretHoneypotRoom.paint` — shattered pot (geom) + honeypot + Bomb.random().
pub(super) fn secret_honeypot(room: &Room) -> Vec<PlacedLoot> {
    // brokenPotPos is geometric midpoint of center and entrance — no RNG.
    // Bee spawn does not consume loot RNG.
    let mut out = Vec::new();
    let mut occupied = Vec::new();

    // Shattered pot reported as Honeypot.ShatteredPot for identity
    let mut shattered = GeneratedItem::new("ShatteredPot", ItemCategory::Other);
    shattered.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: shattered,
        heap_type: "heap",
    });

    burn_drop_pos(room, &mut occupied);
    let mut honey = GeneratedItem::new("Honeypot", ItemCategory::Other);
    honey.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: honey,
        heap_type: "heap",
    });

    burn_drop_pos(room, &mut occupied);
    let mut bomb = bomb_random();
    bomb.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: bomb,
        heap_type: "heap",
    });

    out
}
