//! Garden and magic-well special room prizes.

use super::placement::burn_drop_pos;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, CHASM, EMPTY, GRASS, HIGH_GRASS, WALL, WELL};
use crate::random::Random;
use crate::rooms::room::Room;

/// `GardenRoom.paint` — IronKey + 0–2 plant seeds (Sungrass / Blandfruit).
pub(super) fn garden_prizes(
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
    no_herbalism: bool,
) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));

    let bushes = Random::int_max(3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    match bushes {
        0 => {
            plant_pos(room, map, &mut occupied, "Sungrass", 3, no_herbalism);
            if !no_herbalism {
                out.push(plant_loot("SungrassSeed", "GardenRoom"));
            }
        }
        1 => {
            plant_pos(room, map, &mut occupied, "BlandfruitBush", 12, no_herbalism);
            if !no_herbalism {
                out.push(plant_loot("BlandfruitBushSeed", "GardenRoom"));
            }
        }
        _ => {
            // 20% both seeds
            if Random::int_max(5) == 0 {
                plant_pos(room, map, &mut occupied, "Sungrass", 3, no_herbalism);
                if !no_herbalism {
                    out.push(plant_loot("SungrassSeed", "GardenRoom"));
                }
                plant_pos(room, map, &mut occupied, "BlandfruitBush", 12, no_herbalism);
                if !no_herbalism {
                    out.push(plant_loot("BlandfruitBushSeed", "GardenRoom"));
                }
            }
        }
    }

    // GardenRoom.paint seeds Foliage over the complete room interior after
    // placing its bushes. This is additive blob state and consumes no RNG.
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.record_blob_cell("Foliage", false, cell, 1);
            }
        }
    }
    out
}

fn plant_pos(
    room: &Room,
    map: &mut TerrainMap,
    occupied: &mut Vec<(i32, i32)>,
    class_name: &'static str,
    image: u8,
    no_herbalism: bool,
) {
    let (x, y) = if no_herbalism {
        // Barren Land leaves Level.plants empty, so plantPos's uniqueness
        // loop accepts the very first Room.random() pair on every attempt.
        (
            Random::int_range_inclusive(room.left + 1, room.right - 1),
            Random::int_range_inclusive(room.top + 1, room.bottom - 1),
        )
    } else {
        let before = occupied.len();
        burn_drop_pos(room, occupied);
        occupied
            .get(before)
            .copied()
            .expect("plantPos adds an unused cell")
    };
    if let Some(cell) = map.point_to_cell(x, y) {
        // `Level.plant` converts HIGH_GRASS/EMPTY/EMBERS to GRASS even when
        // Barren Land suppresses the actual plant object.
        if map.map[cell] == crate::level::terrain::HIGH_GRASS {
            map.map[cell] = crate::level::terrain::GRASS;
        }
        if !no_herbalism {
            map.record_plant(cell, class_name, image);
        }
    }
}

/// `SecretGardenRoom.paint` — grass patch plus Starflower, Seedpod, Dewcatcher,
/// and a 50% extra Seedpod or Dewcatcher.
pub(super) fn secret_garden_prizes(
    room: &Room,
    map: &mut TerrainMap,
    no_herbalism: bool,
) -> Vec<PlacedLoot> {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] =
                    if x > room.left && x < room.right && y > room.top && y < room.bottom {
                        GRASS
                    } else {
                        WALL
                    };
            }
        }
    }

    // Patch.generate(w-2, h-2, 0.5, clustering=0, forceFillRate=true)
    let pw = (room.width() - 2).max(0);
    let ph = (room.height() - 2).max(0);
    let grass = crate::level::patch::generate(pw, ph, 0.5, 0, true);
    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            let patch_cell = ((x - room.left - 1) + (y - room.top - 1) * pw) as usize;
            if grass.get(patch_cell).copied().unwrap_or(false) {
                let cell = map.point_to_cell(x, y).expect("secret garden lies on map");
                map.map[cell] = HIGH_GRASS;
            }
        }
    }

    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for (seed, plant, image) in [
        ("StarflowerSeed", "Starflower", 9),
        ("SeedpodSeed", "Seedpod", 14),
        ("DewcatcherSeed", "Dewcatcher", 13),
    ] {
        secret_garden_plant(room, map, &mut occupied, plant, image, no_herbalism);
        if !no_herbalism {
            out.push(plant_loot(seed, "SecretGardenRoom"));
        }
    }
    // Java rolls the fourth seed's class before drawing its position, and a
    // repeated position costs another `Room.random` pair.
    let (extra_seed, extra_plant, extra_image) = if Random::int_max(2) == 0 {
        ("SeedpodSeed", "Seedpod", 14)
    } else {
        ("DewcatcherSeed", "Dewcatcher", 13)
    };
    secret_garden_plant(
        room,
        map,
        &mut occupied,
        extra_plant,
        extra_image,
        no_herbalism,
    );
    if !no_herbalism {
        out.push(plant_loot(extra_seed, "SecretGardenRoom"));
    }

    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            let cell = map.point_to_cell(x, y).expect("secret garden lies on map");
            map.record_blob_cell("Foliage", false, cell, 1);
        }
    }
    out
}

