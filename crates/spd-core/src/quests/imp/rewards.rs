//! `Imp.Quest.spawn` rewardOptions (six take-one items).
//!
//! Java draws via `Generator.random` / `randomArtifact` (which run `item.random()`),
//! then overwrites enchant/glyph and level. Match that order; do not skip the
//! inner randomize.

use crate::generator::{Category, GeneratorState};
use crate::items::enchants;
use crate::items::model::{GeneratedItem, ItemCategory, ItemProvenance, QuestRewardRole};
use crate::random::Random;

pub(super) fn generate_reward_options(
    generator: &mut GeneratorState,
    depth: i32,
) -> Vec<GeneratedItem> {
    let mut options = Vec::with_capacity(6);

    let artif = match generator.random_artifact(depth) {
        Some(mut artif) => {
            // identify(false) is a no-op for analyzer identity.
            artif.level = transfer_upgrade_level(&artif.class_name, 5);
            artif
        }
        None => {
            let mut ring = generator.random_category(Category::Ring, depth);
            ring.level = Random::int_range_inclusive(2, 4);
            ring
        }
    };
    let artif_class = artif.class_name.clone();
    options.push(artif);

    let mut ring;
    loop {
        ring = generator.random_category(Category::Ring, depth);
        if ring.class_name != artif_class {
            break;
        }
    }
    ring.level = Random::int_range_inclusive(2, 4);
    options.push(ring);

    if Random::int_max(2) == 0 {
        options.push(overwrite_weapon(generator, Category::WepT5, depth, 2, 4));
        options.push(overwrite_weapon(generator, Category::MisT4, depth, 3, 5));
    } else {
        options.push(overwrite_weapon(generator, Category::MisT5, depth, 2, 4));
        options.push(overwrite_weapon(generator, Category::WepT4, depth, 3, 5));
    }

    options.push(overwrite_plate());

    let mut wand = generator.random_category(Category::Wand, depth);
    wand.level = Random::int_range_inclusive(2, 4);
    options.push(wand);

    for (slot, item) in options.iter_mut().enumerate() {
        item.cursed = false;
        item.source = Some("Imp.Quest".into());
        item.provenance =
            ItemProvenance::Quest(QuestRewardRole::ImpVaultOption { slot: slot as u8 });
    }
    options
}

fn overwrite_weapon(
    generator: &mut GeneratorState,
    cat: Category,
    depth: i32,
    level_min: i32,
    level_max: i32,
) -> GeneratedItem {
    let mut item = generator.random_category(cat, depth);
    // Weapon.enchant() ignores the class already stored by Weapon.random().
    let ignore = item.enchantment.clone();
    item.enchantment = Some(enchants::random_weapon_enchant(ignore.as_deref()).to_string());
    item.level = Random::int_range_inclusive(level_min, level_max);
    item
}

fn overwrite_plate() -> GeneratedItem {
    // `new PlateArmor()` — no ARMOR deck, no Armor.random().
    let mut item = GeneratedItem::new("PlateArmor", ItemCategory::Armor);
    item.enchantment = Some(enchants::random_armor_glyph(None).to_string());
    item.level = Random::int_range_inclusive(2, 4);
    item
}

pub(super) fn transfer_upgrade_level(class_name: &str, transfer_lvl: i32) -> i32 {
    // Artifact.transferUpgrade: upgrade(round(transferLvl * levelCap / 10f)).
    // Fresh artifacts are level 0, so the stored level is that rounded value.
    let cap = artifact_level_cap(class_name);
    ((transfer_lvl * cap) as f32 / 10.0).round() as i32
}

fn artifact_level_cap(class_name: &str) -> i32 {
    match class_name {
        "EtherealChains" | "TimekeepersHourglass" => 5,
        "SandalsOfNature" => 3,
        _ => 10,
    }
}
