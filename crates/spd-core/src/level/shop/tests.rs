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

#[test]
fn generated_stock_is_role_tagged_before_shuffle_and_tags_are_internal() {
    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    dungeon.depth = 6;
    Random::push_generator_seeded(1234);
    let stock = generate_items(&mut dungeon);
    Random::reset_generators();

    assert!(stock
        .iter()
        .all(|item| matches!(item.provenance, ItemProvenance::Shop(_))));
    assert_eq!(
        stock
            .iter()
            .filter(|item| matches!(
                item.provenance,
                ItemProvenance::Shop(ShopStockRole::DeckWeapon { tier: 2 })
            ))
            .count(),
        1
    );
    assert_eq!(
        stock
            .iter()
            .filter(|item| matches!(
                item.provenance,
                ItemProvenance::Shop(ShopStockRole::DeckMissile { tier: 2 })
            ))
            .count(),
        1
    );
    let json = serde_json::to_string(&stock).expect("serialize exact internal stock");
    assert!(!json.contains("provenance"));
    assert!(!json.contains("DeckWeapon"));
}

#[test]
fn artifact_rare_branch_redacts_layout_and_post_callback_floor_tail() {
    let state = (0..100)
        .find_map(|seed| {
            let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(seed));
            let mut floor = None;
            for depth in 1..=6 {
                dungeon.depth = depth;
                floor = Some(crate::level::create_level_partial(&mut dungeon));
            }
            floor.filter(|state| state.runtime_sensitive_layout)
        })
        .expect("an early seed selects the shop artifact-or-ring branch");
    let boundary = state
        .runtime_sensitive_placed_items_from
        .expect("artifact branch boundary");
    assert_eq!(
        boundary, 0,
        "inherited taint must remain earlier than the artifact callback"
    );
    assert!(
        boundary < state.placed_items.len(),
        "real floor has post-shop generated item facts"
    );
    assert!(state.map.is_some(), "internal exact map is retained");

    let public = state.to_floor_report();
    assert!(public.map.is_none());
    assert!(public.builder.is_none());
    assert!(public.rooms.is_empty());
    let quest_boundary = state
        .runtime_sensitive_quests_from
        .expect("artifact branch quest boundary");
    assert_eq!(quest_boundary, 0);
    assert_eq!(
        public.quests,
        state.quests[..quest_boundary],
        "only pre-callback quest selections remain safe"
    );

    let mut safe_prefix = state.clone();
    safe_prefix.placed_items.truncate(boundary);
    safe_prefix.runtime_sensitive_placed_items_from = None;
    safe_prefix.quests.truncate(quest_boundary);
    safe_prefix.runtime_sensitive_quests_from = None;
    safe_prefix.runtime_sensitive_layout = true;
    safe_prefix.map = None;
    let expected = safe_prefix.to_floor_report();
    assert_eq!(
        serde_json::to_value(&public.items).expect("serialize guarded public items"),
        serde_json::to_value(&expected.items).expect("serialize pre-boundary item projection"),
        "no post-shop item fact crosses the public boundary"
    );
    let json = serde_json::to_string(&public).expect("serialize guarded floor");
    assert!(!json.contains("for_sale"));

    let mut altered_hidden_layout = state.clone();
    altered_hidden_layout.builder = Some(crate::rooms::init_rooms::BuilderKind::FigureEight);
    altered_hidden_layout.rooms = vec!["SampledRuntimeRoom".into()];
    altered_hidden_layout.room_public_facts =
        vec![
            crate::level::room_public::RoomPublicFact::new("ArmoryRoom", state.depth)
                .expect("static room contract"),
        ];
    assert_eq!(
        serde_json::to_value(&public).expect("serialize public floor"),
        serde_json::to_value(altered_hidden_layout.to_floor_report())
            .expect("serialize altered hidden layout"),
        "pre-build runtime-sensitive layout metadata must not affect public output"
    );
}

#[test]
fn public_shop_sequence_is_independent_of_internal_shuffle_order() {
    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    let mut floor = None;
    for depth in 1..=6 {
        dungeon.depth = depth;
        floor = Some(crate::level::create_level_partial(&mut dungeon));
    }
    let original = floor.expect("floor 6 state");
    let mut reordered = original.clone();
    let positions: Vec<_> = reordered
        .placed_items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            matches!(item.provenance, ItemProvenance::Shop(_)).then_some(index)
        })
        .collect();
    let reversed: Vec<_> = positions
        .iter()
        .rev()
        .map(|&index| reordered.placed_items[index].clone())
        .collect();
    for (&index, item) in positions.iter().zip(reversed) {
        reordered.placed_items[index] = item;
    }

    let public_shop = |state: &crate::level::LevelState| {
        state
            .to_floor_report()
            .items
            .into_iter()
            .filter(|entry| entry.source.as_deref() == Some("ShopRoom"))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        serde_json::to_value(public_shop(&original)).expect("serialize original public shop"),
        serde_json::to_value(public_shop(&reordered)).expect("serialize reordered public shop")
    );
}
