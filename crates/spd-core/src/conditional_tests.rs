use super::*;

fn selection(options: &[&str], sequence: &[&str]) -> TrinketSelectionReport {
    TrinketSelectionReport {
        catalyst_depth: 1,
        first_alchemy_pot_depth: 1,
        first_alchemy_pot_is_secret: false,
        selection_depth: 1,
        first_effective_depth: 2,
        catalyst_options: options.iter().map(|value| (*value).into()).collect(),
        transmutation_sequence: sequence.iter().map(|value| (*value).into()).collect(),
    }
}

#[test]
fn upgrade_timelines_include_delayed_upgrades() {
    let timelines = nondecreasing_levels(2, 4);
    assert!(timelines.contains(&vec![0, 0, 3]));
    assert!(timelines.contains(&vec![1, 2, 3]));
    assert_eq!(timelines.len(), 20);
}

#[test]
fn direct_routes_use_only_actual_supported_offers() {
    let routes = direct_acquisitions(&selection(
        &["MossyClump", "RatSkull", "SaltCube", "MimicTooth"],
        &[],
    ));
    assert_eq!(routes.len(), 2);
    assert!(matches!(
        routes[0].events[0].action,
        TrinketEventAction::Acquired {
            trinket: TrinketKind::MossyClump,
            ..
        }
    ));
    assert!(matches!(
        routes[1].events[0].action,
        TrinketEventAction::Acquired {
            trinket: TrinketKind::MimicTooth,
            ..
        }
    ));
}

#[test]
fn transmutations_consume_exact_scrolls_in_order() {
    let selection = selection(
        &["SaltCube", "RatSkull", "CrackedSpyglass", "EyeOfNewt"],
        &["MossyClump", "TrapMechanism"],
    );
    assert!(transmuted_acquisitions(&selection, &[], 4).is_empty());

    let one_scroll = transmuted_acquisitions(&selection, &[1], 4);
    assert_eq!(one_scroll.len(), 1);
    assert_eq!(one_scroll[0].effective_depth, 2);
    assert!(matches!(
        one_scroll[0].events.last().unwrap().action,
        TrinketEventAction::Transmuted {
            trinket: TrinketKind::MossyClump
        }
    ));

    let two_scrolls = transmuted_acquisitions(&selection, &[1, 3], 4);
    assert_eq!(two_scrolls.len(), 2);
    assert_eq!(two_scrolls[1].effective_depth, 4);
    assert!(matches!(
        two_scrolls[1].events.last().unwrap().action,
        TrinketEventAction::Transmuted {
            trinket: TrinketKind::TrapMechanism
        }
    ));
}

#[test]
fn profile_conditions_are_structured_dependency_axes() {
    let profile = MapProfile {
        challenges: vec![Challenge::ForbiddenRunes],
        trinket_events: vec![TrinketEvent {
            before_depth: 2,
            action: TrinketEventAction::Acquired {
                trinket: TrinketKind::MossyClump,
                min_upgrades: None,
            },
        }],
        ..MapProfile::default()
    };
    let condition = condition_for_profile(&profile);
    assert_eq!(condition.all_of.len(), 2);
    assert!(matches!(
        condition.all_of[0],
        ItemDependencyCondition::Challenge {
            challenge: Challenge::ForbiddenRunes,
            enabled: true
        }
    ));
    assert!(matches!(
        condition.all_of[1],
        ItemDependencyCondition::Trinket { .. }
    ));
}