fn secret_garden_plant(
    room: &Room,
    map: &mut TerrainMap,
    occupied: &mut Vec<(i32, i32)>,
    class_name: &'static str,
    image: u8,
    no_herbalism: bool,
) {
    let (x, y) = if no_herbalism {
        (
            Random::int_range_inclusive(room.left + 1, room.right - 1),
            Random::int_range_inclusive(room.top + 1, room.bottom - 1),
        )
    } else {
        let before = occupied.len();
        burn_drop_pos(room, occupied);
        occupied
            .get(before)
            .copied()
            .expect("plantPos adds an unused cell")
    };
    let cell = map
        .point_to_cell(x, y)
        .expect("secret garden plant lies on map");
    // `Level.plant` converts high grass before couching the plant.
    if map.map[cell] == HIGH_GRASS {
        map.map[cell] = GRASS;
    }
    if !no_herbalism {
        map.item_allowed[cell] = false;
        map.character_allowed[cell] = false;
        map.record_plant(cell, class_name, image);
    }
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
        // Java `WellWater` instance initializer sets `alwaysVisible = true`.
        map.record_blob_cell(water, true, cell, 1);
    }
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    Vec::new()
}

/// `SecretWellRoom.paint` — chasm-ringed Awareness/Health well facing the
/// hidden entrance (no key).
pub(super) fn secret_well(room: &Room, map: &mut TerrainMap, entrance: Point) -> Vec<PlacedLoot> {
    fill(map, room.left, room.top, room.right, room.bottom, WALL);
    let well = if entrance.x == room.left {
        Point::new(room.right - 2, entrance.y)
    } else if entrance.x == room.right {
        Point::new(room.left + 2, entrance.y)
    } else if entrance.y == room.top {
        Point::new(entrance.x, room.bottom - 2)
    } else {
        Point::new(entrance.x, room.top + 2)
    };
    fill(map, well.x - 1, well.y - 1, well.x + 1, well.y + 1, CHASM);
    fill(
        map,
        entrance.x.min(well.x),
        entrance.y.min(well.y),
        entrance.x.max(well.x),
        entrance.y.max(well.y),
        EMPTY,
    );
    let well_cell = map
        .point_to_cell(well.x, well.y)
        .expect("SecretWellRoom well lies on map");
    map.map[well_cell] = WELL;
    map.character_allowed[well_cell] = false;

    let class = *Random::one_of(&["WaterOfAwareness", "WaterOfHealth"]);
    // Java `WellWater` instance initializer sets `alwaysVisible = true`.
    map.record_blob_cell(class, true, well_cell, 1);
    let mut item = GeneratedItem::new(class, ItemCategory::Other);
    item.source = Some("SecretWellRoom".into());
    vec![PlacedLoot {
        item,
        heap_type: "well",
    }]
}

fn fill(map: &mut TerrainMap, left: i32, top: i32, right: i32, bottom: i32, terrain: i32) {
    for y in top..=bottom {
        for x in left..=right {
            if let Some(cell) = map.point_to_cell(x, y) {
                map.map[cell] = terrain;
            }
        }
    }
}
