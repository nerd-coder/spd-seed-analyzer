use super::*;

#[test]
fn depth_four_search_completes_with_minimum_size_secret_larder() {
    let result = search_seeds(&SeedSearchRequest {
        start_seed: 2_718_251_756_419,
        candidate_count: 10,
        floors: 4,
        constraints: vec![ItemConstraint {
            class_name: "RingOfWealth".into(),
            min_level: None,
            min_depth: 1,
            max_depth: 4,
        }],
        match_mode: MatchMode::Any,
        max_matches: 10,
    })
    .expect("the bounded search should complete");

    assert_eq!(result.candidates_scanned, 10);
    assert_eq!(result.next_seed, Some(2_718_251_756_429));
}

fn constraint(class_name: &str, min_depth: u32, max_depth: u32) -> ItemConstraint {
    ItemConstraint {
        class_name: class_name.to_string(),
        min_level: None,
        min_depth,
        max_depth,
    }
}

fn request(constraints: Vec<ItemConstraint>, match_mode: MatchMode) -> SeedSearchRequest {
    SeedSearchRequest {
        start_seed: 0,
        candidate_count: 2,
        floors: 1,
        constraints,
        match_mode,
        max_matches: 10,
    }
}

fn exact_floor(depth: u32, classes: &[(&str, i32)]) -> crate::FloorReport {
    crate::FloorReport {
        depth,
        feeling: None,
        builder: None,
        rooms: vec![],
        items: classes
            .iter()
            .map(|(class_name, level)| crate::report::ItemEntry {
                name: (*class_name).into(),
                class_name: Some((*class_name).into()),
                candidate_classes: Vec::new(),
                category: "other".into(),
                tier: None,
                tier_range: None,
                level: Some(*level),
                level_range: None,
                cursed: Some(false),
                prediction: ItemPredictionKind::Exact,
                conditional_notes: vec![],
                source: Some("test".into()),
            })
            .collect(),
        quests: vec![],
        map: None,
        assumed_map: None,
    }
}

#[test]
fn validation_rejects_unbounded_and_malformed_requests() {
    let mut value = request(vec![constraint("Food", 1, 1)], MatchMode::Any);
    value.start_seed = TOTAL_SEEDS;
    assert!(matches!(
        value.validate(),
        Err(SearchError::StartSeedOutOfRange)
    ));

    value.start_seed = 0;
    value.candidate_count = MAX_SEARCH_CANDIDATES + 1;
    assert!(matches!(
        value.validate(),
        Err(SearchError::CandidateCountOutOfRange)
    ));

    value.candidate_count = 1;
    value.constraints.clear();
    assert!(matches!(
        value.validate(),
        Err(SearchError::EmptyConstraints)
    ));

    value.constraints = vec![constraint(" \t ", 1, 1)];
    assert!(matches!(
        value.validate(),
        Err(SearchError::EmptyClassName { index: 0 })
    ));

    value.constraints = vec![constraint("Food", 2, 1)];
    assert!(matches!(
        value.validate(),
        Err(SearchError::InvalidDepthRange { index: 0 })
    ));

    value.constraints = vec![constraint("Food", 1, 1)];
    value.constraints[0].min_level = Some(5);
    assert!(matches!(
        value.validate(),
        Err(SearchError::InvalidLevel { index: 0 })
    ));

    value.constraints = vec![constraint("Food", 1, 1)];
    value.max_matches = 0;
    assert!(matches!(
        value.validate(),
        Err(SearchError::MaxMatchesOutOfRange)
    ));
}

#[test]
fn any_and_all_modes_use_each_constraint_independently() {
    let floor = exact_floor(3, &[("Honeypot", 0), ("ShatteredPot", 0)]);
    let any = matching_evidence(
        std::slice::from_ref(&floor),
        &[constraint("Honeypot", 3, 3), constraint("Missing", 3, 3)],
    );
    assert_eq!(any.len(), 1);
    let all = matching_evidence(
        &[floor],
        &[
            constraint("ShatteredPot", 3, 3),
            constraint("Honeypot", 3, 3),
        ],
    );
    assert_eq!(all.len(), 2);
}

