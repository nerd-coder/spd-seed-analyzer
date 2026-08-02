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
        include_baseline: false,
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
        include_baseline: false,
        max_matches: 10,
    }
}

fn exact_floor(depth: u32, classes: &[(&str, i32)]) -> crate::FloorReport {
    crate::FloorReport {
        depth,
        feeling: None,
        builder: None,
        rooms: vec![],
        possible_rooms: vec![],
        guaranteed_appearances: vec![],
        initial_encounters: vec![],
        items: classes
            .iter()
            .map(|(class_name, level)| {
                crate::report::ItemEntry {
                    name: (*class_name).into(),
                    quantity: 1,
                    class_name: Some((*class_name).into()),
                    candidate_classes: Vec::new(),
                    category: "other".into(),
                    tier: None,
                    tier_range: None,
                    level: Some(*level),
                    level_range: None,
                    cursed: Some(false),
                    enchantment: None,
                    prediction: ItemPredictionKind::Exact,
                    spawn_conditions: Vec::new(),
                    conditions: Vec::new(),
                    notes: vec![],
                    source: Some("test".into()),
                }
                .into()
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
fn baseline_search_opt_in_defaults_to_false_when_omitted() {
    let request: SeedSearchRequest = serde_json::from_value(serde_json::json!({
        "startSeed": 0,
        "candidateCount": 1,
        "floors": 1,
        "constraints": [{
            "className": "Food",
            "minDepth": 1,
            "maxDepth": 1
        }],
        "matchMode": "any",
        "maxMatches": 1
    }))
    .expect("request without includeBaseline");

    assert!(!request.include_baseline);
}

#[test]
fn any_and_all_modes_use_each_constraint_independently() {
    let floor = exact_floor(3, &[("Honeypot", 0), ("ShatteredPot", 0)]);
    let any = matching_evidence(
        std::slice::from_ref(&floor),
        &[constraint("Honeypot", 3, 3), constraint("Missing", 3, 3)],
        false,
    );
    assert_eq!(any.len(), 1);
    let all = matching_evidence(
        &[floor],
        &[
            constraint("ShatteredPot", 3, 3),
            constraint("Honeypot", 3, 3),
        ],
        false,
    );
    assert_eq!(all.len(), 2);
}

#[test]
fn depth_ranges_are_inclusive_and_scoped() {
    let floor = exact_floor(3, &[("Honeypot", 0)]);
    assert_eq!(
        matching_evidence(
            std::slice::from_ref(&floor),
            &[constraint("Honeypot", 3, 3)],
            false,
        )[0]
        .depth,
        3
    );
    assert!(matching_evidence(&[floor], &[constraint("Honeypot", 2, 2)], false).is_empty());
}

#[test]
fn minimum_upgrade_levels_are_optional_and_inclusive() {
    let floor = exact_floor(3, &[("Sword", 2)]);
    assert_eq!(
        matching_evidence(
            std::slice::from_ref(&floor),
            &[constraint("Sword", 3, 3)],
            false
        )
        .len(),
        1
    );
    let mut upgraded = constraint("Sword", 3, 3);
    upgraded.min_level = Some(3);
    assert!(matching_evidence(&[floor], &[upgraded], false).is_empty());
}

#[test]
fn constrained_runtime_sensitive_items_never_match_exact_searches() {
    let floor = crate::FloorReport {
        depth: 13,
        feeling: None,
        builder: None,
        rooms: vec!["SacrificeRoom".into()],
        possible_rooms: vec![],
        guaranteed_appearances: vec![],
        initial_encounters: vec![],
        items: vec![crate::report::ItemEntry {
            name: "weapon reward".into(),
            quantity: 1,
            class_name: None,
            candidate_classes: Vec::new(),
            category: "weapon".into(),
            tier: Some(3),
            tier_range: None,
            level: None,
            level_range: None,
            cursed: Some(true),
            enchantment: None,
            prediction: crate::report::ItemPredictionKind::Constrained,
            spawn_conditions: Vec::new(),
            conditions: Vec::new(),
            notes: vec!["Parchment Scrap may alter enchantment chance.".into()],
            source: Some("SacrificeRoom".into()),
        }
        .into()],
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
    assert!(matching_evidence(&[floor], &constraints, false).is_empty());
}

#[test]
fn category_only_imp_ring_never_matches_an_exact_ring_search() {
    let mut floor = exact_floor(18, &[]);
    floor.items.push(
        crate::report::ItemEntry {
            name: "ring reward".into(),
            quantity: 1,
            class_name: None,
            candidate_classes: Vec::new(),
            category: "ring".into(),
            tier: None,
            tier_range: None,
            level: None,
            level_range: Some(crate::report::NumericRange { min: 2, max: 4 }),
            cursed: Some(true),
            enchantment: None,
            prediction: ItemPredictionKind::Constrained,
            spawn_conditions: Vec::new(),
            conditions: Vec::new(),
            notes: vec!["Quest completion is required.".into()],
            source: Some("Imp.Quest".into()),
        }
        .into(),
    );
    assert!(matching_evidence(
        &[floor.clone()],
        &[constraint("RingOfHaste", 18, 18)],
        false
    )
    .is_empty());
    assert!(matching_evidence(&[floor], &[constraint("RingOfWealth", 18, 18)], false).is_empty());
}

#[test]
fn conditional_floor_loot_candidates_keep_levels_for_search() {
    let mut floor = exact_floor(8, &[]);
    floor.items.push(
        crate::report::ItemEntry {
            name: "long sword +2".into(),
            quantity: 1,
            class_name: None,
            candidate_classes: vec!["Longsword".into()],
            category: "weapon".into(),
            tier: None,
            tier_range: None,
            level: Some(2),
            level_range: None,
            cursed: Some(false),
            enchantment: None,
            prediction: ItemPredictionKind::Constrained,
            spawn_conditions: Vec::new(),
            conditions: Vec::new(),
            notes: vec!["Assumes no external artifact acquisition".into()],
            source: Some("heap".into()),
        }
        .into(),
    );
    let mut wanted = constraint("Longsword", 8, 8);
    wanted.min_level = Some(2);

    let evidence = matching_evidence(&[floor], &[wanted], false);
    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].level, 2);
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
            include_baseline: false,
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
        include_baseline: false,
        max_matches: 1,
    })
    .expect("exact floor-one food search");
    assert_eq!(result.matches.len(), 1);
}

#[test]
fn finder_baseline_highlights_exclude_promoted_floor_one_rewards() {
    let numeric = crate::parse_seed("RZN-LKU-EFS")
        .expect("fixture seed")
        .numeric;
    let result = search_seeds(&SeedSearchRequest {
        start_seed: numeric,
        candidate_count: 1,
        floors: 17,
        constraints: vec![constraint("Food", 1, 1)],
        match_mode: MatchMode::Any,
        include_baseline: false,
        max_matches: 1,
    })
    .expect("baseline-highlight finder result");

    let matched = &result.matches[0];
    assert_eq!(matched.evidence.len(), 1);
    assert_eq!(matched.evidence[0].class_name, "Food");
    assert!(matched.baseline_items.iter().all(|item| {
        item.depth != 1 || item.item.class_name.as_deref() != Some("RingOfWealth")
    }));
    assert!(matched.baseline_items.iter().all(|item| {
        item.item.prediction == ItemPredictionKind::Baseline
            && item.item.class_name.as_deref() != Some("Food")
    }));
}

#[test]
fn sacrifice_sickle_requires_baseline_opt_in_and_is_labeled() {
    let numeric = crate::parse_seed("PUB-CLI-VNW")
        .expect("fixture seed")
        .numeric;
    let mut wanted = constraint("Sickle", 4, 4);
    wanted.min_level = Some(2);
    let mut request = SeedSearchRequest {
        start_seed: numeric,
        candidate_count: 1,
        floors: 4,
        constraints: vec![wanted],
        match_mode: MatchMode::All,
        include_baseline: false,
        max_matches: 1,
    };

    let guaranteed_only = search_seeds(&request).expect("guaranteed-only search");
    assert!(guaranteed_only.matches.is_empty());

    request.include_baseline = true;
    let with_baseline = search_seeds(&request).expect("baseline-inclusive search");
    let evidence = &with_baseline.matches[0].evidence[0];
    assert_eq!(evidence.class_name, "Sickle");
    assert_eq!(evidence.depth, 4);
    assert_eq!(evidence.level, 2);
    assert_eq!(evidence.prediction, ItemPredictionKind::Baseline);
    assert_eq!(evidence.source.as_deref(), Some("SacrificeRoom"));
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
            include_baseline: false,
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
        include_baseline: false,
        max_matches: 1,
    })
    .expect("guaranteed Torch search");
    assert_eq!(torches.matches.len(), 1);
}

