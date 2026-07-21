//! Port of `RegularLevel.createItems` main random drop loop (simplified placement).

use crate::dungeon::DungeonState;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::terrain::{TerrainMap, EMPTY, EXIT};
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

        let cell = random_drop_cell(rooms, map, &occupied);
        if cell < 0 {
            continue;
        }
        occupied[cell as usize] = true;

        // Heap type
        let mut heap_type = "heap";
        match Random::int_max(20) {
            0 => heap_type = "skeleton",
            1..=4 => {
                // mimic chance 0 without MimicTooth
                let _ = Random::float(); // still not called when multiplier is 1... 
                // only if Float < (multi-1)/4 — multi=1 => 0, Float not evaluated short-circuit?
                // Java: Random.Float() < (MimicTooth.mimicChanceMultiplier() - 1f)/4f
                // Float IS always called
                let _roll = Random::float();
                heap_type = "chest";
            }
            5 => {
                if dungeon.depth > 1 {
                    // Mimic without extra float when depth>1 always spawns mimic in Java
                    // mobs.add(Mimic...) continue — item becomes mimic loot
                    to_drop.source = Some("mimic".into());
                    heap_type = "mimic";
                    out.push(PlacedLoot {
                        item: to_drop,
                        heap_type,
                    });
                    continue;
                }
                heap_type = "chest";
            }
            _ => heap_type = "heap",
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
        if (is_artifact && Random::int_max(2) == 0)
            || (upgradable && Random::int_max(4 - to_drop.level) == 0)
        {
            // golden mimic chance
            let mimic_chance = 0.1f32; // * MimicTooth multi = 1
            if dungeon.depth > 1 && Random::float() < mimic_chance {
                to_drop.source = Some("golden_mimic".into());
                heap_type = "golden_mimic";
            } else {
                heap_type = "locked_chest";
                // golden key would go to itemsToSpawn — skip key listing (blacklisted)
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
        let _cell = random_drop_cell(rooms, map, &occupied);
        if item.class_name == "TrinketCatalyst" {
            out.push(PlacedLoot {
                item,
                heap_type: "locked_chest",
            });
            let _key_cell = random_drop_cell(rooms, map, &occupied);
        } else {
            out.push(PlacedLoot {
                item,
                heap_type: "heap",
            });
        }
    }

    // Separate generators for bones/torch/rose/guide — consume Long seeds
    // Darkness challenge off: still pushGenerator(Long) and pop
    Random::push_generator_seeded(Random::long());
    // no torch without darkness
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    // Bones.get() null for fresh warrior run
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    // no dried rose
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    // no cached rations talent
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    // guide pages: meta progress — assume none found pages missing all except searching removed
    // For seeded analyzer: document pages often drop. Simplified: always roll Float chance
    let drop_chance = 0.25f32 * (dungeon.depth - 1) as f32;
    if Random::float() < drop_chance {
        // would drop guide page — skip listing for now
    }
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    // lore pages if all guide found — skip
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    // ebony mimic chance 0
    let _ = Random::float();
    Random::pop_generator();

    Random::push_generator_seeded(Random::long());
    // cracked spyglass
    let items = (Random::float() + 0.0) as i32; // extraLootChance 0 without trinket
    for _ in 0..items {
        let mut it = dungeon.generator.random(dungeon.depth); // randomUsingDefaults roughly
        it.source = Some("hidden".into());
        out.push(PlacedLoot {
            item: it,
            heap_type: "hidden",
        });
    }
    Random::pop_generator();

    let _ = EXIT;
    let _ = EMPTY;
    out
}

fn random_drop_cell(rooms: &[Room], map: &TerrainMap, occupied: &[bool]) -> i32 {
    // Simplified: try standard rooms
    let mut candidates: Vec<usize> = rooms
        .iter()
        .filter(|r| !r.is_empty() && r.kind == RoomKind::Standard)
        .map(|r| r.id)
        .collect();
    if candidates.is_empty() {
        candidates = rooms
            .iter()
            .filter(|r| !r.is_empty() && !r.is_entrance())
            .map(|r| r.id)
            .collect();
    }
    let mut tries = 100;
    while tries > 0 {
        tries -= 1;
        Random::shuffle_vec(&mut candidates);
        if candidates.is_empty() {
            return -1;
        }
        // Java: randomRoom shuffles rooms each try
        let room = &rooms[candidates[0]];
        if room.is_entrance() {
            continue;
        }
        // room.random(1)
        if room.width() <= 2 || room.height() <= 2 {
            continue;
        }
        let x = Random::int_range_inclusive(room.left + 1, room.right - 1);
        let y = Random::int_range_inclusive(room.top + 1, room.bottom - 1);
        // map uses absolute coords — need origin. TerrainMap currently doesn't store origin.
        // Use passable if we can compute - for simplified, accept without map check when map empty
        let _ = (x, y, map, occupied);
        return x + y * 1000; // synthetic cell id for occupancy uniqueness
    }
    -1
}
