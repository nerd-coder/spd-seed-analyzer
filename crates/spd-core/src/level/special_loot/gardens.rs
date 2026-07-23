//! Garden and magic-well special room prizes.

use super::placement::burn_drop_pos;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, EMPTY, WALL, WELL};
use crate::random::Random;
use crate::rooms::room::Room;

/// `GardenRoom.paint` — IronKey + 0–2 plant seeds (Sungrass / Blandfruit).
pub(super) fn garden_prizes(
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));

    let bushes = Random::int_max(3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    match bushes {
        0 => {
            burn_drop_pos(room, &mut occupied);
            out.push(plant_loot("SungrassSeed", "GardenRoom"));
        }
        1 => {
            burn_drop_pos(room, &mut occupied);
            out.push(plant_loot("BlandfruitBushSeed", "GardenRoom"));
        }
        _ => {
            // 20% both seeds
            if Random::int_max(5) == 0 {
                burn_drop_pos(room, &mut occupied);
                out.push(plant_loot("SungrassSeed", "GardenRoom"));
                burn_drop_pos(room, &mut occupied);
                out.push(plant_loot("BlandfruitBushSeed", "GardenRoom"));
            }
        }
    }
    out
}

/// `SecretGardenRoom.paint` — Starflower + Seedpod + Dewcatcher + 50% extra.
/// `Patch.generate` grass layout is burned for RNG stream parity (approx plant cells).
pub(super) fn secret_garden_prizes(room: &Room) -> Vec<PlacedLoot> {
    // Patch.generate(w-2, h-2, 0.5, clustering=0, forceFillRate=true)
    let pw = (room.width() - 2).max(0);
    let ph = (room.height() - 2).max(0);
    let _ = crate::level::patch::generate(pw, ph, 0.5, 0, true);

    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for class in ["StarflowerSeed", "SeedpodSeed", "DewcatcherSeed"] {
        burn_drop_pos(room, &mut occupied);
        out.push(plant_loot(class, "SecretGardenRoom"));
    }
    burn_drop_pos(room, &mut occupied);
    let extra = if Random::int_max(2) == 0 {
        "SeedpodSeed"
    } else {
        "DewcatcherSeed"
    };
    out.push(plant_loot(extra, "SecretGardenRoom"));
    out
}

fn plant_loot(class_name: &str, source: &str) -> PlacedLoot {
    let mut item = GeneratedItem::new(class_name, ItemCategory::Seed);
    item.source = Some(source.into());
    PlacedLoot {
        item,
        heap_type: "plant",
    }
}

/// `MagicWellRoom.paint` — locked IronKey + Awareness/Health well type.
pub(super) fn magic_well(
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                let inside = x > room.left && x < room.right && y > room.top && y < room.bottom;
                map.map[cell] = if inside { EMPTY } else { WALL };
            }
        }
    }
    let center = room.as_rect().center_room();
    let well_cell = map.point_to_cell(center.x, center.y);
    if let Some(cell) = well_cell {
        map.map[cell] = WELL;
        map.character_allowed[cell] = false;
    }
    // Well water is a blob, not a heap item, but its class selection is seeded.
    let water = *Random::one_of(&["WaterOfAwareness", "WaterOfHealth"]);
    if let Some(cell) = well_cell {
        map.record_blob_cell(water, false, cell, 1);
    }
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    Vec::new()
}

/// `SecretWellRoom.paint` — Awareness/Health well (no key).
pub(super) fn secret_well() -> Vec<PlacedLoot> {
    secret_well_type("SecretWellRoom")
}

fn secret_well_type(source: &str) -> Vec<PlacedLoot> {
    let waters = ["WaterOfAwareness", "WaterOfHealth"];
    let class = *Random::one_of(&waters);
    let mut item = GeneratedItem::new(class, ItemCategory::Other);
    item.source = Some(source.into());
    vec![PlacedLoot {
        item,
        heap_type: "well",
    }]
}
