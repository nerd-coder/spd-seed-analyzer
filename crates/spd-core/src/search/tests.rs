use super::*;

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
    let any = search_seeds(&request(
        vec![
            constraint("Food", 1, 1),
            constraint("PotionOfStrength", 1, 1),
        ],
        MatchMode::Any,
    ))
    .expect("ANY search");
    assert_eq!(
        any.matches
            .iter()
            .map(|found| found.seed.numeric)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert_eq!(any.matches[0].evidence[0].class_name, "Food");
    assert_eq!(any.matches[1].evidence[0].class_name, "PotionOfStrength");

    let all = search_seeds(&request(
        vec![
            constraint("Pasty", 1, 1),
            constraint("PotionOfStrength", 1, 1),
        ],
        MatchMode::All,
    ))
    .expect("ALL search");
    assert_eq!(all.matches.len(), 1);
    assert_eq!(all.matches[0].seed.numeric, 1);
    assert_eq!(all.matches[0].evidence.len(), 2);
}

#[test]
fn depth_ranges_are_inclusive_and_scoped() {
    let mut value = request(vec![constraint("PotionOfStrength", 2, 2)], MatchMode::All);
    value.candidate_count = 1;
    value.floors = 2;
    let depth_two = search_seeds(&value).expect("depth two search");
    assert_eq!(depth_two.matches[0].evidence[0].depth, 2);

    value.constraints[0] = constraint("PotionOfStrength", 1, 1);
    assert!(search_seeds(&value)
        .expect("depth one search")
        .matches
        .is_empty());
}

#[test]
fn minimum_upgrade_levels_are_optional_and_inclusive() {
    let mut value = request(vec![constraint("PotionOfStrength", 1, 1)], MatchMode::All);
    value.start_seed = 1;
    assert!(!search_seeds(&value)
        .expect("unrestricted upgrade search")
        .matches
        .is_empty());

    value.constraints[0].min_level = Some(1);
    assert!(search_seeds(&value)
        .expect("upgraded search")
        .matches
        .is_empty());
}

#[test]
fn result_limit_preserves_ascending_resume_position() {
    let mut value = request(
        vec![constraint("Food", 1, 1), constraint("Pasty", 1, 1)],
        MatchMode::Any,
    );
    value.max_matches = 1;

    let result = search_seeds(&value).expect("bounded search");
    assert_eq!(result.matches[0].seed.numeric, 0);
    assert_eq!(result.candidates_scanned, 1);
    assert_eq!(result.next_seed, Some(1));
    assert!(result.match_limit_reached);
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
