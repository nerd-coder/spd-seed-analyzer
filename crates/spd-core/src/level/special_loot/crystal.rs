//! Crystal vault special-room prizes and shared entrance geometry.

mod choice;

use super::DoorMap;
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_SP, LOCKED_DOOR, PEDESTAL, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

/// `CrystalVaultRoom.paint` — two crystal-chest prizes (WAND/RING/ARTIFACT rotate).
pub(super) fn crystal_vault(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    fill_room(map, room, WALL);
    fill_inset(map, room, 1, EMPTY_SP);
    fill_inset(map, room, 2, EMPTY);
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

    // Pedestal placement: CIRCLE8 opposite pair, reject if Chebyshev-adjacent
    // to the entrance door. `Level.adjacent` treats all eight neighbours as
    // adjacent, not only cardinal neighbours.
    let entrance = room
        .connected
        .iter()
        .find_map(|&other| doors.get(ri, other))
        .map(|door| Point::new(door.x, door.y))
        .expect("placed CrystalVaultRoom has an entrance");
    let (p1, p2) = crystal_vault_positions(room, entrance);

    // 10% crystal mimic on second chest (no RatSkull / MimicTooth trinkets).
    let second_heap = if Random::float() < 0.1 {
        "crystal_mimic"
    } else {
        "crystal_chest"
    };

    items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));

    set(map, p1, PEDESTAL);
    set(map, p2, PEDESTAL);
    map.record_heap(
        map.point_to_cell(p1.x, p1.y)
            .expect("vault pedestal on map"),
        "crystal_chest",
        i1.clone(),
    );
    let second_cell = map
        .point_to_cell(p2.x, p2.y)
        .expect("vault pedestal on map");
    if second_heap == "crystal_mimic" {
        map.mob_occupied[second_cell] = true;
        map.known_mobs[second_cell] = Some("CrystalMimic");
    } else {
        map.record_heap(second_cell, "crystal_chest", i2.clone());
    }
    set(map, entrance, LOCKED_DOOR);

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

fn crystal_vault_positions(room: &Room, door: Point) -> (Point, Point) {
    let cx = (room.left + room.right) / 2;
    let cy = (room.top + room.bottom) / 2;
    loop {
        let idx = Random::int_max(8) as usize;
        let (dx1, dy1) = CIRCLE8[idx];
        let (dx2, dy2) = CIRCLE8[(idx + 4) % 8];
        let i1 = Point::new(cx + dx1, cy + dy1);
        let i2 = Point::new(cx + dx2, cy + dy2);
        if !adjacent(i1, door) && !adjacent(i2, door) {
            return (i1, i2);
        }
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    fill_inset(map, room, 0, terrain);
}

fn fill_inset(map: &mut TerrainMap, room: &Room, inset: i32, terrain: i32) {
    for y in (room.top + inset)..=(room.bottom - inset) {
        for x in (room.left + inset)..=(room.right - inset) {
            set(map, Point::new(x, y), terrain);
        }
    }
}

fn set(map: &mut TerrainMap, point: Point, terrain: i32) {
    if let Some(cell) = map.point_to_cell(point.x, point.y) {
        map.map[cell] = terrain;
    }
}

/// `Level.adjacent`: Chebyshev distance exactly one.
fn adjacent(a: Point, b: Point) -> bool {
    (a.x - b.x).abs().max((a.y - b.y).abs()) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vault_rejects_diagonal_pedestals_beside_the_entrance() {
        let door = Point::new(10, 10);

        assert!(adjacent(Point::new(11, 10), door));
        assert!(adjacent(Point::new(11, 11), door));
        assert!(!adjacent(Point::new(12, 10), door));
        assert!(!adjacent(door, door));
    }
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
