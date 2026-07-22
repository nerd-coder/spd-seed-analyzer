//! Port of `RegularLevel.createItems` main random drop loop.

use crate::dungeon::DungeonState;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::terrain::{TerrainMap, EXIT, WATER};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

#[derive(Debug, Clone)]
pub struct PlacedLoot {
    pub item: GeneratedItem,
    pub heap_type: &'static str,
}

#[derive(Debug, Clone)]
pub struct CreatedLoot {
    pub loot: PlacedLoot,
    /// Exact `randomDropCell` result; absent for unplaced/unsupported late drops.
    pub cell: Option<usize>,
}

/// Main createItems random drops + forced itemsToSpawn placement.
/// `randomDropCell` matches Java: full-list reshuffle per try, StandardRoom
/// instanceof scan (Entrance/Exit/Standard kinds), one persistent in-place
/// room permutation across all drops, and exact exit-cell exclusion.
pub fn create_items_main(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    map: &TerrainMap,
    feeling_large: bool,
    items_to_spawn: Vec<GeneratedItem>,
) -> Vec<CreatedLoot> {
    let mut out = Vec::new();
    let mut occupied = map.heap_occupied.clone();
    for (occupied, &mob) in occupied.iter_mut().zip(&map.mob_occupied) {
        *occupied |= mob;
    }
    // Java shuffles the level's `rooms` ArrayList itself. Preserve that
    // permutation across every random and forced drop on this floor.
    let mut room_order: Vec<usize> = (0..rooms.len()).collect();

    // nItems = 3 + chances{6,3,1}; +2 if LARGE
    let mut n_items = 3 + Random::chances(&[6., 3., 1.]);
    if feeling_large {
        n_items += 2;
    }

    for _ in 0..n_items {
        let mut to_drop = dungeon.generator.random(dungeon.depth);
        to_drop.source = Some("heap".into());

        let cell = random_drop_cell(rooms, &mut room_order, map, &mut occupied);
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
                    out.push(CreatedLoot {
                        loot: PlacedLoot {
                            item: to_drop,
                            heap_type: "mimic",
                        },
                        cell: Some(cell as usize),
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
        out.push(CreatedLoot {
            loot: PlacedLoot {
                item: to_drop,
                heap_type,
            },
            cell: Some(cell as usize),
        });
    }

    // place itemsToSpawn as heaps
    for mut item in items_to_spawn {
        // Pre-build forced drops arrive already tagged "forced"; room-paint
        // additions (IronKey, PoolRoom potion, …) are tagged separately so the
        // schema-v2 pre-build forced-queue oracle can tell the two apart.
        if item.source.is_none() {
            item.source = Some("items_to_spawn".into());
        }
        let cell = random_drop_cell(rooms, &mut room_order, map, &mut occupied);
        let cell = (cell >= 0).then_some(cell as usize);
        if item.class_name == "TrinketCatalyst" {
            out.push(CreatedLoot {
                loot: PlacedLoot {
                    item,
                    heap_type: "locked_chest",
                },
                cell,
            });
            let key_cell = random_drop_cell(rooms, &mut room_order, map, &mut occupied);
            if key_cell >= 0 {
                let mut key = GeneratedItem::new("GoldenKey", ItemCategory::Other);
                key.source = Some("forced".into());
                out.push(CreatedLoot {
                    loot: PlacedLoot {
                        item: key,
                        heap_type: "heap",
                    },
                    cell: Some(key_cell as usize),
                });
            }
        } else {
            out.push(CreatedLoot {
                loot: PlacedLoot {
                    item,
                    heap_type: "heap",
                },
                cell,
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
        out.push(CreatedLoot {
            loot: PlacedLoot {
                item: it,
                heap_type: "hidden",
            },
            cell: None,
        });
    }
    Random::pop_generator();

    out
}

/// Returns index into `map.map` / `occupied`, or -1.
///
/// Port of `RegularLevel.randomDropCell(StandardRoom.class)`:
/// - Every try calls `randomRoom`, i.e. `Random.shuffle(rooms)` on the ENTIRE
///   room list (entrance, exit, standard, special, secret, connection, shop,
///   AND set-empty rooms), then linear-scans the shuffled order for the first
///   `StandardRoom` instance. In Java `EntranceRoom`/`ExitRoom` extend
///   `StandardRoom`, so Rust kinds Entrance/Exit/Standard all match.
/// - If the scan finds no match, Java returns -1 on the FIRST try (exactly one
///   full-list shuffle burned). There is no fallback room set.
/// - The entrance room never hosts a drop (`room != roomEntrance`): picking it
///   wastes the try with no `Room.random()` draws.
/// - The exit room CAN host a drop, just never on the exit cell itself.
/// - Set-empty rooms stay in the list; if picked, watabou `Int(max <= 0)`
///   returns 0 WITHOUT consuming a draw, so the degenerate
///   `IntRange(left+1, right-1)` pair on a zeroed rect burns zero draws and
///   yields (1, 1), which then fails the terrain checks below.
fn random_drop_cell(
    rooms: &[Room],
    order: &mut [usize],
    map: &TerrainMap,
    occupied: &mut [bool],
) -> i32 {
    // Java `exit()` is the REGULAR_EXIT transition cell — the single cell the
    // exit room painter set to Terrain.EXIT. Invariant during this loop.
    let exit_cell = map.map.iter().position(|&t| t == EXIT);

    let mut tries = 100;
    while tries > 0 {
        tries -= 1;
        // `randomRoom`: Collections.shuffle on the full list, every try.
        Random::shuffle_list(order);
        // `room(type)`: first instanceof-StandardRoom in shuffled order.
        let room = order.iter().map(|&ri| &rooms[ri]).find(|r| {
            matches!(
                r.kind,
                RoomKind::Entrance | RoomKind::Exit | RoomKind::Standard
            )
        });
        let Some(room) = room else {
            return -1;
        };
        if room.is_entrance() {
            // `room != roomEntrance` — try wasted, no point draws.
            continue;
        }
        // `Room.random()` = IntRange(left+1, right-1), IntRange(top+1, bottom-1).
        let x = Random::int_range_inclusive(room.left + 1, room.right - 1);
        let y = Random::int_range_inclusive(room.top + 1, room.bottom - 1);
        let Some(idx) = map.point_to_cell(x, y) else {
            continue;
        };
        if idx >= map.passable.len() || !map.passable[idx] {
            continue;
        }
        if map.is_solid(idx) {
            continue;
        }
        if Some(idx) == exit_cell {
            continue;
        }
        if idx >= occupied.len() || occupied[idx] {
            continue;
        }
        if !map.item_allowed.get(idx).copied().unwrap_or(false) {
            continue;
        }
        // AquariumRoom checks the final terrain dynamically, including water
        // added by RegularPainter after the room's own pool was painted.
        if room.name == "AquariumRoom" && map.map[idx] == WATER {
            continue;
        }
        // `findMob(pos) == null`: room-painted and depth-one ambient mobs are
        // folded into `occupied` by the caller.
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
mod tests;
