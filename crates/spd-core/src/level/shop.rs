//! Port of `ShopRoom.generateItems` (shop stock for analysis reports).
//!
//! Stock is generated lazily when builder placement first asks for the room's
//! minimum size, matching pinned `ShopRoom.spacesNeeded`. The committed
//! fresh-Warrior floor-6 oracle carries both ThrowingStone and Waterskin; its
//! observed equal-score HashMap winner is MagicalHolster.

use crate::dungeon::{BagAffinity, DungeonState, HeroInventory, LimitedDrops};
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory, ItemProvenance, ShopStockRole};
use crate::random::Random;

/// Generate FOR_SALE shop inventory for the current depth.
pub fn generate_items(dungeon: &mut DungeonState) -> Vec<GeneratedItem> {
    let mut items: Vec<GeneratedItem> = Vec::new();
    let depth = dungeon.depth;

    let (wep_tier, mis_tier, armor_class, torches) = match depth {
        11 => (Category::WepT3, Category::MisT3, "MailArmor", 0),
        16 => (Category::WepT4, Category::MisT4, "ScaleArmor", 0),
        20 | 21 => (Category::WepT5, Category::MisT5, "PlateArmor", 3),
        // depth 6 (and default shop depths)
        _ => (Category::WepT2, Category::MisT2, "LeatherArmor", 0),
    };

    // Match Java generateItems: roll w/m first, then push armor, then cleared w/m.
    let mut w = dungeon.generator.random_category(wep_tier, depth);
    let mut m = dungeon.generator.random_category(mis_tier, depth);
    items.push(with_shop_role(
        GeneratedItem::new(armor_class, ItemCategory::Armor),
        ShopStockRole::Fixed,
    ));
    w.enchantment = None;
    w.cursed = false;
    w.level = 0;
    items.push(with_shop_role(
        w,
        ShopStockRole::DeckWeapon {
            tier: weapon_tier(wep_tier),
        },
    ));
    m.enchantment = None;
    m.cursed = false;
    m.level = 0;
    items.push(with_shop_role(
        m,
        ShopStockRole::DeckMissile {
            tier: weapon_tier(mis_tier),
        },
    ));

    items.push(with_shop_role(
        random_tipped_dart(dungeon, 2),
        ShopStockRole::Fixed,
    ));

    let mut alchemize = GeneratedItem::new("Alchemize", ItemCategory::Other);
    alchemize.quantity = Random::int_range_inclusive(2, 3);
    items.push(with_shop_role(alchemize, ShopStockRole::Fixed));

    if let Some(bag) = choose_bag(dungeon) {
        items.push(with_shop_role(bag, ShopStockRole::ChooseBag));
    }

    items.push(with_shop_source(GeneratedItem::new(
        "PotionOfHealing",
        ItemCategory::Potion,
    )));
    items.push(with_shop_source(
        dungeon
            .generator
            .random_using_defaults(Category::Potion, depth),
    ));
    items.push(with_shop_source(
        dungeon
            .generator
            .random_using_defaults(Category::Potion, depth),
    ));

    items.push(with_shop_source(GeneratedItem::new(
        "ScrollOfIdentify",
        ItemCategory::Scroll,
    )));
    items.push(with_shop_source(GeneratedItem::new(
        "ScrollOfRemoveCurse",
        ItemCategory::Scroll,
    )));
    items.push(with_shop_source(GeneratedItem::new(
        "ScrollOfMagicMapping",
        ItemCategory::Scroll,
    )));

    for _ in 0..2 {
        if Random::int_max(2) == 0 {
            items.push(with_shop_source(
                dungeon
                    .generator
                    .random_using_defaults(Category::Potion, depth),
            ));
        } else {
            items.push(with_shop_source(
                dungeon
                    .generator
                    .random_using_defaults(Category::Scroll, depth),
            ));
        }
    }

    items.push(with_shop_source(GeneratedItem::new(
        "SmallRation",
        ItemCategory::Food,
    )));
    items.push(with_shop_source(GeneratedItem::new(
        "SmallRation",
        ItemCategory::Food,
    )));

    match Random::int_max(4) {
        0 => items.push(with_shop_source(GeneratedItem::new(
            "Bomb",
            ItemCategory::Other,
        ))),
        1 | 2 => items.push(with_shop_source(GeneratedItem::new(
            "DoubleBomb",
            ItemCategory::Other,
        ))),
        _ => items.push(with_shop_source(GeneratedItem::new(
            "Honeypot",
            ItemCategory::Other,
        ))),
    }

    items.push(with_shop_source(GeneratedItem::new(
        "Ankh",
        ItemCategory::Other,
    )));
    items.push(with_shop_source(GeneratedItem::new(
        "StoneOfAugmentation",
        ItemCategory::Stone,
    )));

    // No TimekeepersHourglass without a hero — skip sand bags (0 RNG).

    let (mut rare, rare_role) = match Random::int_max(10) {
        0 => {
            let mut w = dungeon.generator.random_category(Category::Wand, depth);
            w.level = 0;
            (w, ShopStockRole::DeckRareWand)
        }
        1 => {
            let mut r = dungeon.generator.random_category(Category::Ring, depth);
            r.level = 0;
            (r, ShopStockRole::DeckRareRing)
        }
        2 => (
            dungeon.generator.random_category(Category::Artifact, depth),
            ShopStockRole::DeckRareArtifactOrRing,
        ),
        _ => (
            GeneratedItem::new("Stylus", ItemCategory::Other),
            ShopStockRole::Fixed,
        ),
    };
    rare.cursed = false;
    items.push(with_shop_role(rare, rare_role));

    for _ in 0..torches {
        items.push(with_shop_source(GeneratedItem::new(
            "Torch",
            ItemCategory::Other,
        )));
    }

    // Isolate shuffle from levelgen RNG (SPD: pushGenerator(Random.Long())).
    Random::push_generator_seeded(Random::long());
    Random::shuffle_list(&mut items);
    Random::pop_generator();

    items
}

