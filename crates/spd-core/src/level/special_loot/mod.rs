//! Special / secret room prize generation (partial paint loot).
//!
//! Called after minimal geometry paint and **before** `createItems`, matching
//! RegularPainter room paint order (shuffle rooms → placeDoors → room prizes).
//! Returns door map + paint order for subsequent `paintDoors`.

mod crystal;
mod crystal_path;
mod gardens;
mod geometry;
mod pit_secrets;
mod placement;
mod quest_rooms;
mod secret_rooms;
mod shop_room;
mod special_rooms;
mod standard_rooms;
mod trap_rooms;

#[cfg(test)]
mod tests;

use crate::dungeon::DungeonState;
use crate::items::model::GeneratedItem;
use crate::level::create_items::PlacedLoot;
use crate::level::painter::{
    apply_room_door_types, paint_connection_room, paint_standard_room, place_doors_for_room,
    DoorMap,
};
use crate::level::terrain::TerrainMap;
use crate::level::Feeling;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

// Re-export for crystal path door placement.
pub(super) use crate::level::painter::door_spots;

/// Result of special-room paint pass (prizes + doors for `paintDoors`).
pub struct SpecialPaintResult {
    pub loot: Vec<PlacedLoot>,
    pub doors: DoorMap,
    /// Room indices in RegularPainter shuffle order (for `paintDoors` iteration).
    pub paint_order: Vec<usize>,
}

/// Generate special/secret room prizes; may consume items from `items_to_spawn`
/// via `findPrizeItem` (TrinketCatalyst, forced potions, etc.).
pub fn special_room_loot(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
    shop_items: &[GeneratedItem],
    feeling: Feeling,
) -> SpecialPaintResult {
    let mut out = Vec::new();
    let mut doors = DoorMap::new();
    if rooms.is_empty() {
        return SpecialPaintResult {
            loot: out,
            doors,
            paint_order: Vec::new(),
        };
    }

    let mut order: Vec<usize> = (0..rooms.len()).collect();
    // RegularPainter.paint shuffles its ArrayList with Collections.shuffle.
    Random::shuffle_list(&mut order);

    for &ri in &order {
        place_doors_for_room(rooms, ri, &mut doors);

        let room = &rooms[ri];
        if room.is_empty() {
            continue;
        }
        // Room.paint door-type upgrades (LOCKED / HIDDEN / REGULAR / …).
        apply_room_door_types(rooms, ri, &mut doors);
        if room.kind == RoomKind::Connection {
            paint_connection_room(map, rooms, ri, &doors, feeling == Feeling::Chasm);
        }
        let standard_paint = paint_standard_room(
            map,
            rooms,
            room,
            ri,
            &doors,
            &mut dungeon.generator,
            dungeon.depth,
        );
        let mut standard_loot = standard_rooms::paint_center_loot(
            dungeon,
            room,
            map,
            standard_paint.center_loot,
            items_to_spawn,
        );
        out.append(&mut standard_loot);

        match room.kind {
            RoomKind::Special | RoomKind::Secret => {
                geometry::paint(map, room, ri, &doors);
                let mut loot = paint_special(dungeon, rooms, ri, map, &doors, items_to_spawn);
                out.append(&mut loot);
            }
            RoomKind::Standard
                if room.name == "RitualSiteRoom" || room.name == "BlacksmithRoom" =>
            {
                geometry::paint(map, room, ri, &doors);
                let mut loot = paint_special(dungeon, rooms, ri, map, &doors, items_to_spawn);
                out.append(&mut loot);
            }
            RoomKind::Shop => {
                shop_room::paint(map, room, ri, &doors, shop_items);
            }
            _ => {}
        }
    }

    SpecialPaintResult {
        loot: out,
        doors,
        paint_order: order,
    }
}