#[test]
fn depth_ranges_are_inclusive_and_scoped() {
    let floor = exact_floor(3, &[("Honeypot", 0)]);
    assert_eq!(
        matching_evidence(
            std::slice::from_ref(&floor),
            &[constraint("Honeypot", 3, 3)]
        )[0]
        .depth,
        3
    );
    assert!(matching_evidence(&[floor], &[constraint("Honeypot", 2, 2)]).is_empty());
}

#[test]
fn minimum_upgrade_levels_are_optional_and_inclusive() {
    let floor = exact_floor(3, &[("Sword", 2)]);
    assert_eq!(
        matching_evidence(std::slice::from_ref(&floor), &[constraint("Sword", 3, 3)]).len(),
        1
    );
    let mut upgraded = constraint("Sword", 3, 3);
    upgraded.min_level = Some(3);
    assert!(matching_evidence(&[floor], &[upgraded]).is_empty());
}

#[test]
fn constrained_runtime_sensitive_items_never_match_exact_searches() {
    let floor = crate::FloorReport {
        depth: 13,
        feeling: None,
        builder: None,
        rooms: vec!["SacrificeRoom".into()],
        items: vec![crate::report::ItemEntry {
            name: "weapon reward".into(),
            class_name: None,
            candidate_classes: Vec::new(),
            category: "weapon".into(),
            tier: Some(3),
            tier_range: None,
            level: None,
            level_range: None,
            cursed: Some(true),
            prediction: crate::report::ItemPredictionKind::Constrained,
            conditional_notes: vec!["Parchment Scrap may alter enchantment chance.".into()],
            source: Some("SacrificeRoom".into()),
        }],
        quests: Vec::new(),
        map: None,
        assumed_map: None,
    };
    let constraints = [ItemConstraint {
        class_name: "Sword".into(),
        min_level: None,
        min_depth: 13,
        max_depth: 13,
    }];
    assert!(matching_evidence(&[floor], &constraints).is_empty());
}

#[test]
fn ordered_imp_ring_candidates_are_searchable() {
    let mut floor = exact_floor(18, &[]);
    floor.items.push(crate::report::ItemEntry {
        name: "+3 ring reward".into(),
        class_name: None,
        candidate_classes: vec!["RingOfForce".into(), "RingOfHaste".into()],
        category: "ring".into(),
        tier: None,
        tier_range: None,
        level: Some(3),
        level_range: None,
        cursed: Some(true),
        prediction: ItemPredictionKind::Constrained,
        conditional_notes: vec!["Mimic Tooth may shift the ring deck.".into()],
        source: Some("Imp.Quest".into()),
    });
    assert_eq!(
        matching_evidence(&[floor.clone()], &[constraint("RingOfHaste", 18, 18)]).len(),
        1
    );
    assert!(matching_evidence(&[floor], &[constraint("RingOfWealth", 18, 18)]).is_empty());
}

#[test]
fn later_identity_unknown_food_spawns_never_match_exact_item_searches() {
    for class_name in ["Food", "Pasty"] {
        let result = search_seeds(&SeedSearchRequest {
            start_seed: 0,
            candidate_count: 4,
            floors: 2,
            constraints: vec![constraint(class_name, 2, 2)],
            match_mode: MatchMode::Any,
            max_matches: 4,
        })
        .expect("forced queue search");
        assert!(
            result.matches.is_empty(),
            "{class_name} identity constraint leaked into exact search"
        );
    }
}

#[test]
fn floor_one_exact_food_identity_matches_exact_item_search() {
    let report = crate::analyze_seed("0", 1).expect("floor-one report");
    let class_name = report.floors[0]
        .items
        .iter()
        .find(|item| item.category == "food")
        .and_then(|item| item.class_name.clone())
        .expect("exact floor-one food class");
    let result = search_seeds(&SeedSearchRequest {
        start_seed: 0,
        candidate_count: 1,
        floors: 1,
        constraints: vec![constraint(&class_name, 1, 1)],
        match_mode: MatchMode::Any,
        max_matches: 1,
    })
    .expect("exact floor-one food search");
    assert_eq!(result.matches.len(), 1);
}

