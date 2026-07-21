//! Port of `ShopRoom.generateItems` (shop stock for analysis reports).
//!
//! In SPD, stock is generated during room `setSize` (builder). We generate
//! after layout build and before other special-room paint so Generator decks
//! still advance before `createItems`. Timing is approximate vs mid-build
//! `setSize`. Bag choice assumes no hero inventory (velvet pouch already
//! dropped for all classes).

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
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
    items.push(with_shop_source(GeneratedItem::new(
        armor_class,
        ItemCategory::Armor,
    )));
    w.enchantment = None;
    w.cursed = false;
    w.level = 0;
    items.push(with_shop_source(w));
    m.enchantment = None;
    m.cursed = false;
    m.level = 0;
    items.push(with_shop_source(m));

    items.push(with_shop_source(random_tipped_dart(dungeon, 2)));

    let mut alchemize = GeneratedItem::new("Alchemize", ItemCategory::Other);
    alchemize.quantity = Random::int_range_inclusive(2, 3);
    items.push(with_shop_source(alchemize));

    if let Some(bag) = choose_bag(dungeon) {
        items.push(with_shop_source(bag));
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

    let mut rare = match Random::int_max(10) {
        0 => {
            let mut w = dungeon.generator.random_category(Category::Wand, depth);
            w.level = 0;
            w
        }
        1 => {
            let mut r = dungeon.generator.random_category(Category::Ring, depth);
            r.level = 0;
            r
        }
        2 => dungeon.generator.random_category(Category::Artifact, depth),
        _ => GeneratedItem::new("Stylus", ItemCategory::Other),
    };
    rare.cursed = false;
    items.push(with_shop_source(rare));

    for _ in 0..torches {
        items.push(with_shop_source(GeneratedItem::new(
            "Torch",
            ItemCategory::Other,
        )));
    }

    // Isolate shuffle from levelgen RNG (SPD: pushGenerator(Random.Long())).
    Random::push_generator_seeded(Random::long());
    Random::shuffle_vec(&mut items);
    Random::pop_generator();

    items
}

fn with_shop_source(mut item: GeneratedItem) -> GeneratedItem {
    item.source = Some("ShopRoom".into());
    item
}

/// Hero-less bag pick: velvet already dropped; prefer scroll holder → bandolier → holster.
fn choose_bag(dungeon: &mut DungeonState) -> Option<GeneratedItem> {
    // Without backpack contents all remaining bags score 0; pick a stable order.
    if !dungeon.limited.scroll_holder {
        dungeon.limited.scroll_holder = true;
        return Some(GeneratedItem::new("ScrollHolder", ItemCategory::Other));
    }
    if !dungeon.limited.potion_bandolier {
        dungeon.limited.potion_bandolier = true;
        return Some(GeneratedItem::new("PotionBandolier", ItemCategory::Other));
    }
    if !dungeon.limited.magical_holster {
        dungeon.limited.magical_holster = true;
        return Some(GeneratedItem::new("MagicalHolster", ItemCategory::Other));
    }
    if !dungeon.limited.velvet_pouch {
        dungeon.limited.velvet_pouch = true;
        return Some(GeneratedItem::new("VelvetPouch", ItemCategory::Other));
    }
    None
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
