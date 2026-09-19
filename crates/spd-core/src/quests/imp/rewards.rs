//! `Imp.Quest.spawn` rewardOptions (six take-one items).
//!
//! Java draws via `Generator.random` / `randomArtifact` (which run `item.random()`),
//! then overwrites enchant/glyph and level. Match that order; do not skip the
//! inner randomize.

use crate::generator::{Category, GeneratorState};
use crate::items::enchants;
use crate::items::model::{GeneratedItem, ItemCategory, ItemProvenance, QuestRewardRole};
use crate::random::Random;

/// Which draw site filled a slot, and the deck index it consumed. Captured at
/// draw time because the ring slot's index depends on whether slot 1 fell back
/// to the RING deck and on how many duplicate rejects it burned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImpSlotDraw {
    pub kind: ImpSlotKind,
    /// `None` for the plate armor slot, which uses no deck.
    pub category: Option<Category>,
    /// Deck index this slot drew, or `-1` when it used no deck.
    pub deck_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpSlotKind {
    Artifact,
    Ring,
    Weapon,
    Missile,
    Armor,
    Wand,
}

pub(super) fn generate_reward_options(
    generator: &mut GeneratorState,
    depth: i32,
) -> (Vec<GeneratedItem>, Vec<ImpSlotDraw>) {
    let mut options = Vec::with_capacity(6);
    let mut draws: Vec<ImpSlotDraw> = Vec::with_capacity(6);

    let artifact_index = generator.deck_dropped(Category::Artifact);
    let ring_index_slot0 = generator.deck_dropped(Category::Ring);
    let artif = match generator.random_artifact(depth) {
        Some(mut artif) => {
            // identify(false) is a no-op for analyzer identity.
            artif.level = transfer_upgrade_level(&artif.class_name, 5);
            draws.push(ImpSlotDraw {
                kind: ImpSlotKind::Artifact,
                category: Some(Category::Artifact),
                deck_index: artifact_index,
            });
            artif
        }
        None => {
            let mut ring = generator.random_category(Category::Ring, depth);
            ring.level = Random::int_range_inclusive(2, 4);
            // Artifact deck exhausted: this slot is a RING-deck draw.
            draws.push(ImpSlotDraw {
                kind: ImpSlotKind::Ring,
                category: Some(Category::Ring),
                deck_index: ring_index_slot0,
            });
            ring
        }
    };
    let artif_class = artif.class_name.clone();
    options.push(artif);

    let mut ring;
    let mut ring_index;
    loop {
        ring_index = generator.deck_dropped(Category::Ring);
        ring = generator.random_category(Category::Ring, depth);
        if ring.class_name != artif_class {
            break;
        }
    }
    ring.level = Random::int_range_inclusive(2, 4);
    draws.push(ImpSlotDraw {
        kind: ImpSlotKind::Ring,
        category: Some(Category::Ring),
        deck_index: ring_index,
    });
    options.push(ring);

    if Random::int_max(2) == 0 {
        push_weapon(
            generator,
            &mut options,
            &mut draws,
            Category::WepT5,
            ImpSlotKind::Weapon,
            depth,
            2,
            4,
        );
        push_weapon(
            generator,
            &mut options,
            &mut draws,
            Category::MisT4,
            ImpSlotKind::Missile,
            depth,
            3,
            5,
        );
    } else {
        push_weapon(
            generator,
            &mut options,
            &mut draws,
            Category::MisT5,
            ImpSlotKind::Missile,
            depth,
            2,
            4,
        );
        push_weapon(
            generator,
            &mut options,
            &mut draws,
            Category::WepT4,
            ImpSlotKind::Weapon,
            depth,
            3,
            5,
        );
    }

    options.push(overwrite_plate());
    draws.push(ImpSlotDraw {
        kind: ImpSlotKind::Armor,
        category: None,
        deck_index: -1,
    });

    let wand_index = generator.deck_dropped(Category::Wand);
    let mut wand = generator.random_category(Category::Wand, depth);
    wand.level = Random::int_range_inclusive(2, 4);
    draws.push(ImpSlotDraw {
        kind: ImpSlotKind::Wand,
        category: Some(Category::Wand),
        deck_index: wand_index,
    });
    options.push(wand);

    for (slot, item) in options.iter_mut().enumerate() {
        item.cursed = false;
        item.source = Some("Imp.Quest".into());
        item.provenance =
            ItemProvenance::Quest(QuestRewardRole::ImpVaultOption { slot: slot as u8 });
    }
    (options, draws)
}

#[allow(clippy::too_many_arguments)]
fn push_weapon(
    generator: &mut GeneratorState,
    options: &mut Vec<GeneratedItem>,
    draws: &mut Vec<ImpSlotDraw>,
    cat: Category,
    kind: ImpSlotKind,
    depth: i32,
    level_min: i32,
    level_max: i32,
) {
    let deck_index = generator.deck_dropped(cat);
    options.push(overwrite_weapon(
        generator, cat, depth, level_min, level_max,
    ));
    draws.push(ImpSlotDraw {
        kind,
        category: Some(cat),
        deck_index,
    });
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
