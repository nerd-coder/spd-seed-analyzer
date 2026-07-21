//! Garden, well, pit, maze, traps, sacrifice, toxic gas, and related rooms.

use super::crystal::vault_entrance_cell;
use super::placement::{burn_drop_pos, burn_drop_pos_margin, find_prize_item};
use super::special_rooms::{bomb_random, is_curse_enchant, storage_prize};
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::Point;
use crate::items::enchants;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
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

/// `PitRoom.paint` — skeleton main loot (ring/artifact/equip) + 1–2 consumables + CrystalKey.
pub(super) fn pit_prizes(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Well corner: Random.Int(2) (door side is geometric from entrance).
    let _ = Random::int_max(2);

    // Main loot: ring / artifact / equip (weapon×2, missile, armor×2).
    // Challenges.isItemBlocked is always false without challenges — single draw.
    let mut main = match Random::int_max(3) {
        0 => dungeon
            .generator
            .random_category(Category::Ring, dungeon.depth),
        1 => dungeon
            .generator
            .random_category(Category::Artifact, dungeon.depth),
        _ => {
            let cats = [
                Category::Weapon,
                Category::Weapon,
                Category::Missile,
                Category::Armor,
                Category::Armor,
            ];
            let cat = *Random::one_of(&cats);
            dungeon.generator.random_category(cat, dungeon.depth)
        }
    };
    main.source = Some("PitRoom".into());
    let mut out = vec![PlacedLoot {
        item: main,
        heap_type: "skeleton",
    }];

    let n = Random::int_range_inclusive(1, 2);
    for _ in 0..n {
        let cats = [
            Category::Potion,
            Category::Scroll,
            Category::Food,
            Category::Gold,
        ];
        let cat = *Random::one_of(&cats);
        let mut prize = dungeon.generator.random_category(cat, dungeon.depth);
        prize.source = Some("PitRoom".into());
        out.push(PlacedLoot {
            item: prize,
            heap_type: "skeleton",
        });
    }

    items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    out
}

/// `SecretMazeRoom.paint` prize — +1 floor-set weapon/armor, never cursed, 33% upgrade.
/// Maze layout RNG is not fully ported (PathFinder distance pick skipped) — prize stream is approximate.
pub(super) fn secret_maze_prize(dungeon: &mut DungeonState) -> PlacedLoot {
    let floor = (dungeon.depth / 5) + 1;
    let mut prize = if Random::int_max(2) == 0 {
        let mut w = dungeon.generator.random_weapon(floor, true, dungeon.depth);
        if is_curse_enchant(&w) {
            w.enchantment = None;
        }
        w
    } else {
        let mut a = dungeon.generator.random_armor(floor, dungeon.depth);
        if is_curse_enchant(&a) {
            a.enchantment = None;
        }
        a
    };
    prize.cursed = false;
    // cursedKnown = true is UI-only in full game
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("SecretMazeRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

/// `SecretSummoningRoom.paint` — center skeleton with `Generator.random()`.
pub(super) fn secret_summoning_prize(dungeon: &mut DungeonState) -> PlacedLoot {
    // Trap reveal chance is 0 without TrapMechanism trinket — no extra RNG in trap loop.
    let mut item = dungeon.generator.random(dungeon.depth);
    item.source = Some("SecretSummoningRoom".into());
    PlacedLoot {
        item,
        heap_type: "skeleton",
    }
}

/// `SecretChestChasmRoom.paint` — 4 locked chests (`randomUsingDefaults`) + golden keys + levitation.
pub(super) fn secret_chest_chasm(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    // Geometry is fixed; four locked chests always drop (heaps non-null without blocked items).
    for _ in 0..4 {
        let mut item = dungeon.generator.random_using_defaults_any(dungeon.depth);
        item.source = Some("SecretChestChasmRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "locked_chest",
        });
    }
    // Four golden keys for the four chests (reported so seed-finder can see key count)
    for _ in 0..4 {
        let mut key = GeneratedItem::new("GoldenKey", ItemCategory::Other);
        key.source = Some("SecretChestChasmRoom".into());
        out.push(PlacedLoot {
            item: key,
            heap_type: "heap",
        });
    }
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLevitation",
        ItemCategory::Potion,
    ));
    out
}

