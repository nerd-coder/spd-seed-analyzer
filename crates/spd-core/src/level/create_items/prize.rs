//! `Mimic.generatePrize` and the golden/ebony follow-up passes.
//!
//! The carried list is Java's `items` order: the heap item passed to
//! `spawnAt`, then the base prize, then the Mimic Tooth extra, then the
//! ebony extra. Golden and ebony normalization walk that same order.

use super::generate_base_mimic_prize;
use crate::dungeon::DungeonState;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::random::Random;

#[derive(Clone, Copy)]
pub(crate) enum MimicPrize {
    Ordinary,
    Golden,
    /// `EbonyMimic.generatePrize(false)`: no seed item, decks off.
    Ebony,
}

/// Items the mimic carries after `generatePrize`.
pub(crate) fn mimic_carried_items(
    dungeon: &mut DungeonState,
    mut items: Vec<GeneratedItem>,
    use_decks: bool,
    kind: MimicPrize,
) -> Vec<GeneratedItem> {
    items.push(generate_base_mimic_prize(dungeon, use_decks));
    if crate::level::trinkets::has_mimic_tooth() {
        items.push(dungeon.generator.random_using_defaults_any(dungeon.depth));
    }
    if matches!(kind, MimicPrize::Ebony) {
        items.push(dungeon.generator.random_using_defaults_any(dungeon.depth));
    }
    match kind {
        MimicPrize::Ordinary => {}
        MimicPrize::Golden => {
            for item in &mut items {
                golden_normalize(item);
            }
        }
        MimicPrize::Ebony => {
            for item in &mut items {
                ebony_normalize(item);
            }
        }
    }
    items
}

fn gear(item: &GeneratedItem) -> bool {
    matches!(
        item.category,
        ItemCategory::Weapon
            | ItemCategory::Armor
            | ItemCategory::Missile
            | ItemCategory::Wand
            | ItemCategory::Ring
            | ItemCategory::Artifact
    )
}

/// `GoldenMimic.generatePrize`: uncursed, and a +0 non-artifact rolls +1.
fn golden_normalize(item: &mut GeneratedItem) {
    if !gear(item) {
        return;
    }
    if item.cursed && matches!(item.category, ItemCategory::Weapon | ItemCategory::Armor) {
        item.enchantment = None;
    }
    item.cursed = false;
    if item.category != ItemCategory::Artifact && item.level == 0 && Random::int_max(2) == 0 {
        item.level += 1;
    }
}

/// `EbonyMimic.generatePrize`: uncursed, and every +0 non-artifact becomes +1.
fn ebony_normalize(item: &mut GeneratedItem) {
    if !gear(item) {
        return;
    }
    if item.cursed && matches!(item.category, ItemCategory::Weapon | ItemCategory::Armor) {
        item.enchantment = None;
    }
    item.cursed = false;
    if item.category != ItemCategory::Artifact && item.level == 0 {
        item.level += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::trinkets::{self, set_held};
    use crate::run::{dungeon_from_run, init_run};
    use crate::trinkets::ActiveTrinket;
    use crate::TrinketKind;

    fn dungeon() -> DungeonState {
        let mut dungeon = dungeon_from_run(init_run(3));
        dungeon.depth = 6;
        dungeon
    }

    fn hold_tooth() {
        trinkets::reset(1);
        set_held(Some(ActiveTrinket {
            trinket: TrinketKind::MimicTooth,
            level: 3,
            instance: 0,
        }));
    }

    fn clear_tooth() {
        trinkets::reset(1);
        set_held(None);
    }

    #[test]
    fn ordinary_mimic_keeps_the_seed_item_and_the_prize() {
        clear_tooth();
        let mut dungeon = dungeon();
        let seed = GeneratedItem::new("PotionOfHealing", ItemCategory::Potion);
        Random::push_generator_seeded(4);
        let items = mimic_carried_items(&mut dungeon, vec![seed], true, MimicPrize::Ordinary);
        Random::pop_generator();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].class_name, "PotionOfHealing");
    }

    #[test]
    fn mimic_tooth_adds_one_extra_carried_item() {
        hold_tooth();
        let mut dungeon = dungeon();
        Random::push_generator_seeded(4);
        let items = mimic_carried_items(
            &mut dungeon,
            vec![GeneratedItem::new("Gold", ItemCategory::Gold)],
            true,
            MimicPrize::Ordinary,
        );
        Random::pop_generator();
        clear_tooth();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn ebony_carries_three_items_and_upgrades_plain_gear() {
        hold_tooth();
        let mut dungeon = dungeon();
        Random::push_generator_seeded(11);
        let items = mimic_carried_items(&mut dungeon, Vec::new(), false, MimicPrize::Ebony);
        Random::pop_generator();
        clear_tooth();
        assert_eq!(items.len(), 3);
        for item in items {
            if !gear(&item) || item.category == ItemCategory::Artifact {
                continue;
            }
            assert!(!item.cursed, "{}", item.class_name);
            assert!(item.level >= 1, "{}", item.class_name);
        }
    }
}
