//! Garden and magic-well special room prizes.

use super::placement::burn_drop_pos;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::random::Random;
use crate::rooms::room::Room;

/// `GardenRoom.paint` — IronKey + 0–2 plant seeds (Sungrass / Blandfruit).
pub(super) fn garden_prizes(room: &Room, items_to_spawn: &mut Vec<GeneratedItem>) -> Vec<PlacedLoot> {
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
    burn_patch_generate(pw, ph, 0.5, 0, true);

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

/// Minimal port of `Patch.generate` RNG consumption (for secret-garden parity).
fn burn_patch_generate(w: i32, h: i32, fill: f32, clustering: i32, force_fill_rate: bool) {
    if w <= 0 || h <= 0 {
        return;
    }
    let length = (w * h) as usize;
    let mut fill = fill;
    let mut fill_diff = -((length as f32 * fill).round() as i32);

    if force_fill_rate && clustering > 0 {
        fill += (0.5 - fill) * 0.5;
    }

    let mut off = vec![false; length];
    let mut cur = vec![false; length];
    for cell in &mut off {
        *cell = Random::float() < fill;
        if *cell {
            fill_diff += 1;
        }
    }

    for _ in 0..clustering {
        for y in 0..h {
            for x in 0..w {
                let pos = (x + y * w) as usize;
                let mut count = 0i32;
                let mut neighbours = 0i32;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < w && ny >= 0 && ny < h {
                            neighbours += 1;
                            if off[(nx + ny * w) as usize] {
                                count += 1;
                            }
                        }
                    }
                }
                cur[pos] = 2 * count >= neighbours;
                if cur[pos] != off[pos] {
                    fill_diff += if cur[pos] { 1 } else { -1 };
                }
            }
        }
        std::mem::swap(&mut cur, &mut off);
    }

    // force fill-rate adjustment (uses Random while fillDiff != 0)
    if force_fill_rate && w.min(h) > 2 {
        let growing = fill_diff < 0;
        // Cap iterations so pathological rooms never hang the analyzer.
        let mut guard = length * 20 + 64;
        while fill_diff != 0 && guard > 0 {
            guard -= 1;
            let mut tries = 0;
            // random interior cell; try length/10 times to match growing state
            // Java: Random.Int(1, w-1) exclusive upper → [1, w-2]
            let cell = loop {
                let cx = Random::int_range_inclusive(1, w - 2);
                let cy = Random::int_range_inclusive(1, h - 2);
                let c = (cx + cy * w) as usize;
                tries += 1;
                if off[c] == growing || tries * 10 >= length as i32 {
                    break c;
                }
            };
            // 3×3 neighbourhood around cell (including self)
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if fill_diff == 0 {
                        break;
                    }
                    let nx = (cell as i32 % w) + dx;
                    let ny = (cell as i32 / w) + dy;
                    if nx < 0 || nx >= w || ny < 0 || ny >= h {
                        continue;
                    }
                    let npos = (nx + ny * w) as usize;
                    if off[npos] != growing {
                        off[npos] = growing;
                        fill_diff += if growing { 1 } else { -1 };
                    }
                }
            }
        }
    }
}

/// `MagicWellRoom.paint` — locked IronKey + Awareness/Health well type.
pub(super) fn magic_well(items_to_spawn: &mut Vec<GeneratedItem>) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    secret_well_type("MagicWellRoom")
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
