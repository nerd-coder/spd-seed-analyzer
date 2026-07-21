//! Port of `RegularLevel.createItems` main random drop loop (simplified placement).

use crate::dungeon::DungeonState;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::terrain::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

#[derive(Debug, Clone)]
pub struct PlacedLoot {
    pub item: GeneratedItem,
    pub heap_type: &'static str,
}

/// Main createItems random drops + forced itemsToSpawn placement.
/// Uses simplified randomDropCell (standard rooms only, passable empty).
pub fn create_items_main(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    map: &TerrainMap,
    feeling_large: bool,
    items_to_spawn: Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let mut occupied: Vec<bool> = vec![false; map.len()];

    // nItems = 3 + chances{6,3,1}; +2 if LARGE
    let mut n_items = 3 + Random::chances(&[6., 3., 1.]);
    if feeling_large {
        n_items += 2;
    }

    for _ in 0..n_items {
        let mut to_drop = dungeon.generator.random(dungeon.depth);
        to_drop.source = Some("heap".into());

        let cell = random_drop_cell(rooms, map, &mut occupied);
        if cell < 0 {
            continue;
        }

        // Heap type
        let heap_type;
        match Random::int_max(20) {
            0 => {
                heap_type = "skeleton";
            }
            1..=4 => {
                // Java always evaluates Float for mimic check
                let _roll = Random::float();
                heap_type = "chest";
            }
            5 => {
                if dungeon.depth > 1 {
                    to_drop.source = Some("mimic".into());
                    out.push(PlacedLoot {
                        item: to_drop,
                        heap_type: "mimic",
                    });
                    continue;
                }
                heap_type = "chest";
            }
            _ => {
                heap_type = "heap";
            }
        }

        // locked chest upgrade path
        let upgradable = matches!(
            to_drop.category,
            ItemCategory::Weapon
                | ItemCategory::Armor
                | ItemCategory::Missile
                | ItemCategory::Wand
                | ItemCategory::Ring
        );
        let is_artifact = to_drop.category == ItemCategory::Artifact;
        let mut heap_type = heap_type;
        let level = to_drop.level.max(0);
        if (is_artifact && Random::int_max(2) == 0)
            || (upgradable && Random::int_max((4 - level).max(1)) == 0)
        {
            let mimic_chance = 0.1f32;
            if dungeon.depth > 1 && Random::float() < mimic_chance {
                to_drop.source = Some("golden_mimic".into());
                heap_type = "golden_mimic";
            } else {
                heap_type = "locked_chest";
            }
        }

        out.push(PlacedLoot {
            item: to_drop,
            heap_type,
        });
    }

    // place itemsToSpawn as heaps
    for mut item in items_to_spawn {
        item.source = Some("forced".into());
        let _cell = random_drop_cell(rooms, map, &mut occupied);
        if item.class_name == "TrinketCatalyst" {
            out.push(PlacedLoot {
                item,
                heap_type: "locked_chest",
            });
            let _key_cell = random_drop_cell(rooms, map, &mut occupied);
        } else {
            out.push(PlacedLoot {
                item,
                heap_type: "heap",
            });
        }
    }

    // Separate generators for bones/torch/rose/guide — consume Long seeds
    Random::push_generator_seeded(Random::long());
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    let drop_chance = 0.25f32 * (dungeon.depth - 1) as f32;
    if Random::float() < drop_chance {
        // guide page — skip listing
    }
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    let _ = Random::float(); // ebony mimic
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    let items = (Random::float() + 0.0) as i32;
    for _ in 0..items {
        let mut it = dungeon.generator.random(dungeon.depth);
        it.source = Some("hidden".into());
        out.push(PlacedLoot {
            item: it,
            heap_type: "hidden",
        });
    }
    Random::pop_generator();

    out
}

/// Returns index into `map.map` / `occupied`, or -1.
fn random_drop_cell(rooms: &[Room], map: &TerrainMap, occupied: &mut [bool]) -> i32 {
    let mut candidates: Vec<usize> = rooms
        .iter()
        .enumerate()
        .filter(|(_, r)| !r.is_empty() && r.kind == RoomKind::Standard)
        .map(|(i, _)| i)
        .collect();
    if candidates.is_empty() {
        candidates = rooms
            .iter()
            .enumerate()
            .filter(|(_, r)| !r.is_empty() && !r.is_entrance())
            .map(|(i, _)| i)
            .collect();
    }
    if candidates.is_empty() {
        return -1;
    }

    let mut tries = 100;
    while tries > 0 {
        tries -= 1;
        Random::shuffle_vec(&mut candidates);
        let room = &rooms[candidates[0]];
        if room.is_entrance() {
            continue;
        }
        if room.width() <= 2 || room.height() <= 2 {
            continue;
        }
        let x = Random::int_range_inclusive(room.left + 1, room.right - 1);
        let y = Random::int_range_inclusive(room.top + 1, room.bottom - 1);
        let Some(idx) = map.point_to_cell(x, y) else {
            continue;
        };
        if idx >= occupied.len() || occupied[idx] {
            continue;
        }
        if idx >= map.passable.len() || !map.passable[idx] {
            continue;
        }
        if !map.item_allowed.get(idx).copied().unwrap_or(false) {
            continue;
        }
        // AquariumRoom checks the final terrain dynamically, including water
        // added by RegularPainter after the room's own pool was painted.
        if room.name == "AquariumRoom" && map.map[idx] == crate::level::terrain::WATER {
            continue;
        }
        if map.is_solid(idx) {
            continue;
        }
        // Items cannot spawn on traps that destroy items (Burning/Frost/…/Pitfall).
        if map.trap_destroys_items.get(idx).copied().unwrap_or(false) {
            continue;
        }
        occupied[idx] = true;
        return idx as i32;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::terrain::{self, EMPTY, WATER};

    fn room(name: &str) -> Room {
        let mut room = Room::new(0, name, RoomKind::Standard, 1, 16, 5, 5, 5, 5);
        room.left = 1;
        room.top = 1;
        room.right = 5;
        room.bottom = 5;
        room
    }

    #[test]
    fn random_drop_cell_enforces_room_item_mask() {
        Random::reset_generators();
        let room = room("PlantsRoom");
        let mut map = terrain::paint_minimal(std::slice::from_ref(&room)).expect("map");
        map.item_allowed.fill(false);
        let only = map.point_to_cell(3, 3).expect("center");
        map.item_allowed[only] = true;
        let mut occupied = vec![false; map.len()];
        Random::push_generator_seeded(3);
        let selected = random_drop_cell(&[room], &map, &mut occupied);
        Random::pop_generator();
        assert_eq!(selected, only as i32);
    }

    #[test]
    fn aquarium_rejects_water_from_later_painter_passes() {
        Random::reset_generators();
        let room = room("AquariumRoom");
        let mut map = terrain::paint_minimal(std::slice::from_ref(&room)).expect("map");
        for y in 2..=4 {
            for x in 2..=4 {
                let cell = map.point_to_cell(x, y).expect("interior");
                map.map[cell] = WATER;
            }
        }
        let only = map.point_to_cell(3, 3).expect("center");
        map.map[only] = EMPTY;
        let mut occupied = vec![false; map.len()];
        Random::push_generator_seeded(7);
        let selected = random_drop_cell(&[room], &map, &mut occupied);
        Random::pop_generator();
        assert_eq!(selected, only as i32);
    }
}
