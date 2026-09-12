//! Quest-related room prizes (wandmaker, blacksmith).

use crate::dungeon::DungeonState;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::painter::DoorMap;
use crate::level::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

/// `MassGraveRoom.paint` terrain, skeleton occupancy, and loot.
pub(super) fn mass_grave_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    super::geometry::paint_mass_grave(dungeon, room, map, items_to_spawn)
}

/// `RitualSiteRoom.paint` — cages, ritual marker, and four ceremonial candles.
pub(super) fn ritual_site_setup(
    rooms: &[Room],
    room_index: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    super::geometry::paint_ritual_site(&rooms[room_index], room_index, map, doors, items_to_spawn)
}

/// `RotGardenRoom.paint` key. Geometry, heart, and lasher RNG are painted by
/// `special_loot::geometry` before this helper runs.
pub(super) fn rot_garden_setup(
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    let _ = room;
    Vec::new()
}

/// `BlacksmithRoom.paint` — pedestal equipment, fixed NPC, and quest exit.
pub(super) fn blacksmith_room_prizes(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    room_index: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
) -> Vec<PlacedLoot> {
    super::geometry::paint_blacksmith(dungeon, rooms, room_index, map, doors)
}

/// `AmbitiousImpRoom.paint` NPC offset relative to its single entrance.
pub(super) fn ambitious_imp_room_npc(
    rooms: &[Room],
    room_index: usize,
    map: &mut TerrainMap,
    doors: &DoorMap,
) {
    let room = &rooms[room_index];
    let center = room.as_rect().center_room();
    let Some(door) = room
        .connected
        .iter()
        .find_map(|&other| doors.get(room_index, other))
    else {
        // A valid AmbitiousImpRoom has exactly one entrance. Retain the pinned
        // draw shape if an incomplete partial layout failed to connect it.
        let _ = Random::int_range_inclusive(-1, 1);
        return;
    };

    let (x, y) = if door.x == room.left || door.x == room.right {
        (
            center.x + if door.x == room.left { -2 } else { 2 },
            center.y + Random::int_range_inclusive(-1, 1),
        )
    } else {
        (
            center.x + Random::int_range_inclusive(-1, 1),
            center.y + if door.y == room.top { -2 } else { 2 },
        )
    };
    if let Some(cell) = map.point_to_cell(x, y) {
        map.mob_occupied[cell] = true;
        map.known_mobs[cell] = Some("Imp");
    }
}