fn with_shop_source(item: GeneratedItem) -> GeneratedItem {
    with_shop_role(item, ShopStockRole::Fixed)
}

fn with_shop_role(mut item: GeneratedItem, role: ShopStockRole) -> GeneratedItem {
    item.source = Some("ShopRoom".into());
    item.provenance = ItemProvenance::Shop(role);
    item
}

fn weapon_tier(category: Category) -> i32 {
    match category {
        Category::WepT2 | Category::MisT2 => 2,
        Category::WepT3 | Category::MisT3 => 3,
        Category::WepT4 | Category::MisT4 => 4,
        Category::WepT5 | Category::MisT5 => 5,
        _ => unreachable!("shop equipment category must have a tier"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BagKind {
    VelvetPouch,
    ScrollHolder,
    PotionBandolier,
    MagicalHolster,
}

impl BagKind {
    fn class_name(self) -> &'static str {
        match self {
            Self::VelvetPouch => "VelvetPouch",
            Self::ScrollHolder => "ScrollHolder",
            Self::PotionBandolier => "PotionBandolier",
            Self::MagicalHolster => "MagicalHolster",
        }
    }

    fn affinity(self) -> BagAffinity {
        match self {
            Self::VelvetPouch => BagAffinity::VelvetPouch,
            Self::ScrollHolder => BagAffinity::ScrollHolder,
            Self::PotionBandolier => BagAffinity::PotionBandolier,
            Self::MagicalHolster => BagAffinity::MagicalHolster,
        }
    }
}

/// Pinned `ShopRoom.ChooseBag` scoring over direct backpack items.
fn choose_bag(dungeon: &mut DungeonState) -> Option<GeneratedItem> {
    let bag = choose_bag_kind(&mut dungeon.limited, &dungeon.hero_inventory)?;
    Some(GeneratedItem::new(bag.class_name(), ItemCategory::Other))
}

fn choose_bag_kind(limited: &mut LimitedDrops, inventory: &HeroInventory) -> Option<BagKind> {
    // Java stores candidates in a HashMap and only replaces the best bag on a
    // strictly greater score. Identity-hash iteration makes equal-score ties
    // non-portable, so keep the observed fresh-Warrior floor-6 winner as the
    // stable fallback. Unique winners still match source exactly.
    let candidates = [
        BagKind::MagicalHolster,
        BagKind::ScrollHolder,
        BagKind::PotionBandolier,
        BagKind::VelvetPouch,
    ];
    let mut best: Option<(BagKind, usize)> = None;
    for bag in candidates {
        if bag_was_dropped(limited, bag) {
            continue;
        }
        let base_score = usize::from(bag == BagKind::VelvetPouch);
        let score = base_score
            + inventory
                .main_backpack
                .iter()
                .filter(|affinity| **affinity == bag.affinity())
                .count();
        if best.is_none_or(|(_, best_score)| score > best_score) {
            best = Some((bag, score));
        }
    }

    let (bag, _) = best?;
    mark_bag_dropped(limited, bag);
    Some(bag)
}

fn bag_was_dropped(limited: &LimitedDrops, bag: BagKind) -> bool {
    match bag {
        BagKind::VelvetPouch => limited.velvet_pouch,
        BagKind::ScrollHolder => limited.scroll_holder,
        BagKind::PotionBandolier => limited.potion_bandolier,
        BagKind::MagicalHolster => limited.magical_holster,
    }
}

fn mark_bag_dropped(limited: &mut LimitedDrops, bag: BagKind) {
    match bag {
        BagKind::VelvetPouch => limited.velvet_pouch = true,
        BagKind::ScrollHolder => limited.scroll_holder = true,
        BagKind::PotionBandolier => limited.potion_bandolier = true,
        BagKind::MagicalHolster => limited.magical_holster = true,
    }
}

fn random_tipped_dart(dungeon: &mut DungeonState, quantity: i32) -> GeneratedItem {
    // TippedDart.randomTipped: randomUsingDefaults(SEED) until mapped (all non-rotberry are).
    let seed = loop {
        let s = dungeon
            .generator
            .random_using_defaults(Category::Seed, dungeon.depth);
        if seed_has_tipped_dart(&s.class_name) {
            break s;
        }
    };
    let mut dart = GeneratedItem::new(
        tipped_dart_for_seed(&seed.class_name),
        ItemCategory::Missile,
    );
    dart.quantity = quantity;
    dart
}

fn seed_has_tipped_dart(class_name: &str) -> bool {
    !matches!(class_name, "RotberrySeed")
}

fn tipped_dart_for_seed(seed: &str) -> &'static str {
    match seed {
        "RotberrySeed" => "RotDart",
        "SungrassSeed" => "HealingDart",
        "FadeleafSeed" => "DisplacingDart",
        "IcecapSeed" => "ChillingDart",
        "FirebloomSeed" => "IncendiaryDart",
        "SorrowmossSeed" => "PoisonDart",
        "SwiftthistleSeed" => "AdrenalineDart",
        "BlindweedSeed" => "BlindingDart",
        "StormvineSeed" => "ShockingDart",
        "EarthrootSeed" => "ParalyticDart",
        "MageroyalSeed" => "CleansingDart",
        "StarflowerSeed" => "HolyDart",
        _ => "HealingDart",
    }
}

#[cfg(test)]
#[path = "shop/tests.rs"]
mod tests;
