use serde::Deserialize;

use super::*;

const ORACLE_JSON: &str =
    include_str!("../../../../../tools/java-oracle/fixtures/shop/aaa-aaa-aaa-shop-bags.json");

#[derive(Debug, Deserialize)]
struct OracleFixture {
    schema_version: u32,
    contract: String,
    spd: SpdPin,
    scenarios: Vec<OracleScenario>,
}

#[derive(Debug, Deserialize)]
struct SpdPin {
    version: String,
    commit: String,
}

#[derive(Debug, Deserialize)]
struct OracleScenario {
    name: String,
    depth: i32,
    main_backpack: Vec<String>,
    scores: Vec<OracleScore>,
    selected: String,
}

#[derive(Debug, Deserialize)]
struct OracleScore {
    bag: String,
    score: usize,
}

fn affinity(class_name: &str) -> BagAffinity {
    match class_name {
        "Waterskin" | "PotionOfHealing" | "PotionOfStrength" | "PotionOfMindVision" => {
            BagAffinity::PotionBandolier
        }
        "ScrollOfIdentify" | "ScrollOfRemoveCurse" | "ScrollOfMagicMapping" => {
            BagAffinity::ScrollHolder
        }
        "ThrowingStone" => BagAffinity::MagicalHolster,
        "Food" | "VelvetPouch" => BagAffinity::None,
        other => panic!("unmapped oracle backpack item {other}"),
    }
}

#[test]
fn later_shop_bag_scoring_matches_pinned_java_unique_winners() {
    let fixture: OracleFixture = serde_json::from_str(ORACLE_JSON).expect("shop bag fixture JSON");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.contract, "shop_bag_selection");
    assert_eq!(fixture.spd.version, crate::SPD_VERSION);
    assert_eq!(fixture.spd.commit, crate::SPD_COMMIT);

    for scenario in fixture.scenarios {
        assert_eq!(
            scenario.depth, 11,
            "{} is a later-shop profile",
            scenario.name
        );
        let inventory = HeroInventory {
            main_backpack: scenario
                .main_backpack
                .iter()
                .map(|class_name| affinity(class_name))
                .collect(),
        };
        let mut limited = LimitedDrops::reset();
        limited.magical_holster = true;

        let selected = choose_bag_kind(&mut limited, &inventory).expect("available shop bag");
        assert_eq!(
            selected.class_name(),
            scenario.selected,
            "{}",
            scenario.name
        );

        let best_score = scenario
            .scores
            .iter()
            .map(|score| score.score)
            .max()
            .expect("oracle scores");
        assert_eq!(
            scenario
                .scores
                .iter()
                .filter(|score| score.score == best_score)
                .count(),
            1,
            "{} deliberately avoids Java HashMap tie behavior",
            scenario.name
        );
        assert_eq!(
            scenario
                .scores
                .iter()
                .find(|score| score.bag == scenario.selected)
                .map(|score| score.score),
            Some(best_score),
            "{} selected the unique maximum",
            scenario.name
        );
    }
}

#[test]
fn fresh_warrior_shop_progression_uses_inventory_scores() {
    let inventory = HeroInventory::fresh_warrior();
    let mut limited = LimitedDrops::reset();

    assert_eq!(
        choose_bag_kind(&mut limited, &inventory),
        Some(BagKind::MagicalHolster),
        "the stable tie fallback retains the committed floor-6 observation"
    );
    assert_eq!(
        choose_bag_kind(&mut limited, &inventory),
        Some(BagKind::PotionBandolier),
        "Waterskin uniquely selects the bandolier at the later shop"
    );
    assert_eq!(
        choose_bag_kind(&mut limited, &inventory),
        Some(BagKind::ScrollHolder)
    );
    assert_eq!(choose_bag_kind(&mut limited, &inventory), None);
}