/// `SentryRoom.paint` prize — chest equip or findPrizeItem + PotionOfHaste.
pub(super) fn sentry_prize(dungeon: &mut DungeonState, items_to_spawn: &mut Vec<GeneratedItem>) -> PlacedLoot {
    // Layout (center/sentry/treasure) is geometric from entrance — no RNG before prize.
    let mut prize = if Random::int_max(2) == 0 {
        find_prize_item(items_to_spawn, None).unwrap_or_else(|| sentry_equip(dungeon))
    } else {
        sentry_equip(dungeon)
    };
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("SentryRoom".into());
    items_to_spawn.push(GeneratedItem::new("PotionOfHaste", ItemCategory::Potion));
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

fn sentry_equip(dungeon: &mut DungeonState) -> GeneratedItem {
    let floor = (dungeon.depth / 5) + 1;
    // Random.Int(5): 0,1 weapon; 2 missile; 3,4 armor
    match Random::int_max(5) {
        0 | 1 => dungeon.generator.random_weapon(floor, false, dungeon.depth),
        2 => dungeon
            .generator
            .random_missile(floor, false, dungeon.depth),
        _ => dungeon.generator.random_armor(floor, dungeon.depth),
    }
}

/// `TrapsRoom.paint` — trap class RNG then chest prize + PotionOfLevitation.
pub(super) fn traps_prize(dungeon: &mut DungeonState, items_to_spawn: &mut Vec<GeneratedItem>) -> PlacedLoot {
    // Trap class selection (layout only; no trap instances in report).
    // Int(4)==0 → chasm (null traps); else oneOf(levelTraps[depth/5]).
    if Random::int_max(4) != 0 {
        let region = (dungeon.depth / 5).clamp(0, 4) as usize;
        let n = TRAPS_ROOM_TRAP_COUNTS[region];
        let _ = Random::int_max(n);
    }

    // Pedestal vs free chest: Random.Int(3) == 0 skips pedestal (geometry only).
    let _ = Random::int_max(3);

    let mut prize = if Random::int_max(3) != 0 {
        find_prize_item(items_to_spawn, None).unwrap_or_else(|| traps_equip(dungeon))
    } else {
        traps_equip(dungeon)
    };
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("TrapsRoom".into());
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLevitation",
        ItemCategory::Potion,
    ));
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

/// Counts for `TrapsRoom.levelTraps` oneOf per region (sewers…halls).
const TRAPS_ROOM_TRAP_COUNTS: [i32; 5] = [3, 3, 3, 3, 1];

fn traps_equip(dungeon: &mut DungeonState) -> GeneratedItem {
    let floor = (dungeon.depth / 5) + 1;
    if Random::int_max(2) == 0 {
        dungeon.generator.random_weapon(floor, false, dungeon.depth)
    } else {
        dungeon.generator.random_armor(floor, dungeon.depth)
    }
}

/// `MagicalFireRoom.paint` — 3–4 honeypot/consumable drops + PotionOfFrost.
pub(super) fn magical_fire_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Fire wall geometry from entrance — no RNG. behindFire drops use random(0).
    let mut honey = Random::int_max(2) == 0;
    let n = Random::int_range_inclusive(3, 4);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos_margin(room, 0, &mut occupied);
        let mut item = if honey {
            honey = false;
            GeneratedItem::new("Honeypot", ItemCategory::Other)
        } else {
            // Same prize table as StorageRoom / MagicalFireRoom.prize
            storage_prize(dungeon, items_to_spawn)
        };
        item.source = Some("MagicalFireRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    items_to_spawn.push(GeneratedItem::new("PotionOfFrost", ItemCategory::Potion));
    out
}

