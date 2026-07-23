use serde::Deserialize;

use super::Category;
use crate::random::Random;
use crate::run::init_run;

const ORACLE_JSON: &str = include_str!(
    "../../../../../tools/java-oracle/fixtures/generator/aaa-aaa-aaa-food-rollover.json"
);

#[derive(Debug, Deserialize)]
struct OracleFixture {
    schema_version: u32,
    contract: String,
    spd: SpdPin,
    input: OracleInput,
    category: String,
    initial_weight: i32,
    draws: Vec<OracleDraw>,
}

#[derive(Debug, Deserialize)]
struct SpdPin {
    version: String,
    commit: String,
}

#[derive(Debug, Deserialize)]
struct OracleInput {
    seed: String,
    numeric: i64,
    base_stream_seed: i64,
}

#[derive(Debug, Deserialize)]
struct OracleDraw {
    draw: i32,
    #[serde(rename = "class")]
    class_name: String,
    dropped: i32,
    remaining_weight: i32,
    private_rng: [i32; 2],
    base_rng: [i32; 2],
}

#[test]
fn food_deck_rollover_matches_pinned_java_sequence_and_rng_states() {
    let fixture: OracleFixture = serde_json::from_str(ORACLE_JSON).expect("rollover fixture JSON");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.contract, "generator_deck_rollover");
    assert_eq!(fixture.spd.version, crate::SPD_VERSION);
    assert_eq!(fixture.spd.commit, crate::SPD_COMMIT);
    assert_eq!(fixture.input.seed, "AAA-AAA-AAA");
    assert_eq!(fixture.category, "FOOD");
    assert_eq!(fixture.initial_weight, 5);

    let mut generator = init_run(fixture.input.numeric).generator;
    Random::push_generator_seeded(fixture.input.base_stream_seed);

    for expected in &fixture.draws {
        assert_eq!(
            Random::stack_len(),
            2,
            "stack before draw {}",
            expected.draw
        );
        let mutations_before = Random::stack_mutations();
        let item = generator.random_category(Category::Food, 1);
        assert_eq!(
            item.class_name, expected.class_name,
            "draw {}",
            expected.draw
        );
        assert_eq!(Random::stack_len(), 2, "stack after draw {}", expected.draw);
        let mutations_after = Random::stack_mutations();
        assert_eq!(
            (
                mutations_after.0 - mutations_before.0,
                mutations_after.1 - mutations_before.1,
            ),
            (1, 1),
            "Java keeps one private generator frame across draw {}",
            expected.draw
        );

        let (seed, dropped, remaining_weight) = generator.deck_state(Category::Food);
        assert_eq!(
            dropped, expected.dropped,
            "dropped after draw {}",
            expected.draw
        );
        assert_eq!(
            remaining_weight as i32, expected.remaining_weight,
            "remaining weight after draw {}",
            expected.draw
        );

        Random::push_generator_seeded(seed);
        for _ in 0..dropped {
            Random::long();
        }
        let private_rng = [Random::int(), Random::int()];
        Random::pop_generator();
        assert_eq!(
            private_rng, expected.private_rng,
            "private deck state after draw {}",
            expected.draw
        );

        let base_rng = [Random::int(), Random::int()];
        assert_eq!(
            base_rng, expected.base_rng,
            "base stream after draw {}",
            expected.draw
        );
    }

    assert_eq!(fixture.draws[4].remaining_weight, 0);
    assert_eq!(
        fixture.draws[5].remaining_weight,
        fixture.initial_weight - 1
    );
    Random::pop_generator();
    assert_eq!(Random::stack_len(), 1);
}