#[test]
fn puzzle_solution_potions_match_exact_item_searches() {
    for (seed, class_name, room_source, evidence_source) in [
        (25, "PotionOfHaste", "SentryRoom", "SentryRoom"),
        (53, "PotionOfPurity", "ToxicGasRoom", "RingRoom"),
        (99_162_322, "PotionOfLevitation", "TrapsRoom", "TrapsRoom"),
        (
            8_687_205_886,
            "PotionOfFrost",
            "MagicalFireRoom",
            "MagicalFireRoom",
        ),
    ] {
        let report = crate::analyze_seed(&seed.to_string(), 1).expect("puzzle solution report");
        assert!(report.floors[0].items.iter().any(|item| {
            item.class_name.as_deref() == Some(class_name)
                && item.source.as_deref() == Some(room_source)
        }));

        let result = search_seeds(&SeedSearchRequest {
            start_seed: seed,
            candidate_count: 1,
            floors: 1,
            constraints: vec![constraint(class_name, 1, 1)],
            match_mode: MatchMode::Any,
            include_baseline: false,
            max_matches: 1,
        })
        .expect("puzzle solution search");

        let evidence = &result.matches[0].evidence[0];
        assert_eq!(evidence.class_name, class_name);
        assert_eq!(evidence.depth, 1);
        assert_eq!(evidence.level, 0);
        // Seed 53 also relocates another guaranteed Purity potion into a
        // RingRoom, which is the first exact copy selected as search evidence.
        assert_eq!(evidence.source.as_deref(), Some(evidence_source));
    }
}

