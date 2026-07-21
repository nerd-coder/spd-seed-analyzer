//! Crystal vault / choice / path special rooms.

use super::door_spots;
use super::placement::burn_drop_pos_margin;
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::random::Random;
use crate::rooms::room::Room;

/// `CrystalVaultRoom.paint` — two crystal-chest prizes (WAND/RING/ARTIFACT rotate).
pub(super) fn crystal_vault(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    // prizeClasses rotate: shuffle then take/rotate twice
    let mut prize_classes = [Category::Wand, Category::Ring, Category::Artifact];
    Random::shuffle(&mut prize_classes);

    let mut take_prize = |classes: &mut [Category; 3]| -> GeneratedItem {
        let cat = classes[0];
        classes.rotate_left(1);
        // do { prize = Generator.random(cat) } while blocked — no challenges here
        dungeon.generator.random_category(cat, dungeon.depth)
    };

    let mut i1 = take_prize(&mut prize_classes);
    let mut i2 = take_prize(&mut prize_classes);
    i1.source = Some("CrystalVaultRoom".into());
    i2.source = Some("CrystalVaultRoom".into());

    // Pedestal placement: CIRCLE8 opposite pair, reject if adjacent to entrance door.
    burn_crystal_vault_positions(rooms, ri);

    // 10% crystal mimic on second chest (no RatSkull / MimicTooth trinkets).
    let second_heap = if Random::float() < 0.1 {
        "crystal_mimic"
    } else {
        "crystal_chest"
    };

    items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));

    let _ = room; // keys/chests reported; geometry is approximate
    vec![
        PlacedLoot {
            item: i1,
            heap_type: "crystal_chest",
        },
        PlacedLoot {
            item: i2,
            heap_type: second_heap,
        },
    ]
}

/// PathFinder.CIRCLE8 clockwise offsets (dx, dy) for unit steps.
const CIRCLE8: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn burn_crystal_vault_positions(rooms: &[Room], ri: usize) {
    let room = &rooms[ri];
    if room.is_empty() {
        let _ = Random::int_max(8);
        return;
    }
    let cx = (room.left + room.right) / 2;
    let cy = (room.top + room.bottom) / 2;
    let door = vault_entrance_cell(rooms, ri).unwrap_or_else(|| Point::new(room.left, cy));

    // Match Java do-while: keep rolling until neither pedestal is adjacent to the door.
    for _ in 0..32 {
        let idx = Random::int_max(8) as usize;
        let (dx1, dy1) = CIRCLE8[idx];
        let (dx2, dy2) = CIRCLE8[(idx + 4) % 8];
        let i1 = Point::new(cx + dx1, cy + dy1);
        let i2 = Point::new(cx + dx2, cy + dy2);
        if !adjacent_4(i1, door) && !adjacent_4(i2, door) {
            return;
        }
    }
}

/// Entrance door cell used by crystal vault pedestals and sacrifice center offset.
pub(super) fn vault_entrance_cell(rooms: &[Room], ri: usize) -> Option<Point> {
    let room = &rooms[ri];
    let ni = *room.connected.first()?;
    let other = rooms.get(ni)?;
    if other.is_empty() {
        return None;
    }
    let spots = door_spots(room, other);
    // placeDoors already burned element(); door is geometric mid-edge — pick mid.
    if spots.is_empty() {
        None
    } else {
        Some(spots[spots.len() / 2])
    }
}

fn adjacent_4(a: Point, b: Point) -> bool {
    (a.x - b.x).abs() + (a.y - b.y).abs() == 1
}

/// `CrystalChoiceRoom.paint` — 3–4 potion/scroll piles + one chest (wand/ring/artifact).
pub(super) fn crystal_choice(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // room1/room2 may swap
    let _ = Random::int_max(2);

    let n = Random::normal_int_range(3, 4);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        // room.random(1) or random(0) depending on square — burn margin-0 style
        burn_drop_pos_margin(room, 0, &mut occupied);
        let cat = *Random::one_of(&[Category::Potion, Category::Scroll]);
        let mut item = dungeon.generator.random_category(cat, dungeon.depth);
        item.source = Some("CrystalChoiceRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }

    let hidden_cat = *Random::one_of(&[Category::Wand, Category::Ring, Category::Artifact]);
    let mut hidden = dungeon.generator.random_category(hidden_cat, dungeon.depth);
    hidden.source = Some("CrystalChoiceRoom".into());
    out.push(PlacedLoot {
        item: hidden,
        heap_type: "chest",
    });

    items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    out
}

/// `CrystalPathRoom.paint` loot — 3 potions + 3 scrolls (deduped), XP/transmutation branch.
pub(super) fn crystal_path(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut potions: Vec<GeneratedItem> = Vec::new();
    let mut scrolls: Vec<GeneratedItem> = Vec::new();
    let mut duplicates: Vec<GeneratedItem> = Vec::new();

    // ExoticCrystals trinket chance is 0 → always base XP / Transmutation.
    if Random::int_max(2) == 0 {
        add_reward_item(dungeon, Category::Potion, &mut potions, &mut duplicates);
        scrolls.push(GeneratedItem::new(
            "ScrollOfTransmutation",
            ItemCategory::Scroll,
        ));
    } else {
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

    // Undo deck pulls from rejected duplicates (matches Generator.undoDrop).
    for d in &duplicates {
        dungeon.generator.undo_drop(&d.class_name);
    }

    // Sort by defaultProbsTotal descending (common first / rare last).
    sort_by_default_value(dungeon, Category::Potion, &mut potions);
    sort_by_default_value(dungeon, Category::Scroll, &mut scrolls);

    // Placement shuffle: rooms 2&3 / 0&1 / prizes — one Int(2).
    let _shuffle = Random::int_max(2);

    let mut out = Vec::new();
    // three potions + three scrolls in placement order (values still reported fully)
    for mut p in potions {
        p.source = Some("CrystalPathRoom".into());
        out.push(PlacedLoot {
            item: p,
            heap_type: "heap",
        });
    }
    for mut s in scrolls {
        s.source = Some("CrystalPathRoom".into());
        out.push(PlacedLoot {
            item: s,
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
    cat: Category,
    items: &mut Vec<GeneratedItem>,
    dupes: &mut Vec<GeneratedItem>,
) {
    loop {
        let reward = dungeon.generator.random_category(cat, dungeon.depth);
        let dupe = items.iter().any(|i| i.class_name == reward.class_name);
        if dupe {
            dupes.push(reward);
        } else {
            items.push(reward);
            return;
        }
    }
}

fn sort_by_default_value(dungeon: &DungeonState, cat: Category, items: &mut [GeneratedItem]) {
    items.sort_by(|a, b| {
        let av = dungeon.generator.default_prob_total(cat, &a.class_name);
        let bv = dungeon.generator.default_prob_total(cat, &b.class_name);
        // Java: return bVal - aVal → higher total first
        bv.partial_cmp(&av).unwrap_or(std::cmp::Ordering::Equal)
    });
}
