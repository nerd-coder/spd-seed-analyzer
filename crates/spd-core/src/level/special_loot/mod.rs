//! Special / secret room prize generation (partial paint loot).
//!
//! Called after minimal geometry paint and **before** `createItems`, matching
//! RegularPainter room paint order (shuffle rooms → placeDoors RNG → room prizes).
//! Placement loops consume RNG; layout water/grass/traps still incomplete so
//! results remain approximate vs full game parity.

mod crystal;
mod hazards;
mod placement;
mod quest_rooms;
mod secret_rooms;
mod special_rooms;

#[cfg(test)]
mod tests;

use crate::dungeon::DungeonState;
use crate::geom::Point;
use crate::items::model::GeneratedItem;
use crate::level::create_items::PlacedLoot;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

/// Generate special/secret room prizes; may consume items from `items_to_spawn`
/// via `findPrizeItem` (TrinketCatalyst, forced potions, etc.).
pub fn special_room_loot(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    if rooms.is_empty() {
        return out;
    }

    let mut order: Vec<usize> = (0..rooms.len()).collect();
    Random::shuffle_vec(&mut order);

    // placeDoors-style RNG for each undirected connection (door cell pick).
    let mut doors_placed: Vec<(usize, usize)> = Vec::new();
    for &ri in &order {
        place_doors_rng(rooms, ri, &mut doors_placed);

        let room = &rooms[ri];
        if room.is_empty() {
            continue;
        }
        match room.kind {
            RoomKind::Special | RoomKind::Secret => {
                let mut loot = paint_special(dungeon, rooms, ri, items_to_spawn);
                out.append(&mut loot);
            }
            RoomKind::Standard
                if room.name == "RitualSiteRoom" || room.name == "BlacksmithRoom" =>
            {
                let mut loot = paint_special(dungeon, rooms, ri, items_to_spawn);
                out.append(&mut loot);
            }
            RoomKind::Shop => {
                // Shop contents deferred (FOR_SALE filter); still burn no RNG here.
            }
            _ => {}
        }
    }

    out
}

fn place_doors_rng(rooms: &[Room], ri: usize, doors_placed: &mut Vec<(usize, usize)>) {
    let room = &rooms[ri];
    for &ni in &room.connected {
        let a = ri.min(ni);
        let b = ri.max(ni);
        if doors_placed.contains(&(a, b)) {
            continue;
        }
        let other = &rooms[ni];
        if other.is_empty() {
            continue;
        }
        let spots = door_spots(room, other);
        if !spots.is_empty() {
            let _ = Random::element(&spots);
        }
        doors_placed.push((a, b));
    }
}

pub(super) fn door_spots(a: &Room, b: &Room) -> Vec<Point> {
    let left = a.left.max(b.left);
    let right = a.right.min(b.right);
    let top = a.top.max(b.top);
    let bottom = a.bottom.min(b.bottom);
    let mut spots = Vec::new();
    for x in left..=right {
        for y in top..=bottom {
            let p = Point::new(x, y);
            if can_connect(a, p) && can_connect(b, p) {
                spots.push(p);
            }
        }
    }
    spots
}

fn can_connect(r: &Room, p: Point) -> bool {
    (p.x == r.left || p.x == r.right) != (p.y == r.top || p.y == r.bottom)
}

fn paint_special(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    let name = room.name.as_str();
    match name {
        "CryptRoom" => vec![special_rooms::crypt_prize(dungeon)],
        "ArmoryRoom" => special_rooms::armory_prizes(dungeon, room, items_to_spawn),
        "LibraryRoom" => special_rooms::library_prizes(dungeon, room, items_to_spawn),
        "TreasuryRoom" => special_rooms::treasury_prizes(dungeon, room, items_to_spawn),
        "PoolRoom" => vec![special_rooms::pool_prize(dungeon, room, items_to_spawn)],
        "StorageRoom" => special_rooms::storage_prizes(dungeon, room, items_to_spawn),
        "RunestoneRoom" => special_rooms::runestone_prizes(dungeon, room, items_to_spawn),
        "LaboratoryRoom" => special_rooms::laboratory_prizes(dungeon, room, items_to_spawn),
        "StatueRoom" => vec![special_rooms::statue_weapon(dungeon, room)],
        "SecretLibraryRoom" => secret_rooms::secret_library(dungeon, room, items_to_spawn),
        "SecretRunestoneRoom" => secret_rooms::secret_runestone(dungeon, room, items_to_spawn),
        "SecretArtilleryRoom" => secret_rooms::secret_artillery(dungeon, room),
        "SecretLaboratoryRoom" => secret_rooms::secret_laboratory(dungeon, room, items_to_spawn),
        "SecretLarderRoom" => secret_rooms::secret_larder(dungeon, room),
        "SecretHoardRoom" => secret_rooms::secret_hoard(dungeon, room),
        // Garden / well / pit / remaining secrets with portable prizes
        "GardenRoom" => hazards::garden_prizes(room, items_to_spawn),
        "SecretGardenRoom" => hazards::secret_garden_prizes(room),
        "MagicWellRoom" => hazards::magic_well(items_to_spawn),
        "SecretWellRoom" => hazards::secret_well(),
        "PitRoom" => hazards::pit_prizes(dungeon, items_to_spawn),
        "SecretMazeRoom" => vec![hazards::secret_maze_prize(dungeon)],
        "SecretSummoningRoom" => vec![hazards::secret_summoning_prize(dungeon)],
        "SecretChestChasmRoom" => hazards::secret_chest_chasm(dungeon, items_to_spawn),
        // Layout-only (no portable prize items)
        "WeakFloorRoom" | "DemonSpawnerRoom" => Vec::new(),
        "SentryRoom" => vec![hazards::sentry_prize(dungeon, items_to_spawn)],
        "TrapsRoom" => vec![hazards::traps_prize(dungeon, items_to_spawn)],
        "MagicalFireRoom" => hazards::magical_fire_prizes(dungeon, room, items_to_spawn),
        "SacrificeRoom" => vec![hazards::sacrifice_prize(dungeon, rooms, ri)],
        "ToxicGasRoom" => hazards::toxic_gas_prizes(dungeon, room, items_to_spawn),
        "SecretHoneypotRoom" => hazards::secret_honeypot(room),
        "CrystalVaultRoom" => crystal::crystal_vault(dungeon, rooms, ri, items_to_spawn),
        "CrystalChoiceRoom" => crystal::crystal_choice(dungeon, room, items_to_spawn),
        "CrystalPathRoom" => crystal::crystal_path(dungeon, items_to_spawn),
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