/// `SacrificeRoom.paint` — cursed upgraded weapon on sacrificial fire.
pub(super) fn sacrifice_prize(dungeon: &mut DungeonState, rooms: &[Room], ri: usize) -> PlacedLoot {
    // Center offset when door is mid-wall aligned with room center.
    burn_sacrifice_center_offset(rooms, ri);

    // 1 floor set higher than normal
    let mut prize = dungeon
        .generator
        .random_weapon((dungeon.depth / 5) + 1, false, dungeon.depth);

    // Always generate curse (parchment scrap isolation), matching CryptRoom pattern.
    let curse = enchants::random_weapon_curse(None).to_string();
    if !prize.cursed {
        prize.level += 1;
        if !is_good_weapon_enchant(&prize) {
            prize.enchantment = Some(curse);
        }
    }
    prize.cursed = true;
    prize.source = Some("SacrificeRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "sacrificial",
    }
}

fn is_good_weapon_enchant(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => !matches!(
            e,
            "Annoying"
                | "Displacing"
                | "Dazzling"
                | "Explosive"
                | "Sacrificial"
                | "Wayward"
                | "Polarized"
                | "Friendly"
        ),
        None => false,
    }
}

/// Burn `Random.Int(2)` center nudge when entrance is mid-edge (SacrificeRoom).
fn burn_sacrifice_center_offset(rooms: &[Room], ri: usize) {
    let room = &rooms[ri];
    if room.is_empty() {
        return;
    }
    let c = Point::new((room.left + room.right) / 2, (room.top + room.bottom) / 2);
    let Some(door) = vault_entrance_cell(rooms, ri) else {
        return;
    };
    let side_door = (door.x == room.left || door.x == room.right) && door.y == c.y;
    let end_door = (door.y == room.top || door.y == room.bottom) && door.x == c.x;
    if side_door || end_door {
        let _ = Random::int_max(2);
    }
}

/// `ToxicGasRoom.paint` — skeleton 2×gold + 2 chests (cata/gold) + trap placement RNG.
pub(super) fn toxic_gas_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Vent traps: min(w-2, h-2) placements at random(2) on EMPTY (approx unique).
    let traps = (room.width() - 2).min(room.height() - 2).max(0);
    let mut occupied = Vec::new();
    for _ in 0..traps {
        burn_drop_pos_margin(room, 2, &mut occupied);
    }

    // 8 candidate gold positions at random(2); furthest becomes skeleton (trueDistance
    // pick is pure geometry — no extra RNG).
    for _ in 0..8 {
        burn_drop_pos_margin(room, 2, &mut occupied);
    }

    let mut out = Vec::new();

    // Main gold ×2 on skeleton heap (blacklisted from report; still for RNG parity).
    let mut main = GeneratedItem::new("Gold", ItemCategory::Gold);
    randomize_item(&mut main, dungeon.depth);
    main.quantity = main.quantity.saturating_mul(2);
    main.source = Some("ToxicGasRoom".into());
    out.push(PlacedLoot {
        item: main,
        heap_type: "skeleton",
    });

    // Two chests: TrinketCatalyst prize item or random gold
    for _ in 0..2 {
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
                randomize_item(&mut g, dungeon.depth);
                g
            });
        item.source = Some("ToxicGasRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "chest",
        });
    }

    items_to_spawn.push(GeneratedItem::new("PotionOfPurity", ItemCategory::Potion));
    out
}

/// `SecretHoneypotRoom.paint` — shattered pot (geom) + honeypot + Bomb.random().
pub(super) fn secret_honeypot(room: &Room) -> Vec<PlacedLoot> {
    // brokenPotPos is geometric midpoint of center and entrance — no RNG.
    // Bee spawn does not consume loot RNG.
    let mut out = Vec::new();
    let mut occupied = Vec::new();

    // Shattered pot reported as Honeypot.ShatteredPot for identity
    let mut shattered = GeneratedItem::new("ShatteredPot", ItemCategory::Other);
    shattered.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: shattered,
        heap_type: "heap",
    });

    burn_drop_pos(room, &mut occupied);
    let mut honey = GeneratedItem::new("Honeypot", ItemCategory::Other);
    honey.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: honey,
        heap_type: "heap",
    });

    burn_drop_pos(room, &mut occupied);
    let mut bomb = bomb_random();
    bomb.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: bomb,
        heap_type: "heap",
    });

    out
}
