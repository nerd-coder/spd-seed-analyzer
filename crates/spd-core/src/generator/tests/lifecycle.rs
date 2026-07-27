use serde::Deserialize;

use super::super::state::DeckSnapshot;
use super::super::Category;
use crate::level::create_level_partial;
use crate::run::{dungeon_from_run, init_run};

const ORACLE_JSON: &str =
    include_str!("../../../../../tools/java-oracle/fixtures/generator/aaa-aaa-aaa-lifecycle.json");
const GFX_ORACLE_JSON: &str = include_str!(
    "../../../../../tools/java-oracle/fixtures/generator/gfx-pzh-dch-lifecycle.json"
);

#[derive(Debug, Deserialize)]
struct Fixture {
    schema_version: u32,
    contract: String,
    spd: SpdPin,
    input: Input,
    boundaries: Vec<Boundary>,
}

#[derive(Debug, Deserialize)]
struct SpdPin {
    version: String,
    commit: String,
}

#[derive(Debug, Deserialize)]
struct Input {
    seed: String,
    numeric: i64,
}

#[derive(Debug, Deserialize)]
struct Boundary {
    boundary: String,
    wep_t2: ExpectedDeck,
    wep_t4: ExpectedDeck,
}

#[derive(Debug, Deserialize)]
struct ExpectedDeck {
    seed: i64,
    dropped: i32,
    probabilities: Vec<f32>,
}

impl ExpectedDeck {
    fn snapshot(&self) -> DeckSnapshot {
        DeckSnapshot {
            seed: self.seed,
            dropped: self.dropped,
            probabilities: self.probabilities.clone(),
        }
    }
}

#[test]
fn aaa_weapon_decks_match_pinned_java_lifecycle() {
    let fixture: Fixture = serde_json::from_str(ORACLE_JSON).expect("lifecycle fixture JSON");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.contract, "generator_lifecycle");
    assert_eq!(fixture.spd.version, crate::SPD_VERSION);
    assert_eq!(fixture.spd.commit, crate::SPD_COMMIT);
    assert_eq!(fixture.input.seed, "AAA-AAA-AAA");

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    for depth in 1..=6 {
        dungeon.depth = depth;
        let level = create_level_partial(&mut dungeon);
        if !matches!(depth, 2 | 3 | 4 | 6) {
            continue;
        }
        let boundary_name = match depth {
            2 => "floor_2_complete",
            3 => "floor_3_create_items_complete",
            4 => "floor_4_create_items_complete",
            6 => "floor_6_create_items_complete",
            _ => unreachable!(),
        };
        let expected = fixture
            .boundaries
            .iter()
            .find(|boundary| boundary.boundary == boundary_name)
            .expect("Java boundary");
        assert_eq!(
            dungeon.generator.deck_snapshot(Category::WepT2),
            expected.wep_t2.snapshot(),
            "{boundary_name} WEP_T2"
        );
        assert_eq!(
            dungeon.generator.deck_snapshot(Category::WepT4),
            expected.wep_t4.snapshot(),
            "{boundary_name} WEP_T4"
        );

        if depth == 3 {
            let sacrifice = level
                .placed_items
                .iter()
                .find(|item| item.source.as_deref() == Some("SacrificeRoom"))
                .expect("floor-3 SacrificeRoom prize");
            assert_eq!(sacrifice.class_name, "Spear");
            let after_sacrifice = fixture
                .boundaries
                .iter()
                .find(|boundary| boundary.boundary == "floor_3_after_sacrifice_room")
                .expect("Java post-SacrificeRoom boundary");
            assert_eq!(
                expected.wep_t2.probabilities,
                after_sacrifice.wep_t2.probabilities
            );
            assert_eq!(
                expected.wep_t4.probabilities,
                after_sacrifice.wep_t4.probabilities
            );
        }
    }
}

#[test]
fn gfx_floor_three_golden_mimic_advances_the_extra_weapon_deck() {
    let fixture: Fixture = serde_json::from_str(GFX_ORACLE_JSON).expect("lifecycle fixture JSON");
    assert_eq!(fixture.input.seed, "GFX-PZH-DCH");
    let expected = fixture
        .boundaries
        .iter()
        .find(|boundary| boundary.boundary == "floor_3_create_items_complete")
        .expect("Java floor-three boundary");

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    for depth in 1..=3 {
        dungeon.depth = depth;
        create_level_partial(&mut dungeon);
    }
    assert_eq!(
        dungeon.generator.deck_snapshot(Category::WepT2),
        expected.wep_t2.snapshot(),
        "GoldenMimic mandatory HandAxe reward"
    );
    assert_eq!(
        dungeon.generator.deck_snapshot(Category::WepT4),
        expected.wep_t4.snapshot()
    );
}