#[test]
fn guaranteed_limited_drop_spawns_match_exact_item_searches() {
    for class_name in [
        "PotionOfStrength",
        "ScrollOfUpgrade",
        "Stylus",
        "StoneOfIntuition",
        "TrinketCatalyst",
    ] {
        let result = search_seeds(&SeedSearchRequest {
            start_seed: 0,
            candidate_count: 64,
            floors: 4,
            constraints: vec![constraint(class_name, 1, 4)],
            match_mode: MatchMode::Any,
            max_matches: 1,
        })
        .expect("guaranteed limited drop search");
        assert!(
            !result.matches.is_empty(),
            "missing {class_name} spawn match"
        );
    }

    let torches = search_seeds(&SeedSearchRequest {
        start_seed: 0,
        candidate_count: 1,
        floors: 24,
        constraints: vec![constraint("Torch", 21, 24)],
        match_mode: MatchMode::Any,
        max_matches: 1,
    })
    .expect("guaranteed Torch search");
    assert_eq!(torches.matches.len(), 1);
}

#[test]
fn constrained_shop_stock_never_matches_its_internal_concrete_class() {
    let floor = crate::FloorReport {
        depth: 6,
        feeling: None,
        builder: None,
        rooms: vec!["ShopRoom".into()],
        items: vec![crate::report::ItemEntry {
            name: "weapon stock".into(),
            class_name: None,
            candidate_classes: Vec::new(),
            category: "weapon".into(),
            tier: Some(2),
            tier_range: None,
            level: Some(0),
            level_range: None,
            cursed: Some(false),
            prediction: crate::report::ItemPredictionKind::Constrained,
            conditional_notes: vec![],
            source: Some("ShopRoom".into()),
        }],
        quests: vec![],
        map: None,
        assumed_map: None,
    };
    let constraints = [ItemConstraint {
        class_name: "Quarterstaff".into(),
        min_level: None,
        min_depth: 6,
        max_depth: 6,
    }];
    assert!(matching_evidence(&[floor], &constraints).is_empty());
}

#[test]
fn real_constrained_quest_class_never_matches_exact_search() {
    use crate::items::model::{ItemProvenance, QuestRewardRole};

    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    for depth in 1..=19 {
        dungeon.depth = depth;
        let state = crate::level::create_level_partial(&mut dungeon);
        let Some(internal) = state.placed_items.iter().find(|item| {
            matches!(
                item.provenance,
                ItemProvenance::Quest(
                    QuestRewardRole::GhostWeapon { .. }
                        | QuestRewardRole::WandmakerWand
                        | QuestRewardRole::BlacksmithRoomWeapon { .. }
                        | QuestRewardRole::BlacksmithRoomMissile { .. }
                        | QuestRewardRole::ImpRing { .. }
                )
            )
        }) else {
            continue;
        };
        let constraints = [ItemConstraint {
            class_name: internal.class_name.clone(),
            min_level: None,
            min_depth: depth as u32,
            max_depth: depth as u32,
        }];
        assert!(matching_evidence(&[state.to_floor_report()], &constraints).is_empty());
        return;
    }
    panic!("expected a constrained quest reward");
}

#[test]
fn bounded_search_preserves_ascending_resume_position_without_matches() {
    let mut value = request(vec![constraint("NoSuchItemClass", 1, 1)], MatchMode::Any);
    value.max_matches = 1;

    let result = search_seeds(&value).expect("bounded search");
    assert!(result.matches.is_empty());
    assert_eq!(result.candidates_scanned, 2);
    assert_eq!(result.next_seed, Some(2));
    assert!(!result.match_limit_reached);
    assert!(!result.exhausted);
    assert_eq!(result.status, "partial");
}

#[test]
fn search_does_not_wrap_at_total_seeds() {
    let value = SeedSearchRequest {
        start_seed: TOTAL_SEEDS - 1,
        candidate_count: 2,
        floors: 1,
        constraints: vec![constraint("NoSuchItemClass", 1, 1)],
        match_mode: MatchMode::Any,
        max_matches: 1,
    };

    let result = search_seeds(&value).expect("last seed search");
    assert_eq!(result.candidates_scanned, 1);
    assert!(result.matches.is_empty());
    assert_eq!(result.next_seed, None);
    assert!(result.exhausted);
    assert!(!result.match_limit_reached);
}
