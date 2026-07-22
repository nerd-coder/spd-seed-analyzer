use super::*;
use crate::random::Random;

#[test]
fn full_reset_and_random_deterministic() {
    Random::reset_generators();
    Random::push_generator_seeded(999);
    let mut a = GeneratorState::full_reset_ordered();
    let items_a: Vec<_> = (0..20).map(|_| a.random(1).class_name).collect();
    Random::pop_generator();

    Random::reset_generators();
    Random::push_generator_seeded(999);
    let mut b = GeneratorState::full_reset_ordered();
    let items_b: Vec<_> = (0..20).map(|_| b.random(1).class_name).collect();
    Random::pop_generator();

    assert_eq!(items_a, items_b);
}

#[test]
fn potion_deck_never_strength_from_random() {
    // Strength has weight 0 in deck
    Random::reset_generators();
    Random::push_generator_seeded(1);
    let mut gen = GeneratorState::full_reset_ordered();
    for _ in 0..50 {
        let item = gen.random_category(Category::Potion, 1);
        assert_ne!(item.class_name, "PotionOfStrength");
    }
    Random::pop_generator();
}

#[test]
fn consumable_decks_advance_level_stream_for_exotic_conversion_check() {
    Random::reset_generators();
    Random::push_generator_seeded(314_159);
    let mut generator = GeneratorState::full_reset_ordered();

    // The class draw uses the scroll category's private seeded generator.
    // Java then evaluates ExoticScroll.regToExo's Float check on the restored
    // level stream, even when ExoticCrystals makes conversion impossible.
    let before_scroll = Random::peek_ints(2);
    let scroll = generator.random_category(Category::Scroll, 1);
    assert_eq!(scroll.category, crate::items::model::ItemCategory::Scroll);
    assert_eq!(Random::int(), before_scroll[1]);

    // Non-convertible categories do not perform that extra base-stream draw.
    let before_stone = Random::peek_ints(1);
    let stone = generator.random_category(Category::Stone, 1);
    assert_eq!(stone.category, crate::items::model::ItemCategory::Stone);
    assert_eq!(Random::int(), before_stone[0]);

    Random::pop_generator();
}

#[test]
fn cumulative_default_consumable_selection_skips_exotic_conversion_draw() {
    Random::reset_generators();
    Random::push_generator_seeded(271_828);
    let mut generator = GeneratorState::full_reset_ordered();

    // Potion/Scroll use Java's cumulative `defaultProbsTotal` table. That
    // branch returns immediately after the class draw, before the regular
    // default path's exotic-conversion check.
    let before = Random::peek_ints(2);
    let potion = generator.random_using_defaults(Category::Potion, 1);
    assert_eq!(potion.category, crate::items::model::ItemCategory::Potion);
    assert_eq!(Random::int(), before[1]);

    Random::pop_generator();
}
