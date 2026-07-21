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