#[test]
fn secret_honeypot_fixed_items_match_exact_search_evidence() {
    let floor = exact_floor(12, &[("ShatteredPot", 0), ("Honeypot", 0)]);
    let constraints = [
        constraint("ShatteredPot", 12, 12),
        constraint("Honeypot", 12, 12),
        constraint("Bomb", 12, 12),
    ];

    let evidence = matching_evidence(&[floor], &constraints, false);
    assert_eq!(evidence.len(), 2);
    assert_eq!(evidence[0].class_name, "ShatteredPot");
    assert_eq!(evidence[1].class_name, "Honeypot");
}

#[test]
fn constrained_shop_stock_never_matches_its_internal_concrete_class() {
    let floor = crate::FloorReport {
        depth: 6,
        feeling: None,
        builder: None,
        rooms: vec!["ShopRoom".into()],
        possible_rooms: vec![],
        guaranteed_appearances: vec![],
        initial_encounters: vec![],
        items: vec![crate::report::ItemEntry {
            name: "weapon stock".into(),
            quantity: 1,
            class_name: None,
            candidate_classes: Vec::new(),
            category: "weapon".into(),
            tier: Some(2),
            tier_range: None,
            level: Some(0),
            level_range: None,
            cursed: Some(false),
            enchantment: None,
            prediction: crate::report::ItemPredictionKind::Constrained,
            spawn_conditions: Vec::new(),
            conditions: Vec::new(),
            notes: vec![],
            source: Some("ShopRoom".into()),
        }
        .into()],
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
    assert!(matching_evidence(&[floor], &constraints, false).is_empty());
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
                    QuestRewardRole::WandmakerWand
                        | QuestRewardRole::BlacksmithRoomWeapon { .. }
                        | QuestRewardRole::BlacksmithRoomMissile { .. }
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
        assert!(matching_evidence(&[state.to_floor_report()], &constraints, false).is_empty());
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
        include_baseline: false,
        max_matches: 1,
    };

    let result = search_seeds(&value).expect("last seed search");
    assert_eq!(result.candidates_scanned, 1);
    assert!(result.matches.is_empty());
    assert_eq!(result.next_seed, None);
    assert!(result.exhausted);
    assert!(!result.match_limit_reached);
}
