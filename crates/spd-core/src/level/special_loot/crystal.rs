//! Crystal vault special-room prizes and shared entrance geometry.

mod choice;

use super::door_spots;
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
    let mut prize_classes = vec![Category::Wand, Category::Ring, Category::Artifact];
    Random::shuffle_list(&mut prize_classes);

    let mut take_prize = |classes: &mut Vec<Category>| -> GeneratedItem {
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
    rooms: &[Room],
    room_index: usize,
    map: &mut crate::level::terrain::TerrainMap,
    doors: &crate::level::painter::DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    choice::paint(dungeon, rooms, room_index, map, doors, items_to_spawn)
}
