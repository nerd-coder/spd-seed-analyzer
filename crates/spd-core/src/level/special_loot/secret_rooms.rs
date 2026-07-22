//! Secret room prize painters.

use super::placement::{burn_drop_pos, find_prize_item};
use super::special_rooms::laboratory_prizes_shared;
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::random::Random;
use crate::rooms::room::Room;

pub(super) fn secret_library(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Approximate: 2-3 scrolls
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                dungeon
                    .generator
                    .random_category(Category::Scroll, dungeon.depth)
            });
        item.source = Some("SecretLibraryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn secret_runestone(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // SecretRunestoneRoom.java:64 — pushed before the stone drops (zero-RNG append)
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLiquidFlame",
        ItemCategory::Potion,
    ));
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                dungeon
                    .generator
                    .random_category(Category::Stone, dungeon.depth)
            });
        item.source = Some("SecretRunestoneRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn secret_artillery(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = dungeon
            .generator
            .random_missile(dungeon.depth / 5, false, dungeon.depth);
        item.source = Some("SecretArtilleryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn secret_laboratory(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Java SecretLaboratoryRoom extends SecretRoom with its own paint() that
    // never calls addItemToSpawn — shared lab body without the IronKey push.
    laboratory_prizes_shared(dungeon, room, items_to_spawn)
        .into_iter()
        .map(|mut p| {
            p.item.source = Some("SecretLaboratoryRoom".into());
            p
        })
        .collect()
}

pub(super) fn secret_larder(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = dungeon
            .generator
            .random_category(Category::Food, dungeon.depth);
        item.source = Some("SecretLarderRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

pub(super) fn secret_hoard(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    // Approximate: gold piles (blacklisted from report — still burn RNG)
    let n = Random::int_range_inclusive(3, 5);
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        g.source = Some("SecretHoardRoom".into());
        let _ = g;
    }
    Vec::new()
}
