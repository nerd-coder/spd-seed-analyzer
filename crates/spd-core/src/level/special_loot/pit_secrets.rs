//! Pit, secret maze/summoning/chest-chasm prizes.

use super::special_rooms::is_curse_enchant;
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::create_items::PlacedLoot;
use crate::random::Random;

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
