use super::*;

#[test]
fn tile_variance_is_deterministic_and_does_not_consume_level_rng() {
    Random::reset_generators();
    Random::push_generator_seeded(77);
    let first = Random::int();
    let variance = map_facts::tile_variance(8, 1234);
    let after_variance = Random::int();
    Random::pop_generator();

    Random::reset_generators();
    Random::push_generator_seeded(77);
    assert_eq!(Random::int(), first);
    assert_eq!(Random::int(), after_variance);
    Random::pop_generator();

    assert_eq!(variance, map_facts::tile_variance(8, 1234));
    assert!(variance.iter().all(|&value| value < 100));
}
