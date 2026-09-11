use super::*;
use crate::items::model::ItemCategory;
use crate::random::Random;

#[path = "tests/lifecycle.rs"]
mod lifecycle;

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
fn artifact_draw_marks_itself_and_later_generated_items_conditional() {
    Random::reset_generators();
    Random::push_generator_seeded(1_234);
    let mut generator = GeneratorState::full_reset_ordered();

    let artifact = generator
        .random_artifact(6)
        .expect("fresh artifact deck has an item");
    assert!(artifact.artifact_conditional);
    assert!(generator.random(6).artifact_conditional);

    Random::pop_generator();
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
    assert_eq!(scroll.category, ItemCategory::Scroll);
    assert_eq!(Random::int(), before_scroll[1]);

    // Non-convertible categories do not perform that extra base-stream draw.
    let before_stone = Random::peek_ints(1);
    let stone = generator.random_category(Category::Stone, 1);
    assert_eq!(stone.category, ItemCategory::Stone);
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
    assert_eq!(potion.category, ItemCategory::Potion);
    assert_eq!(Random::int(), before[1]);

    Random::pop_generator();
}

#[test]
fn unstable_spellbook_constructor_burns_one_roll_per_selectable_scroll() {
    Random::reset_generators();
    Random::push_generator_seeded(0x5_0E11);
    let generator = GeneratorState::full_reset_ordered();
    let before = Random::peek_ints(12);

    generator.burn_unstable_spellbook_setup();

    assert_eq!(Random::int(), before[11]);
    Random::pop_generator();
}

#[test]
fn concrete_undo_drop_preserves_pinned_java_no_op_semantics() {
    Random::reset_generators();
    Random::push_generator_seeded(0xA11CE);
    let mut generator = GeneratorState::full_reset_ordered();

    let wand = generator.random_category(Category::Wand, 7);
    let state_after_draw = generator.deck_state(Category::Wand);
    generator.undo_drop(&wand.class_name);

    assert_eq!(
        generator.deck_state(Category::Wand),
        state_after_draw,
        "Generator.undoDrop(concreteClass) is a no-op at pinned SPD's inverted assignability check"
    );
    Random::pop_generator();
}

#[test]
fn exhausted_artifact_fallback_uses_ring_defaults_without_moving_ring_deck() {
    Random::reset_generators();
    Random::push_generator_seeded(0xA471_FAC7);
    let mut generator = GeneratorState::full_reset_ordered();

    let mut artifacts = 0;
    while let Some(item) = generator.random_artifact(1) {
        assert_eq!(item.category, ItemCategory::Artifact);
        artifacts += 1;
        assert!(
            artifacts <= 11,
            "fresh artifact deck has 11 positive-weight classes"
        );
    }
    assert_eq!(artifacts, 11);

    let ring_before = generator.deck_snapshot(Category::Ring);
    let fallback = generator.random_category(Category::Artifact, 1);
    assert_eq!(fallback.category, ItemCategory::Ring);
    assert_eq!(generator.deck_snapshot(Category::Ring), ring_before);
    assert_eq!(
        generator.deck_dropped(Category::Ring),
        ring_before.dropped,
        "v4 random(ARTIFACT) miss uses randomUsingDefaults(RING)"
    );

    let ring = generator.random_category(Category::Ring, 1);
    assert_eq!(ring.category, ItemCategory::Ring);
    assert_eq!(
        generator.deck_dropped(Category::Ring),
        ring_before.dropped + 1,
        "deck-backed random(RING) still advances RING.dropped"
    );
    Random::pop_generator();
}

#[test]
fn full_reset_clones_wep_t3_default_probs() {
    Random::reset_generators();
    Random::push_generator_seeded(0x7E93);
    let generator = GeneratorState::full_reset_ordered();
    let snapshot = generator.deck_snapshot(Category::WepT3);
    assert_eq!(snapshot.probabilities.len(), 6);
    assert_eq!(snapshot.probabilities, vec![2., 2., 2., 2., 2., 2.]);
    Random::pop_generator();
}
