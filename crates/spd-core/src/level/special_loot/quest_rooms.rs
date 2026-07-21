//! Quest-related room prizes (wandmaker, blacksmith).

use super::placement::{burn_drop_pos, burn_drop_pos_margin};
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::random::Random;
use crate::rooms::room::Room;

/// `MassGraveRoom.paint` loot (approx placement; skeleton mobs skipped).
pub(super) fn mass_grave_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Barricade entrance → PotionOfLiquidFlame into itemsToSpawn
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLiquidFlame",
        ItemCategory::Potion,
    ));

    // 50% 1 skeleton, 50% 2 — placement burns room.random() (terrain check approx)
    let n_skel = Random::int_max(2); // 0 or 1 → loop i=0..=n_skel → 1 or 2
    let mut occupied = Vec::new();
    for _ in 0..=n_skel {
        burn_drop_pos(room, &mut occupied);
    }

    let mut out = Vec::new();
    // 100% corpse dust, 2x gold(1), 2x30% gold, 1x60% random, 1x30% armor
    let mut items: Vec<GeneratedItem> = Vec::new();
    items.push(GeneratedItem::new("CorpseDust", ItemCategory::Other));
    {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        g.quantity = 1;
        items.push(g);
    }
    {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        g.quantity = 1;
        items.push(g);
    }
    if Random::float() <= 0.3 {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        items.push(g);
    }
    if Random::float() <= 0.3 {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        items.push(g);
    }
    if Random::float() <= 0.6 {
        items.push(dungeon.generator.random(dungeon.depth));
    }
    if Random::float() <= 0.3 {
        items.push(
            dungeon
                .generator
                .random_armor(dungeon.depth / 5, dungeon.depth),
        );
    }

    for mut item in items {
        burn_drop_pos(room, &mut occupied);
        // Haunted-if-cursed: no extra RNG for analysis
        item.source = Some("MassGraveRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "skeleton",
        });
    }
    out
}

/// `RitualSiteRoom.paint` — four ceremonial candles into itemsToSpawn.
pub(super) fn ritual_site_setup(items_to_spawn: &mut Vec<GeneratedItem>) -> Vec<PlacedLoot> {
    for _ in 0..4 {
        items_to_spawn.push(GeneratedItem::new("CeremonialCandle", ItemCategory::Other));
    }
    Vec::new()
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

/// `BlacksmithRoom.paint` — two random armor/weapon/missile drops + NPC/exit placement RNG.
pub(super) fn blacksmith_room_prizes(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..2 {
        // drop on EMPTY_SP: room.random() until free
        burn_drop_pos(room, &mut occupied);
        let cat = *Random::one_of(&[Category::Armor, Category::Weapon, Category::Missile]);
        let mut item = dungeon.generator.random_category(cat, dungeon.depth);
        item.source = Some("BlacksmithRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    // NPC pos: random(2) until free of heaps
    burn_drop_pos_margin(room, 2, &mut occupied);
    // entrancePos: random(2) until free of heaps and NPC
    burn_drop_pos_margin(room, 2, &mut occupied);
    out
}