fn paint_special(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    let name = room.name.as_str();
    match name {
        "CryptRoom" => vec![special_rooms::crypt_prize(dungeon, items_to_spawn)],
        "ArmoryRoom" => {
            let entrance = room
                .connected
                .iter()
                .find_map(|&other| doors.get(ri, other))
                .map(|door| crate::geom::Point::new(door.x, door.y))
                .expect("placed ArmoryRoom has an entrance");
            special_rooms::armory_prizes_on_map(dungeon, room, map, entrance, items_to_spawn)
        }
        "LibraryRoom" => special_rooms::library_prizes(dungeon, room, items_to_spawn),
        "TreasuryRoom" => special_rooms::treasury_prizes_on_map(dungeon, room, map, items_to_spawn),
        "PoolRoom" => vec![special_rooms::pool_prize_on_map(
            dungeon,
            room,
            map,
            items_to_spawn,
        )],
        "StorageRoom" => special_rooms::storage_prizes(dungeon, room, items_to_spawn),
        "RunestoneRoom" => {
            special_rooms::runestone_prizes_on_map(dungeon, room, map, items_to_spawn)
        }
        "LaboratoryRoom" => special_rooms::laboratory_prizes(dungeon, room, items_to_spawn),
        "StatueRoom" => vec![special_rooms::statue_weapon(dungeon, room, items_to_spawn)],
        "SecretLibraryRoom" => secret_rooms::secret_library(dungeon, room, items_to_spawn),
        "SecretRunestoneRoom" => secret_rooms::secret_runestone(dungeon, room, items_to_spawn),
        "SecretArtilleryRoom" => secret_rooms::secret_artillery(dungeon, room),
        "SecretLaboratoryRoom" => secret_rooms::secret_laboratory(dungeon, room, items_to_spawn),
        "SecretLarderRoom" => secret_rooms::secret_larder(dungeon, room),
        "SecretHoardRoom" => secret_rooms::secret_hoard(dungeon, room),
        // Garden / well / pit / remaining secrets with portable prizes
        "GardenRoom" => gardens::garden_prizes(room, items_to_spawn),
        "SecretGardenRoom" => gardens::secret_garden_prizes(room),
        "MagicWellRoom" => gardens::magic_well(room, map, items_to_spawn),
        "SecretWellRoom" => gardens::secret_well(),
        "PitRoom" => pit_secrets::pit_prizes(dungeon, items_to_spawn),
        "SecretMazeRoom" => vec![pit_secrets::secret_maze_prize(dungeon)],
        "SecretSummoningRoom" => vec![pit_secrets::secret_summoning_prize(dungeon)],
        "SecretChestChasmRoom" => pit_secrets::secret_chest_chasm(dungeon, items_to_spawn),
        // Layout-only (no portable prize items)
        "WeakFloorRoom" | "DemonSpawnerRoom" => Vec::new(),
        "SentryRoom" => vec![trap_rooms::sentry_prize(dungeon, items_to_spawn)],
        "TrapsRoom" => vec![trap_rooms::traps_prize(
            dungeon,
            rooms,
            ri,
            map,
            doors,
            items_to_spawn,
        )],
        "MagicalFireRoom" => {
            trap_rooms::magical_fire_prizes(dungeon, rooms, ri, map, doors, items_to_spawn)
        }
        "SacrificeRoom" => vec![trap_rooms::sacrifice_prize(dungeon, rooms, ri)],
        "ToxicGasRoom" => trap_rooms::toxic_gas_prizes(dungeon, room, items_to_spawn),
        "SecretHoneypotRoom" => trap_rooms::secret_honeypot(room),
        "CrystalVaultRoom" => crystal::crystal_vault(dungeon, rooms, ri, items_to_spawn),
        "CrystalChoiceRoom" => crystal::crystal_choice(dungeon, room, items_to_spawn),
        "CrystalPathRoom" => crystal_path::paint(dungeon, rooms, ri, map, doors, items_to_spawn),
        // Wandmaker quest rooms
        "MassGraveRoom" => quest_rooms::mass_grave_prizes(dungeon, room, items_to_spawn),
        "RitualSiteRoom" => quest_rooms::ritual_site_setup(items_to_spawn),
        "RotGardenRoom" => quest_rooms::rot_garden_setup(room, items_to_spawn),
        // Blacksmith quest room — two random equip drops + NPC / exit placement RNG
        "BlacksmithRoom" => quest_rooms::blacksmith_room_prizes(dungeon, room),
        // Imp quest room — NPC offset burns IntRange(-1,1); no prize items
        "AmbitiousImpRoom" => {
            let _ = Random::int_range_inclusive(-1, 1);
            Vec::new()
        }
        _ => Vec::new(),
    }
}
